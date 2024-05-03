use crate::parser::types::{Whamm, WhammVisitor};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::SymbolTable;

pub fn verify(ast: &Whamm, verify: bool) -> SymbolTable {
    if verify {
        // TODO do typechecking!
        unimplemented!()
    }
    build_symbol_table(&ast)
}

// ================
// = SYMBOL TABLE =
// ================

fn build_symbol_table(ast: &Whamm) -> SymbolTable {
    let mut visitor = SymbolTableBuilder::new();
    visitor.visit_whamm(ast);
    visitor.table
}
