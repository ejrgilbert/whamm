pub mod rules;
pub mod module_emitter;
pub mod visiting_emitter;

use crate::common::error::{ErrorGen, WhammError};
use crate::parser::types::{BinOp, DataType, Expr, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};

use orca::ir::types::{Global, InitExpr, Value as OrcaValue};
use wasmparser::ValType;
use orca::opcode::Opcode;

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
// TODO: Do we really want to depend on wasmparser::ValType, or create a wrapper?
fn whamm_type_to_wasm(ty: &DataType) -> Global {
    match ty {
        DataType::I32 | DataType::U32 | DataType::Boolean => Global {
            ty: wasmparser::GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            init_expr: InitExpr::Value(OrcaValue::I32(0)),
        },
        // the ID used to track this var in the lib
        DataType::Map { .. } => Global {
            ty: wasmparser::GlobalType {
                content_type: ValType::I32,
                mutable: true,
                shared: false,
            },
            init_expr: InitExpr::Value(OrcaValue::I32(0)),
        },
        DataType::Null => unimplemented!(),
        DataType::Str => unimplemented!(),
        DataType::Tuple { .. } => unimplemented!(),
        DataType::AssumeGood => unimplemented!(),
    }
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

fn emit_set<'a, T: Opcode<'a>>(
    table: &mut SymbolTable,
    var_id: &mut Expr,
    injector: &mut T,
    err_msg: &str
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
                    Some(VarAddr::Global { addr: _addr }) => {
                        // todo
                        // injector.global_set(*addr);
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
            Some(format!(
                "{err_msg} Expected VarId."
            )),
            None,
        )))
    }
}

fn emit_expr<'a, T: Opcode<'a>>(
    table: &mut SymbolTable,
    // module_data: &mut Vec<DataSegment>, // todo -- fix once we have this available
    expr: &mut Expr,
    injector: &mut T,
    metadata: &mut InsertionMetadata,
    err_msg: &str
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match expr {
        Expr::UnOp { op, expr, .. } => {
            is_success &= emit_expr(table, expr, injector, metadata, err_msg)?;
            is_success &= emit_unop(op, injector);
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            is_success &= emit_expr(table, lhs, injector, metadata, err_msg)?;
            is_success &= emit_expr(table, rhs, injector, metadata, err_msg)?;
            is_success &= emit_binop(op, injector);
        }
        Expr::Ternary {
            cond: _cond,
            conseq: _conseq,
            alt: _alt,
            ..
        } => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "{err_msg} \
                            Ternary expressions should be handled before this point!"
                )),
                None,
            )));
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
                for boxed_arg in args.iter_mut() {
                    let arg = &mut **boxed_arg; // unbox
                    is_success &=
                        emit_expr(table, arg, injector, metadata, err_msg)?;
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
                        Some(VarAddr::Global { addr: _addr }) => {
                            // todo
                            // injector.global_get(*addr);
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
            is_success &= emit_value(table, val, injector, metadata, err_msg)?;
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

fn emit_value<'a, T: Opcode<'a>>(
    table: &mut SymbolTable,
    // module_data: &mut Vec<DataSegment>, // todo -- fix once we have this available
    val: &mut Value,
    injector: &mut T,
    metadata: &mut InsertionMetadata,
    err_msg: &str
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match val {
        Value::Integer { val, .. } => {
            injector.i32_const(*val);
            is_success &= true;
        }
        Value::Str { val: _val, addr: _addr, ty: _ty } => {
            // TODO -- assuming that the data ID is the index of the object in the Vec
            // TODO -- need an API that allows the addition of data segments.
            //     there is currently an ownership issue since I can't insert
            //     an owned byte array with same lifetime as the Module data segments.
            //     For more info, uncomment the below and read error.
            // let data_id = module_data.len();
            // let val_bytes = val.as_bytes().to_owned();
            // let data_segment = DataSegment {
            //     data: val_bytes.as_slice(),
            //     kind: DataSegmentKind::Active {
            //         memory_index: metadata.mem_id,
            //         offset_expr: ConstExpr::new(BinaryReader::new(
            //             val_bytes.as_slice(),
            //             metadata.curr_mem_offset,
            //             WasmFeatures::empty()
            //         ))
            //     }
            // };
            // module_data.push(
            //     data_segment
            // );
            // 
            // // save the memory addresses/lens, so they can be used as appropriate
            // *addr = Some((data_id as u32, metadata.curr_mem_offset, val.len()));
            // 
            // // emit Wasm instructions for the memory address and string length
            // injector.i32(metadata.curr_mem_offset as i32);
            // injector.i32(val.len() as i32);
            // 
            // // update curr_mem_offset to account for new data
            // metadata.curr_mem_offset += val.len();
            is_success &= true;
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter_mut() {
                is_success &= emit_expr(table, val, injector, metadata, err_msg)?;
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

// ==============================
// ==== WasmRewritingEmitter ====
// ==============================

struct InsertionMetadata {
    // curr_event: String,
    mem_id: u32,
    curr_mem_offset: usize,
}
