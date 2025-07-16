// =======================
// ==== CodeGenerator ====
// =======================

use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::generator::{emit_needed_funcs, GeneratingVisitor};
use crate::lang_features::report_vars::LocationData;
use crate::parser::types::{DataType, Location, Statement, Value, Whamm, WhammVisitorMut};
use crate::verifier::types::Record;
use std::collections::{HashMap, HashSet};
use wirm::ir::id::FunctionID;
use wirm::Module;

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-defined functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f>,
    pub context_name: String,
    pub err: &'g mut ErrorGen,
    pub injected_funcs: &'h mut Vec<FunctionID>,
    pub used_fns_per_lib: HashMap<String, HashSet<String>>,
    pub user_lib_modules: HashMap<String, (Option<String>, Module<'i>)>,
}
impl InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(
        &mut self,
        whamm: &mut Whamm,
        used_bound_funcs: HashSet<(String, String)>,
        strings_to_emit: Vec<String>,
        has_probe_state_init: bool,
    ) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.injected_funcs
            .extend(self.emitter.setup_module(true, has_probe_state_init));
        emit_needed_funcs(used_bound_funcs, &mut self.emitter, self.injected_funcs);
        self.emitter.emit_strings(strings_to_emit);
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        self.visit_whamm(whamm)
    }
}

impl GeneratingVisitor for InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn emit_string(&mut self, val: &mut Value) -> bool {
        self.emitter.emit_string(val)
    }

    fn emit_func(&mut self, f: &mut crate::parser::types::Fn) -> Option<FunctionID> {
        self.emitter.emit_fn("TODO", f)
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

    fn link_user_lib(&mut self, lib_name: &str, loc: &Option<Location>) {
        // Perform import now! (we'll be in the right table scope at this point)
        if let Some(used_fns) = self.used_fns_per_lib.get(lib_name) {
            let Some((lib_name_import_override, lib_wasm)) = self.user_lib_modules.get(lib_name)
            else {
                panic!("Could not find wasm module for library '{lib_name}'");
            };
            self.injected_funcs.extend(
                crate::lang_features::libraries::linking::import_lib::link_user_lib(
                    self.emitter.app_wasm,
                    loc,
                    lib_wasm,
                    lib_name.to_string(),
                    lib_name_import_override,
                    used_fns,
                    self.emitter.table,
                ),
            );
        }
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
        self.emitter.enter_scope();
    }

    fn exit_scope(&mut self) {
        self.emitter.exit_scope();
    }
    fn lookup_var_mut(&mut self, name: &str) -> Option<&mut Record> {
        self.emitter.table.lookup_var_mut(name, true)
    }

    fn visit_global_stmts(&mut self, stmts: &mut [Statement]) -> bool {
        self.visit_stmts(stmts);
        self.emitter.emit_global_stmts(stmts, self.err)
    }
}
