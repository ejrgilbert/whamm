// =======================
// ==== CodeGenerator ====
// =======================

use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::report_var_metadata::LocationData;
use crate::generator::GeneratingVisitor;
use crate::parser::types::{DataType, Definition, Fn, FnId, Value, Whamm, WhammVisitorMut};
use crate::verifier::types::Record;
use log::{debug, info};
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::ir::types::DataType as OrcaType;
use orca_wasm::ir::types::Value as OrcaValue;
use orca_wasm::InitExpr;

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f>,
    pub context_name: String,
    pub err: &'g mut ErrorGen,
    pub injected_funcs: &'h mut Vec<FunctionID>,
}
impl InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.on_startup();
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        let is_success = self.visit_whamm(whamm);
        self.emitter.memory_grow(); // account for emitted strings in memory

        is_success
    }

    // Private helper functions
    fn on_startup(&mut self) {
        self.create_start();
        if self.emitter.map_lib_adapter.is_used {
            self.create_global_map_init();
            self.create_print_map_meta();
        }
        self.create_print_global_meta();
    }
    fn create_start(&mut self) {
        match self.emitter.app_wasm.start {
            Some(_) => {
                debug!("Start function already exists");
            }
            None => {
                //time to make a start fn
                info!("No start function found, creating one");
                match self
                    .emitter
                    .app_wasm
                    .functions
                    .get_local_fid_by_name("_start")
                {
                    Some(_) => {
                        debug!("start function is _start");
                    }
                    None => {
                        let start_fn = FunctionBuilder::new(&[], &[]);
                        let start_id = start_fn.finish_module(self.emitter.app_wasm);
                        self.injected_funcs.push(start_id);
                        self.emitter.app_wasm.start = Some(start_id);
                        self.emitter
                            .app_wasm
                            .set_fn_name(start_id, "start".to_string());
                    } //strcmp doesn't need to call add_export_fn so this probably doesn't either
                      //in app_wasm, not sure if we need to have it in the ST
                }
            }
        }
    }

    fn create_global_map_init(&mut self) {
        //make a global bool for whether to run the global_map_init fn
        self.emitter.map_lib_adapter.init_bool_location = *self.emitter.app_wasm.add_global(
            InitExpr::Value(OrcaValue::I32(1)),
            OrcaType::I32,
            true,
            false,
        );
        match self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("global_map_init")
        {
            Some(_) => {
                debug!("global_map_init function already exists");
                self.err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(
                        "global_map_init function already exists - needs to be created by Whamm"
                            .to_string(),
                    ),
                    None,
                ));
            }
            None => {
                //time to make a global_map_init fn
                debug!("No global_map_init function found, creating one");
                let global_map_init_fn = FunctionBuilder::new(&[], &[]);
                let global_map_init_id = global_map_init_fn.finish_module(self.emitter.app_wasm);
                self.emitter
                    .app_wasm
                    .set_fn_name(global_map_init_id, "global_map_init".to_string());
            }
        }
    }

    fn create_print_map_meta(&mut self) {
        if self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("print_map_meta")
            .is_some()
        {
            debug!("print_map_meta function already exists");
            self.err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(
                    "print_map_meta function already exists - needs to be created by Whamm"
                        .to_string(),
                ),
                None,
            ));
            return;
        }

        debug!("Creating the print_map_meta function");
        let print_map_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_map_meta_id = print_map_meta_fn.finish_module(self.emitter.app_wasm);
        self.emitter
            .app_wasm
            .set_fn_name(print_map_meta_id, "print_map_meta".to_string());

        self.emitter.table.put(
            "print_map_meta".to_string(),
            Record::Fn {
                name: FnId {
                    name: "print_map_meta".to_string(),
                    loc: None,
                },
                params: vec![],
                ret_ty: DataType::Tuple { ty_info: vec![] },
                def: Definition::CompilerStatic,
                addr: Some(*print_map_meta_id),
                loc: None,
            },
        );
    }

    fn create_print_global_meta(&mut self) {
        if self
            .emitter
            .app_wasm
            .functions
            .get_local_fid_by_name("print_global_meta")
            .is_some()
        {
            debug!("print_global_meta function already exists");
            self.err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(
                    "print_global_meta function already exists - needs to be created by Whamm"
                        .to_string(),
                ),
                None,
            ));
            return;
        }

        debug!("Creating the print_global_meta function");
        let print_global_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_global_meta_id = print_global_meta_fn.finish_module(self.emitter.app_wasm);
        self.emitter
            .app_wasm
            .set_fn_name(print_global_meta_id, "print_global_meta".to_string());

        self.emitter.table.put(
            "print_global_meta".to_string(),
            Record::Fn {
                name: FnId {
                    name: "print_global_meta".to_string(),
                    loc: None,
                },
                params: vec![],
                ret_ty: DataType::Tuple { ty_info: vec![] },
                def: Definition::CompilerStatic,
                addr: Some(*print_global_meta_id),
                loc: None,
            },
        );
    }
}

impl GeneratingVisitor for InitGenerator<'_, '_, '_, '_, '_, '_, '_, '_> {
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
        self.emitter.report_var_metadata.curr_location = loc;
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
