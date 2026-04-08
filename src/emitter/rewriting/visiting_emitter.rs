use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::{
    get_loc_info_for_active_probes, get_ty_info_for_instr, LocInfo, MatchState, ProbeRule, StackVal,
};
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use std::collections::HashMap;
use wirm::ir::types::DataType as WirmType;

use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::{MemoryAllocator, VAR_BLOCK_BASE_VAR};
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::utils::{
    block_type_to_wasm, emit_expr, emit_probes, emit_stack_val, emit_stack_vals, emit_stmt, EmitCtx,
};
use crate::emitter::{configure_flush_routines, Emitter, InjectStrategy};
use crate::generator::ast::UnsharedVar;
use crate::generator::folding::expr::ExprFolder;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::utils::utils_adapter::UtilsAdapter;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::ReportVars;
use crate::parser;
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use itertools::Itertools;
use log::warn;
use std::iter::Iterator;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, LocalID, TypeID};
use wirm::ir::module::Module;
use wirm::ir::types::BlockType as WirmBlockType;
use wirm::iterator::iterator_trait::{IteratingInstrumenter, Iterator as WirmIterator};
use wirm::iterator::module_iterator::ModuleIterator;
use wirm::module_builder::AddLocal;
use wirm::opcode::{Instrumenter, MacroOpcode, Opcode};
use wirm::wasmparser::Operator;
use wirm::Location;

const UNEXPECTED_ERR_MSG: &str =
    "VisitingEmitter: Looks like you've found a bug...please report this behavior!";

pub struct VisitingEmitter<'a, 'ir> {
    pub strategy: InjectStrategy,
    pub app_iter: ModuleIterator<'a, 'ir>,
    pub init_func: &'a mut FunctionBuilder<'ir>,
    pub in_init: bool,

    pub table: &'a mut SymbolTable,
    pub mem_allocator: &'a mut MemoryAllocator,
    pub locals_tracker: LocalsTracker,
    pub init_func_locals_tracker: LocalsTracker,
    pub utils_adapter: &'a mut UtilsAdapter,
    pub map_lib_adapter: &'a mut MapLibAdapter,
    pub io_adapter: &'a mut IOAdapter,
    pub(crate) report_vars: &'a mut ReportVars,
    pub(crate) unshared_var_handler: &'a mut UnsharedVarHandler,
    instr_created_args: Vec<(String, usize)>,
    instr_created_results: Vec<(String, usize)>,
    pub curr_unshared: Vec<UnsharedVar>,

    pub registry: &'a mut WasmRegistry,
}

impl<'a, 'ir> VisitingEmitter<'a, 'ir> {
    // note: only used in integration test
    pub fn new(
        strategy: InjectStrategy,
        app_wasm: &'a mut Module<'ir>,
        init_func: &'a mut FunctionBuilder<'ir>,
        injected_funcs: &Vec<FunctionID>,
        table: &'a mut SymbolTable,
        mem_allocator: &'a mut MemoryAllocator,
        utils_adapter: &'a mut UtilsAdapter,
        map_lib_adapter: &'a mut MapLibAdapter,
        io_adapter: &'a mut IOAdapter,
        report_vars: &'a mut ReportVars,
        unshared_var_handler: &'a mut UnsharedVarHandler,
        registry: &'a mut WasmRegistry,
    ) -> Self {
        Self {
            strategy,
            app_iter: ModuleIterator::new(app_wasm, injected_funcs),
            init_func,
            in_init: false,
            table,
            mem_allocator,
            locals_tracker: LocalsTracker::default(),
            init_func_locals_tracker: LocalsTracker::default(),
            utils_adapter,
            map_lib_adapter,
            io_adapter,
            report_vars,
            unshared_var_handler,
            instr_created_args: vec![],
            instr_created_results: vec![],
            curr_unshared: vec![],
            registry,
        }
    }

    /// bool -> whether there is a next instruction to process
    pub fn next_instr(&mut self) -> bool {
        self.app_iter.next().is_some()
    }

    pub fn before(&mut self) {
        self.app_iter.before();
    }

    pub fn after(&mut self) {
        self.app_iter.after();
    }

    pub fn alternate(&mut self) {
        self.app_iter.alternate();
    }

    pub fn semantic_after(&mut self) {
        self.app_iter.semantic_after();
    }

    pub fn block_entry(&mut self) {
        self.app_iter.block_entry();
    }

    pub fn block_exit(&mut self) {
        self.app_iter.block_exit();
    }

    pub fn block_alt(&mut self) {
        self.app_iter.block_alt();
    }

    pub fn func_entry(&mut self) {
        self.app_iter.func_entry();
    }

    pub fn func_exit(&mut self) {
        self.app_iter.func_exit();
    }

    pub fn finish_instr(&mut self) {
        self.app_iter.finish_instr();
    }

    pub(crate) fn enter_scope_via_rule(
        &mut self,
        script_id: &str,
        probe_rule: &ProbeRule,
        scope_id: usize,
    ) -> bool {
        self.table.enter_scope_via_rule(
            script_id,
            &parser::types::ProbeRule {
                provider: probe_rule.provider.clone(),
                package: probe_rule.package.clone(),
                event: probe_rule.event.clone(),
                mode: Some(RulePart::new(
                    probe_rule.mode.as_ref().unwrap().name(),
                    None,
                )),
            },
            scope_id,
        )
    }

    pub(crate) fn reset_table(&mut self) {
        self.table.reset();
    }

    pub(crate) fn curr_instr_name(&self) -> String {
        if let Some(curr_op) = self.app_iter.curr_op() {
            format!("{:?}", curr_op)
        } else {
            "curr instr not defined".to_string()
        }
    }

    pub(crate) fn get_loc_info(
        &self,
        state: &mut MatchState,
        ast: &mut SimpleAST,
        err: &mut ErrorGen,
    ) -> Option<LocInfo> {
        let (loc, at_func_end) = self.app_iter.curr_loc();

        if let Some(curr_instr) = self.app_iter.curr_op() {
            get_loc_info_for_active_probes(
                self.app_iter.module,
                state,
                loc,
                at_func_end,
                curr_instr,
                ast,
                err,
            )
        } else {
            None
        }
    }

    pub(crate) fn emit_dynamic_compiler_data(
        &mut self,
        data: &HashMap<String, Block>,
        err: &mut ErrorGen,
    ) -> bool {
        let mut is_success = true;
        for (_, Block { stmts, .. }) in data.iter() {
            for stmt in stmts.iter() {
                is_success &= emit_stmt(
                    stmt,
                    self.strategy,
                    &mut self.app_iter,
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
            }
        }
        is_success
    }

    pub(crate) fn save_args(&mut self, args: &[StackVal]) -> bool {
        self.instr_created_args = self.save_stack_vals(args);
        true
    }

    pub(crate) fn emit_args(&mut self, err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot re-emit stack values as a variable initialization.");
            return false;
        }
        emit_stack_vals(
            &self.instr_created_args,
            &mut self.app_iter,
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
        true
    }
    fn emit_arg_n(&mut self, n: usize, err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot re-emit stack values as a variable initialization.");
            return false;
        }

        let (_, param_rec_id) = &self.instr_created_args[n];
        emit_stack_val(
            *param_rec_id,
            &mut self.app_iter,
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
        true
    }

    pub(crate) fn save_results(&mut self, results: &[StackVal]) -> bool {
        self.instr_created_results = self.save_stack_vals(results);
        true
    }

    fn save_stack_vals(&mut self, vals: &[StackVal]) -> Vec<(String, usize)> {
        // No opcodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        let mut recs: Vec<(String, usize)> = vec![]; // vec to retain order!
        let mut locals: Vec<(String, u32)> = vec![];
        vals.iter().for_each(
            |StackVal {
                 name,
                 ty: val_ty,
             }| {
                let ty = if let Some(ty) = val_ty {
                    *ty
                } else {
                    warn!("The current way that probes with polymorphic result types is supported for the bytecode rewriting target is incomplete.\
                           In a future version, we need to have a virtual stack on the side to compute the actual result type and compare with the \
                           result bounds of the probe to see if the location is a map. For now, it may generate invalid instrumented modules!");
                    let Some(Record::Var {
                                 ty,
                                 ..
                             }) = self.table.lookup_var(name, true)
                    else {
                        unreachable!("unexpected type");
                    };
                    let wasm_ty = if ty.to_wasm_type().len() > 1 {
                        unimplemented!()
                    } else {
                        *ty.to_wasm_type().first().unwrap()
                    };
                    wasm_ty
                };
                // create local for the result in the module
                let local_id = LocalID(self.locals_tracker.use_local(ty, &mut self.app_iter));
                locals.push((name.to_string(), *local_id));
            },
        );

        // Save vals in reverse order (the leftmost val is at the bottom of the stack)
        locals.iter().for_each(|(name, local_id)| {
            // emit an opcode in the event to assign the ToS to this new local
            self.app_iter.local_set(LocalID(*local_id));
            let new_addr = Some(vec![VarAddr::Local { addr: *local_id }]);

            // place in symbol table with var addr for future reference
            let rec = self.table.lookup_var_with_id_mut(name, false);
            let id = if let Some((id, Record::Var { addr, .. })) = rec {
                *addr = new_addr;
                id
            } else {
                self.table.put(
                    name.to_string(),
                    Record::Var {
                        ty: DataType::I32, // TODO we only support integers right now. Need an abstract interpreter!
                        value: None,
                        def: Definition::CompilerDynamic,
                        addr: new_addr,
                        times_set: 0,
                        loc: None,
                    },
                )
            };

            recs.insert(0, (name.to_string(), id));
        });

        recs
    }

    pub(crate) fn emit_results(&mut self, err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot re-emit stack values as a variable initialization.");
            return false;
        }
        emit_stack_vals(
            &self.instr_created_results,
            &mut self.app_iter,
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
        true
    }

    pub(crate) fn emit_empty_alternate(&mut self) -> Result<bool, Box<WhammError>> {
        self.app_iter.empty_alternate();
        Ok(true)
    }

    pub(crate) fn emit_empty_block_alt(&mut self) -> Result<bool, Box<WhammError>> {
        self.app_iter.empty_block_alt();
        Ok(true)
    }

    pub(crate) fn define_alias(
        &mut self,
        var_name: &str,
        var_ty: &WirmType,
        alias_addr: &VarAddr,
    ) -> bool {
        self.table.override_record_addr(
            var_name,
            DataType::from_wasm_type(var_ty),
            Some(vec![alias_addr.clone()]),
        );
        true
    }

    pub(crate) fn reset_table_data(&mut self, loc_info: &LocInfo) {
        // reset static_data
        self.table
            .reset_record_vals(&loc_info.static_data.keys().collect_vec());

        // reset dynamic_alias
        loc_info
            .dynamic_alias
            .iter()
            .for_each(|(symbol_name, (ty, ..))| {
                self.table
                    .override_record_addr(symbol_name, DataType::from_wasm_type(ty), None);
            });

        // reset dynamic_data
        let mut override_val = |name: &str| {
            self.table.override_record_val(name, None, false);
        };
        loc_info.dynamic_data.iter().for_each(|(symbol_name, ..)| {
            override_val(symbol_name);
        });
        for i in 0..loc_info.args.len() {
            override_val(&format!("arg{i}"));
        }
        for i in 0..loc_info.results.len() {
            override_val(&format!("res{i}"));
        }
        self.instr_created_args.clear();
        self.instr_created_results.clear();
    }

    pub(crate) fn fold_expr(&mut self, expr: &Expr, err: &mut ErrorGen) -> Expr {
        ExprFolder::fold_expr(
            expr,
            self.registry,
            false,
            self.table,
            &self.mem_allocator.emitted_strings,
            self.app_iter.module,
            err,
        )
    }

    pub fn emit_orig(&mut self) -> bool {
        // ORCA TODO: can i get around this curr_op_owned() thing by curr_op?
        let orig = self.app_iter.curr_op_owned().unwrap().clone();
        let loc = self.app_iter.curr_loc().0;
        self.app_iter.add_instr_at(loc, orig);
        true
    }

    pub fn emit_if(
        &mut self,
        condition: &Expr,
        conseq: &Block,
        err: &mut ErrorGen,
    ) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition, err);

        // emit the beginning of the if block

        self.app_iter.if_stmt(block_type_to_wasm(conseq));

        is_success &= self.emit_body(conseq, err);

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    pub(crate) fn emit_if_with_orig_as_else(
        &mut self,
        condition: &Expr,
        conseq: &Block,
        err: &mut ErrorGen,
    ) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        // The consequent and alternate blocks must have the same type...
        // this means that the result of the `if` should be the same as
        // the result of the original instruction!
        let (curr_loc, _) = self.app_iter.curr_loc();
        let fid = match curr_loc {
            Location::Module { func_idx, .. } | Location::Component { func_idx, .. } => func_idx,
        };
        let orig_ty_id =
            get_ty_info_for_instr(self.app_iter.module, &fid, self.app_iter.curr_op().unwrap()).2;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition, err);
        // emit the beginning of the if block

        let block_ty = match orig_ty_id {
            Some(ty_id) => {
                let ty = match self.app_iter.module.types.get(TypeID(ty_id)) {
                    Some(ty) => ty.results().unwrap().clone(),
                    None => vec![],
                };

                // we only care about the result of the original
                WirmBlockType::FuncType(self.app_iter.module.types.add_func_type_with_tag(
                    &[],
                    &ty,
                    get_tag_for(&None),
                ))
            }
            None => WirmBlockType::Empty,
        };
        self.app_iter.if_stmt(block_ty);
        is_success &= self.emit_body(conseq, err);

        // emit the beginning of the else
        self.app_iter.else_stmt();

        is_success &= self.emit_args(err);
        is_success &= self.emit_orig();

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    fn handle_dup_at(&mut self, args: &[Expr], err: &mut ErrorGen) -> bool {
        // args: vec![dst_mem: u32, dst_addr: u32]
        // Assume the correct args since we've gone through typechecking at this point!

        // args[0] get target mem_id
        let dst_mem = ExprFolder::get_u32(&args[0]).unwrap();

        // emit: args[1] (dst addr)
        let dst_addr = &args[1];
        self.emit_expr(dst_addr, err);

        // emit: `arg1` (src addr)
        self.emit_arg_n(1, err);
        // emit: `arg2` (len)
        self.emit_arg_n(2, err);

        // duplicate the opcode at the target instr location with new mem_id
        let mut orig = self.app_iter.curr_op_owned().unwrap().clone();
        override_dst_mem(&mut orig, dst_mem);

        let loc = self.app_iter.curr_loc().0;
        self.app_iter.add_instr_at(loc, orig);

        true
    }

    fn handle_alt_call_by_name(&mut self, args: &[Expr], err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot call `alt_call_by_name` as a variable initialization.");
            return false;
        }
        // args: vec![func_name: String]
        // Assume the correct args since we've gone through typechecking at this point!
        let fn_name = match args.first().unwrap() {
            Expr::Primitive {
                val: Value::Str { val, .. },
                ..
            } => val.clone(),
            _ => return false,
        };

        if let Some(func_id) = self
            .app_iter
            .module
            .functions
            .get_local_fid_by_name(fn_name.as_str())
        {
            let is_success = self.emit_args(err);
            self.app_iter.call(func_id);
            is_success
        } else {
            unreachable!(
                "{} Could not find alt function call by name: {}",
                UNEXPECTED_ERR_MSG, fn_name
            );
        }
    }

    fn handle_alt_call_by_id(&mut self, args: &[Expr], err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot call `alt_call_by_id` as a variable initialization.");
            return false;
        }
        // args: vec![func_id: i32]
        // Assume the correct args since we've gone through typechecking at this point!
        let func_id = match args.first().unwrap() {
            Expr::Primitive {
                val:
                    Value::Number {
                        val: NumLit::I32 { val },
                        ..
                    },
                ..
            } => *val,
            _ => return false,
        };

        let is_success = self.emit_args(err);
        self.app_iter.call(FunctionID(func_id as u32));
        is_success
    }

    fn handle_drop_args(&mut self, err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot call `drop_args` as a variable initialization.");
            return false;
        }
        // Generate drops for all args to this opcode!

        let fid = match self.app_iter.curr_loc().0 {
            Location::Module { func_idx, .. } | Location::Component { func_idx, .. } => func_idx,
        };

        // ensure we have the args for this instruction
        let curr_instr_args =
            get_ty_info_for_instr(self.app_iter.module, &fid, self.app_iter.curr_op().unwrap()).0;

        let num_to_drop = curr_instr_args.len() - self.instr_created_args.len();
        for _arg in 0..num_to_drop {
            self.app_iter.drop();
        }
        true
    }

    fn handle_memcpy(&mut self) -> bool {
        // this is handled in the shared emitter utils
        false
    }

    fn handle_mem_size(&mut self) -> bool {
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

    // Args are assumed to be already folded by the pre-emit fold pass in InstrGenerator.
    fn handle_special_fn_call(
        &mut self,
        target_fn_name: String,
        args: &[Expr],
        err: &mut ErrorGen,
    ) -> bool {
        match target_fn_name.as_str() {
            "dup_at" => self.handle_dup_at(args, err),
            "alt_call_by_name" => self.handle_alt_call_by_name(args, err),
            "alt_call_by_id" => self.handle_alt_call_by_id(args, err),
            "drop_args" => self.handle_drop_args(err),
            "memcpy" => self.handle_memcpy(),
            "mem_size" => self.handle_mem_size(),
            "write_str" => self.handle_write_str(),
            "read_str" => self.handle_read_str(),
            _ => {
                unreachable!(
                    "{} Could not find handler for static function with name: {}",
                    UNEXPECTED_ERR_MSG, target_fn_name
                );
            }
        }
    }

    pub fn configure_flush_routines(
        &mut self,
        has_reports: bool,
        err: &mut ErrorGen,
        ast: &mut SimpleAST,
    ) {
        // create the function to call at the end
        // TODO -- this can be cleaned up to use the wei logic instead!
        // check if ast overrides report logic
        let report_probe = if let Some(wasm) = ast.provs.get_mut("wasm") {
            if let Some(report_probe) = wasm.pkgs.get_mut("report") {
                let report_probes = report_probe
                    .evts
                    .get_mut("")
                    .unwrap()
                    .modes
                    .get_mut(&ModeKind::Null)
                    .unwrap();

                // enter mode scope
                let probe = report_probes.first().unwrap();
                assert!(
                    self.enter_scope_via_rule(
                        &probe.script_id.to_string(),
                        &(&probe.rule).into(),
                        probe.scope_id
                    ),
                    "Failed to enter scope"
                );
                self.table
                    .enter_named_scope(&probe.probe_number.to_string()); // enter probe scope

                Some(report_probes)
            } else {
                None
            }
        } else {
            None
        };

        if has_reports || report_probe.is_some() {
            let var_flush_fid = if report_probe.is_none() {
                configure_flush_routines(
                    self.app_iter.module,
                    self.unshared_var_handler,
                    self.report_vars,
                    self.map_lib_adapter,
                    self.mem_allocator,
                    self.io_adapter,
                    err,
                )
            } else {
                None
            };

            let on_exit_id = if let Some(fid) = self
                .app_iter
                .module
                .functions
                .get_local_fid_by_name("on_exit")
            {
                let Ok(mut on_exit) = self.app_iter.module.functions.get_fn_modifier(fid) else {
                    panic!(
                        "{UNEXPECTED_ERR_MSG} \
                                No on_exit found in the module!"
                    );
                };
                if let Some(probes) = report_probe {
                    // handle wasm:report override
                    emit_probes(
                        probes,
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
                } else {
                    // there wasn't an override for wasm:report, emit the default reporting logic
                    if let Some(flush_fid) = var_flush_fid {
                        on_exit.call(FunctionID(flush_fid));
                        let op_idx = on_exit.curr_instr_len() as u32;
                        on_exit.append_tag_at(
                            get_probe_tag_data(&None, op_idx),
                            Location::Module {
                                func_idx: FunctionID(*fid),
                                instr_idx: 0,
                            },
                        );
                    }
                }
                fid
            } else {
                let mut on_exit = FunctionBuilder::new(&[], &[]);

                if let Some(probes) = report_probe {
                    // handle wasm:report override
                    emit_probes(
                        probes,
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
                } else {
                    // there wasn't an override for wasm:report, emit the default reporting logic
                    if let Some(flush_fid) = var_flush_fid {
                        on_exit.call(FunctionID(flush_fid));
                    }
                }
                let on_exit_id =
                    on_exit.finish_module_with_tag(self.app_iter.module, get_tag_for(&None));
                self.app_iter
                    .module
                    .set_fn_name(on_exit_id, "on_exit".to_string());
                on_exit_id
            };

            // now find where the "exit" is in the bytecode
            // exit of export "main"
            // OR if that doesn't exist, the end of the "start" function
            let fid = if let Some(main_fid) = self
                .app_iter
                .module
                .exports
                .get_func_by_name("main".to_string())
            {
                main_fid
            } else if let Some(main_fid) = self
                .app_iter
                .module
                .exports
                .get_func_by_name("_start".to_string())
            {
                main_fid
            } else if let Some(start_fid) = self.app_iter.module.start {
                start_fid
            } else {
                // neither exists, unsure how to support this...this would be a library instead of an application I guess?
                // Maybe the answer is to expose query functions that can give a status update of the `report` vars?
                unimplemented!("Your target Wasm has no main or start function...we do not support report variables in this scenario.")
            };
            let mut main = self.app_iter.module.functions.get_fn_modifier(fid).unwrap();

            main.func_exit();
            main.call(on_exit_id);
            let op_idx = main.curr_instr_len() as u32;
            main.append_tag_at(
                get_probe_tag_data(&None, op_idx),
                // location is unused
                Location::Module {
                    func_idx: FunctionID(0),
                    instr_idx: 0,
                },
            );

            main.finish_instr();
        }
    }

    pub fn init_probe_state(&mut self, init_logic: &mut [Statement], err: &mut ErrorGen) {
        // Create the variable pointing to the start of the allocated memory block
        let offset_info = if !self.curr_unshared.is_empty() {
            // Create the required globals for this probe
            // Sort by datatype to make generation deterministic!
            // translate unshared vars to the correct format
            let sorted_unshared = self
                .curr_unshared
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.ty, &b.ty));

            let loc = self.app_iter.curr_loc().0;
            let (fid, pc) = match loc {
                Location::Module { func_idx, .. } | Location::Component { func_idx, .. } => (
                    *func_idx,
                    VisitingEmitter::lookup_pc_offset_for(self.app_iter.module, &loc),
                ),
            };
            let fname = self
                .app_iter
                .module
                .functions
                .get_name(FunctionID(fid))
                .clone()
                .unwrap_or_default();

            let offset_value = self.unshared_var_handler.get_curr_offset();
            self.unshared_var_handler.allocate_vars(
                sorted_unshared.as_slice(),
                &fname,
                fid,
                pc,
                self.table,
                self.mem_allocator,
                self.map_lib_adapter,
                self.report_vars,
                self.app_iter.module,
                err,
            );

            let id_probe = self
                .locals_tracker
                .use_local(WirmType::I32, &mut self.app_iter);
            let addr_probe = VarAddr::Local { addr: id_probe };

            // Define the memory address in the table for the state initialization logic.
            if !init_logic.is_empty() {
                let id_init = *self.init_func.add_local(WirmType::I32);
                let addr_init = VarAddr::Local { addr: id_init };
                redefine_offset(
                    offset_value,
                    addr_init,
                    id_init,
                    VAR_BLOCK_BASE_VAR,
                    self.table,
                    self.init_func,
                );
            }

            Some((id_probe, addr_probe, offset_value))
        } else {
            None
        };

        // now that the unshared variables have been allocated, I need to emit
        // initialization logic into the instr_init function
        if !init_logic.is_empty() {
            self.in_init = true;
            for stmt in init_logic.iter() {
                self.emit_stmt(stmt, err);
            }
            self.in_init = false;
        }

        // If there was an offset override, now redefine it for the probe body
        if let Some((id, local, offset)) = offset_info {
            redefine_offset(
                offset,
                local,
                id,
                VAR_BLOCK_BASE_VAR,
                self.table,
                &mut self.app_iter,
            );
        }

        fn redefine_offset<'a, T: Opcode<'a> + MacroOpcode<'a>>(
            offset: u32,
            local: VarAddr,
            id: u32,
            var_name: &str,
            table: &mut SymbolTable,
            target_func: &mut T,
        ) {
            if let Some(Record::Var { value, addr, .. }) = table.lookup_var_mut(var_name, false) {
                *value = Some(Value::gen_u32(offset));
                *addr = Some(vec![local]);
            } else {
                table.put(
                    VAR_BLOCK_BASE_VAR.to_string(),
                    Record::Var {
                        ty: DataType::I32,
                        value: Some(Value::gen_u32(offset)),
                        def: Definition::CompilerStatic,
                        addr: Some(vec![local]),
                        times_set: 0,
                        loc: None,
                    },
                );
            };
            target_func.u32_const(offset).local_set(LocalID(id));
        }
    }

    pub fn lookup_pc_offset_for(wasm: &Module, loc: &Location) -> u32 {
        match loc {
            Location::Module {
                func_idx,
                instr_idx,
                ..
            }
            | Location::Component {
                func_idx,
                instr_idx,
                ..
            } =>
            // increment by one to match with Wizard definition (points to right after the opcode)
            {
                wasm.functions
                    .unwrap_local(*func_idx)
                    .unwrap()
                    .lookup_pc_offset_for(*instr_idx)
                    .unwrap() as u32
                    + 1
            }
        }
    }
}
impl Emitter for VisitingEmitter<'_, '_> {
    fn reset_locals_for_probe(&mut self) {
        self.locals_tracker.reset_probe(&mut self.app_iter);
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
            let (def, ret_ty) = if let Some(Record::Fn { def, ret_ty, .. }) =
                self.table.lookup_fn(fn_name.as_str(), true)
            {
                (*def, ret_ty.clone())
            } else {
                unreachable!("unexpected type");
            };
            if matches!(def, Definition::CompilerStatic) {
                // We want to handle this as unique logic rather than a simple function call to be emitted
                if self.handle_special_fn_call(fn_name, args, err) {
                    if self.in_init {
                        crate::emitter::utils::drop_results(&ret_ty, self.init_func);
                    } else {
                        crate::emitter::utils::drop_results(&ret_ty, &mut self.app_iter);
                    }
                    return true;
                }
            }
        }

        // everything else can be emitted as normal!
        let mut ctx = EmitCtx::new(
            self.table,
            self.mem_allocator,
            if self.in_init {
                &mut self.init_func_locals_tracker
            } else {
                &mut self.locals_tracker
            },
            self.utils_adapter,
            self.map_lib_adapter,
            UNEXPECTED_ERR_MSG,
            err,
        );
        if self.in_init {
            emit_stmt(stmt, self.strategy, self.init_func, &mut ctx)
        } else {
            emit_stmt(stmt, self.strategy, &mut self.app_iter, &mut ctx)
        }
    }

    fn emit_expr(&mut self, expr: &Expr, err: &mut ErrorGen) -> bool {
        emit_expr(
            expr,
            None,
            self.strategy,
            &mut self.app_iter,
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
    }
}

fn override_dst_mem<'a>(op: &mut Operator<'a>, dst_mem: u32) {
    let memarg = match op {
        // --- Bulk mem ops ---
        Operator::MemoryCopy { dst_mem: mem, .. }
        | Operator::MemoryFill { mem }
        | Operator::MemoryInit { mem, .. } => {
            *mem = dst_mem;
            return
        },

        // --- Scalar loads ---
        Operator::I32Load { memarg }
        | Operator::I64Load { memarg }
        | Operator::F32Load { memarg }
        | Operator::F64Load { memarg }
        | Operator::I32Load8S { memarg }
        | Operator::I32Load8U { memarg }
        | Operator::I32Load16S { memarg }
        | Operator::I32Load16U { memarg }
        | Operator::I64Load8S { memarg }
        | Operator::I64Load8U { memarg }
        | Operator::I64Load16S { memarg }
        | Operator::I64Load16U { memarg }
        | Operator::I64Load32S { memarg }
        | Operator::I64Load32U { memarg }

        // --- Scalar stores ---
        | Operator::I32Store { memarg }
        | Operator::I64Store { memarg }
        | Operator::F32Store { memarg }
        | Operator::F64Store { memarg }
        | Operator::I32Store8 { memarg }
        | Operator::I32Store16 { memarg }
        | Operator::I64Store8 { memarg }
        | Operator::I64Store16 { memarg }
        | Operator::I64Store32 { memarg }

        // --- SIMD loads ---
        | Operator::V128Load { memarg }
        | Operator::V128Load8x8S { memarg }
        | Operator::V128Load8x8U { memarg }
        | Operator::V128Load16x4S { memarg }
        | Operator::V128Load16x4U { memarg }
        | Operator::V128Load32x2S { memarg }
        | Operator::V128Load32x2U { memarg }
        | Operator::V128Load8Splat { memarg }
        | Operator::V128Load16Splat { memarg }
        | Operator::V128Load32Splat { memarg }
        | Operator::V128Load64Splat { memarg }
        | Operator::V128Load32Zero { memarg }
        | Operator::V128Load64Zero { memarg }

        // --- SIMD stores ---
        | Operator::V128Store { memarg }
        | Operator::V128Store8Lane { memarg, .. }
        | Operator::V128Store16Lane { memarg, .. }
        | Operator::V128Store32Lane { memarg, .. }
        | Operator::V128Store64Lane { memarg, .. }

        // --- Atomic loads ---
        | Operator::I32AtomicLoad { memarg }
        | Operator::I64AtomicLoad { memarg }
        | Operator::I32AtomicLoad8U { memarg }
        | Operator::I32AtomicLoad16U { memarg }
        | Operator::I64AtomicLoad8U { memarg }
        | Operator::I64AtomicLoad16U { memarg }
        | Operator::I64AtomicLoad32U { memarg }

        // --- Atomic stores ---
        | Operator::I32AtomicStore { memarg }
        | Operator::I64AtomicStore { memarg }
        | Operator::I32AtomicStore8 { memarg }
        | Operator::I32AtomicStore16 { memarg }
        | Operator::I64AtomicStore8 { memarg }
        | Operator::I64AtomicStore16 { memarg }
        | Operator::I64AtomicStore32 { memarg }

        // --- Atomic RMW ---
        | Operator::I32AtomicRmwAdd { memarg }
        | Operator::I64AtomicRmwAdd { memarg }
        | Operator::I32AtomicRmw8AddU { memarg }
        | Operator::I32AtomicRmw16AddU { memarg }
        | Operator::I64AtomicRmw8AddU { memarg }
        | Operator::I64AtomicRmw16AddU { memarg }
        | Operator::I64AtomicRmw32AddU { memarg }

        | Operator::I32AtomicRmwSub { memarg }
        | Operator::I64AtomicRmwSub { memarg }
        | Operator::I32AtomicRmw8SubU { memarg }
        | Operator::I32AtomicRmw16SubU { memarg }
        | Operator::I64AtomicRmw8SubU { memarg }
        | Operator::I64AtomicRmw16SubU { memarg }
        | Operator::I64AtomicRmw32SubU { memarg }

        | Operator::I32AtomicRmwAnd { memarg }
        | Operator::I64AtomicRmwAnd { memarg }
        | Operator::I32AtomicRmw8AndU { memarg }
        | Operator::I32AtomicRmw16AndU { memarg }
        | Operator::I64AtomicRmw8AndU { memarg }
        | Operator::I64AtomicRmw16AndU { memarg }
        | Operator::I64AtomicRmw32AndU { memarg }

        | Operator::I32AtomicRmwOr { memarg }
        | Operator::I64AtomicRmwOr { memarg }
        | Operator::I32AtomicRmw8OrU { memarg }
        | Operator::I32AtomicRmw16OrU { memarg }
        | Operator::I64AtomicRmw8OrU { memarg }
        | Operator::I64AtomicRmw16OrU { memarg }
        | Operator::I64AtomicRmw32OrU { memarg }

        | Operator::I32AtomicRmwXor { memarg }
        | Operator::I64AtomicRmwXor { memarg }
        | Operator::I32AtomicRmw8XorU { memarg }
        | Operator::I32AtomicRmw16XorU { memarg }
        | Operator::I64AtomicRmw8XorU { memarg }
        | Operator::I64AtomicRmw16XorU { memarg }
        | Operator::I64AtomicRmw32XorU { memarg }

        | Operator::I32AtomicRmwXchg { memarg }
        | Operator::I64AtomicRmwXchg { memarg }
        | Operator::I32AtomicRmw8XchgU { memarg }
        | Operator::I32AtomicRmw16XchgU { memarg }
        | Operator::I64AtomicRmw8XchgU { memarg }
        | Operator::I64AtomicRmw16XchgU { memarg }
        | Operator::I64AtomicRmw32XchgU { memarg }

        | Operator::I32AtomicRmwCmpxchg { memarg }
        | Operator::I64AtomicRmwCmpxchg { memarg }
        | Operator::I32AtomicRmw8CmpxchgU { memarg }
        | Operator::I32AtomicRmw16CmpxchgU { memarg }
        | Operator::I64AtomicRmw8CmpxchgU { memarg }
        | Operator::I64AtomicRmw16CmpxchgU { memarg }
        | Operator::I64AtomicRmw32CmpxchgU { memarg }

        // --- Wait ops ---
        | Operator::MemoryAtomicWait32 { memarg }
        | Operator::MemoryAtomicWait64 { memarg }

        => memarg,

        _ => panic!("Operator does not contain a MemArg: {:?}", op),
    };

    memarg.memory = dst_mem;
}
