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
use crate::parser::types::{Block, Expr, Statement};
use log::debug;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::Module;

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
    report_vars: &mut ReportVars,
    map_lib_adapter: &mut MapLibAdapter,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    err: &mut ErrorGen,
) -> Option<u32> {
    if report_vars.variable_metadata.is_empty() {
        return None;
    }

    if wasm
        .functions
        .get_local_fid_by_name("print_global_meta")
        .is_some()
    {
        debug!("print_global_meta function already exists");
        err.add_error(ErrorGen::get_unexpected_error(
            true,
            Some(
                "print_global_meta function already exists - needs to be created by Whamm"
                    .to_string(),
            ),
            None,
        ));
        return None;
    }

    debug!("Creating the print_global_meta function");
    let mut print_global_meta_fn = FunctionBuilder::new(&[], &[]);

    //convert the metadata into strings, add those to the data section, then use those to populate the maps
    let var_meta = report_vars.get_var_metadata(wasm, mem_allocator);

    // prepare the CSV header data segment
    let (header_addr, header_len) = Metadata::setup_csv_header(wasm, mem_allocator);

    // output the header data segment
    io_adapter.putsln(header_addr, header_len, &mut print_global_meta_fn, err);

    report_vars.emit_globals_flush(
        &mut print_global_meta_fn,
        &var_meta,
        io_adapter,
        map_lib_adapter,
        err,
    );

    let print_global_meta_id = print_global_meta_fn.finish_module(wasm);
    wasm.set_fn_name(print_global_meta_id, "print_global_meta".to_string());

    Some(*print_global_meta_id)
}
