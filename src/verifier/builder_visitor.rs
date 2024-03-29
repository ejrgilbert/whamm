use std::any::Any;
use crate::parser::types as parser_types;
use parser_types::{Dtrace};
use crate::parser::types::Expression;

trait Visit {
    fn visit(&self, table: SymbolTable);
}

impl Visit for Dtrace {

}

impl Expression for BinOp {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }

    fn as_str(&self, indent: &mut i32) -> String {
        todo!()
    }

    fn visit(&self, table: SymbolTable) {
        todo!()
    }
}