pub mod map_lib_adapter;
pub mod report_var_metadata;
pub mod rewriting;
#[cfg(test)]
pub mod tests;

use crate::common::error::WhammError;
use crate::emitter::rewriting::rules::{Arg, LocInfo, WhammProvider};
use crate::parser::types::{Block, DataType, Expr, Fn, ProbeSpec, Statement, Value};

use orca_wasm::ir::types::DataType as OrcaType;
use wasmparser::Operator;

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait ModuleEmitter {
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn exit_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn reset_children(&mut self);
    fn save_args(&mut self, args: &[Arg]) -> bool;
    fn emit_args(&mut self) -> Result<bool, Box<WhammError>>;
    fn define(&mut self, var_name: &str, var_rec: &Option<Value>) -> Result<bool, Box<WhammError>>;
    fn reset_table_data(&mut self, loc_info: &LocInfo);
    fn fold_expr(&mut self, expr: &mut Expr) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>>;

    fn emit_fn(&mut self, context_name: &str, f: &Fn) -> Result<bool, Box<WhammError>>;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>>;
    fn remove_orig(&mut self) -> bool;
    fn emit_orig(&mut self) -> bool;
    fn emit_if(&mut self) -> bool;
    fn emit_if_else(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    fn emit_condition(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool;
    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>>;
    fn emit_body(&mut self, body: &mut Block) -> Result<bool, Box<WhammError>>;
    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>>;

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>>;
}

pub trait VisitingEmitter {
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn enter_scope_via_spec(&mut self, script_id: &str, probe_spec: &ProbeSpec) -> bool;
    fn exit_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn reset_children(&mut self);

    fn init_instr_iter(&mut self) -> Result<(), Box<WhammError>>;
    fn has_next_instr(&self) -> bool;
    fn init_first_instr(&mut self) -> bool;
    fn next_instr(&mut self) -> bool;
    fn curr_instr(&self) -> &Operator;
    fn curr_instr_name(&self) -> &str;
    fn incr_loc_pointer(&mut self);
    fn get_loc_info<'d>(&self, rule: &'d WhammProvider) -> Option<LocInfo<'d>>;

    fn save_args(&mut self, args: &[OrcaType]) -> bool;
    fn emit_args(&mut self) -> Result<bool, Box<WhammError>>;
    fn define(&mut self, var_name: &str, var_rec: &Option<Value>) -> Result<bool, Box<WhammError>>;
    fn reset_table_data(&mut self, loc_info: &LocInfo);
    fn fold_expr(&mut self, expr: &mut Expr) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>>;

    fn emit_fn(&mut self, context_name: &str, f: &Fn) -> Result<bool, Box<WhammError>>;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>>;
    fn remove_orig(&mut self) -> bool;
    fn emit_orig(&mut self) -> bool;
    fn emit_if(&mut self) -> bool;
    fn emit_if_else(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    fn emit_condition(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool;
    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>>;
    fn emit_body(&mut self, body: &mut Block) -> Result<bool, Box<WhammError>>;
    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>>;

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>>;
}
