//! Per-site monomorphization of generic probes for the rewriting backend.
//!
//! A generic probe keeps its type parameters symbolic through type-checking. At each match
//! site the concrete operand types are known, so we bind each type parameter to the site's
//! concrete type and substitute it into a per-site clone of the probe.

use std::collections::HashMap;

use crate::emitter::rewriting::rules::{nth_prefixed, StackVal};
use crate::generator::ast::Probe;
use crate::parser::generic_constraint::GenericConstraint;
use crate::parser::types::{Block, DataType, Expr, Statement, UnOp, Value};

/// Resolve a type through a type parameter binding (recursively through Map/Tuple).
fn resolve_dt(ty: &DataType, binding: &HashMap<String, DataType>) -> DataType {
    match ty {
        DataType::TypeParam(name) => binding.get(name).cloned().unwrap_or_else(|| ty.clone()),
        DataType::Map { key_ty, val_ty } => DataType::Map {
            key_ty: Box::new(resolve_dt(key_ty, binding)),
            val_ty: Box::new(resolve_dt(val_ty, binding)),
        },
        DataType::Tuple { ty_info } => DataType::Tuple {
            ty_info: ty_info.iter().map(|t| resolve_dt(t, binding)).collect(),
        },
        other => other.clone(),
    }
}

/// Look up an operand's concrete type at this site from the appropriate stack.
fn operand_site_type(
    name: &str,
    args: &[StackVal],
    results: &[StackVal],
    locals: &[StackVal],
) -> Option<DataType> {
    let (prefix, stack) = if name.starts_with("arg") {
        ("arg", args)
    } else if name.starts_with("res") {
        ("res", results)
    } else if name.starts_with("local") {
        ("local", locals)
    } else {
        return None;
    };
    let n = nth_prefixed(name, prefix)?;
    stack
        .get(n as usize)
        .and_then(|sv| sv.ty)
        .map(|wt| DataType::from_wasm_type(&wt))
}

/// Bind each of the probe's type parameters to the concrete operand type at this site.
/// Returns `None` (no match) if an operand is missing, its type isn't admitted by the
/// var's effective constraint, or two operands sharing a var disagree.
pub fn bind_type_params_at_site(
    probe: &Probe,
    args: &[StackVal],
    results: &[StackVal],
    locals: &[StackVal],
) -> Option<HashMap<String, DataType>> {
    let constraints: HashMap<&str, &GenericConstraint> = probe
        .type_params
        .iter()
        .map(|(n, c)| (n.as_str(), c))
        .collect();
    let mut binding: HashMap<String, DataType> = HashMap::new();

    for (var, ty) in probe.type_bounds.iter() {
        let DataType::TypeParam(tp) = ty else {
            continue;
        };
        let Expr::VarId { name, .. } = var else {
            continue;
        };
        let concrete = operand_site_type(name, args, results, locals)?;
        let constraint = constraints.get(tp.as_str())?;
        if !constraint.admits(&concrete) {
            return None;
        }
        match binding.get(tp) {
            Some(prev) if *prev != concrete => return None,
            Some(_) => {}
            None => {
                binding.insert(tp.clone(), concrete);
            }
        }
    }
    Some(binding)
}

/// Substitute the site binding into a per-site probe clone, making it fully concrete.
pub fn monomorphize_probe(probe: &mut Probe, binding: &HashMap<String, DataType>) {
    if let Some(body) = &mut probe.body {
        subst_block(body, binding);
    }
    if let Some(pred) = &mut probe.predicate {
        subst_expr(pred, binding);
    }
    for (_, ty) in probe.type_bounds.iter_mut() {
        *ty = resolve_dt(ty, binding);
    }
    for uv in probe.unshared_to_alloc.iter_mut() {
        uv.ty = resolve_dt(&uv.ty, binding);
        if let Some(meta) = &mut uv.report_metadata {
            let concrete = resolve_dt(&meta.get_whamm_ty(), binding);
            meta.set_wasm_tys(concrete.to_wasm_type());
            meta.set_whamm_ty(concrete);
        }
    }
    probe.type_params.clear();
}

fn subst_block(block: &mut Block, binding: &HashMap<String, DataType>) {
    for stmt in block.stmts.iter_mut() {
        subst_stmt(stmt, binding);
    }
}

fn subst_stmt(stmt: &mut Statement, binding: &HashMap<String, DataType>) {
    match stmt {
        Statement::VarDecl { ty, init, .. } => {
            *ty = resolve_dt(ty, binding);
            if let Some(init) = init {
                subst_expr(init, binding);
                resolve_literal(init, ty);
            }
        }
        Statement::Assign { var_id, expr, .. } => {
            subst_expr(var_id, binding);
            subst_expr(expr, binding);
        }
        Statement::SetMap { key, val, .. } => {
            subst_expr(key, binding);
            subst_expr(val, binding);
        }
        Statement::Expr { expr, .. } | Statement::Return { expr, .. } => {
            subst_expr(expr, binding);
        }
        Statement::If {
            cond, conseq, alt, ..
        } => {
            subst_expr(cond, binding);
            subst_block(conseq, binding);
            subst_block(alt, binding);
        }
        Statement::LibImport { .. } => {}
    }
}

fn subst_expr(expr: &mut Expr, binding: &HashMap<String, DataType>) {
    match expr {
        Expr::BinOp {
            lhs, rhs, done_on, ..
        } => {
            subst_expr(lhs, binding);
            subst_expr(rhs, binding);
            *done_on = resolve_dt(done_on, binding);
            // Resolve deferred numeric literals to the operation's concrete type.
            resolve_literal(lhs, done_on);
            resolve_literal(rhs, done_on);
        }
        Expr::UnOp {
            op, expr, done_on, ..
        } => {
            subst_expr(expr, binding);
            *done_on = resolve_dt(done_on, binding);
            // Resolve a cast target `x as T` to its concrete type so `emit_unop` can lower it.
            if let UnOp::Cast { target } = op {
                *target = resolve_dt(target, binding);
            }
            resolve_literal(expr, done_on);
        }
        Expr::Ternary {
            cond,
            conseq,
            alt,
            ty,
            ..
        } => {
            subst_expr(cond, binding);
            subst_expr(conseq, binding);
            subst_expr(alt, binding);
            *ty = resolve_dt(ty, binding);
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            subst_expr(fn_target, binding);
            for a in args.iter_mut() {
                subst_expr(a, binding);
            }
        }
        Expr::MapGet { key, .. } => subst_expr(key, binding),
        Expr::TupleGet { tuple, .. } => subst_expr(tuple, binding),
        Expr::VarId { .. } | Expr::Primitive { .. } => {}
    }
}

/// Resolve a numeric-literal operand to the concrete type `ty` for this site. Handles both
/// the deferred form (`NumericLiteral`, from arithmetic where the type parameter propagated) and
/// an already-resolved `Number` (from a comparison, which resolved the literal to a default
/// type before the type parameter was known) — the latter is re-cast to `ty`.
fn resolve_literal(expr: &mut Expr, ty: &DataType) {
    let Expr::Primitive { val, .. } = expr else {
        return;
    };
    match val {
        Value::NumericLiteral { raw, fmt, token } => {
            if let Some(num_lit) = Value::num_lit_from_raw(*raw, ty) {
                *val = Value::Number {
                    val: num_lit,
                    ty: ty.clone(),
                    token: token.clone(),
                    fmt: fmt.clone(),
                };
            }
        }
        Value::Number { .. } => {
            if val.ty() != *ty {
                let _ = val.implicit_cast(ty);
            }
        }
        _ => {}
    }
}
