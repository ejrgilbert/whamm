#![allow(clippy::too_many_arguments)]
use crate::common::error::ErrorGen;
use crate::emitter::locals_tracker::LocalsTracker;
use crate::emitter::memory_allocator::{EmitMode, MemoryAllocator, PtrSource};
use crate::emitter::tag_handler::get_tag_for;
use crate::emitter::InjectStrategy;
use crate::generator::ast::Probe;
use crate::generator::folding::expr::ExprFolder;
use crate::lang_features::libraries::core::maps::map_adapter::{MapLibAdapter, MAP_LIB_MEM_OFFSET};
use crate::lang_features::libraries::core::utils::utils_adapter::UtilsAdapter;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::type_utils::strings::StringUtils;
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

pub struct EmitCtx<'a> {
    registry: &'a mut WasmRegistry,
    table: &'a mut SymbolTable,
    pub(crate) mem_allocator: &'a MemoryAllocator,
    pub(crate) locals_tracker: &'a mut LocalsTracker,
    in_map_op: bool,
    in_obj_call_on: Option<String>,
    utils_adapter: &'a mut UtilsAdapter,
    map_lib_adapter: &'a mut MapLibAdapter,
    err_msg: String,
    err: &'a mut ErrorGen,
}
impl<'a> EmitCtx<'a> {
    pub fn new(
        registry: &'a mut WasmRegistry,
        table: &'a mut SymbolTable,
        mem_allocator: &'a MemoryAllocator,
        locals_tracker: &'a mut LocalsTracker,
        utils_adapter: &'a mut UtilsAdapter,
        map_lib_adapter: &'a mut MapLibAdapter,
        err_msg: &str,
        err: &'a mut ErrorGen,
    ) -> Self {
        Self {
            registry,
            table,
            mem_allocator,
            locals_tracker,
            in_map_op: false,
            in_obj_call_on: None,
            utils_adapter,
            map_lib_adapter,
            err_msg: err_msg.to_string(),
            err,
        }
    }
}

pub fn emit_probes<'h, T: Opcode<'h> + MacroOpcode<'h> + AddLocal>(
    probes: &mut [Probe],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) {
    for probe in probes.iter_mut() {
        if let Some(body) = &mut probe.body {
            emit_body(body, strategy, injector, ctx);
        }
    }
}

pub fn emit_body<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    body: &Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;
    for stmt in body.stmts.iter() {
        is_success &= emit_stmt(stmt, strategy, injector, ctx);
    }
    is_success
}

pub fn emit_stmt<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    stmt: &Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // Probe bodies are pre-folded by FoldPass (dead-branch elimination).
    // Check whether this is a call to a bound, static function.
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
        let Some(Record::Fn { def, .. }) = ctx.table.lookup_fn(fn_name.as_str(), true) else {
            unreachable!("unexpected type");
        };
        if matches!(def, Definition::CompilerStatic) {
            return handle_special_fn_call(fn_name, args, strategy, injector, ctx);
        }
    }
    emit_stmt_inner(stmt, strategy, injector, ctx)
}

fn handle_special_fn_call<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    target_fn_name: String,
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut folded_args = vec![];
    for arg in args.iter() {
        folded_args.push(ExprFolder::fold_expr(
            arg,
            ctx.registry,
            strategy.as_monitor_module(),
            ctx.table,
            &ctx.mem_allocator.emitted_strings,
            ctx.err,
        ));
    }

    match target_fn_name.as_str() {
        "dup_at" | "alt_call_by_name" | "alt_call_by_id" | "drop_args" => {
            unreachable!("static function call should already be handled: {target_fn_name}")
        }
        "memcpy" => handle_memcpy(&folded_args, strategy, injector, ctx),
        "write_str" => handle_write_str(&folded_args, strategy, injector, ctx),
        "read_str" => handle_read_str(&folded_args, strategy, injector, ctx),
        _ => {
            unreachable!(
                "{} Could not find handler for static function with name: {}",
                ctx.err_msg, target_fn_name
            );
        }
    }
}

fn handle_memcpy<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // usage: `memcpy(dst_mem: u32, src_mem: u32, start_addr: u32, len: u32) -> ()`
    let src_mem = args[0]
        .get_primitive_u32()
        .unwrap_or_else(|| unreachable!());
    let dst_mem = args[2]
        .get_primitive_u32()
        .unwrap_or_else(|| unreachable!());

    // TODO: Fix utils.rs to not need `&mut Expr` at all (make immutable)
    let src_ptr = args[1].clone();
    let dst_ptr = args[3].clone();
    let src_len = args[4].clone();

    ctx.mem_allocator.mem_cpy(
        src_mem,
        &mut PtrSource::Expr(src_ptr),
        &mut PtrSource::Expr(src_len),
        dst_mem,
        &mut PtrSource::Expr(dst_ptr),
        EmitMode::WithCtx { ctx, strategy },
        injector,
    );
    true
}

fn handle_write_str<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // usage: `write_str(target_mem: u32, ptr: i32, s: str) -> ()`
    let target_mem = args[0]
        .get_primitive_u32()
        .unwrap_or_else(|| unreachable!());

    // handle the string parameter
    emit_expr(&args[2], None, strategy, injector, ctx);
    let str_len = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
    injector.local_set(str_len);
    let str_ptr = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
    injector.local_set(str_ptr);
    let dst_ptr = args[1].clone();

    ctx.mem_allocator.mem_cpy(
        ctx.mem_allocator.mem_id,
        &mut PtrSource::Local(str_ptr),
        &mut PtrSource::Local(str_len),
        target_mem,
        &mut PtrSource::Expr(dst_ptr),
        EmitMode::WithCtx { ctx, strategy },
        injector,
    );
    true
}

fn handle_read_str<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // usage: `read_str(src_mem: u32, ptr: i32, l: u32) -> ()`
    let src_mem = args[0]
        .get_primitive_u32()
        .unwrap_or_else(|| unreachable!());

    let str_ptr = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
    emit_expr(&args[1], None, strategy, injector, ctx);
    injector.local_set(str_ptr);

    let str_len = LocalID(ctx.locals_tracker.use_local(WirmType::I32, injector));
    emit_expr(&args[2], None, strategy, injector, ctx);
    injector.local_set(str_len);

    let dst_ptr = ctx.mem_allocator.mem_tracker_global;
    ctx.mem_allocator.copy_to_mem_global_ptr(
        src_mem,
        str_ptr,
        str_len,
        ctx.mem_allocator.mem_id,
        dst_ptr,
        EmitMode::WithCtx { ctx, strategy },
        injector,
    );

    // return the string's (ptr, len)
    injector
        .global_get(dst_ptr)
        .local_get(str_len)
        .i32_sub()
        .local_get(str_len);

    true
}

fn emit_stmt_inner<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    stmt: &Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::LibImport { .. } => true, // already handled!
        Statement::VarDecl { modifiers, .. } if !modifiers.is_unshared => {
            emit_decl_stmt(stmt, injector, ctx)
        }
        Statement::VarDecl { .. } => emit_unshared_decl_stmt(stmt, ctx),
        Statement::Assign { .. } => emit_assign_stmt(stmt, strategy, injector, ctx),
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => {
            emit_expr(expr, None, strategy, injector, ctx)
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
        Statement::SetMap { .. } => {
            ctx.in_map_op = true;
            let res = emit_set_map_stmt(stmt, strategy, injector, ctx);
            ctx.in_map_op = false;
            res
        }
    }
}

fn emit_decl_stmt<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    stmt: &Statement,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::VarDecl {
            ty,
            name,
            definition,
            ..
        } => {
            // look up in symbol table
            let var_rec_id = match ctx.table.lookup(name) {
                Some(rec_id) => rec_id,
                None => {
                    // add variables from body into symbol table (at this point, the verifier should have run to catch variable initialization without declaration)
                    ctx.table.put(
                        name.clone(),
                        Record::Var {
                            ty: ty.clone(),
                            value: None,
                            def: *definition,
                            addr: None,
                            times_set: 0,
                            loc: None,
                        },
                    )
                }
            };
            let mut addr = match ctx.table.get_record_mut(var_rec_id) {
                Some(Record::Var { addr, .. }) => addr,
                Some(ty) => {
                    unreachable!(
                        "{} Incorrect variable record, expected Record::Var, found: {:?}",
                        ctx.err_msg, ty
                    );
                }
                None => {
                    unreachable!("{} Variable symbol does not exist!", ctx.err_msg);
                }
            };

            if let DataType::Map { .. } = ty {
                ctx.in_map_op = true;
                // TODO -- this behavior doesn't seem right for wei
                //    The map_id would need to be dynamic...not statically known!
                let map_id = ctx
                    .map_lib_adapter
                    .map_create(ty.clone(), injector, ctx.err);
                *addr = Some(vec![VarAddr::MapId { addr: map_id }]);
                ctx.in_map_op = false;

                return true;
            }
            match &mut addr {
                Some(addrs) => {
                    match addrs.first().unwrap() {
                        VarAddr::Global { addr: _addr } | VarAddr::MapId { addr: _addr } => {
                            //ignore, initial setup is done in init_gen
                            true
                        }
                        VarAddr::MemLoc { .. } => {
                            //ignore, initial setup is done in $alloc
                            true
                        }
                        VarAddr::Local { .. } => {
                            handle_decl(addr, ty, ctx.locals_tracker, injector)
                        }
                    }
                }
                None => handle_decl(addr, ty, ctx.locals_tracker, injector),
            }
        }
        _ => {
            unreachable!("{} Wrong statement type, should be `assign`", ctx.err_msg);
        }
    }
}
fn handle_decl<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    addr: &mut Option<Vec<VarAddr>>,
    ty: &DataType,
    locals_tracker: &mut LocalsTracker,
    injector: &mut T,
) -> bool {
    // If the local already exists, it would be because the probe has been
    // emitted at another opcode location. Simply overwrite the previously saved
    // address.
    let wasm_ty = ty.to_wasm_type();
    let mut addrs = vec![];
    for ty in wasm_ty.iter() {
        let id = locals_tracker.use_local(*ty, injector);
        addrs.push(VarAddr::Local { addr: id });
    }
    *addr = Some(addrs);
    true
}

fn emit_unshared_decl_stmt(stmt: &Statement, ctx: &mut EmitCtx) -> bool {
    if let Statement::VarDecl { modifiers, .. } = stmt {
        if modifiers.is_unshared {
            // ignore, this statement has already been processed!
            return true;
        }
    }
    unreachable!(
        "{} Wrong statement type, should be `unshared` decl, got: {:?}",
        ctx.err_msg, stmt
    );
}

fn emit_assign_stmt<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    stmt: &Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match stmt {
        Statement::Assign { var_id, expr, .. } => {
            // Save off primitives to symbol table
            let part_tys = if let Expr::VarId { name, .. } = &var_id {
                let rec = ctx.table.lookup_var_mut(name, true).unwrap();
                let is_stable = rec.val_is_stable();
                let Record::Var {
                    def,
                    value,
                    ty,
                    addr,
                    ..
                } = rec
                else {
                    unreachable!("unexpected type");
                };

                let mut locally_bound = false;
                if let Some(a) = addr {
                    locally_bound = true;
                    for part in a.iter() {
                        locally_bound &= matches!(part, VarAddr::Local { .. });
                    }
                }

                if let Expr::Primitive { val, .. } = expr {
                    if is_stable {
                        *value = Some(val.clone()); // assign the value so we can do folding

                        // We can only do this shortcut if the assignment is:
                        // - stable: only happens once
                        // - bound to local function state: does not have side effects on program state (memory, globals, etc.)
                        // - is self-contained in the probe body: cannot interact with the application (e.g. through bound variables)
                        // TODO: Dynamic string building will break this
                        if locally_bound && !def.is_comp_defined() {
                            return true;
                        }
                    }
                }

                ty.to_wasm_type()
            } else {
                unreachable!("{} Expected VarId.", ctx.err_msg);
            };

            let mut is_success = true;
            if !emit_expr(expr, None, strategy, injector, ctx) {
                return false;
            }
            // now the full expression is on the stack
            // save the parts to local values so we can store 1-by-1
            let mut part_locals = vec![];
            for ty in part_tys.iter() {
                let local = LocalID(ctx.locals_tracker.use_local(*ty, injector));
                part_locals.push(local);

                injector.local_set(local);
            }

            // part_locals is in REVERSE from the assumed order.
            // e.g. stack is: [ptr, len] -> part_locals: [len, ptr]
            for (idx, local) in part_locals.iter().rev().enumerate() {
                // memory offset goes BEFORE the value to store
                possibly_emit_memaddr_calc_offset(var_id, injector, ctx);

                // the VALUE goes next
                injector.local_get(*local);

                // Emit the instruction that sets the variable's value to the emitted expression
                is_success &= emit_set(var_id, Some(idx), injector, ctx);
            }
            is_success
        }
        _ => {
            unreachable!(
                "{} \
                    Wrong statement type, should be `assign`",
                ctx.err_msg
            );
        }
    }
}

fn emit_set_map_stmt<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    stmt: &Statement,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    ctx.in_map_op = true;
    if let Statement::SetMap { map, key, val, .. } = stmt {
        let Some((map_addr, key_ty, val_ty)) = get_map_info(map, ctx) else {
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
                );
            }
            other => unreachable!("Did not expect this address type: {:?}", other),
        };
        emit_expr(key, None, strategy, injector, ctx);
        emit_expr(val, None, strategy, injector, ctx);
        ctx.map_lib_adapter.map_insert(
            key_ty,
            val_ty,
            injector,
            ctx.utils_adapter,
            ctx.mem_allocator,
            ctx.err,
        );
    } else {
        unreachable!(
            "{} \
            Wrong statement type, should be `set_map`",
            ctx.err_msg
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
) -> Vec<(GlobalID, WirmType)> {
    let wirm_ty = ty.to_wasm_type();

    let mut globals = vec![];
    for ty in wirm_ty.iter() {
        match ty {
            WirmType::I32 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .clone()
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::I32(0))])),
                    WirmType::I32,
                    true,
                    false,
                    get_tag_for(loc),
                );
                globals.push((global_id, WirmType::I32));
            }
            WirmType::I64 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .clone()
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::I64(0))])),
                    WirmType::I64,
                    true,
                    false,
                    get_tag_for(loc),
                );
                globals.push((global_id, WirmType::I64));
            }
            WirmType::F32 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .clone()
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::F32(0f32))])),
                    WirmType::F32,
                    true,
                    false,
                    get_tag_for(loc),
                );
                globals.push((global_id, WirmType::F32));
            }
            WirmType::F64 => {
                let global_id = app_wasm.add_global_with_tag(
                    init_expr
                        .clone()
                        .unwrap_or(InitExpr::new(vec![InitInstr::Value(WirmValue::F64(0f64))])),
                    WirmType::F64,
                    true,
                    false,
                    get_tag_for(loc),
                );
                globals.push((global_id, WirmType::F64));
            }
            _ => unimplemented!(),
        }
    }

    globals
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
    app_wasm
        .exports
        .add_export_func_with_tag(fn_name, *getter_id, get_tag_for(&None));

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

fn possibly_emit_memaddr_calc_offset<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    var_id: &Expr,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    if let Expr::VarId { name, .. } = var_id {
        let Some(Record::Var { addr, .. }) = ctx.table.lookup_var_mut(name, true) else {
            unreachable!("unexpected type");
        };

        // this will be different based on if this is a global or local var
        let mut is_mem_loc = false;
        if let Some(addrs) = addr {
            for addr in addrs.iter() {
                if let VarAddr::MemLoc { .. } = addr {
                    is_mem_loc = true;
                };
            }
        }
        if is_mem_loc {
            ctx.mem_allocator.emit_addr(ctx.table, injector);
        }
        true
    } else {
        unreachable!("{} Expected VarId, got: {var_id:?}", ctx.err_msg);
    }
}

fn emit_set<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    var_id: &Expr,
    idx: Option<usize>, // optionally specify a specific part of the expression to set
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    if let Expr::VarId { name, .. } = var_id {
        let Some(Record::Var { addr, loc, .. }) = ctx.table.lookup_var(name, true) else {
            unreachable!("unexpected type");
        };

        // this will be different based on if this is a global or local var
        if let Some(addrs) = addr {
            if let Some(idx) = idx {
                if let Some(addr) = addrs.get(idx) {
                    addr_set(addr, None, name, injector, ctx);
                    return true;
                }
            }

            for addr in addrs.iter().rev() {
                addr_set(addr, idx, name, injector, ctx);
            }

            true
        } else {
            ctx.err.type_check_error(
                format!("Variable assigned before declared: {}", name),
                &line_col_from_loc(loc),
            );
            false
        }
    } else {
        unreachable!("{} Expected VarId.", ctx.err_msg);
    }
}

fn addr_set<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    addr: &VarAddr,
    idx: Option<usize>, // optionally specify a specific part of the expression to set
    name: &str,
    injector: &mut T,
    ctx: &EmitCtx,
) {
    match addr {
        VarAddr::Global { addr } => {
            injector.global_set(GlobalID(*addr));
        }
        VarAddr::MemLoc {
            mem_id,
            ty,
            var_offset,
            ..
        } => {
            ctx.mem_allocator
                .set_in_mem(*var_offset, *mem_id, &ty.clone(), idx, injector);
        }
        VarAddr::Local { addr } => {
            injector.local_set(LocalID(*addr));
        }
        VarAddr::MapId { .. } => {
            unreachable!("This should have been caught during type checking. Attempted to assign a var to Map: {}", name)
        }
    }
}

fn emit_if_preamble<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    condition: &Expr,
    conseq: &Block,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;

    // emit the condition of the `if` expression
    is_success &= emit_expr(condition, None, strategy, injector, ctx);
    // emit the beginning of the if block
    injector.if_stmt(block_type_to_wasm(conseq));
    // emit the consequent body
    is_success &= emit_body(conseq, strategy, injector, ctx);

    // INTENTIONALLY DON'T END IF BLOCK
    is_success
}

fn emit_if_else_preamble<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    condition: &Expr,
    conseq: &Block,
    alternate: &Block,
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

fn emit_if<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    condition: &Expr,
    conseq: &Block,
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

fn emit_if_else<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    condition: &Expr,
    conseq: &Block,
    alternate: &Block,
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

pub(crate) fn emit_expr<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    expr: &Expr,
    idx: Option<usize>, // optionally specify a specific part of the expression to emit
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    // fold it first!
    let folded_expr = ExprFolder::fold_expr(
        expr,
        ctx.registry,
        strategy.as_monitor_module(),
        ctx.table,
        &ctx.mem_allocator.emitted_strings,
        ctx.err,
    );
    match &folded_expr {
        Expr::UnOp {
            op, expr, done_on, ..
        } => {
            let mut is_success = emit_expr(expr, None, strategy, injector, ctx);
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
            let mut is_success = emit_expr(lhs, None, strategy, injector, ctx);
            is_success &= emit_expr(rhs, None, strategy, injector, ctx);
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
                unreachable!(
                        "{} \
                                The result type of the ternary should have been set in the type checker.", ctx.err_msg);
            }

            let conseq_block = Block {
                stmts: vec![Statement::Expr {
                    expr: *(*conseq).clone(),
                    loc: None,
                }],
                results: Some(ty.clone()),
                loc: None,
            };
            let alt_block = Block {
                stmts: vec![Statement::Expr {
                    expr: *(*alt).clone(),
                    loc: None,
                }],
                results: Some(ty.clone()),
                loc: None,
            };
            emit_if_else(cond, &conseq_block, &alt_block, strategy, injector, ctx)
        }
        Expr::ObjCall {
            obj_name: lib_name,
            call,
            ..
        } => {
            ctx.in_obj_call_on = Some(lib_name.clone());
            let is_success = emit_expr(call, None, strategy, injector, ctx);

            ctx.in_obj_call_on = None;
            is_success
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            let fn_name = match fn_target.as_ref() {
                Expr::VarId { name, .. } => name.clone(),
                _ => return false,
            };

            // first save off current context's state on whether we're in a lib call
            let in_obj_call_on = ctx.in_obj_call_on.clone();

            let addr = if let Some(obj_name) = &in_obj_call_on {
                let rec = ctx
                    .table
                    .lookup(obj_name)
                    .and_then(|id| ctx.table.get_record(id))
                    .cloned();
                match rec {
                    Some(Record::Library { fns, .. }) => {
                        let Some(rec) = fns.get(&fn_name) else {
                            unreachable!("should have gotten a rec")
                        };
                        let Record::LibFn { addr, .. } = ctx.table.get_record(*rec).unwrap() else {
                            unreachable!("should have gotten lib func rec")
                        };
                        *addr
                    }
                    Some(Record::Var { def, ty, .. }) => {
                        ctx.in_obj_call_on = None;
                        return handle_type_utils_call(
                            obj_name, &ty, def, fn_name, args, strategy, injector, ctx,
                        );
                    }
                    _ => unreachable!("unexpected type for {obj_name}: {rec:?}"),
                }
            } else {
                let Some(Record::Fn { def, addr, .. }) = ctx.table.lookup_fn(&fn_name, true) else {
                    unreachable!("unexpected type");
                };
                // Check if this is calling a bound, static function!
                if matches!(def, Definition::CompilerStatic) {
                    // We want to handle this as unique logic rather than a simple function call to be emitted
                    return handle_special_fn_call(fn_name, args, strategy, injector, ctx);
                }
                *addr
            };

            // emit the arguments
            let mut is_success = true;
            for arg in args.iter() {
                is_success = emit_expr(arg, None, strategy, injector, ctx);
            }

            // now that we've emitted the arguments, restore the original lib call tracking
            ctx.in_obj_call_on = in_obj_call_on;

            if let Some(f_id) = addr {
                injector.call(FunctionID(f_id));
            } else {
                ctx.err.add_internal_error(&format!("{}\n\tfn_target address not in symbol table for '{}{}', not emitted yet...",
                                                    ctx.err_msg,
                                                    if let Some(lib_name) = &ctx.in_obj_call_on {
                                                        format!("{lib_name}.")
                                                    } else {
                                                        "".to_string()
                                                    },
                                                    fn_name), folded_expr.loc());
            }
            is_success
        }
        Expr::VarId { name, .. } => {
            let addr = if let Some(Record::Var { addr, def, .. }) =
                ctx.table.lookup_var_mut(name, true)
            {
                if matches!(def, Definition::CompilerStatic) && addr.is_none() {
                    unreachable!("{} \
                    Variable is bound statically by the compiler, it should've been folded by this point: {}", ctx.err_msg,
                                 name);
                }
                // this is done to avoid two mutable borrows of `ctx.table`
                addr.clone()
            } else {
                unreachable!("unexpected type");
            };
            // this will be different based on if this is a global or local var
            if let Some(addrs) = addr {
                let is_success = true;
                for addr in addrs.clone().iter() {
                    match addr {
                        VarAddr::Global { addr } => {
                            injector.global_get(GlobalID(*addr));
                        }
                        VarAddr::Local { addr } => {
                            injector.local_get(LocalID(*addr));
                        }
                        VarAddr::MemLoc {
                            mem_id,
                            ty,
                            var_offset,
                        } => {
                            ctx.mem_allocator.get_from_mem(
                                *mem_id,
                                &ty.clone(),
                                *var_offset,
                                ctx.table,
                                injector,
                            );
                        }
                        VarAddr::MapId { .. } => {
                            unreachable!(
                                "{} \
                                Variable you are trying to use in expr is a Map object {}",
                                ctx.err_msg, name
                            );
                        }
                    }
                }
                is_success
            } else {
                panic!(
                    "{} \
                Variable does not exist in scope: {}",
                    ctx.err_msg, name
                );
            }
        }
        Expr::Primitive { val, .. } => emit_value(val, idx, strategy, injector, ctx),
        Expr::MapGet { .. } => emit_map_get(&folded_expr, strategy, injector, ctx),
    }
}

fn handle_type_utils_call<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    target_var_name: &str,
    target_var_ty: &DataType,
    target_var_def: Definition,
    target_fn_name: String,
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    match target_var_ty {
        DataType::Str => handle_type_utils_string(
            target_var_name,
            target_var_def,
            target_fn_name,
            args,
            strategy,
            injector,
            ctx,
        ),
        _ => unreachable!("unhandled type utils variant: {:?}", target_var_ty),
    }
}
fn handle_type_utils_string<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    target_var_name: &str,
    target_var_def: Definition,
    target_fn_name: String,
    args: &[Expr],
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let target = Expr::VarId {
        name: target_var_name.to_string(),
        definition: target_var_def,
        loc: None,
    };
    match target_fn_name.as_str() {
        "addr" => StringUtils::addr_of(&target, strategy, injector, ctx),
        "len" => StringUtils::len_dynamic(&target, strategy, injector, ctx),
        "starts_with" => StringUtils::starts_with_dynamic(&target, args, strategy, injector, ctx),
        "ends_with" => StringUtils::ends_with_dynamic(&target, args, strategy, injector, ctx),
        "contains" => StringUtils::contains_dynamic(&target, args, strategy, injector, ctx),
        _ => {
            unreachable!(
                "{} Could not find handler for static function with name: {}",
                ctx.err_msg, target_fn_name
            );
        }
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                    // todo: str
                    unimplemented!("We do not support equal for {done_on}")
                }
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                    // todo: str
                    unimplemented!("We do not support not equal for {done_on}")
                }
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
                | DataType::Str     // todo
                | DataType::Tuple { .. }
                | DataType::Map { .. }
                | DataType::Unknown => {
                    unimplemented!()
                }
                DataType::Lib | DataType::AssumeGood => unreachable!(),
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
                DataType::Lib
                | DataType::F32
                | DataType::F64
                | DataType::AssumeGood
                | DataType::Unknown => {
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
                DataType::Lib
                | DataType::F32
                | DataType::F64
                | DataType::AssumeGood
                | DataType::Unknown => {
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
                DataType::Lib
                | DataType::F32
                | DataType::F64
                | DataType::AssumeGood
                | DataType::Unknown => {
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
                DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
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
            DataType::Lib | DataType::AssumeGood | DataType::Unknown => {
                unreachable!("Attempted NOT for {done_on}")
            }
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
                DataType::Lib
                | DataType::F32
                | DataType::F64
                | DataType::AssumeGood
                | DataType::Unknown => {
                    unreachable!("Attempted bitwise NOT for {done_on}")
                }
            };
        }
    }
    true
}

fn emit_value<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    val: &Value,
    idx: Option<usize>, // optionally specify a specific part of the expression to emit
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    let mut is_success = true;
    match val {
        Value::NumericLiteral { .. } => {
            unreachable!("NumericLiteral must be resolved by the type checker before emission")
        }
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
                let to_emit = if ctx.in_map_op {
                    // If in the context of a map operation, we will likely have to send
                    // this emitted string over to the MapLibrary through interfacing
                    // with its memory. Let's save this string's address in the MapLibAdapter
                    // to enable this logic.
                    ctx.map_lib_adapter.curr_str_offset = Some(str_addr.mem_offset as u32);
                    ctx.map_lib_adapter.curr_str_len = Some(str_addr.len as u32);

                    vec![MAP_LIB_MEM_OFFSET, str_addr.len as u32]
                } else {
                    vec![str_addr.mem_offset as u32, str_addr.len as u32]
                };
                // emit Wasm instructions for the memory address and string length
                if let Some(idx) = idx {
                    injector.u32_const(to_emit[idx]);
                } else {
                    for i in to_emit.iter() {
                        injector.u32_const(*i);
                    }
                }
                is_success &= true;
            } else {
                unreachable!(
                    "{} String has not been emitted yet for value: '{val}'!",
                    ctx.err_msg
                );
            }
        }
        Value::Tuple { vals, .. } => {
            for val in vals.iter() {
                is_success &= emit_expr(val, None, strategy, injector, ctx);
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
        Value::U32U32Map { .. } => unreachable!(
            "{} \
            `emit_value` shouldn't be called with a U32U32Map type...should already be handled!",
            ctx.err_msg
        ),
    }
    is_success
}

fn emit_map_get<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    expr: &Expr,
    strategy: InjectStrategy,
    injector: &mut T,
    ctx: &mut EmitCtx,
) -> bool {
    ctx.in_map_op = true;
    if let Expr::MapGet { map, key, .. } = expr {
        return match get_map_info(map, ctx) {
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
                        );
                    }
                    other => unreachable!("Did not expect this address type: {:?}", other),
                };
                emit_expr(key.as_ref(), None, strategy, injector, ctx);
                ctx.map_lib_adapter.map_get(
                    key_ty,
                    val_ty,
                    injector,
                    ctx.utils_adapter,
                    ctx.mem_allocator,
                    ctx.err,
                );
                true
            }
            None => false,
        };
    }
    unreachable!(
        "{} \
            Wrong statement type, should be `map_get`",
        ctx.err_msg
    );
}
fn get_map_info(name: &str, ctx: &mut EmitCtx) -> Option<(VarAddr, DataType, DataType)> {
    let Some(Record::Var { ty, addr, .. }) = ctx.table.lookup_var(name, true) else {
        unreachable!("unexpected type");
    };

    if let Some(addrs) = addr {
        let addr = addrs.first().unwrap();
        if !matches!(
            addr,
            VarAddr::MapId { .. } | VarAddr::Local { .. } | VarAddr::MemLoc { .. }
        ) {
            assert_eq!(addrs.len(), 1);
            unreachable!("We don't support map locations being stored in addresses other than Local or constant MapId --> {}:{:?}", name, addr)
        }
        if let DataType::Map {
            key_ty: k,
            val_ty: v,
        } = ty
        {
            let key_ty = *k.clone();
            let val_ty = *v.clone();
            Some((addr.clone(), key_ty, val_ty))
        } else {
            unreachable!(
                "Incorrect DataType, expected Map, found: {:?}",
                addr.clone()
            );
        }
    } else {
        unreachable!("map ID address not set yet.");
    }
}

pub fn emit_stack_vals<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    created_stack_vals: &[(String, usize)],
    injector: &mut T,
    ctx: &mut EmitCtx,
) {
    for (_param_name, param_rec_id) in created_stack_vals.iter() {
        emit_stack_val(*param_rec_id, injector, ctx);
    }
}
pub fn emit_stack_val<'ir, T: Opcode<'ir> + MacroOpcode<'ir> + AddLocal>(
    param_rec_id: usize,
    injector: &mut T,
    ctx: &mut EmitCtx,
) {
    let param_rec = ctx.table.get_record_mut(param_rec_id);
    if let Some(Record::Var {
                    addr: Some(addrs), ..
                }) = param_rec
    {
        let VarAddr::Local { addr } = addrs.first().unwrap() else {
            assert_eq!(addrs.len(), 1);
            panic!("arg address should be represented with a single address")
        };
        // Inject at tracker.orig_instr_idx to make sure that this actually emits the args
        // for the instrumented instruction right before that instruction is called!
        injector.local_get(LocalID(*addr));
    } else {
        unreachable!(
            "{} Could not emit parameters, something went wrong...",
            ctx.err_msg
        );
    }
}
