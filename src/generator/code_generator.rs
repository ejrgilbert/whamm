// =======================
// ==== CodeGenerator ====
// =======================

use std::collections::HashMap;
use log::trace;
use crate::generator::emitters::Emitter;
use crate::parser::types::{DataType, Dscript, Dtrace, DtraceVisitorMut, Expr, Function, Module, Op, Probe, Provider, Statement, Value};

/// The code generator traverses the AST and calls the passed emitter to
/// emit some instruction/code/function/etc.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct CodeGenerator {
    pub emitter: Box<dyn Emitter>,
    pub context_name: String
}
impl CodeGenerator {
    pub fn new(emitter: Box<dyn Emitter>) -> Self {
        Self {
            emitter,
            context_name: "".to_string()
        }
    }
    pub fn generate(&mut self, dtrace: &mut Dtrace) -> bool {
        self.visit_dtrace(dtrace)
    }
    pub fn dump_to_file(&mut self, output_wasm_path: String) -> bool {
        self.emitter.dump_to_file(output_wasm_path)
    }

    // Private helper functions
    fn visit_globals(&mut self, globals: &HashMap<String, (DataType, Expr, Option<Value>)>) -> bool {
        let mut is_success = true;
        for (name, (ty, _expr, val)) in globals.iter() {
            is_success &= self.emitter.emit_global(name.clone(), ty.clone(), val);
        }

        is_success
    }
}
impl DtraceVisitorMut<bool> for CodeGenerator {
    fn visit_dtrace(&mut self, dtrace: &mut Dtrace) -> bool {
        trace!("Entering: CodeGenerator::visit_dtrace");
        self.emitter.enter_scope();
        self.context_name  = "dtrace".to_string();
        let mut is_success = self.emitter.emit_dtrace(dtrace);

        // visit fns
        dtrace.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)
        // inject dscripts
        dtrace.dscripts.iter_mut().for_each(|dscript| {
            is_success &= self.visit_dscript(dscript);
        });

        trace!("Exiting: CodeGenerator::visit_dtrace");
        self.emitter.exit_scope();
        // Remove from `context_name`
        self.context_name = "".to_string();
        is_success
    }

    fn visit_dscript(&mut self, dscript: &mut Dscript) -> bool {
        trace!("Entering: CodeGenerator::visit_dscript");
        self.emitter.enter_scope();
        self.context_name += &format!(":{}", dscript.name.clone());
        let mut is_success = self.emitter.emit_dscript(dscript);

        // visit fns
        dscript.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&dscript.globals);
        // inject providers
        dscript.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_dscript");
        self.emitter.exit_scope();
        // Remove from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_provider(&mut self, provider: &mut Provider) -> bool {
        trace!("Entering: CodeGenerator::visit_provider");
        self.emitter.enter_scope();
        self.context_name += &format!(":{}", provider.name.clone());
        let mut is_success = true;

        // visit fns
        provider.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)
        // inject module fns/globals
        provider.modules.iter_mut().for_each(|(_name, module)| {
            is_success &= self.visit_module(module);
        });

        // At this point we've traversed the entire tree to generate necessary
        // globals and fns!
        // Now, we emit_provider which will do the actual instrumentation step!
        is_success &= self.emitter.emit_provider(&self.context_name, provider);

        trace!("Exiting: CodeGenerator::visit_provider");
        self.emitter.exit_scope();
        // Remove this module from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_module(&mut self, module: &mut Module) -> bool {
        trace!("Entering: CodeGenerator::visit_module");
        self.emitter.enter_scope();
        let mut is_success = true;
        self.context_name += &format!(":{}", module.name.clone());

        // visit fns
        module.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)
        // inject function fns/globals
        module.functions.iter_mut().for_each(|(_name, function)| {
            is_success &= self.visit_function(function);
        });

        trace!("Exiting: CodeGenerator::visit_module");
        self.emitter.exit_scope();
        // Remove this module from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_function(&mut self, function: &mut Function) -> bool {
        trace!("Entering: CodeGenerator::visit_function");
        self.emitter.enter_scope();
        // let mut is_success = self.emitter.emit_function(function);
        self.context_name += &format!(":{}", function.name.clone());
        let mut is_success = true;

        // visit fns
        function.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)
        // inject probe fns/globals
        function.probe_map.iter_mut().for_each(|(_name, probes)| {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        });

        trace!("Exiting: CodeGenerator::visit_function");
        self.emitter.exit_scope();
        // Remove this function from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_probe(&mut self, probe: &mut Probe) -> bool {
        trace!("Entering: CodeGenerator::visit_probe");
        self.emitter.enter_scope();
        // let mut is_success = self.emitter.emit_probe(probe);
        self.context_name += &format!(":{}", probe.name.clone());
        let mut is_success = true;

        // visit fns
        probe.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)

        trace!("Exiting: CodeGenerator::visit_probe");
        self.emitter.exit_scope();
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_fn(&mut self, f: &mut crate::parser::types::Fn) -> bool {
        trace!("Entering: CodeGenerator::visit_fn");
        self.emitter.enter_scope();
        let is_success = self.emitter.emit_fn(&self.context_name, f);
        trace!("Exiting: CodeGenerator::visit_fn");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_formal_param(&mut self, param: &mut (Expr, DataType)) -> bool {
        trace!("Entering: CodeGenerator::visit_formal_param");
        let is_success = self.emitter.emit_formal_param(param);
        trace!("Exiting: CodeGenerator::visit_formal_param");
        is_success
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> bool {
        trace!("Entering: CodeGenerator::visit_stmt");
        let is_success = self.emitter.emit_stmt(stmt);
        trace!("Exiting: CodeGenerator::visit_stmt");
        is_success
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> bool {
        trace!("Entering: CodeGenerator::visit_expr");
        let is_success = self.emitter.emit_expr(expr);
        trace!("Exiting: CodeGenerator::visit_expr");
        is_success
    }

    fn visit_op(&mut self, op: &mut Op) -> bool {
        trace!("Entering: CodeGenerator::visit_op");
        let is_success = self.emitter.emit_op(op);
        trace!("Exiting: CodeGenerator::visit_op");
        is_success
    }

    fn visit_datatype(&mut self, datatype: &mut DataType) -> bool {
        trace!("Entering: CodeGenerator::visit_datatype");
        let is_success = self.emitter.emit_datatype(datatype);
        trace!("Exiting: CodeGenerator::visit_datatype");
        is_success
    }

    fn visit_value(&mut self, val: &mut Value) -> bool {
        trace!("Entering: CodeGenerator::visit_value");
        let is_success = self.emitter.emit_value(val);
        trace!("Exiting: CodeGenerator::visit_value");
        is_success
    }
}