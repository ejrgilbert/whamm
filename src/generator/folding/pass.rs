use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::StringAddr;
use crate::generator::folding::stmt::StmtFolder;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::parser::types::{Annotation, Block, Definition, Expr, Fn, Statement, Value};
use crate::verifier::types::{Record, SymbolTable};
use std::collections::HashMap;
use wirm::Module;

fn fold_fn<'ir>(
    f: &mut Fn,
    as_monitor_module: bool,
    table: &mut SymbolTable,
    registry: &mut WasmRegistry,
    emitted_strings: &HashMap<String, StringAddr>,
    app_wasm: &Module<'ir>,
    err: &mut ErrorGen,
) {
    fold_block(
        &mut f.body,
        as_monitor_module,
        table,
        registry,
        emitted_strings,
        app_wasm,
        err,
    );
}

pub fn fold_block<'ir>(
    block: &mut Block,
    as_monitor_module: bool,
    table: &mut SymbolTable,
    registry: &mut WasmRegistry,
    emitted_strings: &HashMap<String, StringAddr>,
    app_wasm: &Module<'ir>,
    err: &mut ErrorGen,
) {
    fold_stmts(
        &mut block.stmts,
        as_monitor_module,
        table,
        registry,
        emitted_strings,
        app_wasm,
        err,
    );
}

/// Fold expressions within each statement in a slice in-place (1:1 replacement).
/// Unlike `fold_stmts`, this does not handle `If`-branch elimination that could change
/// the number of statements. Use this when the caller holds a `&mut [Statement]`.
pub fn fold_stmts_slice<'ir>(
    stmts: &mut [Statement],
    as_monitor_module: bool,
    table: &mut SymbolTable,
    registry: &mut WasmRegistry,
    emitted_strings: &HashMap<String, StringAddr>,
    app_wasm: &Module<'ir>,
    err: &mut ErrorGen,
) {
    for stmt in stmts.iter_mut() {
        let folded = StmtFolder::fold_stmt(
            stmt,
            as_monitor_module,
            table,
            registry,
            emitted_strings,
            app_wasm,
            err,
        );
        // A folded non-If statement always produces exactly one statement.
        // (If statements are not expected in global_stmts.)
        if let Some(new_stmt) = folded.stmts.into_iter().next() {
            propagate_primitive_assign(&new_stmt, table);
            *stmt = new_stmt;
        }
    }
}

/// Replace each statement with its `StmtFolder`-folded form.
/// `StmtFolder` eliminates dead `If`-branches; all other statements pass through.
pub fn fold_stmts<'ir>(
    stmts: &mut Vec<Statement>,
    as_monitor_module: bool,
    table: &mut SymbolTable,
    registry: &mut WasmRegistry,
    emitted_strings: &HashMap<String, StringAddr>,
    app_wasm: &Module<'ir>,
    err: &mut ErrorGen,
) {
    let original = std::mem::take(stmts);
    for stmt in original {
        let folded = StmtFolder::fold_stmt(
            &stmt,
            as_monitor_module,
            table,
            registry,
            emitted_strings,
            app_wasm,
            err,
        );
        if !is_at_static(&stmt) {
            // if it's @static call, propagating the assign
            // would override other probe body values.
            for new_stmt in &folded.stmts {
                propagate_primitive_assign(new_stmt, table);
            }
        }
        stmts.extend(folded.stmts);
    }
}

fn is_at_static(stmt: &Statement) -> bool {
    matches!(
        stmt,
        Statement::VarDecl {
            init: Some(Expr::ObjCall {
                annotation: Some(Annotation::Static),
                ..
            }),
            ..
        } | Statement::Assign {
            expr: Expr::ObjCall {
                annotation: Some(Annotation::Static),
                ..
            },
            ..
        }
    )
}

/// After folding a statement, if it is a stable user-defined primitive assignment
/// (`var = <constant>` or `var x = <constant>`), record the value in the symbol
/// table so that subsequent `fold_var_id` calls inline the constant at every use site.
fn propagate_primitive_assign(stmt: &Statement, table: &mut SymbolTable) {
    let (name, val) = match stmt {
        Statement::Assign {
            var_id: Expr::VarId { name, .. },
            expr: Expr::Primitive { val, .. },
            ..
        } => (name, val),
        Statement::VarDecl {
            name,
            init: Some(Expr::Primitive { val, .. }),
            ..
        } => (name, val),
        _ => return,
    };
    let rec = table.lookup_var_mut(name, false);
    let Some(Record::Var {
        value,
        times_set,
        def,
        ..
    }) = rec
    else {
        return;
    };
    // Only propagate when the variable is assigned at most once (stable) and is
    // user-defined (not a compiler-injected bound variable).
    if *times_set <= 1 && matches!(def, Definition::User) {
        if !matches!(val, Value::Tuple { .. }) {
            *value = Some(val.clone());
        }
    }
}
