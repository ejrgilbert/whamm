use crate::common::error::{ErrorGen, WhammError};
use crate::generator::types::ExprFolder;
use crate::parser::types::{BinOp, DataType, Expr, Fn, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca;
use wasmparser;

// =================================================
// ==== Emitter Trait --> Used By All Emitters! ====
// =================================================

pub trait Emitter {
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn enter_named_scope(&mut self, scope_name: &str) -> bool;
    fn exit_scope(&mut self) -> Result<(), Box<WhammError>>;
    fn reset_children(&mut self);

    fn init_instr_iter(&mut self, instrs_of_interest: &[String]) -> Result<(), Box<WhammError>>;
    fn has_next_instr(&self) -> bool;
    fn init_first_instr(&mut self) -> bool;
    fn next_instr(&mut self) -> bool;
    fn curr_instr_type(&mut self) -> String;
    fn incr_loc_pointer(&mut self);

    fn has_params(&mut self) -> Result<bool, Box<WhammError>>;
    fn save_params(&mut self) -> bool;
    fn emit_params(&mut self) -> Result<bool, Box<WhammError>>;
    fn define_compiler_var(
        &mut self,
        context: &str,
        var_name: &str,
    ) -> Result<bool, Box<WhammError>>;
    // fn emit_event(&mut self, context: &str, event: &mut Event) -> bool;
    fn fold_expr(&mut self, expr: &mut Expr) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>>;

    fn emit_fn(&mut self, context_name: &str, f: &Fn) -> Result<bool, Box<WhammError>>;
    fn emit_formal_param(&mut self, param: &(Expr, DataType)) -> bool;
    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>>;
    fn remove_orig(&mut self) -> bool;
    fn emit_orig(&mut self) -> bool;
    fn emit_if(&mut self) -> bool;
    fn emit_if_else(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    fn emit_condition(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool;
    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool;
    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>>;
    fn emit_body(&mut self, body: &mut Vec<Statement>) -> Result<bool, Box<WhammError>>;
    fn has_alt_call(&mut self) -> bool; // TODO -- remove need for this
    fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>>; // TODO -- remove need for this
    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>>;

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>>;
}

// =================================================================================
// ================ WasmRewritingEmitter - HELPER FUNCTIONS ========================
// Necessary to extract common logic between Emitter and InstrumentationVisitor.
// Can't pass an Emitter instance to InstrumentationVisitor due to Rust not
// allowing nested references to a common mutable object. So I can't pass the
// Emitter to the InstrumentationVisitor since I must iterate over Emitter.app_wasm
// with a construction of InstrumentationVisitor inside that loop.
// =================================================================================
// =================================================================================

const UNEXPECTED_ERR_MSG: &str =
    "WasmRewritingEmitter: Looks like you've found a bug...please report this behavior!";

// TODO https://github.com/thesuhas/orca/issues/13
// used to define default value of a global
fn data_type_to_val_type(ty: &DataType) -> orca::ir::Global {
    match ty {
        // DataType::I32 => orca::ir::Global {
        //     ty: wasmparser::GlobalType {
        //         content_type: wasmparser::ValType::I32,
        //         mutable: true,
        //         shared: false,
        //     },
        //     init_expr:
        // },
        _ => unimplemented!(),
        // DataType::U32 => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        // DataType::I32 => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        // DataType::Boolean => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        // DataType::Null => unimplemented!(),
        // DataType::Str => unimplemented!(),
        // DataType::Tuple { .. } => unimplemented!(),
        // // the ID used to track this var in the lib
        // DataType::Map { .. } => (ValType::I32, InitExpr::Value(walrus::ir::Value::I32(0))),
        // &DataType::AssumeGood => unimplemented!(),
    }
}

// TODO: the following helper function is an unfortunate workaround for some problems
// with interacting with Self
// emit_set, emit_expr, emit_binop, emit_value

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

pub struct WasmRewritingEmitter<'a> {
    pub app_wasm: orca::ir::Module<'a>,
    pub table: SymbolTable,

    // TODO change instr_iter and emitting_instr with orca
    // TODO: figure out what metadata is doing
    fn_providing_contexts: Vec<String>,
}
impl<'a> WasmRewritingEmitter<'a> {
    pub fn new(app_wasm: orca::ir::Module<'a>, table: SymbolTable) -> Self {
        Self {
            app_wasm,
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    fn emit_provided_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        if context == "whamm" && f.name.name == "strcmp" {
            // TODO: emit strcmp function
            // self.emit_whamm_strcmp_fn(f)
            Ok(true)
        } else {
            Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Provided function, but could not find a context to provide the definition, context: {}",
                    context
                )),
                None,
            )))
        }
    }

    fn emit_decl_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        todo!();
    }

    fn emit_assign_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        return match stmt {
            Statement::Assign { var_id, expr, .. } => {
                let mut folded_expr = ExprFolder::fold_expr(expr, &self.table);

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
                        todo!();
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
}

impl Emitter for WasmRewritingEmitter<'_> {
    fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }
    fn enter_named_scope(&mut self, scope_name: &str) -> bool {
        self.table.enter_named_scope(scope_name)
    }
    fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    fn reset_children(&mut self) {
        self.table.reset_children();
    }

    fn init_instr_iter(&mut self, instrs_of_interest: &[String]) -> Result<(), Box<WhammError>> {
        Ok(())
    }

    /// bool -> whether there is a next instruction to process
    fn has_next_instr(&self) -> bool {
        // TODO
        false
    }

    fn init_first_instr(&mut self) -> bool {
        false
    }

    /// bool -> whether it found a next instruction
    fn next_instr(&mut self) -> bool {
        // TODO
        false
    }

    /// bool -> whether the current instruction is one of the passed list of types
    fn curr_instr_type(&mut self) -> String {
        // TODO
        "".to_string()
    }

    fn incr_loc_pointer(&mut self) {
        // TODO
    }

    fn has_params(&mut self) -> Result<bool, Box<WhammError>> {
        todo!();
        Ok(true)
    }

    fn save_params(&mut self) -> bool {
        todo!();
        false
    }

    fn emit_params(&mut self) -> Result<bool, Box<WhammError>> {
        todo!();
        Ok(false)
    }

    // TODO: Elizabeth is going to rewrite this anyways
    fn define_compiler_var(
        &mut self,
        context: &str,
        var_name: &str,
    ) -> Result<bool, Box<WhammError>> {
        Ok(true)
    }

    fn fold_expr(&mut self, expr: &mut Expr) -> bool {
        *expr = ExprFolder::fold_expr(expr, &self.table);
        true
    }
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                is_success &= self.emit_if_else();
                is_success &= self.emit_condition();
                is_success &= self.emit_expr(cond)?;
                is_success &= self.emit_consequent();
                is_success &= self.emit_expr(conseq)?;
                is_success &= self.emit_alternate();
                is_success &= self.emit_expr(alt)?;
                is_success &= self.finish_branch();
            }
            Expr::VarId { .. }
            | Expr::UnOp { .. }
            | Expr::BinOp { .. }
            | Expr::Primitive { .. }
            | Expr::Call { .. } => {
                // // Anything else can be emitted as normal
                // if let Some(curr_loc) = self.instr_iter.curr_mut() {
                //     if let Some(tracker) = &mut self.emitting_instr {
                //         let func = self
                //             .app_wasm
                //             .funcs
                //             .get_mut(curr_loc.wasm_func_id)
                //             .kind
                //             .unwrap_local_mut();
                //         let func_builder = func.builder_mut();
                //         let mut instr_builder = func_builder.instr_seq(tracker.curr_seq_id);

                //         is_success &= emit_expr(
                //             &mut self.table,
                //             &mut self.app_wasm.data,
                //             expr,
                //             &mut instr_builder,
                //             &mut self.metadata,
                //             &mut tracker.curr_idx,
                //         )?;
                //     } else {
                //         return Err(Box::new(ErrorGen::get_unexpected_error(
                //             true,
                //             Some(format!(
                //                 "{UNEXPECTED_ERR_MSG} \
                //             Something went wrong while emitting an instruction."
                //             )),
                //             None,
                //         )));
                //     }
                // } else {
                //     return Err(Box::new(ErrorGen::get_unexpected_error(
                //         true,
                //         Some(format!(
                //             "{UNEXPECTED_ERR_MSG} \
                //         Something went wrong while emitting an instruction."
                //         )),
                //         None,
                //     )));
                // }
            }
        }
        Ok(is_success)
    }

    fn emit_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        // figure out if this is a provided fn.
        if f.is_comp_provided {
            return if self.fn_providing_contexts.contains(&context.to_string()) {
                self.emit_provided_fn(context, f)
            } else {
                Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Provided fn, but could not find a context to provide the definition, context: {}",
                        context
                    )),
                    None,
                )))
            };
        }

        // emit non-provided fn
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_formal_param(&mut self, _param: &(Expr, DataType)) -> bool {
        // TODO: only when we're supporting user-defined fns in script...
        unimplemented!();
    }

    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        _val: &Option<Value>,
    ) -> Result<bool, Box<WhammError>> {
        let rec_id = match self.table.lookup(&name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Global variable symbol does not exist in this scope!"
                    )),
                    None,
                )));
            } // Ignore, continue to emit
        };

        let rec = self.table.get_record_mut(&rec_id);
        println!("{:?}", rec);
        match rec {
            Some(Record::Var { ref mut addr, .. }) => {
                // emit global variable and set addr in symbol table
                // this is used for user-defined global vars in the script...

                // TODO

                // let (walrus_ty, init_expr) = data_type_to_val_type(&ty);
                // self.app_wasm.globals.add_local(walrus_ty, true, init_expr);
                // let id = self.app_wasm.globals.add_local(walrus_ty, true, init_expr);
                // *addr = Some(VarAddr::Global { addr: id });

                Ok(true)
            }
            Some(&mut ref ty) => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Incorrect global variable record, expected Record::Var, found: {:?}",
                    ty
                )),
                None,
            ))),
            None => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Global variable symbol does not exist!"
                )),
                None,
            ))),
        }
    }

    fn remove_orig(&mut self) -> bool {
        // todo!();
        false
    }

    fn emit_orig(&mut self) -> bool {
        // todo!();
        false
    }

    fn emit_if(&mut self) -> bool {
        // todo!()
        false
    }

    fn emit_if_else(&mut self) -> bool {
        // todo!()
        false
    }

    /// Will configure the emitter to emit subsequent expression as the condition of an if or if/else stmt
    /// Then emits the passed condition at that location.
    fn emit_condition(&mut self) -> bool {
        // todo!();
        false
    }

    /// Will configure the emitter to emit subsequent statements into the consequent body of an if or if/else stmt
    fn emit_consequent(&mut self) -> bool {
        // todo!();
        false
    }

    /// Will configure the emitter to emit subsequent statements into the alternate body of an if/else stmt
    fn emit_alternate(&mut self) -> bool {
        // todo!()
        false
    }

    /// Will configure the emitter to emit subsequent statements in the outer block of some branching logic
    fn finish_branch(&mut self) -> bool {
        // todo!()
        false
    }

    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        // TODO: need to reason with start funciton (dfinity case)

        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::Decl { .. } => {
                    // This is fine
                }
                _ => {
                    // This is NOT fine...error!
                    // Cannot emit this at the moment since there's no entrypoint for our module to emit initialization instructions into
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(
                            "This module has no configured entrypoint, \
            unable to emit a `script` with global state"
                                .to_string(),
                        ),
                        None,
                    )));
                }
            }
        }
        Ok(true)
    }

    fn emit_body(&mut self, body: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        for stmt in body.iter_mut() {
            self.emit_stmt(stmt)?;
        }
        Ok(true)
    }

    fn has_alt_call(&mut self) -> bool {
        todo!()
    }

    fn emit_alt_call(&mut self) -> Result<bool, Box<WhammError>> {
        todo!()
    }

    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>> {
        match stmt {
            Statement::Decl { .. } => self.emit_decl_stmt(stmt),
            Statement::Assign { .. } => self.emit_assign_stmt(stmt),
            Statement::Expr { expr, .. } => self.emit_expr(expr),
            Statement::Return { .. } => unimplemented!(),
            Statement::If {
                // cond, conseq, alt, .. -- for eventual implimentation
                ..
            } => {
                unimplemented!()
            }
        }
    }

    fn dump_to_file(&mut self, output_wasm_path: String) -> Result<bool, Box<WhammError>> {
        // TODO: clone for now
        let res = self.app_wasm.clone().encode();
        match res {
            Ok(module) => {
                let mut file = std::fs::File::create(&output_wasm_path).unwrap();
                use std::io::Write;
                let bytes = module.finish();
                file.write_all(&bytes).unwrap();

                Ok(true)
            }
            Err(err) => Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
                Failed to dump instrumented wasm to {} from error: {}",
                    &output_wasm_path, err
                )),
                None,
            ))),
        }
    }
}

// =====================================
// ==== WasmRewritingEmitter (Orca) ====
// =====================================

// =====================
// ==== WasiEmitter ====
// =====================
// unimplemented

// =======================
// ==== VirgilEmitter ====
// =======================
// unimplemented
