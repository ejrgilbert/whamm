// =======================
// ==== CodeGenerator ====
// =======================

use crate::common::error::ErrorGen;
use crate::emitter::rewriting::Emitter;
use crate::emitter::wizard::module_emitter::ModuleEmitter;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Fn, Global, ProvidedFunction, Script, Statement,
    UnOp, Value, Whamm, WhammVisitorMut,
};
use log::trace;
use orca::{FunctionBuilder, Opcode};
use std::collections::HashMap;

/// Old doc from rewriting emitter
/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd>,
    pub context_name: String,
    pub err: &'e mut ErrorGen,
}
impl InitGenerator<'_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_children();
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        self.visit_whamm(whamm).0
    }

    // Private helper functions
    fn visit_globals(&mut self, globals: &HashMap<String, Global>) -> bool {
        let mut is_success = true;
        for (name, global) in globals.iter() {
            // do not inject globals into Wasm that are used/defined by the compiler
            if global.is_from_user() {
                match self
                    .emitter
                    .emit_global(name.clone(), global.ty.clone(), &global.value)
                {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        }

        is_success
    }

    fn visit_stmts(&mut self, stmts: &mut [Statement]) -> (bool, Option<u32>) {
        let mut is_success = true;
        stmts.iter_mut().for_each(|stmt| {
            is_success &= self.visit_stmt(stmt).0;
        });
        (is_success, None)
    }
}

// Note: this is kinda crappy, but I want visit_probe to return a function index
impl WhammVisitorMut<(bool, Option<u32>)> for InitGenerator<'_, '_, '_, '_, '_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_whamm");
        self.context_name = "whamm".to_string();
        let mut is_success = true;

        // visit fns
        // Note: imported functions, like `puts` and `puti` must come before local functions
        whamm
            .fns
            .iter_mut()
            .for_each(|ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function).0;
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit scripts
        whamm.scripts.iter_mut().for_each(|script| {
            is_success &= self.visit_script(script).0;
        });

        trace!("Exiting: CodeGenerator::visit_whamm");
        // Remove from `context_name`
        self.context_name = "".to_string();
        (is_success, None)
    }

    fn visit_script(&mut self, script: &mut Script) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_script");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        self.context_name += &format!(":{}", script.name.clone());
        let mut is_success = true;

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            is_success &= self.visit_fn(f).0;
        });
        // inject globals
        is_success &= self.visit_globals(&script.globals);
        // visit providers
        script.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider).0;
        });

        trace!("Exiting: CodeGenerator::visit_script");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        (is_success, None)
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_provider");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        self.context_name += &format!(":{}", provider.name());
        let mut is_success = true;

        // visit fns
        provider.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function).0;
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the packages
        provider.packages_mut().for_each(|package| {
            is_success &= self.visit_package(package).0;
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        (is_success, None)
    }

    fn visit_package(&mut self, package: &mut dyn Package) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_package");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        let mut is_success = true;
        self.context_name += &format!(":{}", package.name());

        // visit fns
        package.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function).0;
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the events
        package.events_mut().for_each(|event| {
            is_success &= self.visit_event(event).0;
        });

        trace!("Exiting: CodeGenerator::visit_package");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        (is_success, None)
    }

    fn visit_event(&mut self, event: &mut dyn Event) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_event");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        // let mut is_success = self.emitter.emit_event(event);
        self.context_name += &format!(":{}", event.name());
        let mut is_success = true;

        // visit fns
        event.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function).0;
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // At this point, each event will corresopnd to some actions function
        // skip the whamm context and script index
        let mnemonic = self
            .context_name
            .clone()
            .split(':')
            .skip(2)
            .collect::<Vec<&str>>()
            .join(":");
        let mut action = FunctionBuilder::new(&[], &[]);

        // build a new emitter for the event
        // self.emitter.app_wasm.add_functi

        // TODO: BEFORE and AFTER probe has the same logic
        // there must be a way to rewrite this
        // 1. visit the BEFORE probes
        if let Some(probes) = event.probes_mut().get_mut(&"before".to_string()) {
            // for each probe, it will be a new function
            let probe_action = FunctionBuilder::new(&[], &[]);
            // set the emitting func
            self.emitter.emitting_func = Some(probe_action);
            probes.iter_mut().for_each(|probe| {
                let res = self.visit_probe(probe);
                is_success &= res.0;
                action.call(res.1.unwrap());
            });
            // the emitting function is resetted in visit_probe (see `self.emitter.emitting_func.take()``)
        }
        // 2. visit the ALT probes
        if event.probes_mut().get_mut(&"alt".to_string()).is_some() {
            panic!("Alt probes should not exist in the wizard backend!");
        }
        // 3. visit the AFTER probes
        if let Some(probes) = event.probes_mut().get_mut(&"after".to_string()) {
            // for each probe, it will be a new function
            let probe_action = FunctionBuilder::new(&[], &[]);
            // set the emitting func
            self.emitter.emitting_func = Some(probe_action);
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe).0;
            });
            // the emitting function is resetted in visit_probe (see `self.emitter.emitting_func.take()``)
        }

        trace!("Exiting: CodeGenerator::visit_event");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this event from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();

        // export the event function
        let action_id = action.finish_module(self.emitter.app_wasm);
        self.emitter.export_mnemonic(mnemonic, action_id);

        (is_success, None)
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_probe");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        // let mut is_success = self.emitter.emit_probe(probe);
        self.context_name += &format!(":{}", probe.mode_name());
        let mut is_success = true;

        // visit fns
        probe.get_mode_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function).0;
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit probe predicate/body to get Strings into the Wasm module
        // NOTE -- this means that we are greedy on the String emitting we need
        // for instrumentation to work. This is because lots of Strings will not
        // be needed dynamically (e.g. target_fn_name == "call_new" would be
        // constant-propagated away). Maybe there's a way to avoid this?
        // We could deal with this optimization in the future.
        if let Some(pred) = probe.predicate_mut() {
            is_success &= self.visit_expr(pred).0;
        }
        if let Some(body) = probe.body_mut() {
            is_success &= self.visit_stmts(body.stmts.as_mut_slice()).0;
        }

        trace!("Exiting: CodeGenerator::visit_probe");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }

        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();

        // after visiting the statements, we finished building the function
        if let Some(func) = self.emitter.emitting_func.take() {
            let fid = func.finish_module(self.emitter.app_wasm);
            // TODO: can we set a good name for this function
            (is_success, Some(fid))
        } else {
            panic!("No emitting function was set!");
        }
    }

    fn visit_fn(&mut self, f: &mut Fn) -> (bool, Option<u32>) {
        trace!("Entering: CodeGenerator::visit_fn");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        let mut is_success = true;
        if f.def == Definition::CompilerDynamic {
            // Only emit the functions that will be used dynamically!
            match self.emitter.emit_fn(&self.context_name, f) {
                Err(e) => self.err.add_error(*e),
                Ok(res) => is_success = res,
            }
        } else {
            // user provided function, visit the body to ensure
            // String values are added to the Wasm data section!

            // READ ME WHEN IMPLEMENTING THE LOGIC TO EMIT USER-PROVIDED FUNCTIONS
            // Currently we only visit the bodies of fns/probes to emit Strings into
            // the data section. This means that both fn and probe bodies are handled the
            // same way in this visitor.
            // HOWEVER, when actually visiting user-defined function bodies, we WILL WANT
            // to emit those functions into the Wasm. When visiting probe bodies, we will
            // NOT want to emit any code into the Wasm...ONLY emit Strings into data sections.
            // Make sure this is remembered when emitting functions!!
            is_success &= self.visit_block(&mut f.body).0;
        }
        trace!("Exiting: CodeGenerator::visit_fn");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        (is_success, None)
    }

    fn visit_formal_param(&mut self, _param: &mut (Expr, DataType)) -> (bool, Option<u32>) {
        // never called
        unreachable!();
        // trace!("Entering: CodeGenerator::visit_formal_param");
        // let is_success = self.emitter.emit_formal_param(param);
        // trace!("Exiting: CodeGenerator::visit_formal_param");
        // is_success
    }

    fn visit_block(&mut self, block: &mut Block) -> (bool, Option<u32>) {
        self.visit_stmts(&mut block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> (bool, Option<u32>) {
        // also emit the statment
        if let Err(e) = self.emitter.emit_stmt(stmt) {
            self.err.add_error(*e)
        }
        match stmt {
            Statement::Decl { .. } => {
                // ignore, this stmt type will not have a string in it!
                (true, None)
            }
            Statement::Assign { expr, .. }
            | Statement::Expr { expr, .. }
            | Statement::Return { expr, .. } => self.visit_expr(expr),
            Statement::If {
                cond, conseq, alt, ..
            } => {
                let mut is_success = true;
                is_success &= self.visit_expr(cond).0;
                is_success &= self.visit_block(conseq).0;
                is_success &= self.visit_block(alt).0;

                (is_success, None)
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> (bool, Option<u32>) {
        match expr {
            Expr::UnOp { expr, .. } => self.visit_expr(expr),
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let mut is_success = true;
                is_success &= self.visit_expr(cond).0;
                is_success &= self.visit_expr(conseq).0;
                is_success &= self.visit_expr(alt).0;

                (is_success, None)
            }
            Expr::BinOp { lhs, rhs, .. } => {
                let mut is_success = true;
                is_success &= self.visit_expr(lhs).0;
                is_success &= self.visit_expr(rhs).0;

                (is_success, None)
            }
            Expr::Call { args, .. } => {
                if let Some(args) = args {
                    let mut is_success = true;
                    args.iter_mut().for_each(|arg| {
                        is_success &= self.visit_expr(arg).0;
                    });
                    (is_success, None)
                } else {
                    // ignore, no arguments
                    (true, None)
                }
            }
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::VarId { .. } => {
                // ignore, will not have a string to emit
                (true, None)
            }
        }
    }

    fn visit_unop(&mut self, _unop: &mut UnOp) -> (bool, Option<u32>) {
        // never called
        unreachable!();
    }

    fn visit_binop(&mut self, _binop: &mut BinOp) -> (bool, Option<u32>) {
        // never called
        unreachable!();
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) -> (bool, Option<u32>) {
        // never called
        unreachable!();
    }

    fn visit_value(&mut self, val: &mut Value) -> (bool, Option<u32>) {
        match val {
            Value::Str { .. } => {
                // Emit the string into the Wasm module data section!
                if let Err(e) = self.emitter.emit_string(val) {
                    self.err.add_error(*e)
                }
                (true, None)
            }
            Value::Tuple { vals, .. } => {
                let mut is_success = true;
                vals.iter_mut().for_each(|arg| {
                    is_success &= self.visit_expr(arg).0;
                });
                (is_success, None)
            }
            Value::Integer { .. } | Value::Boolean { .. } => {
                // ignore, will not have a string to emit
                (true, None)
            }
        }
    }
}
