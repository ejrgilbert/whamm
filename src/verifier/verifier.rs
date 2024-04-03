use crate::parser::types::{Dtrace, DtraceVisitor};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::SymbolTable;

pub fn verify(ast: &Dtrace) -> SymbolTable {
    let table = build_symbol_table(&ast);

    // TODO do typechecking!
    return table;
}

// ================
// = SYMBOL TABLE =
// ================

fn build_symbol_table(ast: &Dtrace) -> SymbolTable {
    let mut visitor = SymbolTableBuilder::new();
    visitor.visit_dtrace(ast);
    visitor.table
}
