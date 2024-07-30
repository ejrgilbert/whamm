pub mod module_emitter;
pub mod rules;
pub mod visiting_emitter;

use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{BinOp, Block, DataType, Expr, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

use crate::emitter::rewriting::module_emitter::MemoryTracker;
use crate::generator::types::ExprFolder;
use orca::ir::types::{BlockType, DataType as OrcaType, Global, Value as OrcaValue};
use orca::opcode::Opcode;
use orca::{InitExpr, ModuleBuilder};
use wasmparser::{GlobalType, ValType};

pub trait Emitter {
    fn emit_body(&mut self, body: &mut Block) -> Result<bool, Box<WhammError>>;
    fn emit_stmt(&mut self, stmt: &mut Statement) -> Result<bool, Box<WhammError>>;
    fn emit_expr(&mut self, expr: &mut Expr) -> Result<bool, Box<WhammError>>;
}

// ==================================================================
// ================ Emitter Helper Functions ========================
// TODO -- add this documentation
// - Necessary to extract common logic between Emitter and InstrumentationVisitor.
// - Can't pass an Emitter instance to InstrumentationVisitor due to Rust not
// - allowing nested references to a common mutable object. So I can't pass the
// - Emitter to the InstrumentationVisitor since I must iterate over Emitter.app_wasm
// - with a construction of InstrumentationVisitor inside that loop.
// ==================================================================
// ==================================================================

fn emit_body<'a, T: Opcode<'a> + ModuleBuilder>(
    body: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    for stmt in body.stmts.iter_mut() {
        emit_stmt(stmt, injector, table, mem_tracker, err_msg)?;
    }
    Ok(true)
}

fn emit_stmt<'a, T: Opcode<'a> + ModuleBuilder>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    match stmt {
        Statement::Decl { .. } => emit_decl_stmt(stmt, injector, table, err_msg),
        Statement::Assign { .. } => emit_assign_stmt(stmt, injector, table, mem_tracker, err_msg),
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => {
            emit_expr(expr, injector, table, mem_tracker, err_msg)
        }
        Statement::If {
            cond, conseq, alt, ..
        } => {
            if alt.stmts.is_empty() {
                emit_if(cond, conseq, injector, table, mem_tracker, err_msg)
            } else {
                emit_if_else(cond, conseq, alt, injector, table, mem_tracker, err_msg)
            }
        }
    }
}

fn emit_decl_stmt<'a, T: Opcode<'a> + ModuleBuilder>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    match stmt {
        Statement::Decl { ty, var_id, .. } => {
            // look up in symbol table
            let mut addr = if let Expr::VarId { name, .. } = var_id {
                let var_rec_id = match table.lookup(name) {
                    Some(rec_id) => *rec_id,
                    None => {
                        // TODO -- add variables from body into symbol table
                        //         (at this point, the verifier should have run to catch variable initialization without declaration)
                        table.put(
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
                match table.get_record_mut(&var_rec_id) {
                    Some(Record::Var { addr, .. }) => addr,
                    Some(ty) => {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "{err_msg} Incorrect variable record, expected Record::Var, found: {:?}",
                                ty
                            )),
                            None,
                        )));
                    }
                    None => {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!("{err_msg} Variable symbol does not exist!")),
                            None,
                        )));
                    }
                }
            } else {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!("{err_msg} Expected VarId.")),
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
                    let wasm_ty = whamm_type_to_wasm_type(ty);
                    let id = injector.add_local(OrcaType::from(wasm_ty));
                    *addr = Some(VarAddr::Local { addr: id });
                    Ok(true)
                }
            }
        }
        _ => Err(Box::new(ErrorGen::get_unexpected_error(
            false,
            Some(format!(
                "{err_msg} Wrong statement type, should be `assign`"
            )),
            None,
        ))),
    }
}

fn emit_assign_stmt<'a, T: Opcode<'a> + ModuleBuilder>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    return match stmt {
        Statement::Assign { var_id, expr, .. } => {
            let mut folded_expr = ExprFolder::fold_expr(expr, table);

            // Save off primitives to symbol table
            // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
            if let (Expr::VarId { name, .. }, Expr::Primitive { val, .. }) = (&var_id, &folded_expr)
            {
                let var_rec_id = match table.lookup(name) {
                    Some(rec_id) => *rec_id,
                    _ => {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "{err_msg} \
                                    Attempting to emit an assign, but VarId '{name}' does not exist in this scope!"
                            )),
                            None,
                        )));
                    }
                };
                match table.get_record_mut(&var_rec_id) {
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
                                "{err_msg} \
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
                                "{err_msg} \
                                    Variable symbol does not exist!"
                            )),
                            None,
                        )));
                    }
                }
            }

            match emit_expr(&mut folded_expr, injector, table, mem_tracker, err_msg) {
                Err(e) => Err(e),
                Ok(_) => {
                    // Emit the instruction that sets the variable's value to the emitted expression
                    emit_set(var_id, injector, table, err_msg)
                }
            }
        }
        _ => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                false,
                Some(format!(
                    "{err_msg} \
                    Wrong statement type, should be `assign`"
                )),
                None,
            )));
        }
    };
}

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
// TODO: Do we really want to depend on wasmparser::ValType, or create a wrapper?
pub fn whamm_type_to_wasm_global(ty: &DataType) -> Global {
    let orca_ty = whamm_type_to_wasm_type(ty);
    match orca_ty {
        OrcaType::I32 => Global {
            ty: GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            init_expr: InitExpr::Value(OrcaValue::I32(0)),
        },
        _ => unimplemented!(),
    }
}
pub fn whamm_type_to_wasm_type(ty: &DataType) -> OrcaType {
    match ty {
        DataType::I32 | DataType::U32 | DataType::Boolean => OrcaType::I32,
        // the ID used to track this var in the lib
        DataType::Map { .. } => OrcaType::I32,
        DataType::Null => unimplemented!(),
        DataType::Str => unimplemented!(),
        DataType::Tuple { .. } => unimplemented!(),
        DataType::AssumeGood => unimplemented!(),
    }
}

pub fn block_type_to_wasm(block: &Block) -> BlockType {
    match &block.return_ty {
        None => BlockType::Empty,
        Some(return_ty) => {
            let wasm_ty = whamm_type_to_wasm_type(return_ty);
            BlockType::Type(wasm_ty)
        }
    }
}

fn emit_set<'a, T: Opcode<'a>>(
    var_id: &mut Expr,
    injector: &mut T,
    table: &mut SymbolTable,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    if let Expr::VarId { name, .. } = var_id {
        let var_rec_id = match table.lookup(name) {
            Some(rec_id) => *rec_id,
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} \
                                                VarId '{name}' does not exist in this scope!"
                    )),
                    None,
                )));
            }
        };
        match table.get_record_mut(&var_rec_id) {
            Some(Record::Var { addr, loc, .. }) => {
                // this will be different based on if this is a global or local var
                match addr {
                    Some(VarAddr::Global { addr }) => {
                        injector.global_set(*addr);
                    }
                    Some(VarAddr::Local { addr }) => {
                        injector.local_set(*addr);
                    },
                    None => {
                        return Err(Box::new(ErrorGen::get_type_check_error_from_loc(false,
                                                                                    format!("Variable assigned before declared: {}", name), loc)));
                    }
                }
                Ok(true)
            },
            Some(ty) => {
                Err(Box::new(ErrorGen::get_unexpected_error(true, Some(format!("{err_msg} \
                                                Incorrect variable record, expected Record::Var, found: {:?}", ty)), None)))
            },
            None => {
                Err(Box::new(ErrorGen::get_unexpected_error(true, Some(format!("{err_msg} \
                                                Variable symbol does not exist!")), None)))
            }
        }
    } else {
        Err(Box::new(ErrorGen::get_unexpected_error(
            true,
            Some(format!("{err_msg} Expected VarId.")),
            None,
        )))
    }
}

fn emit_if_preamble<'a, T: Opcode<'a> + ModuleBuilder>(
    condition: &mut Expr,
    conseq: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    // emit the condition of the `if` expression
    is_success &= emit_expr(condition, injector, table, mem_tracker, err_msg)?;
    // emit the beginning of the if block
    injector.if_stmt(block_type_to_wasm(conseq));

    // emit the consequent body
    is_success &= emit_body(conseq, injector, table, mem_tracker, err_msg)?;

    // INTENTIONALLY DON'T END IF BLOCK

    Ok(is_success)
}

fn emit_if_else_preamble<'a, T: Opcode<'a> + ModuleBuilder>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    is_success &= emit_if_preamble(condition, conseq, injector, table, mem_tracker, err_msg)?;

    // emit the beginning of the else
    injector.else_stmt();

    // emit the alternate body
    is_success &= emit_body(alternate, injector, table, mem_tracker, err_msg)?;

    // INTENTIONALLY DON'T END IF/ELSE BLOCK

    Ok(is_success)
}

fn emit_if<'a, T: Opcode<'a> + ModuleBuilder>(
    condition: &mut Expr,
    conseq: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    is_success &= emit_if_preamble(condition, conseq, injector, table, mem_tracker, err_msg)?;

    // emit the end of the if block
    injector.end();
    Ok(is_success)
}

fn emit_if_else<'a, T: Opcode<'a> + ModuleBuilder>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    is_success &= emit_if_else_preamble(
        condition,
        conseq,
        alternate,
        injector,
        table,
        mem_tracker,
        err_msg,
    )?;

    // emit the end of the if block
    injector.end();
    Ok(is_success)
}

// TODO: emit_expr has two mutable references to the name object, the injector has module data in it
fn emit_expr<'a, T: Opcode<'a> + ModuleBuilder>(
    expr: &mut Expr,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match expr {
        Expr::UnOp { op, expr, .. } => {
            is_success &= emit_expr(expr, injector, table, mem_tracker, err_msg)?;
            is_success &= emit_unop(op, injector);
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            is_success &= emit_expr(lhs, injector, table, mem_tracker, err_msg)?;
            is_success &= emit_expr(rhs, injector, table, mem_tracker, err_msg)?;
            is_success &= emit_binop(op, injector);
        }
        Expr::Ternary {
            cond, conseq, alt, ..
        } => {
            // change conseq and alt types to stmt for easier API call
            is_success &= emit_if_else(
                cond,
                &mut Block {
                    stmts: vec![Statement::Expr {
                        expr: (**conseq).clone(),
                        loc: None,
                    }],
                    return_ty: None,
                    loc: None,
                },
                &mut Block {
                    stmts: vec![Statement::Expr {
                        expr: (**alt).clone(),
                        loc: None,
                    }],
                    return_ty: None,
                    loc: None,
                },
                injector,
                table,
                mem_tracker,
                err_msg,
            )?;
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            let fn_name = match &**fn_target {
                Expr::VarId { name, .. } => name.clone(),
                _ => return Ok(false),
            };

            // emit the arguments
            if let Some(args) = args {
                for arg in args.iter_mut() {
                    is_success &= emit_expr(arg, injector, table, mem_tracker, err_msg)?;
                }
            }

            let fn_rec_id = table.lookup(&fn_name).copied();

            match fn_rec_id {
                Some(rec_id) => {
                    let fn_rec = table.get_record_mut(&rec_id);
                    match fn_rec {
                        Some(Record::Fn { addr, .. }) => {
                            if let Some(f_id) = addr {
                                injector.call(*f_id);
                            } else {
                                return Err(Box::new(ErrorGen::get_unexpected_error(
                                    true,
                                    Some(format!(
                                        "{err_msg} \
                                fn_target address not in symbol table, not emitted yet..."
                                    )),
                                    None,
                                )));
                            }
                        }
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{err_msg} \
                            fn_target not defined in symbol table!"
                                )),
                                None,
                            )));
                        }
                    }
                }
                None => {
                    // Must be defined in the Wasm
                    unimplemented!()
                }
            }
        }
        Expr::VarId { name, .. } => {
            // TODO -- support string vars (unimplemented)
            let var_rec_id = match table.lookup(name) {
                Some(rec_id) => *rec_id,
                _ => {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "{err_msg} \
                    VarId '{}' does not exist in this scope!",
                            name
                        )),
                        None,
                    )));
                }
            };
            return match table.get_record_mut(&var_rec_id) {
                Some(Record::Var { addr, .. }) => {
                    // this will be different based on if this is a global or local var
                    match addr {
                        Some(VarAddr::Global { addr }) => {
                            injector.global_get(*addr);
                        }
                        Some(VarAddr::Local { addr }) => {
                            injector.local_get(*addr);
                        }
                        None => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{err_msg} \
                            Variable does not exist in scope: {}",
                                    name
                                )),
                                None,
                            )));
                        }
                    }
                    Ok(true)
                }
                Some(ty) => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} \
                    Incorrect variable record, expected Record::Var, found: {:?}",
                        ty
                    )),
                    None,
                ))),
                None => Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} \
                    Variable symbol does not exist!"
                    )),
                    None,
                ))),
            };
        }
        Expr::Primitive { val, .. } => {
            is_success &= emit_value(val, injector, table, mem_tracker, err_msg)?;
        }
    }
    Ok(is_success)
}

fn emit_binop<'a, T: Opcode<'a>>(op: &BinOp, injector: &mut T) -> bool {
    match op {
        BinOp::And => {
            // we only support i32's at the moment
            injector.i32_and();
        }
        BinOp::Or => {
            // we only support i32's at the moment
            injector.i32_or();
        }
        BinOp::EQ => {
            // we only support i32's at the moment
            injector.i32_eq();
        }
        BinOp::NE => {
            // we only support i32's at the moment
            injector.i32_ne();
        }
        BinOp::GE => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_gte_signed();
        }
        BinOp::GT => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_gt_signed();
        }
        BinOp::LE => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_lte_signed();
        }
        BinOp::LT => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_lt_signed();
        }
        BinOp::Add => {
            // we only support i32's at the moment
            injector.i32_add();
        }
        BinOp::Subtract => {
            // we only support i32's at the moment
            injector.i32_sub();
        }
        BinOp::Multiply => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_mul();
        }
        BinOp::Divide => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_div_signed();
        }
        BinOp::Modulo => {
            // we only support i32's at the moment (assumes signed)
            injector.i32_rem_signed();
        }
    }
    true
}

fn emit_unop<'a, T: Opcode<'a>>(op: &UnOp, injector: &mut T) -> bool {
    match op {
        UnOp::Not => {
            // return 1 if 0, return 0 otherwise
            injector.i32_eqz();
        }
    }
    true
}

fn emit_value<'a, T: Opcode<'a> + ModuleBuilder>(
    val: &mut Value,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match val {
        Value::Integer { val, .. } => {
            injector.i32_const(*val);
            is_success &= true;
        }
        Value::Str { val, .. } => {
            // At this point the String has been emitted into the Wasm module!
            // See: InitGenerator::visit_value()
            // This is to avoid having to have access to the app_wasm.data here.
            // If this were required, we would have 2 mutable references to app_iter
            // when emitting for VisitingEmitter (won't work for Rust):
            // 1. app_iter.app_wasm.data
            // 2. app_iter

            if let Some(str_addr) = mem_tracker.emitted_strings.get(val) {
                // emit Wasm instructions for the memory address and string length
                injector.i32_const(str_addr.mem_offset as i32);
                injector.i32_const(str_addr.len as i32);
                is_success &= true;
            } else {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} String has not been emitted yet for value: '{val}'!"
                    )),
                    None,
                )));
            }
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter_mut() {
                is_success &= emit_expr(val, injector, table, mem_tracker, err_msg)?;
            }
        }
        Value::Boolean { val, .. } => {
            // "In a boolean context, such as a br_if condition, any non-zero value is interpreted as true
            // and 0 is interpreted as false."
            // https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md#booleans
            if *val {
                // insert true (non-zero)
                injector.i32_const(1);
            } else {
                // insert false (zero)
                injector.i32_const(0);
            }
            is_success &= true;
        }
    }
    Ok(is_success)
}
