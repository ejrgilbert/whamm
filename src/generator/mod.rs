use crate::emitter::module_emitter::ModuleEmitter;
use crate::lang_features::report_vars::{BytecodeLoc, LocationData};
use crate::parser::provider_handler::{BoundFunc, Event, ModeKind, Package, Probe, Provider};
use crate::parser::types::{
    Block, BoundFunction, DataType, Definition, Expr, Fn, FnId, Global, Location, Script,
    Statement, Value, Whamm, WhammVisitorMut,
};
use crate::verifier::types::Record;
use itertools::Itertools;
use log::{debug, warn};
use std::collections::{HashMap, HashSet};
use wirm::ir::id::FunctionID;

pub mod folding;
pub mod rewriting;
pub mod wei;

pub mod analysis_visitor;
pub mod ast;
pub mod metadata_collector;
#[cfg(test)]
pub mod tests;

fn create_curr_loc(curr_script_id: u8, probe: &ast::Probe, wei: bool) -> LocationData {
    let probe_id = probe.to_string(wei);

    //set the current location in bytecode and load some new globals for potential report vars
    LocationData::Local {
        script_id: curr_script_id,
        bytecode_loc: BytecodeLoc::new(0, 0), // TODO -- request this from engine
        probe_id,
    }
}
fn emit_needed_funcs(
    funcs: HashSet<(String, String)>,
    emitter: &mut ModuleEmitter,
    injected_funcs: &mut Vec<FunctionID>,
) {
    // Sort so that dependencies are emitted before dependents.
    // strcmp must come before strcontains since strcontains calls strcmp.
    let mut funcs: Vec<_> = funcs.into_iter().collect();
    funcs.sort_by_key(|(_, name)| match name.as_str() {
        "strcmp" => 0,
        "strcontains" => 1,
        _ => 2,
    });
    for (context, fname) in funcs.iter() {
        if let Some(fid) = emitter.emit_bound_fn(
            context,
            &Fn {
                runnable_in_report_decl_init: true,
                def: Definition::CompilerDynamic,
                name: FnId {
                    name: fname.clone(),
                    loc: None,
                },
                params: vec![],
                results: DataType::Boolean,
                body: Default::default(),
            },
        ) {
            injected_funcs.push(fid);
        };
    }
}

pub trait GeneratingVisitor: WhammVisitorMut<bool> {
    fn add_internal_error(&mut self, message: &str, loc: &Option<Location>);
    fn emit_string(&mut self, val: &mut Value) -> bool;
    fn emit_func(&mut self, f: &mut Fn) -> Option<FunctionID>;
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
    fn link_user_lib(&mut self, lib_name: &str, loc: &Option<Location>);
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
    fn lookup_var_mut(&mut self, name: &str) -> Option<&mut Record>;
    fn visit_stmts(&mut self, stmts: &mut [Statement]) -> bool {
        let mut is_success = true;
        stmts.iter_mut().for_each(|stmt| {
            is_success &= self.visit_stmt(stmt);
        });
        is_success
    }
    fn handle_lib_imports(&mut self, stmts: &mut [Statement]) {
        for stmt in stmts.iter_mut() {
            if let Statement::LibImport { lib_name, loc, .. } = stmt {
                self.link_user_lib(lib_name, loc);
            }
        }
    }
    fn visit_global_stmts(&mut self, stmts: &mut [Statement]) -> bool;
    fn visit_globals(&mut self, globals: &HashMap<String, Global>) -> bool {
        let is_success = true;
        let sorted_globals = globals.iter().sorted_by_key(|data| data.0);
        for (name, global) in sorted_globals.into_iter() {
            // do not inject globals into Wasm that are used/defined by the compiler
            if global.is_from_user() {
                if global.report {
                    //emit global and add the metadata to the report_vars
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

    fn visit_before_probes(&mut self, event: &mut Event) -> bool {
        let mut is_success = true;
        if let Some(probes) = event.probes.get_mut(&ModeKind::Before) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }
        is_success
    }

    fn visit_alt_probes(&mut self, event: &mut Event) -> bool {
        let mut is_success = true;
        if let Some(probes) = event.probes.get_mut(&ModeKind::Alt) {
            // only will emit one alt probe!
            // The last alt probe in the list will be emitted.
            if probes.len() > 1 {
                warn!("Detected multiple `alt` probes, will only emit the last one and ignore the rest!")
            }
            if let Some(probe) = probes.last_mut() {
                is_success &= self.visit_probe(probe);
            }
        }
        is_success
    }

    fn visit_after_probes(&mut self, event: &mut Event) -> bool {
        let mut is_success = true;
        if let Some(probes) = event.probes.get_mut(&ModeKind::After) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }

        is_success
    }
}

/// A get-for-free implementation of the GeneratingVisitor
impl<T: GeneratingVisitor> WhammVisitorMut<bool> for T {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> bool {
        self.set_context_name("whamm".to_string());
        let mut is_success = true;
        // visit fns
        whamm
            .fns
            .iter_mut()
            .for_each(|BoundFunction { function, .. }| {
                is_success &= self.visit_fn(function);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit scripts
        whamm.scripts.iter_mut().for_each(|script| {
            is_success &= self.visit_script(script);
        });

        // Remove from `context_name`
        self.set_context_name("".to_string());
        is_success
    }

    fn visit_script(&mut self, script: &mut Script) -> bool {
        self.set_curr_loc(LocationData::Global {
            script_id: script.id,
        });
        self.enter_scope();
        self.append_context_name(format!(":script{}", script.id));
        let mut is_success = true;

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&script.globals);
        // visit global statements
        is_success &= self.visit_global_stmts(&mut script.global_stmts);
        // visit providers
        script.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        self.exit_scope();
        // Remove from `context_name`
        self.remove_last_context();
        is_success
    }

    fn visit_provider(&mut self, provider: &mut Provider) -> bool {
        self.enter_scope();
        self.append_context_name(format!(":{}", provider.def.name));
        let mut is_success = true;

        // visit fns
        provider
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, .. }| {
                is_success &= self.visit_fn(func);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the packages
        provider.packages.values_mut().for_each(|package| {
            is_success &= self.visit_package(package);
        });

        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_package(&mut self, package: &mut Package) -> bool {
        self.enter_scope();
        self.append_context_name(format!(":{}", package.def.name));
        let mut is_success = true;

        // visit fns
        package
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, .. }| {
                is_success &= self.visit_fn(func);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit the events
        package.events.values_mut().for_each(|event| {
            is_success &= self.visit_event(event);
        });

        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_event(&mut self, event: &mut Event) -> bool {
        self.enter_scope();
        self.append_context_name(format!(":{}", event.def.name));
        let mut is_success = true;

        // visit fns
        event
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, .. }| {
                is_success &= self.visit_fn(func);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // (this is where wei on wizard implementation starts diverting)
        // 1. visit the BEFORE probes
        self.visit_before_probes(event);
        // 2. visit the ALT probes
        self.visit_alt_probes(event);
        // 3. visit the AFTER probes
        self.visit_after_probes(event);

        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_probe(&mut self, probe: &mut Probe) -> bool {
        self.enter_scope();
        self.append_context_name(format!(":{}", probe.kind.name()));
        let mut is_success = true;

        // visit fns
        probe
            .def
            .bound_fns
            .iter_mut()
            .for_each(|BoundFunc { func, .. }| {
                is_success &= self.visit_fn(func);
            });
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // visit probe predicate/body to get Strings into the Wasm module
        // NOTE -- this means that we are greedy on the String emitting we need
        // for instrumentation to work. This is because lots of Strings will not
        // be needed dynamically (e.g. target_fn_name == "call_new" would be
        // constant-propagated away). Maybe there's a way to avoid this?
        // We could deal with this optimization in the future.
        if let Some(pred) = &mut probe.predicate {
            is_success &= self.visit_expr(pred);
        }
        if let Some(body) = &mut probe.body {
            is_success &= self.visit_stmts(body.stmts.as_mut_slice());
        }

        self.exit_scope();
        self.remove_last_context();
        is_success
    }

    fn visit_fn(&mut self, f: &mut Fn) -> bool {
        self.enter_scope();
        let mut is_success = true;
        if f.def == Definition::CompilerDynamic {
            // skip, already handled
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
            self.emit_func(f);
        }
        self.exit_scope();
        is_success
    }

    fn visit_formal_param(&mut self, _param: &mut (Expr, DataType)) -> bool {
        unreachable!();
    }

    fn visit_block(&mut self, block: &mut Block) -> bool {
        self.visit_stmts(&mut block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &mut Statement) -> bool {
        match stmt {
            Statement::VarDecl { init, .. } => {
                // The declaration itself won't have a string; only visit init if present.
                if let Some(init_expr) = init {
                    self.visit_expr(init_expr)
                } else {
                    true
                }
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
            Statement::SetMap { key, val, .. } => {
                let mut is_success = true;
                is_success &= self.visit_expr(key);
                is_success &= self.visit_expr(val);

                is_success
            }
            _ => {
                self.add_internal_error(&format!("Should already be handled: {stmt:?}"), &None);
                false
            }
        }
    }

    fn visit_stmt_global(&mut self, stmt: &mut Statement) -> bool {
        match stmt {
            Statement::LibImport { lib_name, loc, .. } => {
                self.link_user_lib(lib_name, loc);
                true
            }
            _ => self.visit_stmt(stmt),
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
            Expr::ObjCall { call, .. } => self.visit_expr(call),
            Expr::Call { args, .. } => {
                let mut is_success = true;
                args.iter_mut().for_each(|arg| {
                    is_success &= self.visit_expr(arg);
                });
                is_success
            }
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::VarId {
                definition, name, ..
            } => {
                if matches!(definition, Definition::CompilerStatic) {
                    // (Hacky to fix the borrow issues with rust)
                    let mut val = {
                        if let Some(Record::Var {
                            value: Some(val), ..
                        }) = self.lookup_var_mut(name)
                        {
                            val.clone()
                        } else {
                            // ignore, nothing to emit here
                            return true;
                        }
                    };

                    if let Value::Str { .. } = val {
                        self.emit_string(&mut val);
                    }

                    // look back up to overwrite the value!
                    let Some(Record::Var {
                        value: Some(old_val),
                        ..
                    }) = self.lookup_var_mut(name)
                    else {
                        panic!("Unable to find the definition for a static compiler variable!")
                    };
                    *old_val = val;
                    true
                } else {
                    // ignore, will not have a string to emit
                    true
                }
            }
            Expr::MapGet { key, .. } => self.visit_expr(key),
        }
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
            Value::Number { .. } | Value::Boolean { .. } | Value::U32U32Map { .. } => {
                // ignore, will not have a string to emit
                true
            }
            Value::NumericLiteral { .. } => {
                unreachable!(
                    "NumericLiteral must be resolved by the type checker before code generation"
                )
            }
        }
    }
}
