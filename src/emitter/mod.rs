pub mod module_emitter;
pub mod rewriting;
#[cfg(test)]
pub mod tests;
pub mod utils;

use crate::common::error::ErrorGen;
use crate::emitter::rewriting::rules::Arg;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::report_vars::{Metadata, ReportVars};
use crate::parser::types::{Block, Expr, Statement};
use crate::verifier::types::{Record, SymbolTable};
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
    io_adapter: &mut IOAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) {
    if report_vars.variable_metadata.is_empty() && report_vars.map_metadata.is_empty() {
        return;
    }

    //convert the metadata into strings, add those to the data section, then use those to populate the maps
    let var_meta: HashMap<u32, String> = report_vars
        .variable_metadata
        .iter()
        .map(|(key, value)| (*key, value.to_csv()))
        .collect();
    let map_meta: HashMap<u32, String> = report_vars
        .map_metadata
        .iter()
        .map(|(key, value)| (*key, value.to_csv()))
        .collect();

    setup_print_global_meta(&var_meta, wasm, table, io_adapter, err_msg, err);
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
    var_meta_str: &HashMap<u32, String>,
    wasm: &mut Module,
    table: &mut SymbolTable,
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
    let header = Metadata::get_csv_header();
    io_adapter.putsln(header.clone(), &mut print_global_meta, err);

    // for each of the report globals, emit the printing logic
    for (key, val) in var_meta_str.iter() {
        io_adapter.puts(format!("i32, {key}, {val}, "), &mut print_global_meta, err);

        // get the value of this report global
        print_global_meta.global_get(GlobalID(*key));
        io_adapter.call_puti(&mut print_global_meta, err);
        io_adapter.putln(&mut print_global_meta, err);
    }
    true
}

fn setup_print_map_meta(
    map_meta_str: &HashMap<u32, String>,
    wasm: &mut Module,
    table: &mut SymbolTable,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    // get the function
    //first, we need to create the maps in global_map_init - where all the other maps are initialized
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
    for (key, val) in map_meta_str.iter() {
        io_adapter.puts(format!("map, {key}, {val}, "), &mut print_map_meta, err);

        // print the value(s) of this map
        map_lib_adapter.print_map(*key, &mut print_map_meta, err);
        io_adapter.putln(&mut print_map_meta, err);
    }
    true
}
