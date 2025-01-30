#![allow(clippy::too_many_arguments)]
pub mod memory_allocator;
pub mod module_emitter;
pub mod rewriting;
#[cfg(test)]
pub mod tests;
pub mod utils;

use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::rewriting::rules::Arg;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::report_vars::{Metadata, ReportVars};
use crate::parser::types::{Block, DataType, Expr, Statement};
use crate::verifier::types::{Record, SymbolTable};
use itertools::Itertools;
use orca_wasm::ir::id::{FunctionID, GlobalID};
use orca_wasm::{Module, Opcode};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum InjectStrategy {
    Wizard,
    Rewriting,
}

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn emit_body(&mut self, curr_instr_args: &[Arg], body: &mut Block, err: &mut ErrorGen) -> bool;
    fn emit_stmt(
        &mut self,
        curr_instr_args: &[Arg],
        stmt: &mut Statement,
        err: &mut ErrorGen,
    ) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool;
}

/// This should run AFTER emitting the full AST
pub fn configure_flush_routines(
    wasm: &mut Module,
    table: &mut SymbolTable,
    report_vars: &mut ReportVars,
    map_lib_adapter: &mut MapLibAdapter,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) {
    if report_vars.variable_metadata.is_empty() && report_vars.map_metadata.is_empty() {
        return;
    }

    //convert the metadata into strings, add those to the data section, then use those to populate the maps
    let var_meta: HashMap<u32, (DataType, (u32, u32))> = report_vars
        .variable_metadata
        .iter()
        .map(|(key, (_, value))| {
            let mut s = format!("{key}, {}, ", value.to_csv());
            mem_allocator.emit_string(wasm, &mut s);
            let addr = mem_allocator.emitted_strings.get(&s).unwrap();

            (
                *key,
                (
                    value.get_whamm_ty(),
                    (addr.mem_offset as u32, addr.len as u32),
                ),
            )
        })
        .collect();
    let map_meta: HashMap<u32, (u32, u32)> = report_vars
        .map_metadata
        .iter()
        .map(|(key, value)| {
            let mut s = format!("map, map_id, {key}, {}, ", value.to_csv());
            mem_allocator.emit_string(wasm, &mut s);
            let addr = mem_allocator.emitted_strings.get(&s).unwrap();

            (*key, (addr.mem_offset as u32, addr.len as u32))
        })
        .collect();

    setup_print_global_meta(
        &var_meta,
        wasm,
        table,
        mem_allocator,
        io_adapter,
        err_msg,
        err,
    );
    if map_lib_adapter.is_used {
        setup_print_map_meta(
            &map_meta,
            wasm,
            table,
            io_adapter,
            map_lib_adapter,
            err_msg,
            err,
        );
    }
}

/// set up the print_global_meta function for insertions
fn setup_print_global_meta(
    var_meta_str: &HashMap<u32, (DataType, (u32, u32))>,
    wasm: &mut Module,
    table: &mut SymbolTable,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    // get the function
    // todo(maps) -- look up the func name instead!
    let print_global_meta_id = if let Some(Record::Fn { addr: Some(id), .. }) =
        table.lookup_fn("print_global_meta", true, err)
    {
        *id
    } else {
        return false;
    };
    let print_global_meta_id = FunctionID(print_global_meta_id);

    // prepare the CSV header data segment
    let (header_addr, header_len) = Metadata::setup_csv_header(wasm, mem_allocator);

    let mut print_global_meta = match wasm.functions.get_fn_modifier(print_global_meta_id) {
        Some(func) => func,
        None => {
            err.unexpected_error(
                true,
                Some(format!(
                    "{err_msg} \
                    No 'print_global_meta' function found in the module!"
                )),
                None,
            );
            return false;
        }
    };

    // output the header data segment
    io_adapter.putsln(header_addr, header_len, &mut print_global_meta, err);

    // for each of the report globals, emit the printing logic
    let sorted_metadata = var_meta_str.iter().sorted_by_key(|data| data.0);
    for (key, (whamm_ty, (str_addr, str_len))) in sorted_metadata.into_iter() {
        io_adapter.puts(*str_addr, *str_len, &mut print_global_meta, err);

        // get the value of this report global
        print_global_meta.global_get(GlobalID(*key));
        match whamm_ty {
            DataType::U8 => io_adapter.call_putu8(&mut print_global_meta, err),
            DataType::I8 => io_adapter.call_puti8(&mut print_global_meta, err),
            DataType::U16 => io_adapter.call_putu16(&mut print_global_meta, err),
            DataType::I16 => io_adapter.call_puti16(&mut print_global_meta, err),
            DataType::I32 | DataType::Boolean => {
                io_adapter.call_puti32(&mut print_global_meta, err)
            }
            // special case for unsigned integers (so the print is correctly signed)
            DataType::U32 => io_adapter.call_putu32(&mut print_global_meta, err),
            DataType::I64 => io_adapter.call_puti64(&mut print_global_meta, err),
            DataType::U64 => io_adapter.call_putu64(&mut print_global_meta, err),
            DataType::F32 => io_adapter.call_putf32(&mut print_global_meta, err),
            DataType::F64 => io_adapter.call_putf64(&mut print_global_meta, err),
            other => unimplemented!("printing for this type has not been implemented: {}", other),
        }
        io_adapter.putln(&mut print_global_meta, err);
    }

    true
}

fn setup_print_map_meta(
    map_meta_str: &HashMap<u32, (u32, u32)>,
    wasm: &mut Module,
    table: &mut SymbolTable,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    // get the function
    //first, we need to create the maps in instr_init - where all the other maps are initialized
    // todo(maps) -- look up the func name instead!
    let print_map_meta_id = if let Some(Record::Fn { addr: Some(id), .. }) =
        table.lookup_fn("print_map_meta", true, err)
    {
        *id
    } else {
        return false;
    };
    let print_map_meta_id = FunctionID(print_map_meta_id);

    let mut print_map_meta = match wasm.functions.get_fn_modifier(print_map_meta_id) {
        Some(func) => func,
        None => {
            err.unexpected_error(
                true,
                Some(format!(
                    "{err_msg} \
                    No 'print_map_meta' function found in the module!"
                )),
                None,
            );
            return false;
        }
    };

    // for each of the report maps, emit the printing logic
    let sorted_metadata = map_meta_str.iter().sorted_by_key(|data| data.0);
    for (key, (str_addr, str_len)) in sorted_metadata.into_iter() {
        io_adapter.puts(*str_addr, *str_len, &mut print_map_meta, err);

        // print the value(s) of this map
        map_lib_adapter.print_map(*key, &mut print_map_meta, err);
        io_adapter.putln(&mut print_map_meta, err);
    }
    true
}
