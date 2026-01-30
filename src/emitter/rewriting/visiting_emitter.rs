use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::{
    LocInfo, MatchState, ProbeRule, StackVal, get_loc_info_for_active_probes, get_ty_info_for_instr,
};
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use std::collections::HashMap;
use wirm::ir::types::DataType as WirmType;

use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::{MemoryAllocator, VAR_BLOCK_BASE_VAR};
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::utils::{
    EmitCtx, block_type_to_wasm, emit_expr, emit_probes, emit_stack_vals, emit_stmt,
};
use crate::emitter::{Emitter, InjectStrategy, configure_flush_routines};
use crate::generator::ast::UnsharedVar;
use crate::generator::folding::expr::ExprFolder;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::ReportVars;
use crate::parser;
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use itertools::Itertools;
use log::warn;
use std::iter::Iterator;
use wirm::Location;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, LocalID, TypeID};
use wirm::ir::module::Module;
use wirm::ir::types::BlockType as WirmBlockType;
use wirm::iterator::iterator_trait::{IteratingInstrumenter, Iterator as WirmIterator};
use wirm::iterator::module_iterator::ModuleIterator;
use wirm::module_builder::AddLocal;
use wirm::opcode::{Instrumenter, MacroOpcode, Opcode};

const UNEXPECTED_ERR_MSG: &str =
    "VisitingEmitter: Looks like you've found a bug...please report this behavior!";

pub struct VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    pub strategy: InjectStrategy,
    pub app_iter: ModuleIterator<'a, 'b>,
    pub init_func: &'c mut FunctionBuilder<'d>,
    pub in_init: bool,

    pub table: &'e mut SymbolTable,
    pub mem_allocator: &'f mut MemoryAllocator,
    pub locals_tracker: LocalsTracker,
    pub map_lib_adapter: &'g mut MapLibAdapter,
    pub io_adapter: &'h mut IOAdapter,
    pub(crate) report_vars: &'i mut ReportVars,
    pub(crate) unshared_var_handler: &'i mut UnsharedVarHandler,
    instr_created_args: Vec<(String, usize)>,
    instr_created_results: Vec<(String, usize)>,
    pub curr_unshared: Vec<UnsharedVar>,

    pub registry: &'j mut WasmRegistry,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
    VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j>
{
    // note: only used in integration test
    pub fn new(
        strategy: InjectStrategy,
        app_wasm: &'a mut Module<'b>,
        init_func: &'c mut FunctionBuilder<'d>,
        injected_funcs: &Vec<FunctionID>,
        table: &'e mut SymbolTable,
        mem_allocator: &'f mut MemoryAllocator,
        map_lib_adapter: &'g mut MapLibAdapter,
        io_adapter: &'h mut IOAdapter,
        report_vars: &'i mut ReportVars,
        unshared_var_handler: &'i mut UnsharedVarHandler,
        registry: &'j mut WasmRegistry,
    ) -> Self {
        Self {
            strategy,
            app_iter: ModuleIterator::new(app_wasm, injected_funcs),
            init_func,
            in_init: false,
            table,
            mem_allocator,
            locals_tracker: LocalsTracker::default(),
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
                    &mut stmt.clone(),
                    self.strategy,
                    &mut self.app_iter,
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
                self.registry,
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
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

            // place in symbol table with var addr for future reference
            let id = self.table.put(
                name.to_string(),
                Record::Var {
                    ty: DataType::I32, // TODO we only support integers right now.
                    value: None,
                    def: Definition::User,
                    addr: Some(vec![VarAddr::Local { addr: *local_id }]),
                    loc: None,
                },
            );
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
                self.registry,
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
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

    pub(crate) fn define_data(&mut self, var_name: &str, var_val: &Option<Value>) -> bool {
        // if the record doesn't exist, it's from a different probe being active
        // at this place in the target application. We can just ignore this (it
        // won't be defined)...it doesn't matter since we do typechecking :)
        self.table
            .override_record_val(var_name, var_val.clone(), false);
        true
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
        loc_info.static_data.iter().for_each(|(symbol_name, ..)| {
            self.table.override_record_val(symbol_name, None, false);
        });

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
    }

    pub(crate) fn fold_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        // todo -- create actual registry
        *expr = ExprFolder::fold_expr(expr, self.registry, false, self.table, err);
        true
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
        condition: &mut Expr,
        conseq: &mut Block,
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
        condition: &mut Expr,
        conseq: &mut Block,
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
                    Some(ty) => ty.results().clone(),
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

    fn handle_alt_call_by_name(&mut self, args: &mut [Expr], err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot call `alt_call_by_name` as a variable initialization.");
            return false;
        }
        // args: vec![func_name: String]
        // Assume the correct args since we've gone through typechecking at this point!
        let fn_name = match args.iter().next().unwrap() {
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

    fn handle_alt_call_by_id(&mut self, args: &mut [Expr], err: &mut ErrorGen) -> bool {
        if self.in_init {
            err.add_instr_error("Cannot call `alt_call_by_name` as a variable initialization.");
            return false;
        }
        // args: vec![func_id: i32]
        // Assume the correct args since we've gone through typechecking at this point!
        let func_id = match args.iter().next().unwrap() {
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

    fn handle_special_fn_call(
        &mut self,
        target_fn_name: String,
        args: &mut [Expr],
        err: &mut ErrorGen,
    ) -> bool {
        match target_fn_name.as_str() {
            "alt_call_by_name" => self.handle_alt_call_by_name(args, err),
            "alt_call_by_id" => self.handle_alt_call_by_id(args, err),
            "drop_args" => self.handle_drop_args(err),
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
                Some(
                    report_probe
                        .evts
                        .get_mut("")
                        .unwrap()
                        .modes
                        .get_mut(&ModeKind::Null)
                        .unwrap(),
                )
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
                let Some(mut on_exit) = self.app_iter.module.functions.get_fn_modifier(fid) else {
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
                            self.registry,
                            self.table,
                            self.mem_allocator,
                            &mut self.locals_tracker,
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
                            self.registry,
                            self.table,
                            self.mem_allocator,
                            &mut self.locals_tracker,
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
                unimplemented!(
                    "Your target Wasm has no main or start function...we do not support report variables in this scenario."
                )
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
            for stmt in init_logic.iter_mut() {
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
                    .lookup_pc_offset_for(*instr_idx)
                    .unwrap() as u32
                    + 1
            }
        }
    }
}
impl Emitter for VisitingEmitter<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn reset_locals_for_probe(&mut self) {
        self.locals_tracker.reset_probe(&mut self.app_iter);
    }

    fn reset_locals_for_function(&mut self) {
        self.locals_tracker.reset_function();
    }

    fn emit_body(&mut self, body: &mut Block, err: &mut ErrorGen) -> bool {
        let mut is_success = true;

        for stmt in body.stmts.iter_mut() {
            is_success &= self.emit_stmt(stmt, err);
        }
        is_success
    }

    fn emit_stmt(&mut self, stmt: &mut Statement, err: &mut ErrorGen) -> bool {
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
                _ => return false,
            };
            let Some(Record::Fn { def, .. }) = self.table.lookup_fn(fn_name.as_str(), true) else {
                unreachable!("unexpected type");
            };
            if matches!(def, Definition::CompilerStatic) {
                // We want to handle this as unique logic rather than a simple function call to be emitted
                return self.handle_special_fn_call(fn_name, args, err);
            }
        }

        // everything else can be emitted as normal!
        let mut ctx = EmitCtx::new(
            self.registry,
            self.table,
            self.mem_allocator,
            &mut self.locals_tracker,
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

    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        emit_expr(
            expr,
            self.strategy,
            &mut self.app_iter,
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
    }
}
