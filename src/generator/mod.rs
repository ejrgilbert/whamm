use crate::emitter::report_var_metadata::LocationData;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Fn, Global, ProvidedFunction, Script, Statement,
    UnOp, Value, Whamm, WhammVisitorMut,
};
use log::{debug, trace, warn};
use orca_wasm::ir::id::FunctionID;
use std::collections::HashMap;

pub mod folding;
pub mod rewriting;
pub mod wizard;

#[cfg(test)]
pub mod tests;

pub trait GeneratingVisitor: WhammVisitorMut<bool> {
    fn emit_string(&mut self, val: &mut Value) -> bool;
    fn emit_fn(&mut self, context: &str, f: &Fn) -> Option<FunctionID>;
    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID>;
    fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID>;
    fn add_injected_func(&mut self, fid: FunctionID);
    fn get_context_name_mut(&mut self) -> &mut String;
    fn get_context_name(&self) -> &String;
    fn set_context_name(&mut self, val: String) {
        *self.get_context_name_mut() = val;
    }
    fn append_context_name(&mut self, val: String) {
        *self.get_context_name_mut() += &val;
    }
    fn remove_last_context(&mut self) {
        let context_name = self.get_context_name();
        self.set_context_name(context_name[..context_name.rfind(':').unwrap()].to_string());
    }
    fn set_curr_loc(&mut self, loc: LocationData);
    fn enter_named_scope(&mut self, name: &str);
    fn enter_scope(&mut self);
    fn exit_scope(&mut self);
    fn visit_stmts(&mut self, stmts: &mut [Statement]) -> bool {
        let mut is_success = true;
        stmts.iter_mut().for_each(|stmt| {
            is_success &= self.visit_stmt(stmt);
        });
        is_success
    }
    fn visit_globals(&mut self, globals: &HashMap<String, Global>) -> bool {
        let is_success = true;
        for (name, global) in globals.iter() {
            // do not inject globals into Wasm that are used/defined by the compiler
            if global.is_from_user() {
                if global.report {
                    //emit global and add the metadata to the report_var_metadata
                    if let Some(fid) =
                        self.emit_report_global(name.clone(), global.ty.clone(), &global.value)
                    {
                        self.add_injected_func(fid);
                    }
                } else if let Some(fid) =
                    self.emit_global(name.clone(), global.ty.clone(), &global.value)
                {
                    debug!("added global_getter: {:?}", fid);
                    self.add_injected_func(fid);
                }
            }
        }

        is_success
    }

    fn visit_before_probes(&mut self, event: &mut dyn Event) -> bool {
        trace!("Entering: CodeGenerator::visit_before_probes");
        let mut is_success = true;
        if let Some(probes) = event.probes_mut().get_mut(&"before".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }
        trace!("Exiting: CodeGenerator::visit_before_probes");
        is_success
    }

    fn visit_alt_probes(&mut self, event: &mut dyn Event) -> bool {
        trace!("Entering: CodeGenerator::visit_alt_probes");
        let mut is_success = true;
        if let Some(probes) = event.probes_mut().get_mut(&"alt".to_string()) {
            // only will emit one alt probe!
            // The last alt probe in the list will be emitted.
            if probes.len() > 1 {
                warn!("Detected multiple `alt` probes, will only emit the last one and ignore the rest!")
            }
            if let Some(probe) = probes.last_mut() {
                is_success &= self.visit_probe(probe);
            }
        }
        trace!("Exiting: CodeGenerator::visit_alt_probes");
        is_success
    }

    fn visit_after_probes(&mut self, event: &mut dyn Event) -> bool {
        trace!("Entering: CodeGenerator::visit_after_probes");
        let mut is_success = true;
        if let Some(probes) = event.probes_mut().get_mut(&"after".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }

        trace!("Exiting: CodeGenerator::visit_after_probes");
        is_success
    }
}

/// A get-for-free implementation of the GeneratingVisitor
impl<T: GeneratingVisitor> WhammVisitorMut<bool> for T {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> bool {
        trace!("Entering: CodeGenerator::visit_whamm");
        self.set_context_name("whamm".to_string());
        let mut is_success = true;
        // visit fns
        whamm
            .fns
            .iter_mut()
            .for_each(|ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit scripts
        whamm.scripts.iter_mut().for_each(|script| {
            is_success &= self.visit_script(script);
        });

        trace!("Exiting: CodeGenerator::visit_whamm");
        // Remove from `context_name`
        self.set_context_name("".to_string());
        is_success
    }

    fn visit_script(&mut self, script: &mut Script) -> bool {
        trace!("Entering: CodeGenerator::visit_script");
        self.set_curr_loc(LocationData::Global {
            script_id: script.name.clone(),
        });
        self.enter_scope();
        self.append_context_name(format!(":{}", script.name.clone()));
        let mut is_success = true;

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&script.globals);
        // visit providers
        script.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.exit_scope();
        // Remove from `context_name`
        self.remove_last_context();
        is_success
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> bool {
        trace!("Entering: CodeGenerator::visit_provider");
        self.enter_scope();
        self.append_context_name(format!(":{}", provider.name()));
        let mut is_success = true;

        // visit fns
        provider.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function);
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the packages
        provider.packages_mut().for_each(|package| {
            is_success &= self.visit_package(package);
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_package(&mut self, package: &mut dyn Package) -> bool {
        trace!("Entering: CodeGenerator::visit_package");
        self.enter_scope();
        self.append_context_name(format!(":{}", package.name()));
        let mut is_success = true;

        // visit fns
        package.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function);
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the events
        package.events_mut().for_each(|event| {
            is_success &= self.visit_event(event);
        });

        trace!("Exiting: CodeGenerator::visit_package");
        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_event(&mut self, event: &mut dyn Event) -> bool {
        trace!("Entering: CodeGenerator::visit_event");
        self.enter_scope();
        self.append_context_name(format!(":{}", event.name()));
        let mut is_success = true;

        // visit fns
        event.get_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function);
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // TODO -- this is where wizard implementation starts diverting
        // 1. visit the BEFORE probes
        self.visit_before_probes(event);
        // 2. visit the ALT probes
        self.visit_alt_probes(event);
        // 3. visit the AFTER probes
        self.visit_after_probes(event);

        trace!("Exiting: CodeGenerator::visit_event");
        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> bool {
        trace!("Entering: CodeGenerator::visit_probe");
        self.enter_scope();
        self.append_context_name(format!(":{}", probe.mode().name()));
        let mut is_success = true;

        // visit fns
        probe.get_mode_provided_fns_mut().iter_mut().for_each(
            |ProvidedFunction { function, .. }| {
                is_success &= self.visit_fn(function);
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
            is_success &= self.visit_expr(pred);
        }
        if let Some(body) = probe.body_mut() {
            is_success &= self.visit_stmts(body.stmts.as_mut_slice());
        }

        trace!("Exiting: CodeGenerator::visit_probe");
        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_fn(&mut self, f: &mut Fn) -> bool {
        trace!("Entering: CodeGenerator::visit_fn");
        self.enter_scope();
        let mut is_success = true;
        if f.def == Definition::CompilerDynamic {
            // Only emit the functions that will be used dynamically!
            let context_name = self.get_context_name();
            if let Some(res) = self.emit_fn(&context_name.clone(), f) {
                self.add_injected_func(res);
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
            is_success &= self.visit_block(&mut f.body);
        }
        trace!("Exiting: CodeGenerator::visit_fn");
        self.exit_scope();
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

    fn visit_block(&mut self, block: &mut Block) -> bool {
        self.visit_stmts(&mut block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> bool {
        match stmt {
            Statement::Decl { .. } => {
                // ignore, this stmt type will not have a string in it!
                true
            }
            Statement::Assign { expr, .. }
            | Statement::Expr { expr, .. }
            | Statement::Return { expr, .. } => self.visit_expr(expr),
            Statement::If {
                cond, conseq, alt, ..
            } => {
                let mut is_success = true;
                is_success &= self.visit_expr(cond);
                is_success &= self.visit_block(conseq);
                is_success &= self.visit_block(alt);

                is_success
            }
            Statement::ReportDecl { decl, .. } => self.visit_stmt(decl),
            Statement::SetMap { map, key, val, .. } => {
                let mut is_success = true;
                is_success &= self.visit_expr(map);
                is_success &= self.visit_expr(key);
                is_success &= self.visit_expr(val);

                is_success
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> bool {
        match expr {
            Expr::UnOp { expr, .. } => self.visit_expr(expr),
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                let mut is_success = true;
                is_success &= self.visit_expr(cond);
                is_success &= self.visit_expr(conseq);
                is_success &= self.visit_expr(alt);

                is_success
            }
            Expr::BinOp { lhs, rhs, .. } => {
                let mut is_success = true;
                is_success &= self.visit_expr(lhs);
                is_success &= self.visit_expr(rhs);

                is_success
            }
            Expr::Call { args, .. } => {
                let mut is_success = true;
                args.iter_mut().for_each(|arg| {
                    is_success &= self.visit_expr(arg);
                });
                is_success
            }
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::VarId { .. } => {
                // ignore, will not have a string to emit
                true
            }
            Expr::MapGet { map, key, .. } => {
                let mut is_success = true;
                is_success &= self.visit_expr(map);
                is_success &= self.visit_expr(key);

                is_success
            }
        }
    }

    fn visit_unop(&mut self, _unop: &mut UnOp) -> bool {
        // never called
        unreachable!();
    }

    fn visit_binop(&mut self, _binop: &mut BinOp) -> bool {
        // never called
        unreachable!();
    }

    fn visit_datatype(&mut self, _datatype: &mut DataType) -> bool {
        // never called
        unreachable!();
    }

    fn visit_value(&mut self, val: &mut Value) -> bool {
        match val {
            Value::Str { .. } => {
                // Emit the string into the Wasm module data section!
                self.emit_string(val)
            }
            Value::Tuple { vals, .. } => {
                let mut is_success = true;
                vals.iter_mut().for_each(|arg| {
                    is_success &= self.visit_expr(arg);
                });
                is_success
            }
            Value::U32 { .. }
            | Value::I32 { .. }
            | Value::F32 { .. }
            | Value::U64 { .. }
            | Value::I64 { .. }
            | Value::F64 { .. }
            | Value::Boolean { .. }
            | Value::U32U32Map { .. } => {
                // ignore, will not have a string to emit
                true
            }
        }
    }
}