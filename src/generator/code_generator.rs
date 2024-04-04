// =======================
// ==== CodeGenerator ====
// =======================

use std::collections::HashMap;
use log::trace;
use crate::generator::emitters::Emitter;
use crate::parser::types::{DataType, Dscript, Dtrace, DtraceVisitor, Expr, Function, Module, Op, Probe, Provider, Statement, Value};

/// The code generator traverses the AST and calls the passed emitter to
/// emit some instruction/code/function/etc.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct CodeGenerator {
    pub emitter: Box<dyn Emitter>
}
impl CodeGenerator {
    pub fn generate(&mut self, dtrace: &Dtrace) -> bool {
        self.visit_dtrace(dtrace)
    }
    pub fn dump_to_file(&mut self, output_wasm_path: String) -> bool {
        self.emitter.dump_to_file(output_wasm_path)
    }

    // Private helper functions
    fn get_var_name(&self, expr: &Expr) -> Option<String> {
        match expr {
            Expr::VarId {name} => Some(name.clone()),
            _ => None
        }
    }
    fn visit_globals(&mut self, globals: &HashMap<(DataType, Expr), Option<Value>>) -> bool {
        let mut is_success = true;
        for ((ty, expr), val) in globals.iter() {
            let name = self.get_var_name(expr).unwrap();
            is_success &= self.emitter.emit_global(name, ty.clone(), val);
        }

        is_success
    }
}
// TODO -- this might need to have a first and second pass traversal.
// 1. emit fns and globals
// 2. emit probes
impl DtraceVisitor<bool> for CodeGenerator {
    fn visit_dtrace(&mut self, dtrace: &Dtrace) -> bool {
        trace!("Entering: CodeGenerator::visit_dtrace");
        self.emitter.enter_scope();
        let mut is_success = self.emitter.emit_dtrace(dtrace);

        // TODO -- do I need to pass the Dtrace type/name as context to emitter...actually this is in the scope...
        // visit fns
        dtrace.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&dtrace.globals);
        // inject dscripts
        dtrace.dscripts.iter().for_each(|dscript| {
            is_success &= self.visit_dscript(dscript);
        });

        trace!("Exiting: CodeGenerator::visit_dtrace");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_dscript(&mut self, dscript: &Dscript) -> bool {
        trace!("Entering: CodeGenerator::visit_dscript");
        self.emitter.enter_scope();
        let mut is_success = self.emitter.emit_dscript(dscript);

        // visit fns
        dscript.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&dscript.globals);
        // inject providers
        dscript.providers.iter().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_dscript");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_provider(&mut self, provider: &Provider) -> bool {
        trace!("Entering: CodeGenerator::visit_provider");
        self.emitter.enter_scope();
        let mut is_success = self.emitter.emit_provider(provider);

        // visit fns
        provider.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&provider.globals);
        // inject module fns/globals
        provider.modules.iter().for_each(|(_name, module)| {
            is_success &= self.visit_module(module);
        });

        // At this point we've traversed the entire tree to generate necessary
        // globals and fns!
        // Now, we emit_provider which will do the actual instrumentation step!
        let mut is_success = self.emitter.emit_provider(provider);

        trace!("Exiting: CodeGenerator::visit_provider");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_module(&mut self, module: &Module) -> bool {
        trace!("Entering: CodeGenerator::visit_module");
        self.emitter.enter_scope();

        // visit fns
        module.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&module.globals);
        // inject function fns/globals
        module.functions.iter().for_each(|(_name, function)| {
            is_success &= self.visit_function(function);
        });

        trace!("Exiting: CodeGenerator::visit_module");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_function(&mut self, function: &Function) -> bool {
        trace!("Entering: CodeGenerator::visit_function");
        self.emitter.enter_scope();
        let mut is_success = self.emitter.emit_function(function);

        // visit fns
        function.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&function.globals);
        // inject probe fns/globals
        function.probe_map.iter().for_each(|(_name, probes)| {
            probes.iter().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        });

        trace!("Exiting: CodeGenerator::visit_function");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_probe(&mut self, probe: &Probe) -> bool {
        trace!("Entering: CodeGenerator::visit_probe");
        self.emitter.enter_scope();
        let mut is_success = self.emitter.emit_probe(probe);

        // visit fns
        probe.fns.iter().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&probe.globals);

        trace!("Exiting: CodeGenerator::visit_probe");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> bool {
        trace!("Entering: CodeGenerator::visit_fn");
        self.emitter.enter_scope();
        let is_success = self.emitter.emit_fn(f);
        trace!("Exiting: CodeGenerator::visit_fn");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> bool {
        trace!("Entering: CodeGenerator::visit_formal_param");
        let is_success = self.emitter.emit_formal_param(param);
        trace!("Exiting: CodeGenerator::visit_formal_param");
        is_success
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> bool {
        trace!("Entering: CodeGenerator::visit_stmt");
        let is_success = self.emitter.emit_stmt(stmt);
        trace!("Exiting: CodeGenerator::visit_stmt");
        is_success
    }

    fn visit_expr(&mut self, expr: &Expr) -> bool {
        trace!("Entering: CodeGenerator::visit_expr");
        let is_success = self.emitter.emit_expr(expr);
        trace!("Exiting: CodeGenerator::visit_expr");
        is_success
    }

    fn visit_op(&mut self, op: &Op) -> bool {
        trace!("Entering: CodeGenerator::visit_op");
        let is_success = self.emitter.emit_op(op);
        trace!("Exiting: CodeGenerator::visit_op");
        is_success
    }

    fn visit_datatype(&mut self, datatype: &DataType) -> bool {
        trace!("Entering: CodeGenerator::visit_datatype");
        let is_success = self.emitter.emit_datatype(datatype);
        trace!("Exiting: CodeGenerator::visit_datatype");
        is_success
    }

    fn visit_value(&mut self, val: &Value) -> bool {
        trace!("Entering: CodeGenerator::visit_value");
        let is_success = self.emitter.emit_val(val);
        trace!("Exiting: CodeGenerator::visit_value");
        is_success
    }
}