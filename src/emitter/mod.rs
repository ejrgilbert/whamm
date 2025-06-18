#![allow(clippy::too_many_arguments)]
mod locals_tracker;
pub mod memory_allocator;
pub mod module_emitter;
pub mod rewriting;
#[cfg(test)]
pub mod tests;
pub mod utils;

use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::rewriting::rules::Arg;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::report_vars::{Metadata, ReportVars};
use crate::parser::types::{Block, Expr, Statement};
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
    fn reset_locals_for_probe(&mut self);
    fn reset_locals_for_function(&mut self);
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
    var_handler: &mut UnsharedVarHandler,
    report_vars: &mut ReportVars,
    map_lib_adapter: &mut MapLibAdapter,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    err: &mut ErrorGen,
) -> Option<u32> {
    // at this point, I want to use the collected metadata in UnsharedVars
    // to generate a new data segment AND generate the necessary globals!
    if report_vars.variable_metadata.is_empty() && report_vars.all_used_report_dts.is_empty() {
        return None;
    }

    let mut flush_reports = FunctionBuilder::new(&[], &[]);

    // call the report_vars to emit calls to all report var flushers
    let (header_addr, header_len) = Metadata::setup_csv_header(wasm, mem_allocator);
    let var_meta = report_vars.setup_flush_data_segments(wasm, mem_allocator);

    let trackers = var_handler.setup_module(wasm);
    report_vars.configure_trackers(trackers);
    report_vars.emit_flush_logic(
        &mut flush_reports,
        &var_meta,
        mem_allocator,
        io_adapter,
        map_lib_adapter,
        (header_addr, header_len),
        var_handler.get_mem_id(),
        wasm,
        err,
    );

    let on_exit = flush_reports.finish_module(wasm, None);
    wasm.set_fn_name(on_exit, "flush_reports".to_string());

    Some(*on_exit)
}
