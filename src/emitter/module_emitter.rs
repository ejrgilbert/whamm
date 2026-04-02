use crate::common::error::ErrorGen;
use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::{MemoryAllocator, VAR_BLOCK_BASE_VAR};
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::utils::{
    emit_expr, emit_global_getter, emit_probes, emit_stmt, whamm_type_to_wasm_global, EmitCtx,
};
use crate::emitter::{Emitter, InjectStrategy};
use crate::generator::ast::{Probe, Script, WhammParams};
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::utils::utils_adapter::UtilsAdapter;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::{Metadata, ReportVars};
use crate::parser::types::{Block, DataType, Definition, Expr, Fn, Location, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::debug;
use std::collections::HashSet;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, LocalID};
use wirm::ir::module::Module;
use wirm::ir::types::{
    BlockType as WirmBlockType, DataType as WirmType, InitExpr, Value as WirmValue,
};
use wirm::module_builder::AddLocal;
use wirm::opcode::{Instrumenter, Opcode};
use wirm::wasmparser::MemArg;
use wirm::InitInstr;

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct ModuleEmitter<'a, 'ir> {
    pub strategy: InjectStrategy,
    pub app_wasm: &'a mut Module<'ir>,
    pub emitting_func: Option<FunctionBuilder<'ir>>,
    pub table: &'a mut SymbolTable,
    pub mem_allocator: &'a mut MemoryAllocator,
    pub locals_tracker: LocalsTracker,
    pub utils_adapter: &'a mut UtilsAdapter,
    pub map_lib_adapter: &'a mut MapLibAdapter,
    pub report_vars: &'a mut ReportVars,
    pub registry: &'a mut WasmRegistry,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'ir> ModuleEmitter<'a, 'ir> {
    // note: only used in integration test
    pub fn new(
        strategy: InjectStrategy,
        app_wasm: &'a mut Module<'ir>,
        table: &'a mut SymbolTable,
        mem_allocator: &'a mut MemoryAllocator,
        utils_adapter: &'a mut UtilsAdapter,
        map_lib_adapter: &'a mut MapLibAdapter,
        report_vars: &'a mut ReportVars,
        registry: &'a mut WasmRegistry,
    ) -> Self {
        Self {
            strategy,
            app_wasm,
            emitting_func: None,
            mem_allocator,
            locals_tracker: LocalsTracker::default(),
            utils_adapter,
            map_lib_adapter,
            report_vars,
            table,
            registry,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    // ============================
    // ==== SYMBOL TABLE LOGIC ====
    // ============================

    pub(crate) fn enter_scope(&mut self) {
        self.table.enter_scope()
    }
    pub(crate) fn exit_scope(&mut self) {
        self.table.exit_scope()
    }
    pub(crate) fn reset_table(&mut self) {
        self.table.reset();
    }

    // =================================
    // ==== BASE MODULE SETUP LOGIC ====
    // =================================

    pub fn setup_module(
        &mut self,
        is_rewriting: bool,
        has_probe_state_init: bool,
    ) -> Vec<FunctionID> {
        let mut injected_funcs = vec![];
        // setup maps and probe state initialization
        // We only do probe state initialization here for rewriting, the engine target handles that in the $alloc methods
        if (is_rewriting && has_probe_state_init) | self.map_lib_adapter.used_in_global_scope {
            injected_funcs.push(self.create_instr_init());
        }
        injected_funcs
    }

    // ===========================
    // ==== EMIT `func` LOGIC ====
    // ===========================

    pub(crate) fn emit_fn(&mut self, context: &str, f: &Fn) -> Option<FunctionID> {
        match f.def {
            Definition::CompilerDynamic => {
                if self.fn_providing_contexts.contains(&context.to_string()) {
                    self.emit_bound_fn(context, f)
                } else {
                    unreachable!(
                            "{} \
                        Provided fn, but could not find a context to provide the definition, context: {}",
                            UNEXPECTED_ERR_MSG, context
                        );
                }
            }
            Definition::CompilerStatic => None, // already handled
            Definition::User => {
                // TODO: only when we're supporting user-defined fns in script...
                // TODO: Remember to reset locals like what follows:
                // self.reset_locals_for_function();
                todo!()
            }
            Definition::CompilerDerived => panic!("invalid function definition context"),
        }
    }

    pub fn emit_special_func(
        &mut self,
        // the base memory offset for this function's var block
        alloc_base: Option<LocalID>,
        lib_calls: &[(Option<u32>, String, WirmType)],
        whamm_params: &WhammParams,
        dynamic_pred: Option<&Expr>,
        results: &[WirmType],
        body: &Block,
        export: bool,
        loc: &Option<Location>,
        err: &mut ErrorGen,
    ) -> (Option<u32>, String) {
        // create the function
        let mut params = vec![];
        let mut param_str = "".to_string();

        // handle $alloc param (if there are unshared vars)
        if let Some(alloc) = alloc_base {
            params.push(WirmType::I32);
            // add param definition to the symbol table
            self.table.put(
                VAR_BLOCK_BASE_VAR.to_string(),
                Record::Var {
                    ty: DataType::I32,
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(vec![VarAddr::Local { addr: *alloc }]),
                    times_set: 0,
                    loc: None,
                },
            );
        }

        // handle static library evaluation parameters
        for (i, (_, _, ty)) in lib_calls.iter().enumerate() {
            let local_id = params.len() as u32;
            params.push(*ty);
            self.table.put(
                Probe::get_call_alias_for(i),
                Record::Var {
                    ty: DataType::from_wasm_type(ty),
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(vec![VarAddr::Local { addr: local_id }]),
                    times_set: 0,
                    loc: None,
                },
            );
        }

        // handle the parameters
        Self::handle_params(whamm_params, &mut params, &mut param_str, self.table);

        let fid = self.emit_special_fn_inner(
            None,
            &params,
            dynamic_pred,
            results,
            body,
            export,
            loc,
            err,
        );

        self.reset_locals_for_function();

        (fid, param_str.to_string())
    }

    pub(crate) fn handle_params(
        whamm_params: &WhammParams,
        params: &mut Vec<WirmType>,
        param_str: &mut String,
        table: &mut SymbolTable,
    ) {
        for param in whamm_params.params.iter() {
            let wasm_tys = param.ty.to_wasm_type();

            // Iterate over the list to create the addresses for referencing
            // (will support types that are represented with multiple wasm
            // types this way, e.g. strings)
            let mut addrs = vec![];
            for ty in wasm_tys.iter() {
                let local_id = params.len() as u32;
                // handle param list
                params.push(*ty);

                addrs.push(VarAddr::Local { addr: local_id });
            }
            // add param definition to the symbol table
            table.put(
                param.name.clone(),
                Record::Var {
                    ty: param.ty.clone(),
                    value: None,
                    def: Definition::CompilerStatic,
                    addr: Some(addrs),
                    times_set: 0,
                    loc: None,
                },
            );

            // handle the param string
            if !param_str.is_empty() {
                param_str.push_str(", ")
            }
            param_str.push_str(&param.name);
        }
    }

    fn emit_special_fn_inner(
        &mut self,
        name: Option<String>,
        params: &[WirmType],
        dynamic_pred: Option<&Expr>,
        results: &[WirmType],
        block: &Block,
        export: bool,
        loc: &Option<Location>,
        err: &mut ErrorGen,
    ) -> Option<u32> {
        let func = FunctionBuilder::new(params, results);
        self.emitting_func = Some(func);

        // emit the function body (wrapping with predicate conditional if needed)
        if let Some(dynamic_pred) = dynamic_pred {
            let wrapped_block = Block {
                stmts: vec![Statement::If {
                    cond: dynamic_pred.clone(),
                    conseq: block.clone(),
                    alt: Block::default(),
                    loc: None,
                }],
                results: None,
                loc: None,
            };
            self.emit_body(&wrapped_block, err);
        } else {
            self.emit_body(block, err);
        }

        // emit the function
        if let Some(func) = self.emitting_func.take() {
            let fid = func.finish_module_with_tag(self.app_wasm, get_tag_for(loc));
            if let Some(name) = name {
                self.app_wasm.set_fn_name(fid, name.clone());
                if export {
                    self.app_wasm
                        .exports
                        .add_export_func_with_tag(name, *fid, get_tag_for(loc));
                }
            } else if export {
                self.app_wasm.exports.add_export_func_with_tag(
                    format!("${}", *fid),
                    *fid,
                    get_tag_for(loc),
                );
            }
            Some(*fid)
        } else {
            None
        }
    }

    pub fn emit_empty_fn_with_alloc_param(&mut self, loc: &Option<Location>) -> FunctionID {
        // must be able to accept the alloc param!
        let func = FunctionBuilder::new(&[WirmType::I32], &[]);
        func.finish_module_with_tag(self.app_wasm, get_tag_for(loc))
    }

    pub(crate) fn emit_bound_fn(&mut self, context: &str, f: &Fn) -> Option<FunctionID> {
        if context == "whamm" {
            match f.name.name.as_str() {
                "strcmp" => self.emit_whamm_strcmp_fn(f),
                "strcontains" => self.emit_whamm_strcontains_fn(f),
                _ => panic!("Provided function ('{}'), but could not find a context to provide the definition, context: {context}", f.name.name),
            }
        } else {
            panic!("Provided function ('{}'), but could not find a context to provide the definition, context: {context}", f.name.name);
        }
    }

    pub(crate) fn emit_end_fn(
        &mut self,
        ast: &[Script],
        used_report_dts: HashSet<DataType>,
        io_adapter: &mut IOAdapter,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        let report_probes = {
            let mut report_probes = vec![];
            for script in ast.iter() {
                for probe in script.probes.iter() {
                    let rule = &probe.rule;
                    if rule.provider.name == "wasm" && rule.package.name == "report" {
                        report_probes.push((script.id, probe.clone()));
                    }
                }
            }
            if !report_probes.is_empty() {
                Some(report_probes)
            } else {
                None
            }
        };

        if !used_report_dts.is_empty() || report_probes.is_some() {
            // (ONLY DO THIS IF THERE ARE REPORT VARIABLES)

            let mut on_exit = FunctionBuilder::new(&[], &[]);

            if let Some(probes) = report_probes {
                for (script_id, probe) in probes {
                    // handle wasm:report override
                    self.table.enter_named_scope(&script_id.to_string());
                    emit_probes(
                        &mut [probe],
                        self.strategy,
                        &mut on_exit,
                        &mut EmitCtx::new(
                            self.table,
                            self.mem_allocator,
                            &mut self.locals_tracker,
                            self.utils_adapter,
                            self.map_lib_adapter,
                            UNEXPECTED_ERR_MSG,
                            err,
                        ),
                    );
                    self.table.exit_scope();
                }
            } else {
                self.report_vars.all_used_report_dts = used_report_dts;

                // prepare the CSV header data segment
                let (header_addr, header_len) =
                    Metadata::setup_csv_header(self.app_wasm, self.mem_allocator);
                let var_meta = self
                    .report_vars
                    .setup_flush_data_segments(self.app_wasm, self.mem_allocator);

                // call the report_vars to emit calls to all report var flushers
                self.report_vars.emit_flush_logic(
                    &mut on_exit,
                    &var_meta,
                    self.mem_allocator,
                    io_adapter,
                    self.map_lib_adapter,
                    (header_addr, header_len),
                    if let Some(id) = self.mem_allocator.alloc_var_mem_id {
                        id
                    } else {
                        self.mem_allocator.mem_id
                    },
                    self.app_wasm,
                    err,
                );
            }

            let on_exit_id = on_exit.finish_module_with_tag(self.app_wasm, get_tag_for(&None));
            self.app_wasm.set_fn_name(on_exit_id, "on_exit".to_string());

            self.app_wasm.exports.add_export_func_with_tag(
                "wasm:exit".to_string(),
                *on_exit_id,
                get_tag_for(&None),
            );
            Some(on_exit_id)
        } else {
            None
        }
    }

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn) -> Option<FunctionID> {
        let strcmp_params = vec![WirmType::I32, WirmType::I32, WirmType::I32, WirmType::I32];
        let strcmp_result = vec![WirmType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // specify params
        let str0_offset = LocalID(0);
        let str0_size = LocalID(1);
        let str1_offset = LocalID(2);
        let str1_size = LocalID(3);

        // create locals
        let i = strcmp.add_local(WirmType::I32);
        let str0_char = strcmp.add_local(WirmType::I32);
        let str1_char = strcmp.add_local(WirmType::I32);

        #[rustfmt::skip]
        strcmp
            .block(WirmBlockType::Empty) // label = @1
            .block(WirmBlockType::Empty) // label = @2
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
            .loop_stmt(WirmBlockType::Empty)
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
                MemArg {
                    align: 0,
                    max_align: 0,
                    offset: 0,
                    memory: self.mem_allocator.mem_id // app memory!
                }
            )
            .local_set(str0_char)

            // get char for str1
            .local_get(str1_offset)
            .local_get(i)
            .i32_add()
            // load a byte from memory
            .i32_load8_u(
                MemArg {
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

        let strcmp_id = strcmp.finish_module_with_tag(self.app_wasm, get_tag_for(&None));
        self.app_wasm.set_fn_name(strcmp_id, "strcmp".to_string());

        let Record::Fn { addr, .. } = self.table.lookup_fn_mut(&f.name.name)? else {
            unreachable!("unexpected type")
        };
        *addr = Some(*strcmp_id);
        Some(strcmp_id)
    }

    fn emit_whamm_strcontains_fn(&mut self, f: &Fn) -> Option<FunctionID> {
        // strcmp must be emitted before strcontains (emit_needed_funcs guarantees this order).
        let strcmp_fid = *self
            .app_wasm
            .functions
            .get_local_fid_by_name("strcmp")
            .expect("strcmp must be emitted before strcontains");

        // strcontains(hs_addr: i32, hs_len: i32, nd_addr: i32, nd_len: i32) -> i32
        // Returns 1 if haystack contains needle, 0 otherwise.
        let params = vec![WirmType::I32, WirmType::I32, WirmType::I32, WirmType::I32];
        let results = vec![WirmType::I32];
        let mut strcontains = FunctionBuilder::new(&params, &results);

        let hs_addr = LocalID(0);
        let hs_len = LocalID(1);
        let nd_addr = LocalID(2);
        let nd_len = LocalID(3);
        let i = strcontains.add_local(WirmType::I32);

        #[rustfmt::skip]
        strcontains
            // if nd_len == 0 → contains trivially, return 1
            .local_get(nd_len)
            .i32_eqz()
            .if_stmt(WirmBlockType::Empty)
            .i32_const(1)
            .return_stmt()
            .end()

            // if nd_len > hs_len → cannot contain, return 0
            .local_get(nd_len)
            .local_get(hs_len)
            .i32_gt_unsigned()
            .if_stmt(WirmBlockType::Empty)
            .i32_const(0)
            .return_stmt()
            .end()

            // i = 0
            .i32_const(0)
            .local_set(i)

            // outer block: jumped to when needle is not found
            .block(WirmBlockType::Empty)

            // loop over each starting position in the haystack
            .loop_stmt(WirmBlockType::Empty)

            // if i > hs_len - nd_len → not found, break to outer block
            .local_get(i)
            .local_get(hs_len)
            .local_get(nd_len)
            .i32_sub()
            .i32_gt_unsigned()
            .br_if(1) // (;@outer_block;)

            // strcmp(hs_addr + i, nd_len, nd_addr, nd_len)
            .local_get(hs_addr)
            .local_get(i)
            .i32_add()
            .local_get(nd_len)
            .local_get(nd_addr)
            .local_get(nd_len)
            .call(FunctionID(strcmp_fid))

            // if strcmp returned nonzero → match found, return 1
            .if_stmt(WirmBlockType::Empty)
            .i32_const(1)
            .return_stmt()
            .end()

            // i++
            .local_get(i)
            .i32_const(1)
            .i32_add()
            .local_set(i)

            .br(0) // continue loop
            .end() // end loop

            .end() // end outer block (not-found)

            // needle not found, return 0
            .i32_const(0)
            .return_stmt();

        let strcontains_id = strcontains.finish_module_with_tag(self.app_wasm, get_tag_for(&None));
        self.app_wasm
            .set_fn_name(strcontains_id, "strcontains".to_string());

        let Record::Fn { addr, .. } = self.table.lookup_fn_mut(&f.name.name)? else {
            unreachable!("unexpected type")
        };
        *addr = Some(*strcontains_id);
        Some(strcontains_id)
    }

    // ==========================
    // ==== EMIT `map` LOGIC ====
    // ===========================

    fn create_instr_init(&mut self) -> FunctionID {
        // TODO -- move this into the MapAdapter
        //make a global bool for whether to run the instr_init fn
        self.map_lib_adapter.init_bool_location = *self.app_wasm.add_global_with_tag(
            InitExpr::new(vec![InitInstr::Value(WirmValue::I32(1))]),
            WirmType::I32,
            true,
            false,
            get_tag_for(&None),
        );
        match self.app_wasm.functions.get_local_fid_by_name("instr_init") {
            Some(_) => {
                unreachable!("instr_init function already exists - needs to be created by Whamm");
            }
            None => {
                debug!("No instr_init function found, creating one");
                let instr_init_fn = FunctionBuilder::new(&[], &[]);
                let instr_init_id =
                    instr_init_fn.finish_module_with_tag(self.app_wasm, get_tag_for(&None));
                self.app_wasm
                    .set_fn_name(instr_init_id, "instr_init".to_string());
                instr_init_id
            }
        }
    }

    // ================================
    // ==== EMIT MEMORY DATA LOGIC ====
    // ================================

    pub fn emit_strings(&mut self, strings_to_emit: Vec<String>) {
        for string in strings_to_emit.iter() {
            self.emit_string(&mut Value::Str {
                val: string.clone(),
            });
        }
    }

    pub fn emit_string(&mut self, value: &mut Value) -> bool {
        match value {
            Value::Str { val, .. } => {
                self.mem_allocator.emit_string(self.app_wasm, val);
                true
            }
            _ => {
                unreachable!(
                    "{} \
                Called 'emit_string', but this is not a string type: {:?}",
                    UNEXPECTED_ERR_MSG, value
                )
            }
        }
    }

    // =============================
    // ==== EMIT `global` LOGIC ====
    // =============================

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
        let Record::Var { addr, ty, loc, .. } = self.table.lookup_var_mut(&name, true)? else {
            unreachable!("unexpected type")
        };

        // emit global variable and set addr in symbol table
        // this is used for user-defined global vars in the script...
        match ty {
            DataType::Map { .. } => {
                let map_id = self.map_lib_adapter.emit_map_init(
                    name,
                    ty,
                    report_mode,
                    true,
                    loc,
                    self.report_vars,
                    self.app_wasm,
                    err,
                );
                *addr = Some(vec![VarAddr::MapId { addr: map_id }]);
                None
            }
            _ => {
                let globals = whamm_type_to_wasm_global(self.app_wasm, ty, loc, None);

                let mut addrs = vec![];
                let only_one = globals.len() == 1;
                let mut getter = None;
                for (global_id, global_ty) in globals.iter() {
                    addrs.push(VarAddr::Global { addr: **global_id });

                    if only_one {
                        getter = Some(emit_global_getter(
                            self.app_wasm,
                            global_id,
                            name.clone(),
                            *global_ty,
                            loc,
                        ));
                    }
                }
                //now save off the global variable metadata
                if report_mode {
                    // todo -- i don't think this works for global strings.
                    self.report_vars
                        .put_global_metadata(addrs.clone(), name.clone(), ty);
                }
                *addr = Some(addrs);
                getter
            }
        }
    }

    // ========================================
    // ==== EMIT `global` Statements LOGIC ====
    // ========================================

    /// It is assumed that the statement passed here is a VALID global statement!
    /// (we've gone through several checks before this)
    /// Returns the start_fid (if it was created)
    pub fn emit_global_stmt(&mut self, stmt: &Statement, err: &mut ErrorGen) -> Option<u32> {
        let (start_fid, was_created) = ModuleEmitter::get_or_create_start_func(self.app_wasm);
        let mut start = self
            .app_wasm
            .functions
            .get_fn_modifier(FunctionID(start_fid))
            .unwrap();
        start.func_entry();
        let _res = emit_stmt(
            stmt,
            self.strategy,
            &mut start,
            &mut EmitCtx::new(
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
                self.utils_adapter,
                self.map_lib_adapter,
                UNEXPECTED_ERR_MSG,
                err,
            ),
        );

        let op_idx = start.curr_instr_len() as u32;
        start.append_tag_at(
            get_probe_tag_data(stmt.loc(), op_idx),
            // location is unused
            wirm::Location::Module {
                func_idx: FunctionID(0),
                instr_idx: 0,
            },
        );
        start.finish_instr();

        if was_created {
            Some(start_fid)
        } else {
            None
        }
    }

    // =============================
    // ==== EMIT `report` LOGIC ====
    // =============================

    pub fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
        err: &mut ErrorGen,
    ) -> Option<FunctionID> {
        self.emit_global_inner(name, ty, val, true, err)
    }

    pub(crate) fn get_or_create_start_func(wasm: &mut Module) -> (u32, bool) {
        let was_created = wasm.start.is_none();
        let fid = *wasm.start.unwrap_or_else(|| {
            let start_func = FunctionBuilder::new(&[], &[]);
            let start_fid = start_func.finish_module_with_tag(wasm, get_tag_for(&None));
            wasm.start = Some(start_fid);
            start_fid
        });

        (fid, was_created)
    }

    fn handle_special_fn_call(&mut self, target_fn_name: String, args: &[Expr]) -> bool {
        // Args are assumed to be already folded by the pre-emit fold pass (FoldPass / WeiGenerator).
        let _folded_args = args;

        match target_fn_name.as_str() {
            "memcpy" => self.handle_memcpy(),
            "write_str" => self.handle_write_str(),
            "read_str" => self.handle_read_str(),
            "dup_at" => unimplemented!("Function not implemented in `wei` yet: {target_fn_name}"),
            "alt_call_by_name" | "alt_call_by_id" | "drop_args" => {
                panic!("Function unsupported in `wei`: {target_fn_name}")
            }
            _ => {
                unreachable!(
                    "{} Could not find handler for static function with name: {}",
                    UNEXPECTED_ERR_MSG, target_fn_name
                );
            }
        }
    }

    fn handle_memcpy(&mut self) -> bool {
        // this is handled in the shared emitter utils
        false
    }

    fn handle_write_str(&mut self) -> bool {
        // this is handled in the shared emitter utils
        false
    }

    fn handle_read_str(&mut self) -> bool {
        // this is handled in the shared emitter utils
        false
    }
}
impl Emitter for ModuleEmitter<'_, '_> {
    fn reset_locals_for_probe(&mut self) {
        if let Some(func) = &mut self.emitting_func {
            self.locals_tracker.reset_probe(func);
        }
    }

    fn reset_locals_for_function(&mut self) {
        self.locals_tracker.reset_function();
    }

    fn emit_body(&mut self, body: &Block, err: &mut ErrorGen) -> bool {
        let mut is_success = true;
        for stmt in body.stmts.iter() {
            is_success &= self.emit_stmt(stmt, err);
        }
        is_success
    }

    fn emit_stmt(&mut self, stmt: &Statement, err: &mut ErrorGen) -> bool {
        // Check if this is calling a bound, static function!
        if let Statement::Expr {
            expr: Expr::Call {
                fn_target, args, ..
            },
            ..
        } = stmt
        {
            let fn_name = match &**fn_target {
                Expr::VarId { name, .. } => name.clone(),
                _ => unreachable!("unexpected type: {fn_target:?}"),
            };
            let Some(Record::Fn { def, .. }) = self.table.lookup_fn(fn_name.as_str(), true) else {
                unreachable!("unexpected type");
            };
            if matches!(def, Definition::CompilerStatic) {
                // We want to handle this as unique logic rather than a simple function call to be emitted
                if self.handle_special_fn_call(fn_name, args) {
                    return true;
                }
            }
        }

        // everything else can be emitted as normal!
        let mut ctx = EmitCtx::new(
            self.table,
            self.mem_allocator,
            &mut self.locals_tracker,
            self.utils_adapter,
            self.map_lib_adapter,
            UNEXPECTED_ERR_MSG,
            err,
        );

        if let Some(emitting_func) = &mut self.emitting_func {
            emit_stmt(stmt, self.strategy, emitting_func, &mut ctx)
        } else {
            false
        }
    }

    fn emit_expr(&mut self, expr: &Expr, err: &mut ErrorGen) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_expr(
                expr,
                None,
                self.strategy,
                emitting_func,
                &mut EmitCtx::new(
                    self.table,
                    self.mem_allocator,
                    &mut self.locals_tracker,
                    self.utils_adapter,
                    self.map_lib_adapter,
                    UNEXPECTED_ERR_MSG,
                    err,
                ),
            )
        } else {
            false
        }
    }
}
