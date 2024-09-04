#![allow(clippy::too_many_arguments)]
pub mod module_emitter;
pub mod rules;
pub mod visiting_emitter;

use crate::common::error::{ErrorGen, WhammError};
use crate::emitter::map_lib_adapter::MapLibAdapter;
use crate::emitter::report_var_metadata::{LocationData, ReportVarMetadata};
use crate::emitter::rewriting::module_emitter::MemoryTracker;
use crate::emitter::rewriting::rules::Arg;
use crate::generator::types::ExprFolder;
use crate::parser::types::{BinOp, Block, DataType, Expr, Statement, UnOp, Value};
use crate::verifier::types::{Record, SymbolTable, VarAddr};
use orca::ir::id::{FunctionID, GlobalID, LocalID};
use orca::ir::types::{BlockType, DataType as OrcaType, Value as OrcaValue};
use orca::module_builder::AddLocal;
use orca::opcode::{MacroOpcode, Opcode};
use orca::{InitExpr, Module};

pub trait Emitter {
    fn emit_body(
        &mut self,
        curr_instr_args: &[Arg],
        body: &mut Block,
    ) -> Result<bool, Box<WhammError>>;
    fn emit_stmt(
        &mut self,
        curr_instr_args: &[Arg],
        stmt: &mut Statement,
    ) -> Result<bool, Box<WhammError>>;
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

fn emit_body<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    body: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    for stmt in body.stmts.iter_mut() {
        emit_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
        )?;
    }
    Ok(true)
}

fn emit_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    match stmt {
        Statement::Decl { .. } => emit_decl_stmt(stmt, injector, table, map_lib_adapter, err_msg),
        Statement::Assign { .. } => emit_assign_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
        ),
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => emit_expr(
            expr,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
        ),

        Statement::If {
            cond, conseq, alt, ..
        } => {
            if alt.stmts.is_empty() {
                emit_if(
                    cond,
                    conseq,
                    injector,
                    table,
                    mem_tracker,
                    map_lib_adapter,
                    report_var_metadata,
                    err_msg,
                )
            } else {
                emit_if_else(
                    cond,
                    conseq,
                    alt,
                    injector,
                    table,
                    mem_tracker,
                    map_lib_adapter,
                    report_var_metadata,
                    err_msg,
                )
            }
        }
        Statement::ReportDecl { .. } => emit_report_decl_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
        ),
        Statement::SetMap { .. } => emit_set_map_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
        ),
    }
}

fn emit_decl_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    map_lib_adapter: &mut MapLibAdapter,
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
                                is_report_var: false,
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

            if let DataType::Map { .. } = ty {
                let (fn_name, map_id) = match map_lib_adapter.create_no_meta_map(ty.clone()) {
                    Ok(to_call) => to_call,
                    Err(e) => return Err(e),
                };
                *addr = Some(VarAddr::MapId {
                    addr: map_id as u32,
                });
                let fn_id = match table.lookup_rec(&fn_name) {
                    Some(Record::LibFn { fn_id, .. }) => fn_id,
                    _ => {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!("{err_msg} Map function not in symbol table")),
                            None,
                        )));
                    }
                };

                injector.i32_const(map_id);
                injector.call(FunctionID(*fn_id));
                return Ok(true);
            }
            match &mut addr {
                Some(VarAddr::Global { addr: _addr }) | Some(VarAddr::MapId { addr: _addr }) => {
                    //ignore, initial setup is done in init_gen
                    Ok(true)
                }
                Some(VarAddr::Local { .. }) | None => {
                    // If the local already exists, it would be because the probe has been
                    // emitted at another opcode location. Simply overwrite the previously saved
                    // address.
                    let wasm_ty = whamm_type_to_wasm_type(ty);
                    let id = injector.add_local(wasm_ty);
                    *addr = Some(VarAddr::Local { addr: *id });
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
fn emit_report_decl_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    _mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    if let Statement::ReportDecl { decl, .. } = stmt {
        match &**decl {
            Statement::Decl { ty, var_id, .. } => {
                // look up in symbol table
                let var_name: String;
                let mut addr = if let Expr::VarId { name, .. } = var_id {
                    var_name = name.clone();
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => *rec_id,
                        None => table.put(
                            name.clone(),
                            Record::Var {
                                ty: ty.clone(),
                                name: name.clone(),
                                value: None,
                                is_comp_provided: false,
                                is_report_var: true,
                                addr: None,
                                loc: None,
                            },
                        ),
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
                if let DataType::Map { .. } = ty {
                    let script_name;
                    let bytecode_loc;
                    let probe_id;
                    match &report_var_metadata.curr_location {
                        LocationData::Local {
                            script_id,
                            bytecode_loc: bytecode_loc_cur,
                            probe_id: probe_id_cur,
                            num_reports: _,
                        } => {
                            script_name = script_id;
                            bytecode_loc = bytecode_loc_cur;
                            probe_id = probe_id_cur;
                        }
                        LocationData::Global { .. } => {
                            //ERR here because the location data should be local at this point via the visiting emitter
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!("{err_msg} Expected Local LocationData - shouldn't be called outside visit-generic")),
                                None,
                            )));
                        }
                    }
                    let (fn_name, map_id) = match map_lib_adapter.create_local_map(
                        var_name.clone(),
                        script_name.clone(),
                        *bytecode_loc,
                        probe_id.clone(),
                        ty.clone(),
                        report_var_metadata,
                    ) {
                        Ok(to_call) => to_call,
                        Err(e) => return Err(e),
                    };
                    *addr = Some(VarAddr::MapId {
                        addr: map_id as u32,
                    });
                    let fn_id = match table.lookup_rec(&fn_name) {
                        Some(Record::LibFn { fn_id, .. }) => fn_id,
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!("{err_msg} Map function not in symbol table")),
                                None,
                            )));
                        }
                    };
                    injector.i32_const(map_id);
                    injector.call(FunctionID(*fn_id));
                    return Ok(true);
                }
                match &mut addr {
                    Some(VarAddr::Global { .. }) | None => {
                        let wasm_ty = whamm_type_to_wasm_type(ty);
                        if wasm_ty != OrcaType::I32 {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{err_msg} Expected I32 type for report var, found: {:?}. Further support is upcoming",
                                    wasm_ty
                                )),
                                None,
                            )));
                        }
                        if report_var_metadata.available_i32_gids.is_empty() {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{err_msg} No available global I32s for report vars"
                                )),
                                None,
                            )));
                        }
                        let id = report_var_metadata.available_i32_gids.remove(0);
                        report_var_metadata.used_i32_gids.push(id);
                        match report_var_metadata.put_local_metadata(id, var_name.clone()) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                        *addr = Some(VarAddr::Global { addr: id });
                        return Ok(true);
                    }
                    Some(VarAddr::Local { .. }) | Some(VarAddr::MapId { .. }) => {
                        //this shouldn't happen for report vars - need to err
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!("{err_msg} Expected Global VarAddr.")),
                            None,
                        )));
                    }
                }
            }
            _ => {
                return Err(Box::new(ErrorGen::get_unexpected_error(
                    false,
                    Some(format!("{err_msg} Wrong statement type, should be `decl`")),
                    None,
                )))
            }
        }
    }
    Err(Box::new(ErrorGen::get_unexpected_error(
        false,
        Some(format!(
            "{err_msg} Wrong statement type, should be `report_decl`"
        )),
        None,
    )))
}

fn emit_assign_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
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
                        is_report_var,
                        ..
                    }) => {
                        *value = Some(val.clone());
                        if *is_comp_provided {
                            return Ok(true);
                        }
                        if *is_report_var {
                            //you changed a report variable: need to turn dirty bool to true and then print somewhere
                            report_var_metadata.flush_soon = true;
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

            match emit_expr(
                &mut folded_expr,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            ) {
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
fn emit_set_map_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    if let Statement::SetMap {
        map: Expr::VarId { name, .. },
        key,
        val,
        ..
    } = stmt
    {
        match get_map_info(table, name) {
            Ok((map_id, key_ty, val_ty)) => {
                //no Record in ST, so always flush after a set_map
                report_var_metadata.flush_soon = true;
                let to_call = match map_lib_adapter.insert_map_fname(key_ty, val_ty) {
                    Ok(to_call) => to_call,
                    Err(e) => return Err(e),
                };
                let fn_id = match table.lookup_rec(&to_call) {
                    Some(Record::LibFn { fn_id, .. }) => *fn_id,
                    _ => {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!("{err_msg} Map function not in symbol table")),
                            None,
                        )));
                    }
                };

                injector.u32_const(map_id);
                emit_expr(
                    key,
                    injector,
                    table,
                    mem_tracker,
                    map_lib_adapter,
                    report_var_metadata,
                    err_msg,
                )?;
                emit_expr(
                    val,
                    injector,
                    table,
                    mem_tracker,
                    map_lib_adapter,
                    report_var_metadata,
                    err_msg,
                )?;
                injector.call(FunctionID(fn_id));
                return Ok(true);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Err(Box::new(ErrorGen::get_unexpected_error(
        false,
        Some(format!(
            "{err_msg} \
            Wrong statement type, should be `set_map`"
        )),
        None,
    )))
}

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
// TODO: Do we really want to depend on wasmparser::ValType, or create a wrapper?
pub fn whamm_type_to_wasm_global(app_wasm: &mut Module, ty: &DataType) -> (GlobalID, OrcaType) {
    let orca_ty = whamm_type_to_wasm_type(ty);

    match orca_ty {
        OrcaType::I32 => {
            let global_id = app_wasm.add_global(
                InitExpr::Value(OrcaValue::I32(0)),
                OrcaType::I32,
                true,
                false,
            );
            (global_id, OrcaType::I32)
        }
        _ => unimplemented!(),
    }
}
pub fn whamm_type_to_wasm_type(ty: &DataType) -> OrcaType {
    match ty {
        DataType::I32 | DataType::U32 | DataType::Boolean => OrcaType::I32,
        DataType::F32 => OrcaType::F32,
        DataType::I64 | DataType::U64 => OrcaType::I64,
        DataType::F64 => OrcaType::F64,
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
                        injector.global_set(GlobalID(*addr));
                    }
                    Some(VarAddr::Local { addr }) => {
                        injector.local_set(LocalID(*addr));
                    },
                    Some(VarAddr::MapId { .. }) => {
                        return Err(Box::new(ErrorGen::get_type_check_error_from_loc(false,
                            format!("Attempted to assign a var to Map: {}", name), loc)));
                    }
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

fn emit_if_preamble<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    // emit the condition of the `if` expression
    is_success &= emit_expr(
        condition,
        injector,
        table,
        mem_tracker,
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;
    // emit the beginning of the if block
    injector.if_stmt(block_type_to_wasm(conseq));
    // emit the consequent body
    is_success &= emit_body(
        conseq,
        injector,
        table,
        mem_tracker,
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;

    // INTENTIONALLY DON'T END IF BLOCK

    Ok(is_success)
}

fn emit_if_else_preamble<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    is_success &= emit_if_preamble(
        condition,
        conseq,
        injector,
        table,
        mem_tracker,
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;

    // emit the beginning of the else
    injector.else_stmt();

    // emit the alternate body
    is_success &= emit_body(
        alternate,
        injector,
        table,
        mem_tracker,
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;

    // INTENTIONALLY DON'T END IF/ELSE BLOCK

    Ok(is_success)
}

fn emit_if<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;

    is_success &= emit_if_preamble(
        condition,
        conseq,
        injector,
        table,
        mem_tracker,
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;

    // emit the end of the if block
    injector.end();
    Ok(is_success)
}

fn emit_if_else<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
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
        map_lib_adapter,
        report_var_metadata,
        err_msg,
    )?;

    // emit the end of the if block
    injector.end();
    Ok(is_success)
}

// TODO: emit_expr has two mutable references to the name object, the injector has module data in it
fn emit_expr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    expr: &mut Expr,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match expr {
        Expr::UnOp { op, expr, .. } => {
            is_success &= emit_expr(
                expr,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            )?;
            is_success &= emit_unop(op, injector);
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            is_success &= emit_expr(
                lhs,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            )?;
            is_success &= emit_expr(
                rhs,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            )?;
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
                map_lib_adapter,
                report_var_metadata,
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
                    is_success &= emit_expr(
                        arg,
                        injector,
                        table,
                        mem_tracker,
                        map_lib_adapter,
                        report_var_metadata,
                        err_msg,
                    )?;
                }
            }

            let fn_rec_id = table.lookup(&fn_name).copied();

            match fn_rec_id {
                Some(rec_id) => {
                    let fn_rec = table.get_record_mut(&rec_id);
                    match fn_rec {
                        Some(Record::Fn { addr, .. }) => {
                            if let Some(f_id) = addr {
                                injector.call(FunctionID(*f_id));
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
                            injector.global_get(GlobalID(*addr));
                        }
                        Some(VarAddr::Local { addr }) => {
                            injector.local_get(LocalID(*addr));
                        }
                        Some(VarAddr::MapId { .. }) => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!(
                                    "{err_msg} \
                                        Variable you are trying to use in expr is a Map object {}",
                                    name
                                )),
                                None,
                            )));
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
            is_success &= emit_value(
                val,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            )?;
        }
        Expr::MapGet { .. } => {
            is_success &= emit_map_get(
                expr,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
            )?;
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

fn emit_value<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    val: &mut Value,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    let mut is_success = true;
    match val {
        Value::U32 { val, .. } => {
            injector.u32_const(*val);
            is_success &= true;
        }
        Value::I32 { val, .. } => {
            injector.i32_const(*val);
            is_success &= true;
        }
        Value::F32 { val, .. } => {
            injector.f32_const(*val);
            is_success &= true;
        }
        Value::U64 { val, .. } => {
            injector.u64_const(*val);
            is_success &= true;
        }
        Value::I64 { val, .. } => {
            injector.i64_const(*val);
            is_success &= true;
        }
        Value::F64 { val, .. } => {
            injector.f64_const(*val);
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
                injector.u32_const(str_addr.mem_offset as u32);
                injector.u32_const(str_addr.len as u32);
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
                is_success &= emit_expr(
                    val,
                    injector,
                    table,
                    mem_tracker,
                    map_lib_adapter,
                    report_var_metadata,
                    err_msg,
                )?;
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
fn emit_map_get<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    expr: &mut Expr,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
) -> Result<bool, Box<WhammError>> {
    if let Expr::MapGet { map, key, .. } = expr {
        let map = &mut (**map);
        if let Expr::VarId { name, .. } = map {
            match get_map_info(table, name) {
                Ok((map_id, key_ty, val_ty)) => {
                    let to_call = match map_lib_adapter.get_map_fname(key_ty, val_ty) {
                        Ok(to_call) => to_call,
                        Err(e) => return Err(e),
                    };
                    let fn_id = match table.lookup_rec(&to_call) {
                        Some(Record::LibFn { fn_id, .. }) => *fn_id,
                        _ => {
                            return Err(Box::new(ErrorGen::get_unexpected_error(
                                true,
                                Some(format!("{err_msg} Map function not in symbol table")),
                                None,
                            )));
                        }
                    };
                    injector.u32_const(map_id);
                    emit_expr(
                        key,
                        injector,
                        table,
                        mem_tracker,
                        map_lib_adapter,
                        report_var_metadata,
                        err_msg,
                    )?;
                    injector.call(FunctionID(fn_id));
                    return Ok(true);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
    Err(Box::new(ErrorGen::get_unexpected_error(
        false,
        Some(format!(
            "{err_msg} \
            Wrong statement type, should be `map_get`"
        )),
        None,
    )))
}
fn get_map_info(
    table: &mut SymbolTable,
    name: &mut String,
) -> Result<(u32, DataType, DataType), Box<WhammError>> {
    let var_rec_id = match table.lookup(name) {
        Some(rec_id) => *rec_id,
        _ => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!("VarId '{name}' does not exist in this scope!")),
                None,
            )));
        }
    };
    let map_id;
    let key_ty;
    let val_ty;
    match table.get_record_mut(&var_rec_id) {
        Some(Record::Var { ty, addr, .. }) => {
            match addr {
                Some(VarAddr::MapId { addr }) => {
                    //save off the map_id for the later set call
                    map_id = *addr;
                    if let DataType::Map {
                        key_ty: k,
                        val_ty: v,
                    } = ty
                    {
                        key_ty = *k.clone();
                        val_ty = *v.clone();
                    } else {
                        return Err(Box::new(ErrorGen::get_unexpected_error(
                            true,
                            Some(format!(
                                "Incorrect variable record, Map address, found: {:?}",
                                addr.clone()
                            )),
                            None,
                        )));
                    }
                }
                _ => {
                    return Err(Box::new(ErrorGen::get_unexpected_error(
                        true,
                        Some(format!(
                            "Incorrect variable record, Map address, found: {:?}",
                            addr
                        )),
                        None,
                    )));
                }
            }
        }
        Some(ty) => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "Incorrect variable record, expected Record::Var, found: {:?}",
                    ty
                )),
                None,
            )));
        }
        None => {
            return Err(Box::new(ErrorGen::get_unexpected_error(
                true,
                Some("Variable symbol does not exist!".to_string()),
                None,
            )));
        }
    }
    Ok((map_id, key_ty, val_ty))
}
fn print_report_all<'a, T: Opcode<'a> + AddLocal>(
    _injector: &mut T,
    _table: &mut SymbolTable,
    report_var_metadata: &mut ReportVarMetadata,
    _err_msg: &str,
) -> Result<(), Box<WhammError>> {
    if !report_var_metadata.flush_soon {
        return Ok(());
    }
    //TODO - uncomment this when we have metadata maps correctly initialized
    // let fn_id = match table.lookup("print_meta") {
    //     Some(rec_id) => *rec_id,
    //     _ => {
    //         return Err(Box::new(ErrorGen::get_unexpected_error(
    //             true,
    //             Some(format!(
    //                 "{err_msg} \
    //                 print_meta function not in symbol table!"
    //             )),
    //             None,
    //         )));
    //     }
    // };
    // injector.call(fn_id as u32);
    // report_var_metadata.flush_soon = false;
    Ok(())
}
