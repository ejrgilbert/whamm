// =======================
// ==== CodeGenerator ====
// =======================

use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::lang_features::report_vars::LocationData;
use crate::generator::GeneratingVisitor;
use crate::parser::types::{DataType, Fn, Value, Whamm, WhammVisitorMut};
use orca_wasm::ir::id::FunctionID;

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub context_name: String,
    pub err: &'h mut ErrorGen,
    pub injected_funcs: &'i mut Vec<FunctionID>,
}
impl InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.emitter.setup_module(self.err);
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        let is_success = self.visit_whamm(whamm);
        self.emitter.memory_grow(); // account for emitted strings in memory

        is_success
    }
}

impl GeneratingVisitor for InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn emit_string(&mut self, val: &mut Value) -> bool {
        self.emitter.emit_string(val, self.err)
    }

    fn emit_fn(&mut self, context: &str, f: &Fn) -> Option<FunctionID> {
        self.emitter.emit_fn(context, f, self.err)
    }

    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_global(name, ty, value, self.err)
    }

    fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_report_global(name, ty, value, self.err)
    }

    fn add_injected_func(&mut self, fid: FunctionID) {
        self.injected_funcs.push(fid);
    }

    fn get_context_name_mut(&mut self) -> &mut String {
        &mut self.context_name
    }

    fn get_context_name(&self) -> &String {
        &self.context_name
    }

    fn set_curr_loc(&mut self, loc: LocationData) {
        self.emitter.report_vars.curr_location = loc;
    }

    fn enter_named_scope(&mut self, name: &str) {
        self.emitter.table.enter_named_scope(name);
    }

    fn enter_scope(&mut self) {
        self.emitter.enter_scope(self.err);
    }

    fn exit_scope(&mut self) {
        self.emitter.exit_scope(self.err);
    }
}
