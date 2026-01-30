use crate::common::error::ErrorGen;
use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::{MemoryAllocator, VAR_BLOCK_BASE_VAR};
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::utils::{
    EmitCtx, emit_body, emit_expr, emit_global_getter, emit_probes, emit_stmt,
    whamm_type_to_wasm_global,
};
use crate::emitter::{Emitter, InjectStrategy};
use crate::generator::ast::{Probe, Script, WhammParams};
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::{Metadata, ReportVars};
use crate::parser::types::{Block, DataType, Definition, Expr, Fn, Location, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use log::debug;
use std::collections::HashSet;
use wirm::InitInstr;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, LocalID};
use wirm::ir::module::Module;
use wirm::ir::types::{
    BlockType as WirmBlockType, DataType as WirmType, InitExpr, Value as WirmValue,
};
use wirm::module_builder::AddLocal;
use wirm::opcode::{Instrumenter, Opcode};
use wirm::wasmparser::MemArg;

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub strategy: InjectStrategy,
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,
    pub mem_allocator: &'d mut MemoryAllocator,
    pub locals_tracker: LocalsTracker,
    pub map_lib_adapter: &'e mut MapLibAdapter,
    pub report_vars: &'f mut ReportVars,
    pub registry: &'g mut WasmRegistry,
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
        registry: &'g mut WasmRegistry,
    ) -> Self {
        Self {
            strategy,
            app_wasm,
            emitting_func: None,
            mem_allocator,
            locals_tracker: LocalsTracker::default(),
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
        dynamic_pred: Option<&mut Expr>,
        results: &[WirmType],
        body: &mut Block,
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
        dynamic_pred: Option<&mut Expr>,
        results: &[WirmType],
        block: &mut Block,
        export: bool,
        loc: &Option<Location>,
        err: &mut ErrorGen,
    ) -> Option<u32> {
        let func = FunctionBuilder::new(params, results);
        self.emitting_func = Some(func);

        if let Some(dynamic_pred) = dynamic_pred {
            // overwrite the body by wrapping it with the predicate conditional!
            *block = Block {
                stmts: vec![Statement::If {
                    cond: dynamic_pred.clone(),
                    conseq: block.clone(),
                    alt: Block::default(),
                    loc: None,
                }],
                results: None,
                loc: None,
            };
        }

        // emit the function body
        self.emit_body(block, err);

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

    pub(crate) fn emit_bound_fn(&mut self, context: &str, f: &Fn) -> Option<FunctionID> {
        if context == "whamm" && f.name.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f)
        } else {
            panic!(
                "Provided function, but could not find a context to provide the definition, context: {context}"
            );
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
                            self.registry,
                            self.table,
                            self.mem_allocator,
                            &mut self.locals_tracker,
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
                let (global_id, global_ty) =
                    whamm_type_to_wasm_global(self.app_wasm, ty, loc, None);
                *addr = Some(vec![VarAddr::Global { addr: *global_id }]);
                //now save off the global variable metadata
                if report_mode {
                    self.report_vars
                        .put_global_metadata(*global_id, name.clone(), ty);
                }
                Some(emit_global_getter(
                    self.app_wasm,
                    &global_id,
                    name,
                    global_ty,
                    loc,
                ))
            }
        }
    }

    // ========================================
    // ==== EMIT `global` Statements LOGIC ====
    // ========================================

    /// It is assumed that the statement passed here is a VALID global statement!
    /// (we've gone through several checks before this)
    /// Returns the start_fid (if it was created)
    pub fn emit_global_stmt(&mut self, stmt: &mut Statement, err: &mut ErrorGen) -> Option<u32> {
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
                self.registry,
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
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

        if was_created { Some(start_fid) } else { None }
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
}
impl Emitter for ModuleEmitter<'_, '_, '_, '_, '_, '_, '_> {
    fn reset_locals_for_probe(&mut self) {
        if let Some(func) = &mut self.emitting_func {
            self.locals_tracker.reset_probe(func);
        }
    }

    fn reset_locals_for_function(&mut self) {
        self.locals_tracker.reset_function();
    }
    fn emit_body(&mut self, body: &mut Block, err: &mut ErrorGen) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_body(
                body,
                self.strategy,
                emitting_func,
                &mut EmitCtx::new(
                    self.registry,
                    self.table,
                    self.mem_allocator,
                    &mut self.locals_tracker,
                    self.map_lib_adapter,
                    UNEXPECTED_ERR_MSG,
                    err,
                ),
            )
        } else {
            false
        }
    }

    fn emit_stmt(&mut self, stmt: &mut Statement, err: &mut ErrorGen) -> bool {
        if let Some(emitting_func) = &mut self.emitting_func {
            emit_stmt(
                stmt,
                self.strategy,
                emitting_func,
                &mut EmitCtx::new(
                    self.registry,
                    self.table,
                    self.mem_allocator,
                    &mut self.locals_tracker,
                    self.map_lib_adapter,
                    UNEXPECTED_ERR_MSG,
                    err,
                ),
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
                &mut EmitCtx::new(
                    self.registry,
                    self.table,
                    self.mem_allocator,
                    &mut self.locals_tracker,
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
