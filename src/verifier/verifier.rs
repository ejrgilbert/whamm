use crate::parser::types::{Whamm, WhammVisitor};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::SymbolTable;

pub fn build_symbol_table(ast: &Whamm) -> SymbolTable {
    let mut visitor = SymbolTableBuilder::new();
    visitor.visit_whamm(ast);
    visitor.table
}

pub fn verify(_ast: &Whamm) -> bool {
    // TODO do typechecking!
    unimplemented!()
}
