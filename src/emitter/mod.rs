pub mod module_emitter;
pub mod report_var_metadata;
pub mod rewriting;
#[cfg(test)]
pub mod tests;
pub mod utils;

use crate::common::error::ErrorGen;
use crate::emitter::rewriting::rules::Arg;
use crate::parser::types::{Block, Expr, Statement};

#[derive(Copy, Clone)]
pub enum InjectStrategy {
    Wizard,
    Rewriting
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
