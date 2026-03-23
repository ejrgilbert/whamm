use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::StringAddr;
use crate::generator::folding::expr::ExprFolder;
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::parser::types::{Block, Expr, Location, Statement};
use crate::verifier::types::SymbolTable;
use std::collections::HashMap;
use wirm::Module;

// =======================================
// = Constant Propagation via StmtFolder =
// =======================================

pub struct StmtFolder<'a, 'ir> {
    registry: &'a mut WasmRegistry,
    emitted_strings: &'a HashMap<String, StringAddr>,
    as_monitor_module: bool,
    curr_loc: Option<Location>,
    app_wasm: &'a Module<'ir>,
}
impl<'a, 'ir> StmtFolder<'a, 'ir> {
    pub fn fold_stmt(
        stmt: &Statement,
        as_monitor_module: bool,
        table: &SymbolTable,
        registry: &'a mut WasmRegistry,
        emitted_strings: &'a HashMap<String, StringAddr>,
        app_wasm: &'a Module<'ir>,
        err: &mut ErrorGen,
    ) -> Block {
        let mut inst = Self {
            registry,
            emitted_strings,
            as_monitor_module,
            curr_loc: None,
            app_wasm,
        };

        inst.fold_stmt_inner(stmt, table, err)
    }

    fn fold_expr(&mut self, expr: &Expr, table: &SymbolTable, err: &mut ErrorGen) -> Expr {
        ExprFolder::fold_expr(
            expr,
            self.registry,
            self.as_monitor_module,
            table,
            self.emitted_strings,
            self.app_wasm,
            err,
        )
    }

    fn fold_stmt_inner(
        &mut self,
        stmt: &Statement,
        table: &SymbolTable,
        err: &mut ErrorGen,
    ) -> Block {
        match stmt {
            Statement::If { .. } => self.fold_if(stmt, table, err),
            Statement::Assign { var_id, expr, loc } => Block::from(&Statement::Assign {
                // var_id is an assignment target (write destination), not an expression
                // to evaluate — never fold it. See ideas/refactor-plan.md for a
                // longer-term fix that changes var_id to a String in the AST.
                var_id: var_id.clone(),
                expr: self.fold_expr(expr, table, err),
                loc: loc.clone(),
            }),
            Statement::SetMap { map, key, val, loc } => Block::from(&Statement::SetMap {
                map: map.clone(),
                key: self.fold_expr(key, table, err),
                val: self.fold_expr(val, table, err),
                loc: loc.clone(),
            }),
            Statement::Expr { expr, loc } => Block::from(&Statement::Expr {
                expr: self.fold_expr(expr, table, err),
                loc: loc.clone(),
            }),
            Statement::Return { expr, loc } => Block::from(&Statement::Return {
                expr: self.fold_expr(expr, table, err),
                loc: loc.clone(),
            }),
            Statement::VarDecl {
                name,
                ty,
                definition,
                modifiers,
                init: Some(init),
                loc,
            } => Block::from(&Statement::VarDecl {
                name: name.clone(),
                ty: ty.clone(),
                definition: *definition,
                modifiers: modifiers.clone(),
                init: Some(self.fold_expr(init, table, err)),
                loc: loc.clone(),
            }),
            // LibImport and VarDecl with no init have no expressions to fold.
            Statement::LibImport { .. } | Statement::VarDecl { init: None, .. } => {
                Block::from(stmt)
            }
        }
    }

    fn fold_if(&mut self, _if: &Statement, table: &SymbolTable, err: &mut ErrorGen) -> Block {
        self.curr_loc = _if.loc().clone();

        if let Statement::If {
            cond,
            conseq,
            alt,
            loc,
        } = _if
        {
            // if the condition evaluates to:
            // -- true: conseq
            // -- false: alt
            // -- other: orig (with folded condition and sub-blocks)
            let folded_cond = self.fold_expr(cond, table, err);
            if let Some(b) = ExprFolder::get_single_bool(&folded_cond) {
                let mut new_block = Block::default();
                let to_fold = if b { conseq } else { alt };
                for stmt in to_fold.stmts.iter() {
                    new_block.extend(self.fold_stmt_inner(stmt, table, err));
                }
                return new_block;
            }

            // Condition is not constant — fold sub-blocks and return the If.
            let mut folded_conseq = Block::default();
            for stmt in conseq.stmts.iter() {
                folded_conseq.extend(self.fold_stmt_inner(stmt, table, err));
            }
            let mut folded_alt = Block::default();
            for stmt in alt.stmts.iter() {
                folded_alt.extend(self.fold_stmt_inner(stmt, table, err));
            }
            return Block::from(&Statement::If {
                cond: folded_cond,
                conseq: folded_conseq,
                alt: folded_alt,
                loc: loc.clone(),
            });
        }

        // Cannot fold anymore
        Block::from(_if)
    }
}
