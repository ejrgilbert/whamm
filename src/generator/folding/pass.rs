use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::StringAddr;
use crate::generator::ast::Script;
use crate::generator::folding::stmt::StmtFolder;
use crate::parser::types::{Block, Fn, Statement};
use crate::verifier::types::SymbolTable;
use std::collections::HashMap;

/// Runs dead-branch elimination over the generator AST (probe bodies,
/// `init_logic`, `global_stmts`, and user-defined function bodies) before
/// emission, so that emitters can consume `&Statement` immutably.
///
/// This mirrors what `emit_stmt` in `utils.rs` does inline per statement:
/// call `StmtFolder::fold_stmt`, which eliminates any `Statement::If` whose
/// condition is statically constant.  Per-expression constant propagation
/// (`ExprFolder`) is intentionally **not** run here — it is handled by the
/// emitters for specific expression types (bound-function args, predicates,
/// etc.) and must not be run on `ObjCall` / static-lib expressions, which
/// require per-opcode context.
///
/// Predicates are also intentionally **not** touched here: they reference
/// compiler-static variables whose values change per opcode and are re-folded
/// in the rewriting emitter at each instrumentation site.
pub fn run(
    ast: &mut [Script],
    as_monitor_module: bool,
    table: &SymbolTable,
    emitted_strings: &HashMap<String, StringAddr>,
    err: &mut ErrorGen,
) {
    for script in ast.iter_mut() {
        fold_stmts(
            &mut script.global_stmts,
            as_monitor_module,
            table,
            emitted_strings,
            err,
        );
        for f in script.fns.iter_mut() {
            fold_fn(f, as_monitor_module, table, emitted_strings, err);
        }
        for probe in script.probes.iter_mut() {
            if let Some(body) = &mut probe.body {
                fold_block(body, as_monitor_module, table, emitted_strings, err);
            }
            fold_stmts(
                &mut probe.init_logic,
                as_monitor_module,
                table,
                emitted_strings,
                err,
            );
        }
    }
}

fn fold_fn(
    f: &mut Fn,
    as_monitor_module: bool,
    table: &SymbolTable,
    emitted_strings: &HashMap<String, StringAddr>,
    err: &mut ErrorGen,
) {
    fold_block(&mut f.body, as_monitor_module, table, emitted_strings, err);
}

fn fold_block(
    block: &mut Block,
    as_monitor_module: bool,
    table: &SymbolTable,
    emitted_strings: &HashMap<String, StringAddr>,
    err: &mut ErrorGen,
) {
    fold_stmts(
        &mut block.stmts,
        as_monitor_module,
        table,
        emitted_strings,
        err,
    );
}

/// Replace each statement with its `StmtFolder`-folded form.
/// `StmtFolder` eliminates dead `If`-branches; all other statements pass through.
fn fold_stmts(
    stmts: &mut Vec<Statement>,
    as_monitor_module: bool,
    table: &SymbolTable,
    emitted_strings: &HashMap<String, StringAddr>,
    err: &mut ErrorGen,
) {
    let original = std::mem::take(stmts);
    for stmt in original {
        let folded = StmtFolder::fold_stmt(&stmt, as_monitor_module, table, emitted_strings, err);
        stmts.extend(folded.stmts);
    }
}
