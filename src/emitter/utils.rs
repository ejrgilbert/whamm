#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::tag_handler::get_tag_for;
use crate::emitter::InjectStrategy;
use crate::generator::folding::ExprFolder;
use crate::lang_features::libraries::core::maps::map_adapter::{MapLibAdapter, MAP_LIB_MEM_OFFSET};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Location, NumLit, Statement, UnOp, Value,
};
use crate::verifier::types::{line_col_from_loc, Record, SymbolTable, VarAddr};
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::{FunctionID, GlobalID, LocalID};
use wirm::ir::types::{BlockType, DataType as WirmType, InitExpr, Value as WirmValue};
use wirm::module_builder::AddLocal;
use wirm::opcode::{MacroOpcode, Opcode};
use wirm::{InitInstr, Module};
// ==================================================================
// ================ Emitter Helper Functions ========================
// - Necessary to extract common logic between Emitter and InstrumentationVisitor.
// - Can't pass an Emitter instance to InstrumentationVisitor due to Rust not
// - allowing nested references to a common mutable object. So I can't pass the
// - Emitter to the InstrumentationVisitor since I must iterate over Emitter.app_wasm
// - with a construction of InstrumentationVisitor inside that loop.
// ==================================================================
// ==================================================================

pub struct EmitCtx<'a, 'b, 'c, 'd, 'e> {
    table: &'a mut SymbolTable,
    mem_allocator: &'b MemoryAllocator,
    locals_tracker: &'c mut LocalsTracker,
    in_map_op: bool,
    in_lib_call_to: Option<String>,
    map_lib_adapter: &'d mut MapLibAdapter,
    err_msg: String,
    err: &'e mut ErrorGen,
}
impl<'a, 'b, 'c, 'd, 'e> EmitCtx<'a, 'b, 'c, 'd, 'e> {
    pub fn new(
        table: &'a mut SymbolTable,
        mem_allocator: &'b MemoryAllocator,
        locals_tracker: &'c mut LocalsTracker,
        map_lib_adapter: &'d mut MapLibAdapter,
        err_msg: &str,
        err: &'e mut ErrorGen,
    ) -> Self {
        Self {
            table,
            mem_allocator,
            locals_tracker,
            in_map_op: false,
            in_lib_call_to: None,
            map_lib_adapter,
            err_msg: err_msg.to_string(),
            err,
        }
    }
}

pub fn emit_body<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    body: &mut Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;
    for stmt in body.stmts.iter_mut() {
        is_success &= emit_stmt(stmt, strategy, injector, ctx);
    }
    is_success
}

pub fn emit_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::LibImport { .. } => true, // already handled!
        Statement::Decl { .. } => emit_decl_stmt(stmt, injector, ctx),
        Statement::Assign { .. } => emit_assign_stmt(stmt, strategy, injector, ctx),
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => {
            emit_expr(expr, strategy, injector, ctx)
        }

        Statement::If {
            cond, conseq, alt, ..
        } => {
            if alt.stmts.is_empty() {
                emit_if(cond, conseq, strategy, injector, ctx)
            } else {
                emit_if_else(cond, conseq, alt, strategy, injector, ctx)
            }
        }
        Statement::UnsharedDecl { .. } => emit_unshared_decl_stmt(stmt, ctx),
        Statement::SetMap { .. } => {
            ctx.in_map_op = true;
            let res = emit_set_map_stmt(stmt, strategy, injector, ctx);
            ctx.in_map_op = false;
            res
        }
    }
}

fn emit_decl_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::Decl { ty, var_id, .. } => {
            // look up in symbol table
            let mut addr = if let Expr::VarId { name, .. } = var_id {
                let var_rec_id = match ctx.table.lookup(name) {
                    Some(rec_id) => rec_id,
                    None => {
                        // add variables from body into symbol table (at this point, the verifier should have run to catch variable initialization without declaration)
                        ctx.table.put(
                            name.clone(),
                            Record::Var {
                                ty: ty.clone(),
                                value: None,
                                def: Definition::User,
                                addr: None,
                                loc: None,
                            },
                        )
                    }
                };
                match ctx.table.get_record_mut(var_rec_id) {
                    Some(Record::Var { addr, .. }) => addr,
                    Some(ty) => {
                        ctx.err.unexpected_error(
                            true,
                            Some(format!(
                                "{} Incorrect variable record, expected Record::Var, found: {:?}",
                                ctx.err_msg, ty
                            )),
                            None,
                        );
                        return false;
                    }
                    None => {
                        ctx.err.unexpected_error(
                            true,
                            Some(format!("{} Variable symbol does not exist!", ctx.err_msg)),
                            None,
                        );
                        return false;
                    }
                }
            } else {
                ctx.err.unexpected_error(
                    true,
                    Some(format!("{} Expected VarId.", ctx.err_msg)),
                    None,
                );
                return false;
            };

            if let DataType::Map { .. } = ty {
                ctx.in_map_op = true;
                // TODO -- this behavior doesn't seem right for wizard
                //    The map_id would need to be dynamic...not statically known!
                let map_id = ctx
                    .map_lib_adapter
                    .map_create(ty.clone(), injector, ctx.err);
                *addr = Some(VarAddr::MapId { addr: map_id });
                ctx.in_map_op = false;

                return true;
            }
            match &mut addr {
                Some(VarAddr::Global { addr: _addr }) | Some(VarAddr::MapId { addr: _addr }) => {
                    //ignore, initial setup is done in init_gen
                    true
                }
                Some(VarAddr::MemLoc { .. }) => {
                    //ignore, initial setup is done in $alloc
                    true
                }
                Some(VarAddr::Local { .. }) | None => {
                    // If the local already exists, it would be because the probe has been
                    // emitted at another opcode location. Simply overwrite the previously saved
                    // address.
                    let wasm_ty = ty.to_wasm_type();
                    if wasm_ty.len() == 1 {
                        let id = ctx
                            .locals_tracker
                            .use_local(*wasm_ty.first().unwrap(), injector);
                        *addr = Some(VarAddr::Local { addr: id });
                        true
                    } else {
                        todo!("not supported the type yet: {:?} as {:#?}", var_id, ty)
                    }
                }
            }
        }
        _ => {
            ctx.err.unexpected_error(
                false,
                Some(format!(
                    "{} Wrong statement type, should be `assign`",
                    ctx.err_msg
                )),
                None,
            );
            false
        }
    }
}

fn emit_unshared_decl_stmt(stmt: &mut Statement, ctx: &mut EmitCtx) -> bool {
    if let Statement::UnsharedDecl { .. } = stmt {
        // ignore, this statement has already been processed!
        return true;
    }
    ctx.err.unexpected_error(
        false,
        Some(format!(
            "{} Wrong statement type, should be `report_decl`",
            ctx.err_msg
        )),
        None,
    );
    false
}

fn emit_assign_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::Assign { var_id, expr, .. } => {
            // Save off primitives to symbol table
            // TODO -- this is only necessary for `new_target_fn_name`, remove after deprecating!
            if let (Expr::VarId { name, .. }, Expr::Primitive { val, .. }) = (&var_id, &expr) {
                let Some(Record::Var { value, def, .. }) = ctx.table.lookup_var_mut(name, true)
                else {
                    ctx.err
                        .unexpected_error(true, Some("unexpected type".to_string()), None);
                    return false;
                };

                *value = Some(val.clone());
                if def.is_comp_defined() {
                    return true;
                }
            }

            // memory offset goes BEFORE the value to store
            possibly_emit_memaddr_calc_offset(var_id, injector, ctx);

            if !emit_expr(expr, strategy, injector, ctx) {
                return false;
            }

            // Emit the instruction that sets the variable's value to the emitted expression
            emit_set(var_id, injector, ctx)
        }
        _ => {
            ctx.err.unexpected_error(
                false,
                Some(format!(
                    "{} \
                    Wrong statement type, should be `assign`",
                    ctx.err_msg
                )),
                None,
            );
            false
        }
    }
}

fn emit_set_map_stmt<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    stmt: &mut Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    ctx.in_map_op = true;
    if let Statement::SetMap {
        map: Expr::VarId { name, .. },
        key,
        val,
        ..
    } = stmt
    {
        let Some((map_addr, key_ty, val_ty)) = get_map_info(name, ctx) else {
            return false;
        };

        match map_addr {
            VarAddr::MapId { addr } => {
                injector.u32_const(addr);
            }
            VarAddr::Local { addr } => {
                injector.local_get(LocalID(addr));
            }
            VarAddr::MemLoc {
                mem_id,
                ty,
                var_offset,
            } => {
                assert!(matches!(ty, DataType::Map { .. }));
                // Get the map_id from memory!
                ctx.mem_allocator.get_from_mem(
                    mem_id,
                    &DataType::I32,
                    var_offset,
                    ctx.table,
                    injector,
                    ctx.err,
                );
            }
            other => panic!("Did not expect this address type: {:?}", other),
        };
        emit_expr(key, strategy, injector, ctx);
        emit_expr(val, strategy, injector, ctx);
        ctx.map_lib_adapter
            .map_insert(key_ty, val_ty, injector, ctx.mem_allocator, ctx.err);
    } else {
        ctx.err.unexpected_error(
            false,
            Some(format!(
                "{} \
            Wrong statement type, should be `set_map`",
                ctx.err_msg
            )),
            None,
        );
    }
    ctx.in_map_op = false;
    true
}

// transform a whamm type to default wasm type, used for creating new global
// TODO: Might be more generic to also include Local
pub fn whamm_type_to_wasm_global(
    app_wasm: &mut Module,
    ty: &DataType,
    loc: &Option<Location>,
    init_expr: Option<InitExpr>,
) -> (GlobalID, WirmType) {
    let wirm_ty = ty.to_wasm_type();

    if wirm_ty.len() == 1 {
        match wirm_ty.first().unwrap() {
            WirmType::I32 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr.unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::I32(0))])),
                    WirmType::I32,
                    true,
                    false,
                    get_tag_for(loc),
                );
                (global_id, WirmType::I32)
            }
            WirmType::I64 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr.unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::I64(0))])),
                    WirmType::I64,
                    true,
                    false,
                    get_tag_for(loc),
                );
                (global_id, WirmType::I64)
            }
            WirmType::F32 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::F32(0f32))])),
                    WirmType::F32,
                    true,
                    false,
                    get_tag_for(loc),
                );
                (global_id, WirmType::F32)
            }
            WirmType::F64 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::F64(0f64))])),
                    WirmType::F64,
                    true,
                    false,
                    get_tag_for(loc),
                );
                (global_id, WirmType::F64)
            }
            _ => unimplemented!(),
        }
    } else {
        unimplemented!()
    }
}

pub fn emit_global_getter(
    app_wasm: &mut Module,
    global_id: &u32,
    name: String,
    ty: WirmType,
    loc: &Option<Location>,
) -> FunctionID {
    // todo -- make this conditional on 'testing' mode
    let getter_params = vec![];
    let getter_res = vec![ty];

    let mut getter = FunctionBuilder::new(&getter_params, &getter_res);
    getter.global_get(GlobalID(*global_id));

    let getter_id = getter.finish_module_with_tag(app_wasm, get_tag_for(loc));
    let fn_name = format!("get_{name}");
    app_wasm.set_fn_name(getter_id, fn_name.clone());
    app_wasm.exports.add_export_func(fn_name, *getter_id, None);

    getter_id
}

pub fn block_type_to_wasm(block: &Block) -> BlockType {
    match &block.results {
        None => BlockType::Empty,
        Some(return_ty) => {
            let wasm_ty = return_ty.to_wasm_type();
            if wasm_ty.len() == 1 {
                BlockType::Type(*wasm_ty.first().unwrap())
            } else {
                todo!()
            }
        }
    }
}

fn possibly_emit_memaddr_calc_offset<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    var_id: &mut Expr,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    if let Expr::VarId { name, .. } = var_id {
        let Some(Record::Var { addr, .. }) = ctx.table.lookup_var_mut(name, true) else {
            ctx.err
                .unexpected_error(true, Some("unexpected type".to_string()), None);
            return false;
        };

        // this will be different based on if this is a global or local var
        if let Some(VarAddr::MemLoc { .. }) = addr {
            ctx.mem_allocator.emit_addr(ctx.table, injector, ctx.err);
        }
        true
    } else {
        ctx.err
            .unexpected_error(true, Some(format!("{} Expected VarId.", ctx.err_msg)), None);
        false
    }
}

fn emit_set<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    var_id: &mut Expr,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    if let Expr::VarId { name, .. } = var_id {
        let Some(Record::Var { addr, loc, .. }) = ctx.table.lookup_var_mut(name, true) else {
            ctx.err
                .unexpected_error(true, Some("unexpected type".to_string()), None);
            return false;
        };

        // this will be different based on if this is a global or local var
        match addr {
            Some(VarAddr::Global { addr }) => {
                injector.global_set(GlobalID(*addr));
            }
            Some(VarAddr::MemLoc {
                mem_id,
                ty,
                var_offset,
                ..
            }) => {
                ctx.mem_allocator
                    .set_in_mem(*var_offset, *mem_id, &ty.clone(), injector);
            }
            Some(VarAddr::Local { addr }) => {
                injector.local_set(LocalID(*addr));
            }
            Some(VarAddr::MapId { .. }) => {
                ctx.err.type_check_error(
                    false,
                    format!("Attempted to assign a var to Map: {}", name),
                    &line_col_from_loc(loc),
                );
                return false;
            }
            None => {
                ctx.err.type_check_error(
                    false,
                    format!("Variable assigned before declared: {}", name),
                    &line_col_from_loc(loc),
                );
                return false;
            }
        }
        true
    } else {
        ctx.err
            .unexpected_error(true, Some(format!("{} Expected VarId.", ctx.err_msg)), None);
        false
    }
}

fn emit_if_preamble<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;

    // emit the condition of the `if` expression
    is_success &= emit_expr(condition, strategy, injector, ctx);
    // emit the beginning of the if block
    injector.if_stmt(block_type_to_wasm(conseq));
    // emit the consequent body
    is_success &= emit_body(conseq, strategy, injector, ctx);

    // INTENTIONALLY DON'T END IF BLOCK
    is_success
}

fn emit_if_else_preamble<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;

    is_success &= emit_if_preamble(condition, conseq, strategy, injector, ctx);

    // emit the beginning of the else
    injector.else_stmt();

    // emit the alternate body
    is_success &= emit_body(alternate, strategy, injector, ctx);

    // INTENTIONALLY DON'T END IF/ELSE BLOCK

    is_success
}

fn emit_if<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;

    is_success &= emit_if_preamble(condition, conseq, strategy, injector, ctx);

    // emit the end of the if block
    injector.end();
    is_success
}

fn emit_if_else<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    condition: &mut Expr,
    conseq: &mut Block,
    alternate: &mut Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;

    is_success &= emit_if_else_preamble(condition, conseq, alternate, strategy, injector, ctx);

    // emit the end of the if block
    injector.end();
    is_success
}

// TODO: emit_expr has two mutable references to the name object, the injector has module data in it
pub(crate) fn emit_expr<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    expr: &mut Expr,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // fold it first!
    let mut folded_expr = ExprFolder::fold_expr(expr, ctx.table, ctx.err);
    match &mut folded_expr {
        Expr::UnOp {
            op, expr, done_on, ..
        } => {
            let mut is_success = emit_expr(&mut *expr, strategy, injector, ctx);
            is_success &= emit_unop(op, done_on, injector);
            is_success
        }
        Expr::BinOp {
            lhs,
            op,
            rhs,
            done_on,
            ..
        } => {
            let mut is_success = emit_expr(&mut *lhs, strategy, injector, ctx);
            is_success &= emit_expr(&mut *rhs, strategy, injector, ctx);
            is_success &= emit_binop(op, done_on, injector, ctx);
            is_success
        }
        Expr::Ternary {
            cond,
            conseq,
            alt,
            ty,
            ..
        } => {
            if matches!(ty, DataType::Null) {
                ctx.err.unexpected_error(
                    true,
                    Some(format!(
                        "{} \
                                The result type of the ternary should have been set in the type checker.", ctx.err_msg
                    )),
                    None,
                );
                return false;
            }

            emit_if_else(
                &mut *cond,
                &mut Block {
                    stmts: vec![Statement::Expr {
                        expr: *(*conseq).clone(),
                        loc: None,
                    }],
                    results: Some(ty.clone()),
                    loc: None,
                },
                &mut Block {
                    stmts: vec![Statement::Expr {
                        expr: *(*alt).clone(),
                        loc: None,
                    }],
                    results: Some(ty.clone()),
                    loc: None,
                },
                strategy,
                injector,
                ctx,
            )
        }
        Expr::LibCall { lib_name, call, .. } => {
            ctx.in_lib_call_to = Some(lib_name.clone());
            let is_success = emit_expr(&mut *call, strategy, injector, ctx);

            ctx.in_lib_call_to = None;
            is_success
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            let fn_name = match fn_target.as_ref() {
                Expr::VarId { name, .. } => name.clone(),
                _ => return false,
            };

            // emit the arguments
            let mut is_success = true;
            for arg in args.iter_mut() {
                is_success = emit_expr(arg, strategy, injector, ctx);
            }

            let addr = if let Some(lib_name) = &ctx.in_lib_call_to {
                let Some(Record::LibFn { addr, .. }) =
                    ctx.table.lookup_lib_fn(lib_name, &fn_name, ctx.err)
                else {
                    ctx.err
                        .unexpected_error(true, Some("unexpected type".to_string()), None);
                    return false;
                };
                *addr
            } else {
                let Some(Record::Fn { addr, .. }) = ctx.table.lookup_fn(&fn_name, true, ctx.err)
                else {
                    ctx.err
                        .unexpected_error(true, Some("unexpected type".to_string()), None);
                    return false;
                };
                *addr
            };

            if let Some(f_id) = addr {
                injector.call(FunctionID(f_id));
            } else {
                panic!(
                    "{} \
                                fn_target address not in symbol table for '{}', not emitted yet...",
                    ctx.err_msg, fn_name
                );
            }
            is_success
        }
        Expr::VarId { name, .. } => {
            // TODO -- support string vars (unimplemented)
            let Some(Record::Var { addr, def, .. }) = ctx.table.lookup_var_mut(name, true) else {
                ctx.err
                    .unexpected_error(true, Some("unexpected type".to_string()), None);
                return false;
            };
            if matches!(def, Definition::CompilerStatic) && addr.is_none() {
                panic!("{} \
                    Variable is bound statically by the compiler, it should've been folded by this point: {}", ctx.err_msg,
                        name);
            }
            // this will be different based on if this is a global or local var
            match addr {
                Some(VarAddr::Global { addr }) => {
                    injector.global_get(GlobalID(*addr));
                    true
                }
                Some(VarAddr::Local { addr }) => {
                    injector.local_get(LocalID(*addr));
                    true
                }
                Some(VarAddr::MemLoc {
                    mem_id,
                    ty,
                    var_offset,
                }) => {
                    ctx.mem_allocator.get_from_mem(
                        *mem_id,
                        &ty.clone(),
                        *var_offset,
                        ctx.table,
                        injector,
                        ctx.err,
                    );
                    true
                }
                Some(VarAddr::MapId { .. }) => {
                    ctx.err.unexpected_error(
                        true,
                        Some(format!(
                            "{} \
                                Variable you are trying to use in expr is a Map object {}",
                            ctx.err_msg, name
                        )),
                        None,
                    );
                    false
                }
                None => {
                    panic!(
                        "{} \
                Variable does not exist in scope: {}",
                        ctx.err_msg, name
                    );
                }
            }
        }
        Expr::Primitive { val, .. } => emit_value(val, strategy, injector, ctx),
        Expr::MapGet { .. } => emit_map_get(expr, strategy, injector, ctx),
    }
}

fn emit_binop<'a, T: Opcode<'a> + AddLocal>(
    op: &BinOp,
    done_on: &DataType,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match op {
        BinOp::And => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => injector.i32_and(),
                DataType::U64 | DataType::I64 => injector.i64_and(),
                DataType::F32 => {
                    injector.i32_reinterpret_f32();
                    injector.i32_and();
                    injector.f32_reinterpret_i32()
                }
                DataType::F64 => {
                    injector.i64_reinterpret_f64();
                    injector.i64_and();
                    injector.f64_reinterpret_i64()
                }
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support logical AND for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted logical AND for {done_on}")
                }
            };
        }
        BinOp::Or => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => injector.i32_or(),
                DataType::U64 | DataType::I64 => injector.i64_or(),
                DataType::F32 => {
                    injector.i32_reinterpret_f32();
                    injector.i32_or();
                    injector.f32_reinterpret_i32()
                }
                DataType::F64 => {
                    injector.i64_reinterpret_f64();
                    injector.i64_or();
                    injector.f64_reinterpret_i64()
                }
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support logical OR for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted logical OR for {done_on}")
                }
            };
        }
        BinOp::EQ => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => injector.i32_eq(),
                DataType::U64 | DataType::I64 => injector.i64_eq(),
                DataType::F32 => injector.f32_eq(),
                DataType::F64 => injector.f64_eq(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support equal for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted equal for {done_on}")
                }
            };
        }
        BinOp::NE => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => injector.i32_ne(),
                DataType::U64 | DataType::I64 => injector.i64_ne(),
                DataType::F32 => injector.f32_ne(),
                DataType::F64 => injector.f64_ne(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support not equal for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted not equal for {done_on}")
                }
            };
        }
        BinOp::GE => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_gte_signed(),
                DataType::U32 => injector.i32_gte_unsigned(),
                DataType::U64 => injector.i64_gte_unsigned(),
                DataType::I64 => injector.i64_gte_signed(),
                DataType::F32 => injector.f32_ge(),
                DataType::F64 => injector.f64_ge(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support greater than or equal to for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted greater than or equal to for {done_on}")
                }
            };
        }
        BinOp::GT => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_gt_signed(),
                DataType::U32 => injector.i32_gt_unsigned(),
                DataType::U64 => injector.i64_gt_unsigned(),
                DataType::I64 => injector.i64_gt_signed(),
                DataType::F32 => injector.f32_gt(),
                DataType::F64 => injector.f64_gt(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support greater than for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted greater than for {done_on}")
                }
            };
        }
        BinOp::LE => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_lte_signed(),
                DataType::U32 => injector.i32_lte_unsigned(),
                DataType::U64 => injector.i64_lte_unsigned(),
                DataType::I64 => injector.i64_lte_signed(),
                DataType::F32 => injector.f32_le(),
                DataType::F64 => injector.f64_le(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support less than or equal to for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted less then or equal to for {done_on}")
                }
            };
        }
        BinOp::LT => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_lt_signed(),
                DataType::U32 => injector.i32_lt_unsigned(),
                DataType::U64 => injector.i64_lt_unsigned(),
                DataType::I64 => injector.i64_lt_signed(),
                DataType::F32 => injector.f32_lt(),
                DataType::F64 => injector.f64_lt(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support less than for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted less than for {done_on}")
                }
            };
        }
        BinOp::Add => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_add();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U64 | DataType::I64 => injector.i64_add(),
                DataType::F32 => injector.f32_add(),
                DataType::F64 => injector.f64_add(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support addition for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted addition for {done_on}")
                }
            };
        }
        BinOp::Subtract => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_sub();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U64 | DataType::I64 => injector.i64_sub(),
                DataType::F32 => injector.f32_sub(),
                DataType::F64 => injector.f64_sub(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support subtract for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted subtract for {done_on}")
                }
            };
        }
        BinOp::Multiply => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_mul();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U64 | DataType::I64 => injector.i64_mul(),
                DataType::F32 => injector.f32_mul(),
                DataType::F64 => injector.f64_mul(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support multiply for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted multiply for {done_on}")
                }
            };
        }
        BinOp::Divide => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_div_signed();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U32 => injector.i32_div_unsigned(),
                DataType::U64 => injector.i64_div_unsigned(),
                DataType::I64 => injector.i64_div_signed(),
                DataType::F32 => injector.f32_div(),
                DataType::F64 => injector.f64_div(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support divide for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted divide for {done_on}")
                }
            };
        }
        BinOp::Modulo => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_rem_signed();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U32 => injector.i32_rem_unsigned(),
                DataType::U64 => injector.i64_rem_unsigned(),
                DataType::I64 => injector.i64_rem_signed(),
                #[rustfmt::skip]
                DataType::F32 => {
                    let a = LocalID(ctx.locals_tracker.use_local(WirmType::F32, injector));
                    let b = LocalID(ctx.locals_tracker.use_local(WirmType::F32, injector));

                    // Step 0: Do some stack juggling
                    injector.local_set(b)
                        .local_set(a)
                        .local_get(a)
                        .local_get(b)

                        // Step 1: Divide a by b
                        .f32_div()

                        // Step 2: Floor the result
                        .f32_floor()

                        // Step 3: Multiply the floor result by b
                        .local_get(b)
                        .f32_mul()
                        .local_set(b)

                        // Step 4: Subtract the result of the multiplication from a to get the remainder
                        .local_get(a)
                        .local_get(b)
                        .f32_sub()

                        // Step 5: Make sure the sign is the same as the first operand
                        .local_get(a)
                        .f32_copysign()
                }
                #[rustfmt::skip]
                DataType::F64 => {
                    let a = LocalID(ctx.locals_tracker.use_local(WirmType::F64, injector));
                    let b = LocalID(ctx.locals_tracker.use_local(WirmType::F64, injector));

                    // Step 0: Do some stack juggling
                    injector.local_set(b)
                        .local_set(a)
                        .local_get(a)
                        .local_get(b)

                        // Step 1: Divide a by b
                        .f64_div()

                        // Step 2: Floor the result
                        .f64_floor()

                        // Step 3: Multiply the floor result by b
                        .local_get(b)
                        .f64_mul()
                        .local_set(b)

                        // Step 4: Subtract the result of the multiplication from a to get the remainder
                        .local_get(a)
                        .local_get(b)
                        .f64_sub()

                        // Step 5: Make sure the sign is the same as the first operand
                        .local_get(a)
                        .f64_copysign()
                }
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support modulo for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted modulo for {done_on}")
                }
            };
        }
        BinOp::LShift => {
            match done_on {
                DataType::U8
                | DataType::I8
                | DataType::U16
                | DataType::I16
                | DataType::U32
                | DataType::I32
                | DataType::Boolean => {
                    injector.i32_shl();
                    // convert back if smaller than i32 and signed
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::I8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and();
                            injector.i32_extend_8s()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        DataType::I16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and();
                            injector.i32_extend_16s()
                        }
                        _ => injector,
                    }
                }
                DataType::U64 | DataType::I64 => injector.i64_shl(),
                DataType::F32 | DataType::F64 => unreachable!(),
                DataType::Null
                | DataType::Str
                | DataType::Tuple { .. }
                | DataType::Map { .. }
                | DataType::Unknown => {
                    unimplemented!()
                }
                DataType::AssumeGood => unreachable!(),
            };
        }
        BinOp::RShift => {
            match done_on {
                DataType::U8 | DataType::U16 | DataType::U32 | DataType::Boolean => {
                    injector.i32_shr_unsigned()
                }
                DataType::I8 | DataType::I16 | DataType::I32 => injector.i32_shr_signed(),
                DataType::U64 => injector.i64_shr_unsigned(),
                DataType::I64 => injector.i64_shr_signed(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support right shift for {done_on}")
                }
                DataType::F32 | DataType::F64 | DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted right shift for {done_on}")
                }
            };
        }
        BinOp::BitAnd => {
            match done_on {
                DataType::U8
                | DataType::U16
                | DataType::U32
                | DataType::I8
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_and(),
                DataType::U64 | DataType::I64 => injector.i64_and(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support bitwise AND for {done_on}")
                }
                DataType::F32 | DataType::F64 | DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted bitwise AND for {done_on}")
                }
            };
        }
        BinOp::BitOr => {
            match done_on {
                DataType::U8
                | DataType::U16
                | DataType::U32
                | DataType::I8
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_or(),
                DataType::U64 | DataType::I64 => injector.i64_or(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support bitwise OR for {done_on}")
                }
                DataType::F32 | DataType::F64 | DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted bitwise OR for {done_on}")
                }
            };
        }
        BinOp::BitXor => {
            match done_on {
                DataType::U8
                | DataType::U16
                | DataType::U32
                | DataType::I8
                | DataType::I16
                | DataType::I32
                | DataType::Boolean => injector.i32_xor(),
                DataType::U64 | DataType::I64 => injector.i64_xor(),
                DataType::F32 | DataType::F64 => unreachable!(),
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support bitwise XOR for {done_on}")
                }
                DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted bitwise XOR for {done_on}")
                }
            };
        }
    }
    true
}

fn emit_unop<'a, T: Opcode<'a>>(op: &UnOp, done_on: &DataType, injector: &mut T) -> bool {
    match op {
        UnOp::Cast { target } => {
            match (done_on, target) {
                // From U8
                (
                    DataType::U8,
                    DataType::U8
                    | DataType::I8
                    | DataType::U16
                    | DataType::I16
                    | DataType::U32
                    | DataType::I32,
                ) => {} // nothing to do
                (DataType::U8, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::U8, DataType::U64 | DataType::I64) => {
                    injector.i64_extend_i32u();
                }
                (DataType::U8, DataType::F32) => {
                    injector.f32_convert_i32u();
                }
                (DataType::U8, DataType::F64) => {
                    injector.f64_convert_i32u();
                }
                (DataType::U8, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From I8
                (DataType::I8, DataType::U8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::I8, DataType::I8) => {} // nothing to do
                (DataType::I8, DataType::U16) => {
                    // sign extend
                    injector.i32_extend_8s();
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::I8, DataType::I16) => {
                    // sign extend
                    injector.i32_extend_8s();
                }
                (DataType::I8, DataType::I32 | DataType::U32) => {
                    // sign extend
                    injector.i32_extend_8s();
                }
                (DataType::I8, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::I8, DataType::U64 | DataType::I64) => {
                    injector.i32_extend_8s();
                    injector.i64_extend_i32s();
                }
                (DataType::I8, DataType::F32) => {
                    injector.i32_extend_8s();
                    injector.f32_convert_i32s();
                }
                (DataType::I8, DataType::F64) => {
                    injector.i32_extend_8s();
                    injector.f64_convert_i32s();
                }
                (DataType::I8, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From U16
                (DataType::U16, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::U16, DataType::U16 | DataType::I16 | DataType::U32 | DataType::I32) => {} // nothing to do
                (DataType::U16, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::U16, DataType::U64 | DataType::I64) => {
                    injector.i64_extend_i32u();
                }
                (DataType::U16, DataType::F32) => {
                    injector.f32_convert_i32u();
                }
                (DataType::U16, DataType::F64) => {
                    injector.f64_convert_i32u();
                }
                (DataType::U16, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From I16
                (DataType::I16, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::I16, DataType::U16 | DataType::I16) => {} // nothing to do
                (DataType::I16, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::I16, DataType::I32 | DataType::U32) => {
                    injector.i32_extend_16s();
                }
                (DataType::I16, DataType::U64 | DataType::I64) => {
                    injector.i32_extend_16s();
                    injector.i64_extend_i32s();
                }
                (DataType::I16, DataType::F32) => {
                    injector.i32_extend_16s();
                    injector.f32_convert_i32s();
                }
                (DataType::I16, DataType::F64) => {
                    injector.i32_extend_16s();
                    injector.f64_convert_i32s();
                }
                (DataType::I16, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From U32
                (DataType::U32, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::U32, DataType::U16 | DataType::I16) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::U32, DataType::U32 | DataType::I32) => {} // nothing to do
                (DataType::U32, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::U32, DataType::U64 | DataType::I64) => {
                    injector.i64_extend_i32u();
                }
                (DataType::U32, DataType::F32) => {
                    injector.f32_convert_i32u();
                }
                (DataType::U32, DataType::F64) => {
                    injector.f64_convert_i32u();
                }
                (DataType::U32, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From I32
                (DataType::I32, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::I32, DataType::U16 | DataType::I16) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::I32, DataType::U32 | DataType::I32) => {} // nothing to do
                (DataType::I32, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i32_eqz();
                    injector.i32_eqz();
                }
                (DataType::I32, DataType::U64 | DataType::I64) => {
                    injector.i64_extend_i32s();
                }
                (DataType::I32, DataType::F32) => {
                    injector.f32_convert_i32s();
                }
                (DataType::I32, DataType::F64) => {
                    injector.f64_convert_i32s();
                }
                (DataType::I32, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From U64
                (DataType::U64, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i64_const(0xFF);
                    injector.i64_and();
                    injector.i32_wrap_i64();
                }
                (DataType::U64, DataType::U16 | DataType::I16) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i64_const(0xFFFF);
                    injector.i64_and();
                    injector.i32_wrap_i64();
                }
                (DataType::U64, DataType::U32 | DataType::I32) => {
                    // truncating cast for ints (zero out higher bits)
                    injector.i32_wrap_i64();
                }
                (DataType::U64, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i64_eqz();
                    injector.i32_eqz();
                }
                (DataType::U64, DataType::U64 | DataType::I64) => {} // nothing to do
                (DataType::U64, DataType::F32) => {
                    injector.f32_convert_i64u();
                }
                (DataType::U64, DataType::F64) => {
                    injector.f64_convert_i64u();
                }
                (DataType::U64, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From I64
                (DataType::I64, DataType::U8 | DataType::I8) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i64_const(0xFF);
                    injector.i64_and();
                    injector.i32_wrap_i64();
                }
                (DataType::I64, DataType::U16 | DataType::I16) => {
                    //  truncating cast for ints (zero out higher bits)
                    injector.i64_const(0xFFFF);
                    injector.i64_and();
                    injector.i32_wrap_i64();
                }
                (DataType::I64, DataType::U32 | DataType::I32) => {
                    // truncating cast for ints (zero out higher bits)
                    injector.i32_wrap_i64();
                }
                (DataType::I64, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.i64_eqz();
                    injector.i32_eqz();
                }
                (DataType::I64, DataType::U64 | DataType::I64) => {} // nothing to do
                (DataType::I64, DataType::F32) => {
                    injector.f32_convert_i64s();
                }
                (DataType::I64, DataType::F64) => {
                    injector.f64_convert_i64s();
                }
                (DataType::I64, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From F32
                (DataType::F32, DataType::U8) => {
                    // truncating cast for floats
                    injector.i32_trunc_f32u();
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::F32, DataType::I8) => {
                    // truncating cast for floats
                    injector.i32_trunc_f32s();
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::F32, DataType::U16) => {
                    // truncating cast for floats
                    injector.i32_trunc_f32u();
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::F32, DataType::I16) => {
                    // truncating cast for floats
                    injector.i32_trunc_f32s();
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::F32, DataType::U32) => {
                    injector.i32_trunc_f32u();
                }
                (DataType::F32, DataType::I32) => {
                    injector.i32_trunc_f32s();
                }
                (DataType::F32, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.f32_const(0f32);
                    injector.f32_eq();
                    injector.i32_eqz();
                }
                (DataType::F32, DataType::U64) => {
                    injector.i64_trunc_f32u();
                }
                (DataType::F32, DataType::I64) => {
                    injector.i64_trunc_f32s();
                }
                (DataType::F32, DataType::F32) => {} // nothing to do
                (DataType::F32, DataType::F64) => {
                    injector.f64_promote_f32();
                }
                (DataType::F32, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }

                // From F64
                (DataType::F64, DataType::U8) => {
                    // truncating cast for floats
                    injector.i32_trunc_f64u();
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::F64, DataType::I8) => {
                    // truncating cast for floats
                    injector.i32_trunc_f64s();
                    injector.i32_const(0xFF);
                    injector.i32_and();
                }
                (DataType::F64, DataType::U16) => {
                    // truncating cast for floats
                    injector.i32_trunc_f64u();
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::F64, DataType::I16) => {
                    // truncating cast for floats
                    injector.i32_trunc_f64s();
                    injector.i32_const(0xFFFF);
                    injector.i32_and();
                }
                (DataType::F64, DataType::U32) => {
                    injector.i32_trunc_f64u();
                }
                (DataType::F64, DataType::I32) => {
                    injector.i32_trunc_f64s();
                }
                (DataType::F64, DataType::Boolean) => {
                    // "truthy" (if it DOES NOT equal 0)
                    injector.f64_const(0f64);
                    injector.f64_eq();
                    injector.i32_eqz();
                }
                (DataType::F64, DataType::U64) => {
                    injector.i64_trunc_f64u();
                }
                (DataType::F64, DataType::I64) => {
                    injector.i64_trunc_f64s();
                }
                (DataType::F64, DataType::F32) => {
                    injector.f32_demote_f64();
                }
                (DataType::F64, DataType::F64) => {} // nothing to do
                (
                    DataType::Boolean,
                    DataType::U8
                    | DataType::I8
                    | DataType::U16
                    | DataType::I16
                    | DataType::I32
                    | DataType::U32,
                ) => {} // nothing to do
                (DataType::F64, _) => {
                    // should've been handled by type checker
                    unreachable!();
                }
                (from, to) => {
                    // should've been handled by type checker
                    unreachable!("{} to {}", from, to);
                }
            };
        }
        UnOp::Not => match done_on {
            DataType::U8
            | DataType::I8
            | DataType::U16
            | DataType::I16
            | DataType::U32
            | DataType::I32
            | DataType::Boolean => {
                injector.i32_eqz();
            }
            DataType::U64 | DataType::I64 => {
                injector.i64_eqz();
            }
            DataType::F32 => {
                injector.f32_const(0f32);
                injector.f32_eq();
            }
            DataType::F64 => {
                injector.f64_const(0f64);
                injector.f64_eq();
            }
            DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                unimplemented!("We do not support NOT for {done_on}")
            }
            DataType::AssumeGood | DataType::Unknown => unreachable!("Attempted NOT for {done_on}"),
        },
        UnOp::BitwiseNot => {
            match done_on {
                DataType::U8 | DataType::U16 => {
                    // i32.xor(x, -1)
                    injector.i32_const(-1);
                    injector.i32_xor();

                    // should clear out upper bits afterward (since unsigned)!
                    match done_on {
                        DataType::U8 => {
                            injector.i32_const(0xFF);
                            injector.i32_and()
                        }
                        DataType::U16 => {
                            injector.i32_const(0xFFFF);
                            injector.i32_and()
                        }
                        _ => injector,
                    }
                }
                DataType::Boolean
                | DataType::U32
                | DataType::I8
                | DataType::I16
                | DataType::I32 => {
                    injector.i32_const(-1);
                    injector.i32_xor()
                }
                DataType::U64 | DataType::I64 => {
                    injector.i64_const(-1);
                    injector.i64_xor()
                }
                DataType::Null | DataType::Str | DataType::Tuple { .. } | DataType::Map { .. } => {
                    unimplemented!("We do not support bitwise NOT for {done_on}")
                }
                DataType::F32 | DataType::F64 | DataType::AssumeGood | DataType::Unknown => {
                    unreachable!("Attempted bitwise NOT for {done_on}")
                }
            };
        }
    }
    true
}

fn emit_value<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    val: &mut Value,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;
    match val {
        Value::Number { val, .. } => match val {
            NumLit::I8 { val } => {
                injector.u32_const(*val as u32);
                injector.i32_extend_8s();
                is_success &= true;
            }
            NumLit::U8 { val } => {
                injector.u32_const(*val as u32);
                is_success &= true;
            }
            NumLit::I16 { val } => {
                injector.u32_const(*val as u32);
                injector.i32_extend_16s();
                is_success &= true;
            }
            NumLit::U16 { val } => {
                injector.u32_const(*val as u32);
                is_success &= true;
            }
            NumLit::U32 { val } => {
                injector.u32_const(*val);
                is_success &= true;
            }
            NumLit::I32 { val } => {
                injector.i32_const(*val);
                is_success &= true;
            }
            NumLit::U64 { val } => {
                injector.u64_const(*val);
                is_success &= true;
            }
            NumLit::I64 { val } => {
                injector.i64_const(*val);
                is_success &= true;
            }
            NumLit::F32 { val } => {
                injector.f32_const(*val);
                is_success &= true;
            }
            NumLit::F64 { val } => {
                injector.f64_const(*val);
                is_success &= true;
            }
        },
        Value::Str { val, .. } => {
            // At this point the String has been emitted into the Wasm module!
            // See: InitGenerator::visit_value()
            // This is to avoid having to have access to the app_wasm.data here.
            // If this were required, we would have 2 mutable references to app_iter
            // when emitting for VisitingEmitter (won't work for Rust):
            // 1. app_iter.app_wasm.data
            // 2. app_iter

            if let Some(str_addr) = ctx.mem_allocator.emitted_strings.get(val) {
                if ctx.in_map_op {
                    // If in the context of a map operation, we will likely have to send
                    // this emitted string over to the MapLibrary through interfacing
                    // with its memory. Let's save this string's address in the MapLibAdapter
                    // to enable this logic.
                    ctx.map_lib_adapter.curr_str_offset = Some(str_addr.mem_offset as u32);
                    ctx.map_lib_adapter.curr_str_len = Some(str_addr.len as u32);

                    injector.u32_const(MAP_LIB_MEM_OFFSET);
                    injector.u32_const(str_addr.len as u32);
                } else {
                    // emit Wasm instructions for the memory address and string length
                    injector.u32_const(str_addr.mem_offset as u32);
                    injector.u32_const(str_addr.len as u32);
                }
                is_success &= true;
            } else {
                ctx.err.unexpected_error(
                    true,
                    Some(format!(
                        "{} String has not been emitted yet for value: '{val}'!",
                        ctx.err_msg
                    )),
                    None,
                );
                return false;
            }
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter_mut() {
                is_success &= emit_expr(val, strategy, injector, ctx);
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
        Value::U32U32Map { .. } => ctx.err.unexpected_error(
            false,
            Some(format!(
                "{} \
            `emit_value` shouldn't be called with a U32U32Map type...should already be handled!",
                ctx.err_msg
            )),
            None,
        ),
    }
    is_success
}

fn emit_map_get<'a, T: Opcode<'a> + MacroOpcode<'a> + AddLocal>(
    expr: &mut Expr,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    ctx.in_map_op = true;
    if let Expr::MapGet { map, key, .. } = expr {
        let map = &mut (**map);
        if let Expr::VarId { name, .. } = map {
            return match get_map_info(name, ctx) {
                Some((map_addr, key_ty, val_ty)) => {
                    match map_addr {
                        VarAddr::MapId { addr } => {
                            injector.u32_const(addr);
                        }
                        VarAddr::Local { addr } => {
                            injector.local_get(LocalID(addr));
                        }
                        VarAddr::MemLoc {
                            mem_id,
                            ty,
                            var_offset,
                        } => {
                            assert!(matches!(ty, DataType::Map { .. }));
                            // Get the map_id from memory!
                            ctx.mem_allocator.get_from_mem(
                                mem_id,
                                &DataType::I32,
                                var_offset,
                                ctx.table,
                                injector,
                                ctx.err,
                            );
                        }
                        other => panic!("Did not expect this address type: {:?}", other),
                    };
                    emit_expr(key, strategy, injector, ctx);
                    ctx.map_lib_adapter.map_get(
                        key_ty,
                        val_ty,
                        injector,
                        ctx.mem_allocator,
                        ctx.err,
                    );
                    true
                }
                None => false,
            };
        }
    }
    ctx.err.unexpected_error(
        false,
        Some(format!(
            "{} \
            Wrong statement type, should be `map_get`",
            ctx.err_msg
        )),
        None,
    );
    ctx.in_map_op = false;
    false
}
fn get_map_info(name: &mut str, ctx: &mut EmitCtx) -> Option<(VarAddr, DataType, DataType)> {
    let Some(Record::Var { ty, addr, loc, .. }) = ctx.table.lookup_var(name, &None, ctx.err, true)
    else {
        ctx.err
            .unexpected_error(true, Some("unexpected type".to_string()), None);
        return None;
    };

    if !matches!(
        addr,
        Some(VarAddr::MapId { .. }) | Some(VarAddr::Local { .. }) | Some(VarAddr::MemLoc { .. })
    ) {
        panic!("We don't support map locations being stored in addresses other than Local or constant MapId --> {}:{:?}", name, addr)
    }
    if let DataType::Map {
        key_ty: k,
        val_ty: v,
    } = ty
    {
        let key_ty = *k.clone();
        let val_ty = *v.clone();
        Some((addr.clone().unwrap(), key_ty, val_ty))
    } else {
        ctx.err.unexpected_error(
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
