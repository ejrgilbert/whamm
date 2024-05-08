// =======================
// ==== CodeGenerator ====
// =======================

use std::collections::HashMap;
use log::{info, trace, warn};
use crate::generator::emitters::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::{DataType, Whammy, Whamm, WhammVisitorMut, Expr, Function, Module, Op, Probe, Provider, Statement, Value};

/// The code generator traverses the AST and calls the passed emitter to
/// emit some instruction/code/function/etc.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct CodeGenerator {
    pub emitter: Box<dyn Emitter>,
    pub init_pass: bool,
    pub context_name: String
}
impl CodeGenerator {
    pub fn new(emitter: Box<dyn Emitter>) -> Self {
        Self {
            emitter,
            init_pass: true,
            context_name: "".to_string()
        }
    }
    pub fn generate(&mut self, whamm: &mut Whamm) -> bool {
        self.visit_whamm(whamm)
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
impl WhammVisitorMut<bool> for CodeGenerator {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> bool {
        trace!("Entering: CodeGenerator::visit_whamm");
        self.context_name  = "whamm".to_string();
        let mut is_success = self.emitter.emit_whamm(whamm);

        // visit fns
        whamm.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // DO NOT inject globals (used by compiler)
        // inject whammys
        whamm.whammys.iter_mut().for_each(|whammy| {
            is_success &= self.visit_whammy(whammy);
        });

        trace!("Exiting: CodeGenerator::visit_whamm");
        // Remove from `context_name`
        self.context_name = "".to_string();
        is_success
    }

    fn visit_whammy(&mut self, whammy: &mut Whammy) -> bool {
        trace!("Entering: CodeGenerator::visit_whammy");
        self.emitter.enter_scope();
        self.context_name += &format!(":{}", whammy.name.clone());
        let mut is_success = self.emitter.emit_whammy(whammy);

        if self.init_pass {
            // visit fns
            whammy.fns.iter_mut().for_each(| f | {
                is_success &= self.visit_fn(f);
            });
            // inject globals
            is_success &= self.visit_globals(&whammy.globals);
            // init visit of providers
            whammy.providers.iter_mut().for_each(|(_name, provider)| {
                is_success &= self.visit_provider(provider);
            });
        }

        // We've finished the initial pass (for injecting fns/globals).
        // Now revisit the providers to actually inject probe definitions.
        // This structure is necessary since:
        // 1. We need to have the fns/globals injected (a single time)
        //    and ready to use in every body/predicate
        // 2. We need the base provider:module:function context of a probe
        //    definition to know *how* to inject it
        // Performing a 2-phase visit provides these necessary properties.
        self.init_pass = false;

        // Second visit of providers to actually inject the probe definition.
        self.emitter.reset_children();
        self.emitter.enter_named_scope(&self.context_name);
        whammy.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_whammy");
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

        if self.init_pass {
            // visit fns
            provider.fns.iter_mut().for_each(| f | {
                is_success &= self.visit_fn(f);
            });
            // DO NOT inject globals (used by compiler)
        } else {
            // emit this provider (sets up provider context in the emitter)
            is_success &= self.emitter.emit_provider(&self.context_name, provider);

        }

        // visit the modules
        provider.modules.iter_mut().for_each(|(_name, module)| {
            is_success &= self.visit_module(module);
        });

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

        if self.init_pass {
            // visit fns
            module.fns.iter_mut().for_each(| f | {
                is_success &= self.visit_fn(f);
            });
            // DO NOT inject globals (used by compiler)
        } else {
            // emit this module (sets up module context in the emitter)
            is_success &= self.emitter.emit_module(&self.context_name, module);
        }

        // visit the functions
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

        if self.init_pass {
            // visit fns
            function.fns.iter_mut().for_each(| f | {
                is_success &= self.visit_fn(f);
            });
            // DO NOT inject globals (used by compiler)
        } else {
            // emit this function (sets up function context in the emitter)
            is_success &= self.emitter.emit_function(&self.context_name, function);
        }

        // 1. visit the BEFORE probes
        if let Some(probes) = function.probe_map.get_mut(&"before".to_string()) {
            probes.iter_mut().for_each(|probe| {
                // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
                is_success &= self.visit_probe(probe);
            });
        }
        // 2. visit the ALT probes
        if let Some(probes) = function.probe_map.get_mut(&"alt".to_string()) {
            // only will emit one alt probe!
            // The last alt probe in the list will be emitted.
            if probes.len() > 1 {
                warn!("Detected multiple `alt` probes, will only emit the last one and ignore the rest!")
            }
            if let Some(probe) = probes.last_mut() {
                is_success &= self.visit_probe(probe);
            }
        }
        // 3. visit the AFTER probes
        if let Some(probes) = function.probe_map.get_mut(&"after".to_string()) {
            probes.iter_mut().for_each(|probe| {
                // Assumption: after probes push/pop from stack so it is equivalent to what it was originally
                is_success &= self.visit_probe(probe);
            });
        }

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

        if self.init_pass {
            // visit fns
            probe.fns.iter_mut().for_each(| f | {
                is_success &= self.visit_fn(f);
            });
            // DO NOT inject globals (used by compiler)
        } else {
            if probe.body.is_none() {
                // No need to emit the probe...there's no body!
                return true;
            }
            // probe has a body, continue to emit logic!

            // Logic to emit a probe is unfortunately very injection-strategy-specific
            // Problem: Emitter keeps track of all locations in application code to instrument.
            //    EITHER I iterate over it here (and we're tying ourselves to a loop in the generator...injection-strategy-specific).
            //    OR I iterate over it in the emitter, but then I can't put the AST traversal in the generator code.
            //    This is because I need to make make decisions on how to emit based on the current instruction I'm "visiting"
            self.emitter.emit_probe(&self.context_name, probe);
        }

        trace!("Exiting: CodeGenerator::visit_probe");
        self.emitter.exit_scope();
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    // fn visit_predicate(&mut self, predicate: &mut Expr) -> bool {
    //     todo!()
    // }

    // fn visit_predicate(&mut self, predicate: &mut Expr) -> bool {
    //
    // }

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