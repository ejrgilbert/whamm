use log::trace;
use crate::emitter::report_var_metadata::LocationData;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{BinOp, Block, DataType, Expr, ProvidedFunction, Script, Statement, UnOp, Value, Whamm, WhammVisitor};

struct TestModuleGenerator {

}
impl WhammVisitor<()> for TestModuleGenerator {
    fn visit_whamm(&mut self, whamm: &Whamm) -> () {
        // visit scripts
        whamm.scripts.iter().for_each(|script| {
            self.visit_script(script);
        });
    }

    fn visit_script(&mut self, script: &Script) -> () {
        // visit providers
        script.providers.iter_().for_each(|(_name, provider)| {
            self.visit_provider(provider);
        });
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> () {
        // visit the packages
        provider.packages().for_each(|package| {
            self.visit_package(package);
        });
    }

    fn visit_package(&mut self, package: &dyn Package) -> () {
        // visit the events
        package.events().for_each(|event| {
            self.visit_event(event);
        });
    }

    fn visit_event(&mut self, event: &dyn Event) -> () {
        event.probes().iter().for_each(|(_mode, probe_list)| {
            probe_list.iter().for_each(|probe| {
                // TODO create new module
                self.visit_probe(probe);
            });
        });
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> () {
        todo!()
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> () {
        todo!()
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> () {
        todo!()
    }

    fn visit_block(&mut self, block: &Block) -> () {
        todo!()
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> () {
        todo!()
    }

    fn visit_expr(&mut self, expr: &Expr) -> () {
        todo!()
    }

    fn visit_unop(&mut self, unop: &UnOp) -> () {
        todo!()
    }

    fn visit_binop(&mut self, binop: &BinOp) -> () {
        todo!()
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> () {
        todo!()
    }

    fn visit_value(&mut self, val: &Value) -> () {
        todo!()
    }
}