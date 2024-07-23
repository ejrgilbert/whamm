use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::emit_expr;
use crate::emitter::rewriting::module_emitter::MemoryTracker;
use crate::emitter::rewriting::rules::{Arg, LocInfo, Provider, WhammProvider};
use crate::emitter::rewriting::{emit_stmt, Emitter};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Definition, Expr, ProbeSpec, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use core::panic;
use orca::ir::module::Module;
use orca::iterator::iterator_trait::Iterator as OrcaIterator;
use orca::iterator::module_iterator::ModuleIterator;
use orca::opcode::Opcode;
use std::iter::Iterator;
use wasmparser::BlockType;

const UNEXPECTED_ERR_MSG: &str =
    "VisitingEmitter: Looks like you've found a bug...please report this behavior!";

pub struct VisitingEmitter<'a, 'b, 'c, 'd> {
    pub app_iter: ModuleIterator<'a, 'b>,
    pub table: &'c mut SymbolTable,
    mem_tracker: &'d MemoryTracker,
    instr_created_args: Vec<(String, usize)>,
}

impl<'a, 'b, 'c, 'd> VisitingEmitter<'a, 'b, 'c, 'd> {
    // note: only used in integration test
    pub fn new(
        app_wasm: &'a mut Module<'b>,
        table: &'c mut SymbolTable,
        mem_tracker: &'d MemoryTracker,
    ) -> Self {
        let a = Self {
            app_iter: ModuleIterator::new(app_wasm),
            table,
            mem_tracker,
            instr_created_args: vec![],
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

    pub(crate) fn enter_scope_via_spec(&mut self, script_id: &str, probe_spec: &ProbeSpec) -> bool {
        self.table.enter_scope_via_spec(script_id, probe_spec)
    }

    pub(crate) fn reset_children(&mut self) {
        self.table.reset_children();
    }

    pub(crate) fn curr_instr_name(&self) -> String {
        if let Some(curr_op) = self.app_iter.curr_op() {
            format!("{:?}", curr_op)
        } else {
            "`curr instr not defined`".to_string()
        }
    }

    pub(crate) fn get_loc_info<'e>(&self, rule: &'e WhammProvider) -> Option<LocInfo<'e>> {
        if let Some(curr_instr) = self.app_iter.curr_op() {
            rule.get_loc_info(self.app_iter.module, curr_instr)
        } else {
            None
        }
    }

    pub(crate) fn save_args(&mut self, args: &[Arg]) -> bool {
        // No opcodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        let mut arg_recs: Vec<(String, usize)> = vec![]; // vec to retain order!
        args.iter().for_each(
            |Arg {
                 name: arg_name,
                 ty: arg_ty,
             }| {
                // create local for the param in the module
                let arg_local_id = self.app_iter.add_local(arg_ty.clone());

                // emit an opcode in the event to assign the ToS to this new local
                self.app_iter.local_set(arg_local_id);

                // place in symbol table with var addr for future reference
                let id = self.table.put(
                    arg_name.to_string(),
                    Record::Var {
                        ty: DataType::I32, // we only support integers right now.
                        name: arg_name.to_string(),
                        value: None,
                        is_comp_provided: false,
                        addr: Some(VarAddr::Local { addr: arg_local_id }),
                        loc: None,
                    },
                );
                arg_recs.push((arg_name.to_string(), id));
            },
        );
        self.instr_created_args = arg_recs;
        true
    }

    pub(crate) fn emit_args(&mut self) -> Result<bool, Box<WhammError>> {
        for (_param_name, param_rec_id) in self.instr_created_args.iter() {
            let param_rec = self.table.get_record_mut(param_rec_id);
            if let Some(Record::Var {
                addr: Some(VarAddr::Local { addr }),
                ..
            }) = param_rec
            {
                // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
                // for the instrumented instruction right before that instruction is called!
                self.app_iter.local_get(*addr);
            } else {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Could not emit parameters, something went wrong..."
                    )),
                    None,
                )));
            }
        }
        Ok(true)
    }

    fn override_var_val(&mut self, rec_id: &usize, val: Option<Value>) {
        let mut rec = self.table.get_record_mut(rec_id);
        if let Some(Record::Var { value, .. }) = &mut rec {
            *value = val;
        }
    }

    pub(crate) fn define(
        &mut self,
        var_name: &str,
        var_val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>> {
        let rec_id = match self.table.lookup(var_name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                        `{var_name}` symbol does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };
        self.override_var_val(&rec_id, var_val.clone());

        Ok(true)
    }

    pub(crate) fn reset_table_data(&mut self, loc_info: &LocInfo) {
        // reset static_data
        loc_info.static_data.iter().for_each(|(symbol_name, ..)| {
            self.table.remove_record(symbol_name);
        });

        // reset dynamic_data
        for i in 0..loc_info.args.len() {
            let arg_name = format!("arg{}", i);
            self.table.remove_record(&arg_name);
        }
    }

    pub(crate) fn fold_expr(&mut self, expr: &mut Expr) -> bool {
        *expr = ExprFolder::fold_expr(expr, self.table);
        true
    }

    pub fn emit_orig(&mut self) -> bool {
        // ORCA TODO: can i get around this curr_op_owned() thing by curr_op?
        let orig = self.app_iter.curr_op_owned().unwrap().clone();
        let loc = self.app_iter.curr_loc();
        self.app_iter.add_instr_at(loc, orig);
        true
    }

    pub fn emit_if(
        &mut self,
        condition: &mut Expr,
        conseq: &mut [Statement],
    ) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;

        // emit the beginning of the if block
        self.app_iter.if_stmt(BlockType::Empty);

        is_success &= self.emit_body(conseq)?;

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    pub(crate) fn emit_if_with_orig_as_else(
        &mut self,
        condition: &mut Expr,
        conseq: &mut [Statement],
    ) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;
        // emit the beginning of the if block
        self.app_iter.if_stmt(BlockType::Empty);

        is_success &= self.emit_body(conseq)?;

        // emit the beginning of the else
        self.app_iter.else_stmt();

        is_success &= self.emit_args()?;
        is_success &= self.emit_orig();

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    // pub(crate) fn has_alt_call(&mut self) -> bool {
    //     // check if we should inject an alternate call!
    //     // At this point the body has been visited, so "new_target_fn_name" would be defined
    //     let rec_id = self.table.lookup("new_target_fn_name").copied();
    //
    //     if rec_id.is_none() {
    //         info!("`new_target_fn_name` not configured for this probe.");
    //         return false;
    //     } else {
    //         let (name, func_call_id) = match rec_id {
    //             Some(r_id) => {
    //                 let rec = self.table.get_record_mut(&r_id);
    //                 if let Some(Record::Var {
    //                     value: Some(Value::Str { val, .. }),
    //                     ..
    //                 }) = rec
    //                 {
    //                     // TODO: why instr_alt_call: Option<i32>, not Option<u32>?
    //                     let func_id = self.app_iter.module.get_fid_by_name(val);
    //                     (val.clone(), func_id)
    //                 } else {
    //                     ("".to_string(), None)
    //                 }
    //             }
    //             None => ("".to_string(), None),
    //         };
    //         if func_call_id.is_none() {
    //             info!(
    //                 "Could not find function in app Wasm specified by `new_target_fn_name`: {}",
    //                 name
    //             );
    //             return false;
    //         }
    //         self.instr_alt_call = func_call_id;
    //     }
    //     true
    // }

    // pub fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
    //     if let Some(alt_fn_id) = self.instr_alt_call {
    //         self.app_iter.call(alt_fn_id);
    //     } else {
    //         return Err(Box::new(ErrorGen::get_unexpected_error(
    //             true,
    //             Some(format!(
    //                 "{UNEXPECTED_ERR_MSG} \
    //                 Could not inject alternate call to function, something went wrong..."
    //             )),
    //             None,
    //         )));
    //     }
    //     Ok(true)
    // }

    fn handle_alt_call_by_name(
        &mut self,
        args: &mut Option<Vec<Box<Expr>>>,
    ) -> Result<bool, Box<WhammError>> {
        // args: vec![func_name: String]
        // Assume the correct args since we've gone through typechecking at this point!
        let fn_name = match &**args.as_ref().unwrap().iter().next().unwrap() {
            Expr::Primitive {
                val: Value::Str { val, .. },
                ..
            } => val.clone(),
            _ => return Ok(false),
        };

        if let Some(func_id) = self.app_iter.module.get_fid_by_name(fn_name.as_str()) {
            let is_success = self.emit_args()?;
            self.app_iter.call(func_id);
            Ok(is_success)
        } else {
            Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Could not find alt function call by name: {fn_name}"
                )),
                None,
            )))
        }
    }

    fn handle_alt_call_by_id(
        &mut self,
        args: &mut Option<Vec<Box<Expr>>>,
    ) -> Result<bool, Box<WhammError>> {
        // args: vec![func_id: i32]
        // Assume the correct args since we've gone through typechecking at this point!
        let func_id = match &**args.as_ref().unwrap().iter().next().unwrap() {
            Expr::Primitive {
                val: Value::Integer { val, .. },
                ..
            } => val.clone(),
            _ => return Ok(false),
        };

        let is_success = self.emit_args()?;
        self.app_iter.call(func_id as u32);
        Ok(is_success)
    }

    fn handle_special_fn_call(
        &mut self,
        target_fn_name: String,
        args: &mut Option<Vec<Box<Expr>>>,
    ) -> Result<bool, Box<WhammError>> {
        match target_fn_name.as_str() {
            "alt_call_by_name" => {
                self.handle_alt_call_by_name(args)
            },
            "alt_call_by_id" => {
                self.handle_alt_call_by_id(args)
            },
            _ => {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find handler for static function with name: {target_fn_name}"
                    )),
                    None,
                )))
            }
        }
    }
}
impl Emitter for VisitingEmitter<'_, '_, '_, '_> {
    fn emit_body(&mut self, body: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        for stmt in body.iter_mut() {
            is_success &= self.emit_stmt(stmt)?;
        }
        Ok(is_success)
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
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
                _ => return Ok(false),
            };
            let rec_id = self.table.lookup(fn_name.as_str()).copied();

            if rec_id.is_none() {
                // this should never happen!
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} Could not find function with name: {fn_name}"
                    )),
                    None,
                )));
            } else if let Some(r_id) = rec_id {
                let rec = self.table.get_record_mut(&r_id);
                if let Some(Record::Fn {
                    def: Definition::CompilerStatic,
                    ..
                }) = rec
                {
                    // We want to handle this as unique logic rather than a simple function call to be emitted
                    return self.handle_special_fn_call(fn_name, args);
                }
            }
        }

        // everything else can be emitted as normal!
        emit_stmt(
            stmt,
            &mut self.app_iter,
            self.table,
            &mut self.mem_tracker,
            UNEXPECTED_ERR_MSG,
        )
    }

    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        emit_expr(
            expr,
            &mut self.app_iter,
            self.table,
            &mut self.mem_tracker,
            UNEXPECTED_ERR_MSG,
        )
    }
}
