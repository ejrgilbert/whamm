use crate::common::error::ErrorGen;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::verifier::builder_visitor::SymbolTableBuilder;
use crate::verifier::types::SymbolTable;

pub fn build_symbol_table(ast: &Whamm, err: &mut ErrorGen) -> SymbolTable {
    let mut visitor = SymbolTableBuilder {
        table: SymbolTable::new(),
        err,
        curr_whamm: None,
        curr_whammy: None,
        curr_provider: None,
        curr_package: None,
        curr_event: None,
        curr_probe: None,
        curr_fn: None,
    };
    visitor.visit_whamm(ast);
    visitor.table
}

pub fn verify(_ast: &Whamm) -> bool {
    // TODO do typechecking!
    unimplemented!()
}
