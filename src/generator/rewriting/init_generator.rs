// =======================
// ==== CodeGenerator ====
// =======================

use std::collections::HashSet;
use log::trace;
use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::generator::{create_curr_loc, emit_needed_funcs, GeneratingVisitor};
use crate::lang_features::report_vars::LocationData;
use crate::parser::types::{DataType, Fn, Value, WhammVisitorMut};
use crate::verifier::types::Record;
use orca_wasm::ir::id::{FunctionID, LocalID};
use crate::generator::ast::{Probe, Script};
use orca_wasm::Opcode;
use orca_wasm::opcode::Instrumenter;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'i mut ErrorGen,
    pub injected_funcs: &'j mut Vec<FunctionID>,

    pub curr_script_id: u8
}
impl InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, ast: &mut Vec<Script>,
               used_provided_funcs: HashSet<(String, String)>,
               used_report_dts: HashSet<DataType>,
               strings_to_emit: Vec<String>) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.emitter.setup_module(self.err);
        emit_needed_funcs(used_provided_funcs, &mut self.emitter, self.injected_funcs, self.err);
        self.emitter.emit_strings(strings_to_emit, self.err);

        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        self.visit_ast(ast);

        self.emit_end_func(used_report_dts);
        true
    }

    fn emit_end_func(&mut self, used_report_dts: HashSet<DataType>) {
        if let Some(on_exit_id) = self.emitter.emit_end_fn(used_report_dts, self.io_adapter, self.err) {
            // now find where the "exit" is in the bytecode
            // exit of export "main"
            // OR if that doesn't exist, the end of the "start" function
            let fid = if let Some(main_fid) = self
                .emitter
                .app_wasm
                .exports
                .get_func_by_name("main".to_string())
            {
                main_fid
            } else if let Some(start_fid) = self.emitter
                .app_wasm.start {
                start_fid
            } else {
                // neither exists, unsure how to support this...this would be a library instead of an application I guess?
                // Maybe the answer is to expose query functions that can give a status update of the `report` vars?
                unimplemented!("Your target Wasm has no main or start function...we do not support report variables in this scenario.")
            };
            let mut main = self.emitter
                .app_wasm.functions.get_fn_modifier(fid).unwrap();

            main.func_exit();
            main.call(on_exit_id);
            main.finish_instr();
        }
    }

    // Visit the AST
    fn visit_ast(&mut self, ast: &mut Vec<Script>) {
        for script in ast.iter_mut() {
            self.visit_script(script);
        }
    }

    fn visit_script(&mut self, script: &mut Script) {
        trace!("Entering: InitGenerator::visit_script");
        self.enter_named_scope(&script.id.to_string());
        self.set_context_name(script.id.to_string());
        self.curr_script_id = script.id;

        self.set_curr_loc(LocationData::Global {
            script_id: script.id,
        });

        script.fns.iter_mut().for_each(|f| {
            self.visit_fn(f);
        });
        self.visit_globals(&script.globals);
        script.probes.iter_mut().for_each(|probe| {
            self.visit_probe(probe);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.exit_scope();
    }

    fn visit_probe(&mut self, probe: &mut Probe) {
        self.set_curr_loc(create_curr_loc(self.curr_script_id, probe));

        // let (pred_fid, pred_param_str, dynamic_pred) = if let Some(pred) = &mut probe.predicate {
        //     if probe.metadata.pred_is_dynamic {
        //         // TODO -- need to move this logic into the VisitingGenerator...
        //         //     the emitted predicate will be different per probe match location!
        //         // dynamic analysis of the predicate will go here!
        //         // See: https://github.com/ejrgilbert/whamm/issues/163
        //         (None, "".to_string(), Some(pred))
        //     } else {
        //         let mut block = Block {
        //             stmts: vec![Statement::Expr {
        //                 expr: pred.clone(),
        //                 loc: None,
        //             }],
        //             return_ty: None,
        //             loc: None,
        //         };
        //         let (fid, str) = self.emit_special_func(
        //             None,
        //             &probe.metadata.pred_args,
        //             None,
        //             &[OrcaType::I32],
        //             &mut block,
        //             true,
        //         );
        //         (fid, str, None)
        //     }
        // } else {
        //     (None, "".to_string(), None)
        // };

        // TODO -- run static 'alloc' in VisitingEmitter
        // // create the probe's $alloc method
        // let (alloc_fid, alloc_param_str) = self.unshared_var_handler.emit_alloc_func(
        //     &mut probe.unshared_to_alloc,
        //     &mut self.emitter,
        //     self.err,
        // );

        // create the probe body function
        let (body_fid, _) = if let Some(body) = &mut probe.body {
            let alloc_local = if !probe.unshared_to_alloc.is_empty() {
                Some(LocalID(0))
            } else {
                None
            };
            self.emitter.emit_special_func(
                alloc_local,
                &probe.metadata.body_args,
                None,
                &[],
                body,
                false,
                self.err
            )
        } else {
            (None, "".to_string())
        };
        assert!(body_fid.is_some());

        // not naming the probe body to keep from conflicting with other probe names
        // (there may be multiple of the same probe events/signatures in a script)

        probe.body_fid = body_fid;
    }
}

impl GeneratingVisitor for InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
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
    fn lookup_var_mut(&mut self, name: &str) -> Option<&mut Record> {
        self.emitter.table.lookup_var_mut(name, &None, self.err)
    }
}
