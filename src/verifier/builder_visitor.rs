use crate::parser::types as parser_types;
use parser_types::{DataType, Dscript, Dtrace, DtraceVisitor, Expr, Function, Module, Op, Probe, Provider, Statement, Value};
use crate::verifier::types::SymbolTable;

pub struct SymbolTableBuilder {
    pub table: SymbolTable
}
impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            table: SymbolTable::new()
        }
    }
}

impl DtraceVisitor<()> for SymbolTableBuilder {
    fn visit_datatype(&mut self, datatype: &DataType) -> () {
        todo!()
    }

    fn visit_value(&mut self, int: &Value) -> () {
        todo!()
    }

    fn visit_stmt(&mut self, assign: &Statement) -> () {
        todo!()
    }

    fn visit_expr(&mut self, call: &Expr) -> () {
        todo!()
    }

    fn visit_op(&mut self, op: &Op) -> () {
        todo!()
    }

    fn visit_fn(&mut self, f: &parser_types::Fn) -> () {
        todo!()
    }

    fn visit_dtrace(&mut self, dtrace: &Dtrace) -> () {
        todo!()
    }

    fn visit_dscript(&mut self, dscript: &Dscript) -> () {
        todo!()
    }

    fn visit_provider(&mut self, provider: &Provider) -> () {
        todo!()
    }

    fn visit_module(&mut self, module: &Module) -> () {
        todo!()
    }

    fn visit_function(&mut self, function: &Function) -> () {
        todo!()
    }

    fn visit_probe(&mut self, probe: &Probe) -> () {
        todo!()
    }
}