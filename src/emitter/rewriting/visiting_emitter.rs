use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::rules::wasm::OpcodeEvent;
use crate::emitter::rewriting::rules::{Arg, LocInfo, ProbeRule, Provider, WhammProvider};
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use std::collections::HashMap;

use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::utils::{
    block_type_to_wasm, emit_expr, emit_stmt, whamm_type_to_wasm_global, EmitCtx,
};
use crate::emitter::{configure_flush_routines, Emitter, InjectStrategy};
use crate::generator::ast::UnsharedVar;
use crate::generator::folding::ExprFolder;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::report_vars::ReportVars;
use crate::parser;
use crate::parser::types::{Block, DataType, Definition, Expr, NumLit, RulePart, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use itertools::Itertools;
use log::warn;
use orca_wasm::ir::function::FunctionBuilder;
use orca_wasm::ir::id::{FunctionID, LocalID, TypeID};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::BlockType as OrcaBlockType;
use orca_wasm::iterator::iterator_trait::{IteratingInstrumenter, Iterator as OrcaIterator};
use orca_wasm::iterator::module_iterator::ModuleIterator;
use orca_wasm::opcode::{Instrumenter, Opcode};
use orca_wasm::Location;
use std::iter::Iterator;
use crate::emitter::locals_tracker::LocalsTracker;

const UNEXPECTED_ERR_MSG: &str =
    "VisitingEmitter: Looks like you've found a bug...please report this behavior!";

pub struct VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub strategy: InjectStrategy,
    pub app_iter: ModuleIterator<'a, 'b>,
    pub table: &'c mut SymbolTable,
    pub mem_allocator: &'d mut MemoryAllocator,
    pub locals_tracker: LocalsTracker,
    pub map_lib_adapter: &'e mut MapLibAdapter,
    pub io_adapter: &'f mut IOAdapter,
    pub(crate) report_vars: &'g mut ReportVars,
    pub(crate) unshared_var_handler: &'g mut UnsharedVarHandler,
    instr_created_args: Vec<(String, usize)>,
    pub curr_unshared: Vec<UnsharedVar>,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VisitingEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    // note: only used in integration test
    pub fn new(
        strategy: InjectStrategy,
        app_wasm: &'a mut Module<'b>,
        injected_funcs: &Vec<FunctionID>,
        table: &'c mut SymbolTable,
        mem_allocator: &'d mut MemoryAllocator,
        map_lib_adapter: &'e mut MapLibAdapter,
        io_adapter: &'f mut IOAdapter,
        report_vars: &'g mut ReportVars,
        unshared_var_handler: &'g mut UnsharedVarHandler,
    ) -> Self {
        let a = Self {
            strategy,
            app_iter: ModuleIterator::new(app_wasm, injected_funcs),
            table,
            mem_allocator,
            locals_tracker: LocalsTracker::default(),
            map_lib_adapter,
            io_adapter,
            report_vars,
            unshared_var_handler,
            instr_created_args: vec![],
            curr_unshared: vec![],
        };

        a
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

    pub(crate) fn enter_scope_via_rule(&mut self, script_id: &str, probe_rule: &ProbeRule) -> bool {
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

    pub(crate) fn get_loc_info<'h>(&self, rule: &'h WhammProvider) -> Option<LocInfo<'h>> {
        let (curr_loc, at_func_end) = self.app_iter.curr_loc();
        if at_func_end {
            // We're at the 'end' opcode of the function...don't instrument
            return None;
        }
        if let Some(curr_instr) = self.app_iter.curr_op() {
            rule.get_loc_info(self.app_iter.module, curr_loc, curr_instr)
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
                        self.table,
                        self.mem_allocator,
                        &mut self.locals_tracker,
                        self.map_lib_adapter,
                        self.report_vars,
                        self.unshared_var_handler,
                        UNEXPECTED_ERR_MSG,
                        err,
                    ),
                );
            }
        }
        is_success
    }

    pub(crate) fn save_args(&mut self, args: &[Arg], err: &mut ErrorGen) -> bool {
        // No opcodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        let mut arg_recs: Vec<(String, usize)> = vec![]; // vec to retain order!

        let mut arg_locals: Vec<(String, u32)> = vec![];
        args.iter().for_each(
            |Arg {
                 name: arg_name,
                 ty: arg_ty,
             }| {
                let ty = if let Some(ty) = arg_ty {
                    *ty
                } else {
                    warn!("The current way that probes with polymorphic argument types is supported for the bytecode rewriting target is incomplete.\
                           In a future version, we need to have a virtual stack on the side to compute the actual argument type and compare with the \
                           argument bounds of the probe to see if the location is a map. For now, it may generate invalid instrumented modules!");
                    let Some(Record::Var {
                                 ty,
                                 ..
                             }) = self.table.lookup_var(arg_name, &None, err, true)
                    else {
                        err.unexpected_error(true, Some("unexpected type".to_string()), None);
                        return;
                    };
                    let wasm_ty = if ty.to_wasm_type().len() > 1 {
                        unimplemented!()
                    } else {
                        *ty.to_wasm_type().first().unwrap()
                    };
                    wasm_ty
                };
                // create local for the param in the module
                let arg_local_id = LocalID(self.locals_tracker.use_local(ty, &mut self.app_iter));
                arg_locals.push((arg_name.to_string(), *arg_local_id));
            },
        );

        // Save args in reverse order (the leftmost arg is at the bottom of the stack)
        arg_locals.iter().for_each(|(arg_name, arg_local_id)| {
            // emit an opcode in the event to assign the ToS to this new local
            self.app_iter.local_set(LocalID(*arg_local_id));

            // place in symbol table with var addr for future reference
            let id = self.table.put(
                arg_name.to_string(),
                Record::Var {
                    ty: DataType::I32, // we only support integers right now.
                    name: arg_name.to_string(),
                    value: None,
                    def: Definition::User,
                    is_report_var: false,
                    addr: Some(VarAddr::Local {
                        addr: *arg_local_id,
                    }),
                    loc: None,
                },
            );
            arg_recs.insert(0, (arg_name.to_string(), id));
        });
        self.instr_created_args = arg_recs;
        true
    }

    pub(crate) fn emit_args(&mut self, err: &mut ErrorGen) -> bool {
        for (_param_name, param_rec_id) in self.instr_created_args.iter() {
            let param_rec = self.table.get_record_mut(*param_rec_id);
            if let Some(Record::Var {
                addr: Some(VarAddr::Local { addr }),
                ..
            }) = param_rec
            {
                // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
                // for the instrumented instruction right before that instruction is called!
                self.app_iter.local_get(LocalID(*addr));
            } else {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Could not emit parameters, something went wrong..."
                    )),
                    None,
                );
                return false;
            }
        }
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

    pub(crate) fn define(&mut self, var_name: &str, var_val: &Option<Value>) -> bool {
        self.table.override_record_val(var_name, var_val.clone());
        true
    }

    pub(crate) fn reset_table_data(&mut self, loc_info: &LocInfo) {
        // reset static_data
        loc_info.static_data.iter().for_each(|(symbol_name, ..)| {
            self.table.override_record_val(symbol_name, None);
        });

        // reset dynamic_data
        loc_info.dynamic_data.iter().for_each(|(symbol_name, ..)| {
            self.table.override_record_val(symbol_name, None);
        });

        for i in 0..loc_info.args.len() {
            let arg_name = format!("arg{}", i);
            self.table.override_record_val(&arg_name, None);
        }
        self.instr_created_args.clear();
    }

    pub(crate) fn fold_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        *expr = ExprFolder::fold_expr(expr, self.table, err);
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
        curr_instr_args: &[Arg],
        condition: &mut Expr,
        conseq: &mut Block,
        err: &mut ErrorGen,
    ) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition, err);

        // emit the beginning of the if block

        self.app_iter.if_stmt(block_type_to_wasm(conseq));

        is_success &= self.emit_body(curr_instr_args, conseq, err);

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    pub(crate) fn emit_if_with_orig_as_else(
        &mut self,
        curr_instr_args: &[Arg],
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
        let orig_ty_id = OpcodeEvent::get_ty_info_for_instr(
            self.app_iter.module,
            &fid,
            self.app_iter.curr_op().unwrap(),
        )
        .1;

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
                OrcaBlockType::FuncType(self.app_iter.module.types.add_func_type(&[], &ty))
            }
            None => OrcaBlockType::Empty,
        };
        self.app_iter.if_stmt(block_ty);
        is_success &= self.emit_body(curr_instr_args, conseq, err);

        // emit the beginning of the else
        self.app_iter.else_stmt();

        is_success &= self.emit_args(err);
        is_success &= self.emit_orig();

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    fn handle_alt_call_by_name(&mut self, args: &mut [Expr], err: &mut ErrorGen) -> bool {
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
            err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Could not find alt function call by name: {fn_name}"
                )),
                None,
            );
            false
        }
    }

    fn handle_alt_call_by_id(&mut self, args: &mut [Expr], err: &mut ErrorGen) -> bool {
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

    fn handle_drop_args(&mut self) -> bool {
        // Generate drops for all args to this opcode!

        let fid = match self.app_iter.curr_loc().0 {
            Location::Module { func_idx, .. } | Location::Component { func_idx, .. } => func_idx,
        };

        // ensure we have the args for this instruction
        let curr_instr_args = OpcodeEvent::get_ty_info_for_instr(
            self.app_iter.module,
            &fid,
            self.app_iter.curr_op().unwrap(),
        )
        .0;

        let num_to_drop = curr_instr_args.len() - self.instr_created_args.len();
        for _arg in 0..num_to_drop {
            self.app_iter.drop();
        }
        true
    }

    fn handle_special_fn_call(
        &mut self,
        _curr_instr_args: &[Arg],
        target_fn_name: String,
        args: &mut [Expr],
        err: &mut ErrorGen,
    ) -> bool {
        match target_fn_name.as_str() {
            "alt_call_by_name" => self.handle_alt_call_by_name(args, err),
            "alt_call_by_id" => self.handle_alt_call_by_id(args, err),
            "drop_args" => self.handle_drop_args(),
            _ => {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find handler for static function with name: {target_fn_name}"
                    )),
                    None,
                );
                false
            }
        }
    }

    pub fn inject_map_init(&mut self) {
        if !self.map_lib_adapter.used_in_global_scope {
            return;
        }
        self.before();
        let fid = self.map_lib_adapter.get_map_init_fid(self.app_iter.module);
        self.map_lib_adapter
            .inject_map_init_check(&mut self.app_iter, fid);
    }

    pub fn configure_flush_routines(&mut self, has_reports: bool, err: &mut ErrorGen) {
        // create the function to call at the end
        // TODO -- this can be cleaned up to use the wizard logic instead!

        // only do this is there are report variables
        if has_reports {
            let mut on_exit = FunctionBuilder::new(&[], &[]);

            let var_flush = configure_flush_routines(
                self.app_iter.module,
                self.report_vars,
                self.map_lib_adapter,
                self.mem_allocator,
                self.io_adapter,
                err,
            );
            if let Some(flush_fid) = var_flush {
                on_exit.call(FunctionID(flush_fid));
            }

            let on_exit_id = on_exit.finish_module(self.app_iter.module);
            self.app_iter
                .module
                .set_fn_name(on_exit_id, "on_exit".to_string());

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
            main.finish_instr();
        }
    }
}
impl Emitter for VisitingEmitter<'_, '_, '_, '_, '_, '_, '_> {
    fn reset_locals_for_probe(&mut self) {
        self.locals_tracker.reset_probe();
    }

    fn reset_locals_for_function(&mut self) {
        self.locals_tracker.reset_function();
    }

    fn emit_body(&mut self, curr_instr_args: &[Arg], body: &mut Block, err: &mut ErrorGen) -> bool {
        let mut is_success = true;

        // TODO -- this can be removed once we move to calling the generated functions!
        // Create the required globals for this probe
        // Sort by datatype to make generation deterministic!
        // translate unshared vars to the correct format
        let sorted_unshared = self
            .curr_unshared
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.ty, &b.ty));

        for UnsharedVar {
            name,
            ty,
            is_report,
            ..
        } in sorted_unshared.into_iter()
        {
            if matches!(ty, DataType::Map { .. }) {
                // handle maps
                let Some(Record::Var {
                    ref mut addr,
                    ref mut ty,
                    ..
                }) = self.table.lookup_var_mut(name)
                else {
                    err.unexpected_error(true, Some("unexpected type".to_string()), None);
                    return false;
                };

                self.map_lib_adapter.emit_map_init(
                    name.clone(),
                    addr,
                    ty,
                    *is_report,
                    self.report_vars,
                    self.app_iter.module,
                    err,
                );
            } else {
                // if it's not a map, we'll just use this generated GID when
                // we need to during the AST visit
                let (global_id, ..) = whamm_type_to_wasm_global(self.app_iter.module, ty);
                self.unshared_var_handler.add_available_gid(*global_id, ty);
            }
        }

        for stmt in body.stmts.iter_mut() {
            is_success &= self.emit_stmt(curr_instr_args, stmt, err);
        }
        is_success
    }

    fn emit_stmt(
        &mut self,
        curr_instr_args: &[Arg],
        stmt: &mut Statement,
        err: &mut ErrorGen,
    ) -> bool {
        // Check if this is calling a provided, static function!
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
            let Some(Record::Fn { def, .. }) = self.table.lookup_fn(fn_name.as_str(), true, err)
            else {
                err.unexpected_error(true, Some("unexpected type".to_string()), None);
                return false;
            };
            if matches!(def, Definition::CompilerStatic) {
                // We want to handle this as unique logic rather than a simple function call to be emitted
                return self.handle_special_fn_call(curr_instr_args, fn_name, args, err);
            }
        }

        // everything else can be emitted as normal!

        emit_stmt(
            stmt,
            self.strategy,
            &mut self.app_iter,
            &mut EmitCtx::new(
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
                self.map_lib_adapter,
                self.report_vars,
                self.unshared_var_handler,
                UNEXPECTED_ERR_MSG,
                err,
            ),
        )
    }

    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool {
        emit_expr(
            expr,
            self.strategy,
            &mut self.app_iter,
            &mut EmitCtx::new(
                self.table,
                self.mem_allocator,
                &mut self.locals_tracker,
                self.map_lib_adapter,
                self.report_vars,
                self.unshared_var_handler,
                UNEXPECTED_ERR_MSG,
                err,
            ),
        )
    }
}
