use crate::common::error::{ErrorGen, WhammError};
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Expr, Fn, Statement, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

use orca::ir::types::DataType as OrcaType;
use wasmparser::BlockType;

use crate::emitter::rewriting::{emit_expr, emit_set, whamm_type_to_wasm, InsertionMetadata};
use orca::ir::function::FunctionBuilder;
use orca::ir::module::Module;
use orca::opcode::Opcode;

const UNEXPECTED_ERR_MSG: &str =
    "ModuleEmitter: Looks like you've found a bug...please report this behavior!";

pub struct ModuleEmitter<'a, 'b, 'c> {
    pub app_wasm: &'a mut Module<'b>,
    pub emitting_func: Option<FunctionBuilder<'b>>,
    pub table: &'c mut SymbolTable,

    metadata: InsertionMetadata,
    fn_providing_contexts: Vec<String>,
}

impl<'a, 'b, 'c> ModuleEmitter<'a, 'b, 'c> {
    // note: only used in integration test
    pub fn new(app_wasm: &'a mut Module<'b>, table: &'c mut SymbolTable) -> Self {
        if app_wasm.memories.len() > 1 {
            // TODO -- make this work with multi-memory
            panic!("only single memory is supported")
        };

        Self {
            app_wasm,
            emitting_func: None,
            metadata: InsertionMetadata {
                mem_id: 0,                  // Assuming the ID of the first memory is 0!
                curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
            },
            table,
            fn_providing_contexts: vec!["whamm".to_string()],
        }
    }

    fn emit_provided_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
        if context == "whamm" && f.name.name == "strcmp" {
            self.emit_whamm_strcmp_fn(f)
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

    fn emit_whamm_strcmp_fn(&mut self, f: &Fn) -> Result<bool, Box<WhammError>> {
        let strcmp_params = vec![OrcaType::I32, OrcaType::I32, OrcaType::I32, OrcaType::I32];
        let strcmp_result = vec![OrcaType::I32];

        let mut strcmp = FunctionBuilder::new(&strcmp_params, &strcmp_result);

        // specify params
        let str0_offset = 0u32;
        let str0_size = 1u32;
        let str1_offset = 2u32;
        let str1_size = 3u32;

        // create locals
        let i = strcmp.add_local(OrcaType::I32);
        let str0_char = strcmp.add_local(OrcaType::I32);
        let str1_char = strcmp.add_local(OrcaType::I32);

        #[rustfmt::skip]
        strcmp
            .block(BlockType::Empty) // label = @1
            .block(BlockType::Empty) // label = @2
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
            .loop_stmt(BlockType::Empty)
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
                    memory: self.metadata.mem_id
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
                    memory: self.metadata.mem_id
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

        let strcmp_id = strcmp.finish(self.app_wasm);

        let rec_id = match self.table.lookup(&f.name.name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                `strcmp` fn symbol does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };

        return if let Some(rec) = self.table.get_record_mut(&rec_id) {
            if let Record::Fn { addr, .. } = rec {
                *addr = Some(strcmp_id);
                Ok(true)
            } else {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{UNEXPECTED_ERR_MSG} \
                Incorrect global variable record, expected Record::Var, found: {:?}",
                        rec
                    )),
                    None,
                )));
            }
        } else {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} \
            Global variable symbol does not exist!"
                )),
                None,
            )));
        };
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
                        let wasm_ty = whamm_type_to_wasm(ty).ty.content_type;
                        if let Some(func) = &mut self.emitting_func {
                            let id = func.add_local(OrcaType::from(wasm_ty));
                            *addr = Some(VarAddr::Local { addr: id });
                        }
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
                        if let Some(emitting_func) = &mut self.emitting_func {
                            // Emit the instruction that sets the variable's value to the emitted expression
                            emit_set(&mut self.table, var_id, emitting_func, UNEXPECTED_ERR_MSG)
                        } else {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{UNEXPECTED_ERR_MSG} \
                                            Something went wrong while emitting an instruction."
                                )),
                                None,
                            )));
                        }
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

    pub(crate) fn enter_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.enter_scope()
    }

    pub(crate) fn exit_scope(&mut self) -> Result<(), Box<WhammError>> {
        self.table.exit_scope()
    }
    pub(crate) fn reset_children(&mut self) {
        self.table.reset_children();
    }

    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>> {
        let mut is_success = true;
        match expr {
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                // change conseq and alt types to stmt for easier API call
                is_success &= self.emit_if_else(
                    cond,
                    &mut vec![Statement::Expr {
                        expr: (**conseq).clone(),
                        loc: None,
                    }],
                    &mut vec![Statement::Expr {
                        expr: (**alt).clone(),
                        loc: None,
                    }],
                )?;
            }
            Expr::VarId { .. }
            | Expr::UnOp { .. }
            | Expr::BinOp { .. }
            | Expr::Primitive { .. }
            | Expr::Call { .. } => {
                // Anything else can be emitted as normal
                if let Some(emitting_func) = &mut self.emitting_func {
                    // Emit the instruction that sets the variable's value to the emitted expression
                    is_success &= emit_expr(
                        &mut self.table,
                        &mut self.app_wasm.data,
                        expr,
                        emitting_func,
                        &mut self.metadata,
                        UNEXPECTED_ERR_MSG,
                    )?;
                } else {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} \
                                            Something went wrong while emitting an instruction."
                        )),
                        None,
                    )));
                }
            }
        }
        Ok(is_success)
    }

    pub(crate) fn emit_fn(&mut self, context: &str, f: &Fn) -> Result<bool, Box<WhammError>> {
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

    pub(crate) fn emit_global(
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
        match rec {
            Some(Record::Var { ref mut addr, .. }) => {
                // emit global variable and set addr in symbol table
                // this is used for user-defined global vars in the script...
                let default_global = whamm_type_to_wasm(&ty);
                let global_id = self.app_wasm.add_global(default_global);
                *addr = Some(VarAddr::Global { addr: global_id });
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

    fn emit_if(
        &mut self,
        condition: &mut Expr,
        conseq: &mut Vec<Statement>,
    ) -> Result<bool, Box<WhammError>> {
        // NOTE: The structure of this code is wonky, but it's because of
        // overlapping references/calls to self.
        // To avoid that, we place all calls to self.emitting_func in a block.

        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the if block
            emitting_func.if_stmt(BlockType::Empty);
        }

        // emit the consequent body
        is_success &= self.emit_body(conseq)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the end of the if block
            emitting_func.end();
        }

        Ok(is_success)
    }

    fn emit_if_else(
        &mut self,
        condition: &mut Expr,
        conseq: &mut Vec<Statement>,
        alternate: &mut Vec<Statement>,
    ) -> Result<bool, Box<WhammError>> {
        // NOTE: The structure of this code is wonky, but it's because of
        // overlapping references/calls to self.
        // To avoid that, we place all calls to self.emitting_func in a block.

        let mut is_success = true;

        // emit the condition of the `if` expression
        is_success &= self.emit_expr(condition)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the if block
            emitting_func.if_stmt(BlockType::Empty);
        }

        // emit the consequent body
        is_success &= self.emit_body(conseq)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the beginning of the else
            emitting_func.else_stmt();
        }

        // emit the alternate body
        is_success &= self.emit_body(alternate)?;

        if let Some(emitting_func) = &mut self.emitting_func {
            // emit the end of the if/else block
            emitting_func.end();
        }

        Ok(is_success)
    }

    fn emit_global_stmts(&mut self, stmts: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        // NOTE: This should be done in the Module entrypoint
        //       https://docs.rs/walrus/latest/walrus/struct.Module.html

        if let Some(_start_fid) = self.app_wasm.start {
            // 1. create the emitting_func var, assign in self
            // 2. iterate over stmts and emit them! (will be diff for Decl stmts)
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

    fn emit_body(&mut self, body: &mut Vec<Statement>) -> Result<bool, Box<WhammError>> {
        for stmt in body.iter_mut() {
            self.emit_stmt(stmt)?;
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
