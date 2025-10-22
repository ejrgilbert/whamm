use crate::common::error::ErrorGen;
use crate::generator::folding::expr::ExprFolder;
use crate::parser::types::{Block, Location, Statement};
use crate::verifier::types::SymbolTable;

// =======================================
// = Constant Propagation via StmtFolder =
// =======================================

pub struct StmtFolder {
    as_monitor_module: bool,
    curr_loc: Option<Location>,
}
impl StmtFolder {
    pub fn fold_stmt(stmt: &Statement, as_monitor_module: bool, table: &SymbolTable, err: &mut ErrorGen) -> Block {
        let mut inst = Self { as_monitor_module, curr_loc: None };

        inst.fold_stmt_inner(stmt, table, err)
    }

    fn fold_stmt_inner(
        &mut self,
        stmt: &Statement,
        table: &SymbolTable,
        err: &mut ErrorGen,
    ) -> Block {
        match stmt {
            Statement::If { .. } => self.fold_if(stmt, table, err),
            _ => Block::from(stmt), // other statements, we don't need to fold
        }
    }

    fn fold_if(&mut self, _if: &Statement, table: &SymbolTable, err: &mut ErrorGen) -> Block {
        self.curr_loc = _if.loc().clone();

        if let Statement::If {
            cond, conseq, alt, ..
        } = _if
        {
            // if the condition evaluates to:
            // -- true: conseq
            // -- false: alt
            // -- other: orig
            let folded_expr = ExprFolder::fold_expr(cond, self.as_monitor_module, table, err);
            if let Some(b) = ExprFolder::get_single_bool(&folded_expr, self.as_monitor_module) {
                let mut new_block = Block::default();
                let to_fold = if b {
                    // fold to conseq block
                    conseq
                } else {
                    // fold to alt block
                    alt
                };

                for stmt in to_fold.stmts.iter() {
                    new_block.extend(self.fold_stmt_inner(stmt, table, err));
                }

                return new_block;
            }
        }

        // Cannot fold anymore
        Block::from(_if)
    }
}
