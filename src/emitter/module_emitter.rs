use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::rewriting::rules::Arg;
use crate::emitter::utils::{emit_body, emit_expr, emit_stmt, whamm_type_to_wasm_global};
use crate::emitter::{configure_flush_routines, Emitter, InjectStrategy};
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{Block, DataType, Definition, Expr, Fn, FnId, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::debug;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::{BlockType as OrcaBlockType, DataType as OrcaType, InitExpr, Value as OrcaValue};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::{Instrumenter, MacroOpcode, Opcode};
use orca_wasm::{Instructions, Location};

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub strategy: InjectStrategy,
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,
    pub mem_allocator: &'d mut MemoryAllocator,
    pub map_lib_adapter: &'e mut MapLibAdapter,
    pub report_vars: &'f mut ReportVars,
    pub unshared_var_handler: &'g mut UnsharedVarHandler,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    // note: only used in integration test
    pub fn new(
        strategy: InjectStrategy,
        app_wasm: &'a mut Module<'b>,
        table: &'c mut SymbolTable,
        mem_allocator: &'d mut MemoryAllocator,
        map_lib_adapter: &'e mut MapLibAdapter,
        report_vars: &'f mut ReportVars,
        unshared_var_handler: &'g mut UnsharedVarHandler,
    ) -> Self {
        Self {
            strategy,
            app_wasm,
            emitting_func: None,
            mem_allocator,
            map_lib_adapter,
            report_vars,
            unshared_var_handler,
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    // ============================
    // ==== SYMBOL TABLE LOGIC ====
    // ============================

    pub(crate) fn enter_scope(&mut self, err: &mut ErrorGen) {
        self.table.enter_scope(err)
    }
    pub(crate) fn exit_scope(&mut self, err: &mut ErrorGen) {
        self.table.exit_scope(err)
    }
    pub(crate) fn reset_table(&mut self) {
        self.table.reset();
    }

    // =================================
    // ==== BASE MODULE SETUP LOGIC ====
    // =================================

    pub fn setup_module(&mut self, create_tracker_global: bool, err: &mut ErrorGen) {
        // setup maps
        if self.map_lib_adapter.is_used {
            self.create_instr_init(err);
            // setup report maps
            self.create_print_map_meta(err);
        }

        // setup report globals
        self.create_print_global_meta(err);

        // setup mem tracker global
        if create_tracker_global {
            self.mem_allocator.mem_tracker_global = self.app_wasm.add_global(
                InitExpr::new(vec![Instructions::Value(OrcaValue::I32(0))]),
                OrcaType::I32,
                true,
                false,
            );
        }
    }

    // ===========================
    // ==== EMIT `func` LOGIC ====
    // ===========================

    pub(crate) fn emit_fn(
        &mut self,
        context: &str,
        f: &Fn,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        // figure out if this is a provided fn.
        if f.def == Definition::CompilerDynamic {
            return if self.fn_providing_contexts.contains(&context.to_string()) {
                self.emit_provided_fn(context, f, err)
            } else {
                err.add_error(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Provided fn, but could not find a context to provide the definition, context: {}",
                        context
                    )),
                    None,
                ));
                None
            };
        }

        // emit non-provided fn
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    pub fn emit_special_fn(
        &mut self,
        name: Option<String>,
        params: &[OrcaType],
        results: &[OrcaType],
        block: &mut Block,
        export: bool,
        err: &mut ErrorGen,
    ) -> Option<u32> {
        let func = FunctionBuilder::new(params, results);
        self.emitting_func = Some(func);

        // TODO(unshared) -- load unshared vars (check me)
        if self.map_lib_adapter.is_used {
            let fid = self.map_lib_adapter.get_map_init_fid(self.app_wasm, err);
            if let Some(func) = &mut self.emitting_func {
                self.map_lib_adapter.inject_map_init(func, fid);
            };
        }

        // emit the function body
        self.emit_body(&[], block, err);

        // TODO(unshared) -- save unshared vars (check me)
        // emit the function
        if let Some(func) = self.emitting_func.take() {
            let fid = func.finish_module(self.app_wasm);
            if let Some(name) = name {
                self.app_wasm.set_fn_name(fid, name.clone());
                if export {
                    self.app_wasm.exports.add_export_func(name, *fid);
                }
            } else {
                if export {
                    self.app_wasm.exports.add_export_func(format!("${}", fid.to_string()), *fid);
                }
            }
            Some(*fid)
        } else {
            None
        }
    }

    pub(crate) fn emit_provided_fn(
        &mut self,
        context: &str,
        f: &Fn,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        if context == "whamm" && f.name.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f, err)
        } else {
            err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Provided function, but could not find a context to provide the definition, context: {}",
                    context
                )),
                None,
            ));
            None
        }
    }

    pub(crate) fn emit_end_fn(&mut self, flush_reports: bool, io_adapter: &mut IOAdapter, err: &mut ErrorGen) {
        if flush_reports {
            // (ONLY DO THIS IF THERE ARE REPORT VARIABLES)
            let mut on_exit = FunctionBuilder::new(&[], &[]);

            // call the report_vars to emit calls to all report var flushers
            self.report_vars.emit_flush_logic(&mut on_exit, io_adapter, self.mem_allocator.mem_id, self.app_wasm, err);

            let on_exit_id = on_exit.finish_module(self.app_wasm);
            self.app_wasm.set_fn_name(on_exit_id, "on_exit".to_string());

            self.app_wasm.exports.add_export_func("wasm:exit".to_string(), *on_exit_id);
        }
    }

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn, err: &mut ErrorGen) -> Option<FunctionID> {
        let strcmp_params = vec![OrcaType::I32, OrcaType::I32, OrcaType::I32, OrcaType::I32];
        let strcmp_result = vec![OrcaType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // specify params
        let str0_offset = LocalID(0);
        let str0_size = LocalID(1);
        let str1_offset = LocalID(2);
        let str1_size = LocalID(3);

        // create locals
        let i = strcmp.add_local(OrcaType::I32);
        let str0_char = strcmp.add_local(OrcaType::I32);
        let str1_char = strcmp.add_local(OrcaType::I32);

        #[rustfmt::skip]
        strcmp
            .block(OrcaBlockType::Empty) // label = @1
            .block(OrcaBlockType::Empty) // label = @2
            // 1. Check if sizes are equal, if not return 0
            .local_get(str0_size)
            .local_get(str1_size)
            .i32_ne()
            .br_if(1) // (;@1;)

            // 2. Check if mem offset is equal, if yes return non-zero (we are comparing the same data)
            .local_get(str0_offset)
            .local_get(str1_offset)
            .i32_eq()
            .br_if(0) // (;@2;)

            // 3. iterate over each string and check equivalence of chars, if any not equal, return 0
            .i32_const(0)
            .local_set(i)
            .loop_stmt(OrcaBlockType::Empty)
            // Check if we've reached the end of the string
            .local_get(i)
            .local_get(str0_size)  // (can compare with either str size, equal at this point)
            .i32_lt_unsigned()
            .i32_eqz()
            .br_if(1) // (;2;),  We've reached the end without failing equality checks!

            // get char for str0
            .local_get(str0_offset)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                wasmparser::MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: 0 // app memory!
                }
            )
            .local_set(str0_char)

            // get char for str1
            .local_get(str1_offset)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                wasmparser::MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: self.mem_allocator.mem_id // instr memory!
                }
            )
            .local_set(str1_char)

            // compare the two chars
            .local_get(str0_char)
            .local_get(str1_char)
            .i32_ne()
            .br_if(2) // (;@1;), If they are not equal, exit and return '0'

            // Increment i and continue loop
            .local_get(i)
            .i32_const(1)
            .i32_add()
            .local_set(i)
            .br(0) // (;3;)
            .end()

            // 4. Reached the end of each string without returning, return nonzero
            .br(0) // (;2;)
            .end()

            // they are equal, return '1'
            .i32_const(1)
            .return_stmt()
            .end()
            // they are not equal, return '0'
            .i32_const(0)
            .return_stmt();

        let strcmp_id = strcmp.finish_module(self.app_wasm);
        self.app_wasm.set_fn_name(strcmp_id, "strcmp".to_string());

        let Record::Fn { addr, .. } = self.table.lookup_fn_mut(&f.name.name, err)? else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return None;
        };
        *addr = Some(*strcmp_id);
        Some(strcmp_id)
    }

    // ==========================
    // ==== EMIT `map` LOGIC ====
    // ===========================

    fn create_instr_init(&mut self, err: &mut ErrorGen) -> FunctionID {
        // TODO -- move this into the MapAdapter
        //make a global bool for whether to run the instr_init fn
        self.map_lib_adapter.init_bool_location = *self.app_wasm.add_global(
            InitExpr::new(vec![Instructions::Value(OrcaValue::I32(1))]),
            OrcaType::I32,
            true,
            false,
        );
        match self.app_wasm.functions.get_local_fid_by_name("instr_init") {
            Some(_) => {
                debug!("instr_init function already exists");
                err.unexpected_error(
                    true,
                    Some(
                        "instr_init function already exists - needs to be created by Whamm"
                            .to_string(),
                    ),
                    None,
                );
                unreachable!()
            }
            None => {
                //time to make a instr_init fn
                debug!("No instr_init function found, creating one");
                let instr_init_fn = FunctionBuilder::new(&[], &[]);
                let instr_init_id = instr_init_fn.finish_module(self.app_wasm);
                self.app_wasm
                    .set_fn_name(instr_init_id, "instr_init".to_string());
                instr_init_id
            }
        }
    }

    // ================================
    // ==== EMIT MEMORY DATA LOGIC ====
    // ================================

    pub fn emit_string(&mut self, value: &mut Value, err: &mut ErrorGen) -> bool {
        match value {
            Value::Str { val, .. } => {
                self.mem_allocator.emit_string(self.app_wasm, val);
                true
            }
            _ => {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Called 'emit_string', but this is not a string type: {:?}",
                        value
                    )),
                    None,
                );
                false
            }
        }
    }

    pub(crate) fn memory_grow(&mut self) {
        self.mem_allocator.memory_grow(self.app_wasm);
    }

    // =============================
    // ==== EMIT `global` LOGIC ====
    // =============================

    pub(crate) fn emit_global_getter(
        &mut self,
        global_id: &u32,
        name: String,
        ty: OrcaType,
    ) -> FunctionID {
        // todo -- make this conditional on 'testing' mode
        let getter_params = vec![];
        let getter_res = vec![ty];

        let mut getter = FunctionBuilder::new(&getter_params, &getter_res);
        getter.global_get(GlobalID(*global_id));

        let getter_id = getter.finish_module(self.app_wasm);
        let fn_name = format!("get_{name}");
        self.app_wasm.set_fn_name(getter_id, fn_name.clone());
        self.app_wasm.exports.add_export_func(fn_name, *getter_id);

        getter_id
    }

    pub(crate) fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        self.emit_global_inner(name, ty, val, false, err)
    }

    fn emit_global_inner(
        &mut self,
        name: String,
        _ty: DataType,
        _val: &Option<Value>,
        report_mode: bool,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        let Record::Var { addr, ty, .. } = self.table.lookup_var_mut(&name, &None, err)? else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return None;
        };

        // emit global variable and set addr in symbol table
        // this is used for user-defined global vars in the script...
        match ty {
            DataType::Map { .. } => {
                // TODO -- move to MapAdapter
                //time to set up the map_init fn
                let Some(init_id) = self.app_wasm.functions.get_local_fid_by_name("instr_init")
                else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                                No instr_init found in the module!"
                        )),
                        None,
                    );
                    return None;
                };

                let Some(mut init_fn) = self.app_wasm.functions.get_fn_modifier(init_id) else {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                                No instr_init found in the module!"
                        )),
                        None,
                    );
                    return None;
                };
                init_fn.before_at(Location::Module {
                    func_idx: init_id, // not used
                    instr_idx: 0,
                });
                let map_id = if report_mode {
                    self.map_lib_adapter.map_create_report(
                        name,
                        ty.clone(),
                        &mut init_fn,
                        self.report_vars,
                        false,
                        err,
                    )
                } else {
                    self.map_lib_adapter
                        .map_create(ty.clone(), &mut init_fn, err)
                };

                *addr = Some(VarAddr::MapId { addr: map_id });
                None
            }
            _ => {
                let (global_id, global_ty) = whamm_type_to_wasm_global(self.app_wasm, ty);
                *addr = Some(VarAddr::Global { addr: *global_id });
                //now save off the global variable metadata
                if report_mode {
                    self.report_vars
                        .put_global_metadata(*global_id, name.clone(), err);
                }
                Some(self.emit_global_getter(&global_id, name, global_ty))
            }
        }
    }

    // ========================================
    // ==== EMIT `global` Statements LOGIC ====
    // ========================================

    pub fn emit_global_stmts(&mut self, stmts: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        if let Some(_start_fid) = self.app_wasm.start {
            // 1. create the emitting_func var, assign in self
            // 2. iterate over stmts and emit them! (will be different for Decl stmts)
            todo!()
        } else {
            // TODO -- try to create our own start fn (for dfinity case)
            for stmt in stmts.iter_mut() {
                match stmt {
                    Statement::Decl { .. } => {
                        // This is fine
                        todo!()
                    }
                    _ => {
                        // This is NOT fine...error!
                        // Cannot emit this at the moment since there's no entrypoint for our module to emit initialization instructions into
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(
                                "This module has no configured entrypoint, \
            unable to emit a `script` with initialized global state"
                                    .to_string(),
                            ),
                            None,
                        )));
                    }
                }
            }
        }

        Ok(true)
    }

    // =============================
    // ==== EMIT `report` LOGIC ====
    // =============================

    fn create_print_map_meta(&mut self, err: &mut ErrorGen) {
        // TODO -- move this into the IOAdapter (maybe a ReportVarAdapter?)
        if self
            .app_wasm
            .functions
            .get_local_fid_by_name("print_map_meta")
            .is_some()
        {
            debug!("print_map_meta function already exists");
            err.unexpected_error(
                true,
                Some(
                    "print_map_meta function already exists - needs to be created by Whamm"
                        .to_string(),
                ),
                None,
            );
            return;
        }

        debug!("Creating the print_map_meta function");
        let print_map_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_map_meta_id = print_map_meta_fn.finish_module(self.app_wasm);
        self.app_wasm
            .set_fn_name(print_map_meta_id, "print_map_meta".to_string());

        self.table.put(
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

    fn create_print_global_meta(&mut self, err: &mut ErrorGen) {
        // TODO -- move this into the IOAdapter (maybe a ReportVarAdapter?)
        if self
            .app_wasm
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
            return;
        }

        debug!("Creating the print_global_meta function");
        let print_global_meta_fn = FunctionBuilder::new(&[], &[]);
        let print_global_meta_id = print_global_meta_fn.finish_module(self.app_wasm);
        self.app_wasm
            .set_fn_name(print_global_meta_id, "print_global_meta".to_string());

        self.table.put(
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

    pub fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        self.emit_global_inner(name, ty, val, true, err)
    }

    pub fn configure_mem_tracker_global(&mut self, err: &mut ErrorGen) {
        // TODO -- factor out all this dupe logic
        let init_id = if let Some(fid) = self.app_wasm.functions.get_local_fid_by_name("instr_init")
        {
            fid
        } else {
            self.create_instr_init(err)
        };

        let Some(mut init_fn) = self.app_wasm.functions.get_fn_modifier(init_id) else {
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                                No instr_init found in the module!"
                )),
                None,
            );
            return;
        };
        init_fn.before_at(Location::Module {
            func_idx: init_id, // not used
            instr_idx: 0,
        });

        init_fn.u32_const(self.mem_allocator.curr_mem_offset as u32);
        init_fn.global_set(self.mem_allocator.mem_tracker_global);
    }

    pub fn configure_flush_routines(&mut self, io_adapter: &mut IOAdapter, err: &mut ErrorGen) {
        configure_flush_routines(
            self.app_wasm,
            self.table,
            self.report_vars,
            self.map_lib_adapter,
            io_adapter,
            UNEXPECTED_ERR_MSG,
            err,
        );
    }
}
impl Emitter for ModuleEmitter<'_, '_, '_, '_, '_, '_, '_> {
    fn emit_body(
        &mut self,
        _curr_instr_args: &[Arg],
        body: &mut Block,
        err: &mut ErrorGen,
    ) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_body(
                body,
                self.strategy,
                emitting_func,
                self.table,
                self.mem_allocator,
                self.map_lib_adapter,
                self.report_vars,
                self.unshared_var_handler,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }

    fn emit_stmt(
        &mut self,
        _curr_instr_args: &[Arg],
        stmt: &mut Statement,
        err: &mut ErrorGen,
    ) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_stmt(
                stmt,
                self.strategy,
                emitting_func,
                self.table,
                self.mem_allocator,
                self.map_lib_adapter,
                self.report_vars,
                self.unshared_var_handler,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }

    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_expr(
                expr,
                self.strategy,
                emitting_func,
                self.table,
                self.mem_allocator,
                self.map_lib_adapter,
                self.report_vars,
                self.unshared_var_handler,
                UNEXPECTED_ERR_MSG,
                err,
            )
        } else {
            false
        }
    }
}
