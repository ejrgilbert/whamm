#![allow(clippy::too_many_arguments)]
pub mod module_emitter;
pub mod rules;
pub mod visiting_emitter;

use crate::common::error::ErrorGen;
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::emitter::rewriting::module_emitter::MemoryTracker;
use crate::emitter::rewriting::rules::Arg;
use crate::generator::types::ExprFolder;
use crate::linker::core::maps::map_lib_adapter::MapLibAdapter;
use crate::parser::types::{BinOp, Block, DataType, Expr, Location, Statement, UnOp, Value};
use crate::verifier::types::{line_col_from_loc, Record, SymbolTable, VarAddr};
use orca_wasm::ir::id::{FunctionID, GlobalID, LocalID};
use orca_wasm::ir::types::{BlockType, DataType as OrcaType, Value as OrcaValue};
use orca_wasm::module_builder::AddLocal;
use orca_wasm::opcode::{MacroOpcode, Opcode};
use orca_wasm::{InitExpr, Module};

pub trait Emitter {
    fn emit_body(&mut self, curr_instr_args: &[Arg], body: &mut Block, err: &mut ErrorGen) -> bool;
    fn emit_stmt(
        &mut self,
        curr_instr_args: &[Arg],
        stmt: &mut Statement,
        err: &mut ErrorGen,
    ) -> bool;
    fn emit_expr(&mut self, expr: &mut Expr, err: &mut ErrorGen) -> bool;
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
    err: &mut ErrorGen,
) -> bool {
    let mut is_success = true;
    for stmt in body.stmts.iter_mut() {
        is_success &= emit_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        );
    }
    is_success
}

fn emit_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    match stmt {
        Statement::Decl { .. } => {
            emit_decl_stmt(stmt, injector, table, map_lib_adapter, err_msg, err)
        }
        Statement::Assign { .. } => emit_assign_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        ),
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => emit_expr(
            expr,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
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
                    err,
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
                    err,
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
            err,
        ),
        Statement::SetMap { .. } => emit_set_map_stmt(
            stmt,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        ),
    }
}

fn emit_decl_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    map_lib_adapter: &mut MapLibAdapter,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    match stmt {
        Statement::Decl { ty, var_id, .. } => {
            // look up in symbol table
            let mut addr = if let Expr::VarId { name, .. } = var_id {
                let var_rec_id = match table.lookup(name) {
                    Some(rec_id) => rec_id,
                    None => {
                        // add variables from body into symbol table (at this point, the verifier should have run to catch variable initialization without declaration)
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
                match table.get_record_mut(var_rec_id) {
                    Some(Record::Var { addr, .. }) => addr,
                    Some(ty) => {
                        err.unexpected_error(
                            true,
                            Some(format!(
                                "{err_msg} Incorrect variable record, expected Record::Var, found: {:?}",
                                ty
                            )),
                            None,
                        );
                        return false;
                    }
                    None => {
                        err.unexpected_error(
                            true,
                            Some(format!("{err_msg} Variable symbol does not exist!")),
                            None,
                        );
                        return false;
                    }
                }
            } else {
                err.unexpected_error(true, Some(format!("{err_msg} Expected VarId.")), None);
                return false;
            };

            if let DataType::Map { .. } = ty {
                let (map_id, fn_name) = map_lib_adapter.create_map(ty.clone(), err);
                *addr = Some(VarAddr::MapId { addr: map_id });
                if fn_name.is_none() {
                    return false;
                }
                let fn_name = fn_name.unwrap();
                let Some(fn_id) = table.lookup_core_lib_func(&fn_name, &None, err) else {
                    return false;
                };

                injector.u32_const(map_id);
                injector.call(FunctionID(fn_id));
                return true;
            }
            match &mut addr {
                Some(VarAddr::Global { addr: _addr }) | Some(VarAddr::MapId { addr: _addr }) => {
                    //ignore, initial setup is done in init_gen
                    true
                }
                Some(VarAddr::Local { .. }) | None => {
                    // If the local already exists, it would be because the probe has been
                    // emitted at another opcode location. Simply overwrite the previously saved
                    // address.
                    let wasm_ty = whamm_type_to_wasm_type(ty);
                    let id = injector.add_local(wasm_ty);
                    *addr = Some(VarAddr::Local { addr: *id });
                    true
                }
            }
        }
        _ => {
            err.unexpected_error(
                false,
                Some(format!(
                    "{err_msg} Wrong statement type, should be `assign`"
                )),
                None,
            );
            false
        }
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
    err: &mut ErrorGen,
) -> bool {
    if let Statement::ReportDecl { decl, .. } = stmt {
        return match &**decl {
            Statement::Decl { ty, var_id, .. } => {
                // look up in symbol table
                let var_name: String;
                let mut addr = if let Expr::VarId { name, loc, .. } = var_id {
                    var_name = name.clone();
                    let var_rec_id = match table.lookup(name) {
                        Some(rec_id) => rec_id,
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
                    let Some(Record::Var { addr, .. }) =
                        table.get_rec_var_mut(var_rec_id, loc, err)
                    else {
                        err.unexpected_error(true, Some("unexpected type".to_string()), None);
                        return false;
                    };
                    addr
                } else {
                    err.unexpected_error(true, Some(format!("{err_msg} Expected VarId.")), None);
                    return false;
                };
                if let DataType::Map { .. } = ty {
                    let (map_id, fn_name) = map_lib_adapter.create_report_map(
                        var_name.clone(),
                        ty.clone(),
                        report_var_metadata,
                        true,
                        err,
                    );
                    *addr = Some(VarAddr::MapId { addr: map_id });
                    if fn_name.is_none() {
                        return false;
                    }
                    let fn_name = fn_name.unwrap();

                    return if let Some(fn_id) = table.lookup_core_lib_func(&fn_name, &None, err) {
                        injector.u32_const(map_id);
                        injector.call(FunctionID(fn_id));
                        true
                    } else {
                        err.unexpected_error(true, Some("unexpected type".to_string()), None);
                        false
                    };
                }
                match &mut addr {
                    Some(VarAddr::Global { .. }) | None => {
                        let wasm_ty = whamm_type_to_wasm_type(ty);
                        let id = report_var_metadata
                            .use_available_global(var_name, wasm_ty, err_msg, err);
                        if id.is_none() {
                            return false;
                        }
                        let id = id.unwrap();
                        *addr = Some(VarAddr::Global { addr: id });
                        true
                    }
                    Some(VarAddr::Local { .. }) | Some(VarAddr::MapId { .. }) => {
                        //this shouldn't happen for report vars - need to err
                        err.unexpected_error(
                            true,
                            Some(format!("{err_msg} Expected Global VarAddr.")),
                            None,
                        );
                        return false;
                    }
                }
            }
            _ => {
                err.unexpected_error(
                    false,
                    Some(format!("{err_msg} Wrong statement type, should be `decl`")),
                    None,
                );
                false
            }
        };
    }
    err.unexpected_error(
        false,
        Some(format!(
            "{err_msg} Wrong statement type, should be `report_decl`"
        )),
        None,
    );
    false
}

fn emit_assign_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    match stmt {
        Statement::Assign { var_id, expr, .. } => {
            let mut folded_expr = ExprFolder::fold_expr(expr, table, err);

            // Save off primitives to symbol table
            // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
            if let (Expr::VarId { name, .. }, Expr::Primitive { val, .. }) = (&var_id, &folded_expr)
            {
                let Some(Record::Var {
                    value,
                    is_comp_provided,
                    is_report_var,
                    ..
                }) = table.lookup_var_mut(name, &None, err)
                else {
                    err.unexpected_error(true, Some("unexpected type".to_string()), None);
                    return false;
                };

                *value = Some(val.clone());
                if *is_comp_provided {
                    return true;
                }
                if *is_report_var {
                    //you changed a report variable: need to turn dirty bool to true and then print somewhere
                    report_var_metadata.flush_soon = true;
                }
            }

            if !emit_expr(
                &mut folded_expr,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
                err,
            ) {
                return false;
            }

            // Emit the instruction that sets the variable's value to the emitted expression
            emit_set(var_id, injector, table, report_var_metadata, err_msg, err)
        }
        _ => {
            err.unexpected_error(
                false,
                Some(format!(
                    "{err_msg} \
                    Wrong statement type, should be `assign`"
                )),
                None,
            );
            false
        }
    }
}

fn emit_set_map_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    if let Statement::SetMap {
        map: Expr::VarId { name, .. },
        key,
        val,
        ..
    } = stmt
    {
        let Some((map_id, key_ty, val_ty)) = get_map_info(table, name, err) else {
            return false;
        };
        report_var_metadata.mutating_map(map_id);

        injector.u32_const(map_id);
        emit_expr(
            key,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        );
        emit_expr(
            val,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        );
        let fname = map_lib_adapter.map_insert_fname(key_ty, val_ty, err);
        if fname.is_none() {
            return false;
        }
        let fname = fname.unwrap();
        if let Some(fn_id) = table.lookup_core_lib_func(&fname, &None, err) {
            injector.call(FunctionID(fn_id));
            true
        } else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            false
        }
    } else {
        err.unexpected_error(
            false,
            Some(format!(
                "{err_msg} \
            Wrong statement type, should be `set_map`"
            )),
            None,
        );
        false
    }
}

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
// TODO: Do we really want to depend on wasmparser::ValType, or create a wrapper?
pub fn whamm_type_to_wasm_global(app_wasm: &mut Module, ty: &DataType) -> (GlobalID, OrcaType) {
    let orca_wasm_ty = whamm_type_to_wasm_type(ty);

    match orca_wasm_ty {
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
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    if let Expr::VarId { name, .. } = var_id {
        let Some(Record::Var { addr, loc, .. }) = table.lookup_var_mut(name, &None, err) else {
            err.unexpected_error(true, Some("unexpected type".to_string()), None);
            return false;
        };

        // this will be different based on if this is a global or local var
        match addr {
            Some(VarAddr::Global { addr }) => {
                report_var_metadata.mutating_var(*addr);
                injector.global_set(GlobalID(*addr));
            }
            Some(VarAddr::Local { addr }) => {
                report_var_metadata.mutating_var(*addr);
                injector.local_set(LocalID(*addr));
            }
            Some(VarAddr::MapId { .. }) => {
                err.type_check_error(
                    false,
                    format!("Attempted to assign a var to Map: {}", name),
                    &line_col_from_loc(loc),
                );
                return false;
            }
            None => {
                err.type_check_error(
                    false,
                    format!("Variable assigned before declared: {}", name),
                    &line_col_from_loc(loc),
                );
                return false;
            }
        }
        true
    } else {
        err.unexpected_error(true, Some(format!("{err_msg} Expected VarId.")), None);
        false
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
    err: &mut ErrorGen,
) -> bool {
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
        err,
    );
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
        err,
    );

    // INTENTIONALLY DON'T END IF BLOCK
    is_success
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
    err: &mut ErrorGen,
) -> bool {
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
        err,
    );

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
        err,
    );

    // INTENTIONALLY DON'T END IF/ELSE BLOCK

    is_success
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
    err: &mut ErrorGen,
) -> bool {
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
        err,
    );

    // emit the end of the if block
    injector.end();
    is_success
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
    err: &mut ErrorGen,
) -> bool {
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
        err,
    );

    // emit the end of the if block
    injector.end();
    is_success
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
    err: &mut ErrorGen,
) -> bool {
    match expr {
        Expr::UnOp { op, expr, .. } => {
            let mut is_success = emit_expr(
                expr,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
                err,
            );
            is_success &= emit_unop(op, injector);
            is_success
        }
        Expr::BinOp { lhs, op, rhs, .. } => {
            let mut is_success = emit_expr(
                lhs,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
                err,
            );
            is_success &= emit_expr(
                rhs,
                injector,
                table,
                mem_tracker,
                map_lib_adapter,
                report_var_metadata,
                err_msg,
                err,
            );
            is_success &= emit_binop(op, injector);
            is_success
        }
        Expr::Ternary {
            cond, conseq, alt, ..
        } => {
            // change conseq and alt types to stmt for easier API call
            emit_if_else(
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
                err,
            )
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            let fn_name = match &**fn_target {
                Expr::VarId { name, .. } => name.clone(),
                _ => return false,
            };

            // emit the arguments
            let mut is_success = true;
            if let Some(args) = args {
                for arg in args.iter_mut() {
                    is_success = emit_expr(
                        arg,
                        injector,
                        table,
                        mem_tracker,
                        map_lib_adapter,
                        report_var_metadata,
                        err_msg,
                        err,
                    );
                }
            }

            let Some(Record::Fn { addr, .. }) = table.lookup_fn(&fn_name, err) else {
                err.unexpected_error(true, Some("unexpected type".to_string()), None);
                return false;
            };
            if let Some(f_id) = addr {
                injector.call(FunctionID(*f_id));
            } else {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} \
                                fn_target address not in symbol table, not emitted yet..."
                    )),
                    None,
                );
                return false;
            }
            is_success
        }
        Expr::VarId { name, .. } => {
            // TODO -- support string vars (unimplemented)
            let Some(Record::Var { addr, .. }) = table.lookup_var_mut(name, &None, err) else {
                err.unexpected_error(true, Some("unexpected type".to_string()), None);
                return false;
            };
            // this will be different based on if this is a global or local var
            return match addr {
                Some(VarAddr::Global { addr }) => {
                    injector.global_get(GlobalID(*addr));
                    true
                }
                Some(VarAddr::Local { addr }) => {
                    injector.local_get(LocalID(*addr));
                    true
                }
                Some(VarAddr::MapId { .. }) => {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{err_msg} \
                                Variable you are trying to use in expr is a Map object {}",
                            name
                        )),
                        None,
                    );
                    return false;
                }
                None => {
                    err.unexpected_error(
                        true,
                        Some(format!(
                            "{err_msg} \
                    Variable does not exist in scope: {}",
                            name
                        )),
                        None,
                    );
                    return false;
                }
            };
        }
        Expr::Primitive { val, .. } => emit_value(
            val,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        ),
        Expr::MapGet { .. } => emit_map_get(
            expr,
            injector,
            table,
            mem_tracker,
            map_lib_adapter,
            report_var_metadata,
            err_msg,
            err,
        ),
    }
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
    err: &mut ErrorGen,
) -> bool {
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
                err.unexpected_error(
                    true,
                    Some(format!(
                        "{err_msg} String has not been emitted yet for value: '{val}'!"
                    )),
                    None,
                );
                return false;
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
                    err,
                );
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
    is_success
}

fn emit_map_get<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    expr: &mut Expr,
    injector: &mut T,
    table: &mut SymbolTable,
    mem_tracker: &MemoryTracker,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err_msg: &str,
    err: &mut ErrorGen,
) -> bool {
    if let Expr::MapGet { map, key, .. } = expr {
        let map = &mut (**map);
        if let Expr::VarId { name, .. } = map {
            match get_map_info(table, name, err) {
                Some((map_id, key_ty, val_ty)) => {
                    let to_call = map_lib_adapter.map_get_fname(key_ty, val_ty, err);
                    if to_call.is_none() {
                        return false;
                    }
                    let to_call = to_call.unwrap();

                    return if let Some(fn_id) = table.lookup_core_lib_func(&to_call, &None, err) {
                        injector.u32_const(map_id);
                        emit_expr(
                            key,
                            injector,
                            table,
                            mem_tracker,
                            map_lib_adapter,
                            report_var_metadata,
                            err_msg,
                            err,
                        );
                        injector.call(FunctionID(fn_id));
                        true
                    } else {
                        false
                    };
                }
                None => {
                    return false;
                }
            }
        }
    }
    err.unexpected_error(
        false,
        Some(format!(
            "{err_msg} \
            Wrong statement type, should be `map_get`"
        )),
        None,
    );
    false
}
fn get_map_info(
    table: &mut SymbolTable,
    name: &mut str,
    err: &mut ErrorGen,
) -> Option<(u32, DataType, DataType)> {
    let Some(Record::Var { ty, addr, loc, .. }) = table.lookup_var(name, &None, err, true) else {
        err.unexpected_error(true, Some("unexpected type".to_string()), None);
        return None;
    };
    match addr {
        Some(VarAddr::MapId { addr }) => {
            //save off the map_id for the later set call
            let map_id = addr;
            if let DataType::Map {
                key_ty: k,
                val_ty: v,
            } = ty
            {
                let key_ty = *k.clone();
                let val_ty = *v.clone();
                Some((*map_id, key_ty, val_ty))
            } else {
                err.unexpected_error(
                    true,
                    Some(format!(
                        "Incorrect DataType, expected Map, found: {:?}",
                        addr.clone()
                    )),
                    loc.as_ref()
                        .map(|Location { line_col, .. }| line_col.clone()),
                );
                None
            }
        }
        _ => {
            err.unexpected_error(
                true,
                Some(format!(
                    "Incorrect variable record, expected MapId, found: {:?}",
                    addr
                )),
                None,
            );
            None
        }
    }
}
fn print_report_all<'a, T: Opcode<'a> + AddLocal>(
    injector: &mut T,
    table: &mut SymbolTable,
    report_var_metadata: &mut ReportVarMetadata,
    err: &mut ErrorGen,
) {
    if !report_var_metadata.flush_soon {
        return;
    }
    let Some(Record::Fn {
        addr: Some(fid), ..
    }) = table.lookup_fn("print_global_meta", err)
    else {
        err.unexpected_error(true, Some("unexpected type".to_string()), None);
        return;
    };
    injector.call(FunctionID(*fid));

    let Some(Record::Fn {
        addr: Some(fid), ..
    }) = table.lookup_fn("print_map_meta", err)
    else {
        err.unexpected_error(true, Some("unexpected type".to_string()), None);
        return;
    };
    injector.call(FunctionID(*fid));
    report_var_metadata.performed_flush();
}
