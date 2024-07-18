use std::iter::Iterator;
use log::info;

use wasmparser::BlockType;
use orca::ir::module::Module;
use orca::ir::types::DataType as OrcaType;
use orca::iterator::iterator_trait::Iterator as OrcaIterator;
use orca::iterator::module_iterator::ModuleIterator;
use orca::opcode::Opcode;

use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::rewriting::{emit_expr, emit_set, InsertionMetadata};
use crate::emitter::rewriting::rules::{LocInfo, WhammProvider};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Expr, ProbeSpec, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

const UNEXPECTED_ERR_MSG: &str =
    "VisitingEmitter: Looks like you've found a bug...please report this behavior!";

pub struct VisitingEmitter<'a, 'b, 'c>
{
    pub app_iter: ModuleIterator<'a, 'b>,
    pub table: &'c mut SymbolTable,
    instr_alt_call: Option<i32>,
    instr_created_args: Vec<(String, usize)>,

    metadata: InsertionMetadata,
}

impl<'a, 'b, 'c> VisitingEmitter<'a, 'b, 'c>
{
    // note: only used in integration test
    pub fn new(app_wasm: &'a mut Module<'b>, table: &'c mut SymbolTable) -> Self {
        if app_wasm.memories.len() > 1 {
            // TODO -- make this work with multi-memory
            panic!("only single memory is supported")
        };
        // Assuming the ID of the first memory is 0!
        let mem_id = 0;

        let a = Self {
            app_iter: ModuleIterator::new(app_wasm),
            metadata: InsertionMetadata {
                mem_id,
                curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
            },
            table,
            instr_alt_call: None,
            instr_created_args: vec![]
        };

        a
    }

    /// bool -> whether there is a next instruction to process
    pub fn next(&mut self) -> bool {
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

    fn emit_decl_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { ty, var_id, .. } => {
                // look up in symbol table
                let mut addr = if let Expr::VarId { name, .. } = var_id {
                    let var_rec_id = match self.table.lookup(name) {
                        Some(rec_id) => *rec_id,
                        None => {
                            // TODO -- add variables from body into symbol table
                            //         (at this point, the verifier should have run to catch variable initialization without declaration)
                            self.table.put(
                                name.clone(),
                                Record::Var {
                                    ty: ty.clone(),
                                    name: name.clone(),
                                    value: None,
                                    is_comp_provided: false,
                                    addr: None,
                                    loc: None,
                                },
                            )
                        }
                    };
                    match self.table.get_record_mut(&var_rec_id) {
                        Some(Record::Var { addr, .. }) => addr,
                        Some(ty) => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            Incorrect variable record, expected Record::Var, found: {:?}",
                                    ty
                                )),
                                None,
                            )));
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                            Variable symbol does not exist!"
                                )),
                                None,
                            )));
                        }
                    }
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                    Expected VarId."
                        )),
                        None,
                    )));
                };

                match &mut addr {
                    Some(VarAddr::Global { addr: _addr }) => {
                        // The global should already exist, do any initial setup here!
                        match ty {
                            DataType::Map {
                                key_ty: _key_ty,
                                val_ty: _val_ty,
                            } => {
                                // initialize map global variable
                                // also update value at GID (probably need to set ID of map there)
                                unimplemented!()
                            }
                            _ => Ok(true),
                        }
                    }
                    Some(VarAddr::Local { .. }) | None => {
                        // If the local already exists, it would be because the probe has been
                        // emitted at another opcode location. Simply overwrite the previously saved
                        // address.
                        // TODO -- uncomment after there's support for adding locals through the Orca iterator.
                        // let wasm_ty = whamm_type_to_wasm(ty).ty.content_type;
                        // let id = self.app_iter.add_local(OrcaType::from(wasm_ty));
                        // *addr = Some(VarAddr::Local { addr: id });
                        Ok(true)
                    }
                }
            }
            _ => Err(Box::new(ErrorGen::get_unexpected_error(
                false,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Wrong statement type, should be `assign`"
                )),
                None,
            ))),
        }
    }

    fn emit_assign_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        return match stmt {
            Statement::Assign { var_id, expr, .. } => {
                let mut folded_expr = ExprFolder::fold_expr(expr, self.table);

                // Save off primitives to symbol table
                // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
                if let (Expr::VarId { name, .. }, Expr::Primitive { val, .. }) =
                    (&var_id, &folded_expr)
                {
                    let var_rec_id = match self.table.lookup(name) {
                        Some(rec_id) => *rec_id,
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Attempting to emit an assign, but VarId '{name}' does not exist in this scope!"
                                )),
                                None,
                            )));
                        }
                    };
                    match self.table.get_record_mut(&var_rec_id) {
                        Some(Record::Var {
                                 value,
                                 is_comp_provided,
                                 ..
                             }) => {
                            *value = Some(val.clone());

                            if *is_comp_provided {
                                return Ok(true);
                            }
                        }
                        Some(ty) => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Incorrect variable record, expected Record::Var, found: {:?}",
                                    ty
                                )),
                                None,
                            )));
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                    Variable symbol does not exist!"
                                )),
                                None,
                            )));
                        }
                    }
                }

                match self.emit_expr(&mut folded_expr) {
                    Err(e) => Err(e),
                    Ok(_) => {
                        emit_set(
                            self.table,
                            var_id,
                            &mut self.app_iter,
                            UNEXPECTED_ERR_MSG
                        )
                    }
                }
            }
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    false,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                    Wrong statement type, should be `assign`"
                    )),
                    None,
                )));
            }
        };
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

    pub(crate) fn get_loc_info<'d>(&self, _rule: &'d WhammProvider) -> Option<LocInfo<'d>> {
        if let Some(_curr_instr) = self.app_iter.curr_op() {
            // TODO -- rework when I have access to app_wasm through iterator
            // rule.get_loc_info(&self.app_iter.app_wasm, curr_instr)
            None
        } else {
            None
        }
    }

    pub(crate) fn save_args(&mut self, args: &[OrcaType]) -> bool {
        // No opcodes should have been emitted in the module yet!
        // So, we can just save off the first * items in the stack as the args
        // to the call.
        self.app_iter.before(); // should be done before the original opcode
        let mut arg_recs = vec![]; // vec to retain order!
        args.iter().enumerate().for_each(|(num, param_ty)| {
            // create local for the param in the module
            // todo -- rework when we can add locals through the app_iter
            // let arg_local_id = self.app_iter.add_local(*param_ty);
            let arg_local_id = 0;
            
            // emit an opcode in the event to assign the ToS to this new local
            self.app_iter.local_set(arg_local_id);

            // place in symbol table with var addr for future reference
            let arg_name = format!("arg{}", num);
            let id = self.table.put(
                arg_name.clone(),
                Record::Var {
                    ty: DataType::I32, // we only support integers right now.
                    name: arg_name.clone(),
                    value: None,
                    is_comp_provided: false,
                    addr: Some(VarAddr::Local { addr: arg_local_id }),
                    loc: None,
                },
            );
            arg_recs.push((arg_name, id));
        });
        self.instr_created_args = arg_recs;
        true
    }

    pub(crate) fn emit_args(&mut self) -> Result<bool, Box<WhammError>> {
        for (_param_name, param_rec_id) in self.instr_created_args.iter() {
            let param_rec = self.table.get_record_mut(param_rec_id);
            if let Some(Record::Var {
                addr: Some(VarAddr::Local { addr }),
                ..
            }) = param_rec {
                // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
                // for the instrumented instruction right before that instruction is called!
                self.app_iter.local_set(*addr);
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

    pub(crate) fn define(&mut self, var_name: &str, var_val: &Option<Value>) -> Result<bool, Box<WhammError>> {
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

    pub fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                // change conseq and alt types to stmt for easier API call
                is_success &= self.emit_if_else(cond, &mut vec![Statement::Expr {
                    expr: (**conseq).clone(),
                    loc: None
                }], &mut vec![Statement::Expr {
                    expr: (**alt).clone(),
                    loc: None
                }])?;
            }
            Expr::VarId { .. }
            | Expr::UnOp { .. }
            | Expr::BinOp { .. }
            | Expr::Primitive { .. }
            | Expr::Call { .. } => {
                // Anything else can be emitted as normal
                // TODO -- need access to the app_wasm data somehow through the iterator.
                // Emit the instruction that sets the variable's value to the emitted expression
                is_success &= emit_expr(
                    &mut self.table,
                    // &mut self.app_iter.app_wasm.data,
                    expr,
                    &mut self.app_iter,
                    &mut self.metadata,
                    UNEXPECTED_ERR_MSG
                )?;
            }
        }
        Ok(is_success)
    }

    pub fn emit_orig(&mut self) -> bool {
        // TODO -- uncomment after we can say "instr_at"
        // TODO -- cannot pull location info from Location struct (is it private?)
        // let orig = self.app_iter.curr_op();
        // let curr_loc = self.app_iter.curr_loc();
        // 
        // self.app_iter.instr_at(curr_loc.instr_idx, orig);
        todo!()
    }
    
    fn emit_if_preamble(&mut self, condition: &mut Expr, conseq: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;
        // emit the beginning of the if block
        self.app_iter.if_stmt(BlockType::Empty);

        // emit the consequent body
        is_success &= self.emit_body(conseq)?;
        
        // INTENTIONALLY DON'T END IF BLOCK
        
        Ok(is_success)
    }
    
    fn emit_if_else_preamble(&mut self, condition: &mut Expr, conseq: &mut [Statement], alternate: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        is_success &= self.emit_if_preamble(condition, conseq)?;

        // emit the beginning of the else
        self.app_iter.else_stmt();

        // emit the alternate body
        is_success &= self.emit_body(alternate)?;

        // INTENTIONALLY DON'T END IF/ELSE BLOCK

        Ok(is_success)
    }

    pub(crate) fn emit_if(&mut self, condition: &mut Expr, conseq: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        is_success &= self.emit_if_preamble(condition, conseq)?;

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }

    pub(crate) fn emit_if_else(&mut self, condition: &mut Expr, conseq: &mut [Statement], alternate: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;

        is_success &= self.emit_if_else_preamble(condition, conseq, alternate)?;

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }
    
    pub(crate) fn emit_if_with_orig_as_else(&mut self, condition: &mut Expr, conseq: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        
        is_success &= self.emit_if_preamble(condition, conseq)?;

        is_success &= self.emit_args()?;
        is_success &= self.emit_orig();

        // emit the end of the if block
        self.app_iter.end();
        Ok(is_success)
    }
    
    pub fn emit_body(&mut self, body: &mut [Statement]) -> Result<bool, Box<WhammError>> {
        for stmt in body.iter_mut() {
            self.emit_stmt(stmt)?;
        }
        Ok(true)
    }

    pub(crate) fn has_alt_call(&mut self) -> bool {
        // check if we should inject an alternate call!
        // At this point the body has been visited, so "new_target_fn_name" would be defined
        let rec_id = self.table.lookup("new_target_fn_name").copied();

        if rec_id.is_none() {
            info!("`new_target_fn_name` not configured for this probe.");
            return false;
        } else {
            let (name, func_call_id) = match rec_id {
                Some(r_id) => {
                    let rec = self.table.get_record_mut(&r_id);
                    if let Some(Record::Var {
                                    value: Some(Value::Str { val, .. }),
                                    ..
                                }) = rec
                    {
                        // TODO -- how to pull func names from module?
                        // (val.clone(), self.app_wasm.funcs.by_name(val))
                        (val.clone(), Some(1056)) // hardcoded for now to ID for `redirect_to_fault_injector` for users.wasm file
                    } else {
                        ("".to_string(), None)
                    }
                }
                None => ("".to_string(), None),
            };
            if func_call_id.is_none() {
                info!(
                    "Could not find function in app Wasm specified by `new_target_fn_name`: {}",
                    name
                );
                return false;
            }
            self.instr_alt_call = func_call_id;
        }
        true
    }

    pub fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
        if let Some(alt_fn_id) = self.instr_alt_call {
            self.app_iter.call(alt_fn_id as u32);
        } else {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                    Could not inject alternate call to function, something went wrong..."
                )),
                None,
            )));
        }
        Ok(true)
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { .. } => self.emit_decl_stmt(stmt),
            Statement::Assign { .. } => self.emit_assign_stmt(stmt),
            Statement::Expr { expr, .. } => self.emit_expr(expr),
            Statement::Return { .. } => unimplemented!(),
            Statement::If {
                // cond, conseq, alt, .. -- for eventual implementation
                ..
            } => {
                unimplemented!()
            }
        }
    }
}
