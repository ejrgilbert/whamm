// =======================
// ==== CodeGenerator ====
// =======================

use std::collections::HashMap;
use log::{trace, warn};
use crate::generator::emitters::Emitter;
use crate::parser::types::{DataType, Whammy, Whamm, WhammVisitorMut, Expr, Event, Package, Op, Probe, Provider, Statement, Value, Global};

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a> {
    pub emitter: Box<&'a mut dyn Emitter>,
    pub context_name: String
}
impl InitGenerator<'_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        self.visit_whamm(whamm)
    }

    // Private helper functions
    fn visit_globals(&mut self, globals: &HashMap<String, Global>) -> bool {
        let mut is_success = true;
        for (name, global) in globals.iter() {
            // do not inject globals into Wasm that are used/defined by the compiler
            if !global.is_comp_provided {
                is_success &= self.emitter.emit_global(name.clone(), global.ty.clone(), &global.value);
            }
        }

        is_success
    }
}
impl WhammVisitorMut<bool> for InitGenerator<'_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> bool {
        trace!("Entering: CodeGenerator::visit_whamm");
        self.context_name  = "whamm".to_string();
        let mut is_success = true;

        // visit fns
        whamm.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&whamm.globals);
        // visit whammys
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
        let mut is_success = true;

        // visit fns
        whammy.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&whammy.globals);
        // visit providers
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

        // visit fns
        provider.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&provider.globals);
        // visit the packages
        provider.packages.iter_mut().for_each(|(_name, package)| {
            is_success &= self.visit_package(package);
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        self.emitter.exit_scope();
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_package(&mut self, package: &mut Package) -> bool {
        trace!("Entering: CodeGenerator::visit_package");
        self.emitter.enter_scope();
        let mut is_success = true;
        self.context_name += &format!(":{}", package.name.clone());

        // visit fns
        package.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&package.globals);
        // visit the events
        package.events.iter_mut().for_each(|(_name, event)| {
            is_success &= self.visit_event(event);
        });

        trace!("Exiting: CodeGenerator::visit_package");
        self.emitter.exit_scope();
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_event(&mut self, event: &mut Event) -> bool {
        trace!("Entering: CodeGenerator::visit_event");
        self.emitter.enter_scope();
        // let mut is_success = self.emitter.emit_event(event);
        self.context_name += &format!(":{}", event.name.clone());
        let mut is_success = true;

        // visit fns
        event.fns.iter_mut().for_each(| f | {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&event.globals);

        // 1. visit the BEFORE probes
        if let Some(probes) = event.probe_map.get_mut(&"before".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }
        // 2. visit the ALT probes
        if let Some(probes) = event.probe_map.get_mut(&"alt".to_string()) {
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
        if let Some(probes) = event.probe_map.get_mut(&"after".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }

        trace!("Exiting: CodeGenerator::visit_event");
        self.emitter.exit_scope();
        // Remove this event from `context_name`
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
        // inject globals
        is_success &= self.visit_globals(&probe.globals);

        trace!("Exiting: CodeGenerator::visit_probe");
        self.emitter.exit_scope();
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
        is_success
    }

    fn visit_fn(&mut self, f: &mut crate::parser::types::Fn) -> bool {
        trace!("Entering: CodeGenerator::visit_fn");
        self.emitter.enter_scope();
        let mut is_success = true;
        if f.is_comp_provided {
            is_success = self.emitter.emit_fn(&self.context_name, f);
        }
        trace!("Exiting: CodeGenerator::visit_fn");
        self.emitter.exit_scope();
        is_success
    }

    fn visit_formal_param(&mut self, _param: &mut (Expr, DataType)) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_formal_param");
        // let is_success = self.emitter.emit_formal_param(param);
        // trace!("Exiting: CodeGenerator::visit_formal_param");
        // is_success
    }

    fn visit_stmt(&mut self, _stmt: &mut Statement) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_stmt");
        // let is_success = self.emitter.emit_stmt(stmt);
        // trace!("Exiting: CodeGenerator::visit_stmt");
        // is_success
    }

    fn visit_expr(&mut self, _expr: &mut Expr) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_expr");
        // let is_success = self.emitter.emit_expr(expr);
        // trace!("Exiting: CodeGenerator::visit_expr");
        // is_success
    }

    fn visit_op(&mut self, _op: &mut Op) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_op");
        // let is_success = self.emitter.emit_op(op);
        // trace!("Exiting: CodeGenerator::visit_op");
        // is_success
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_datatype");
        // let is_success = self.emitter.emit_datatype(datatype);
        // trace!("Exiting: CodeGenerator::visit_datatype");
        // is_success
    }

    fn visit_value(&mut self, _val: &mut Value) -> bool {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_value");
        // let is_success = self.emitter.emit_value(val);
        // trace!("Exiting: CodeGenerator::visit_value");
        // is_success
    }
}