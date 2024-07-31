// =======================
// ==== CodeGenerator ====
// =======================

use crate::common::error::ErrorGen;
use crate::emitter::rewriting::module_emitter::ModuleEmitter;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Fn, Global, ProvidedFunction, Script, Statement,
    UnOp, Value, Whamm, WhammVisitorMut,
};
use log::{trace, warn};
use orca::FunctionBuilder;
use std::collections::HashMap;

/// Serves as the first phase of instrumenting a module by setting up
/// the groundwork.
///
/// The code generator traverses the AST and calls the passed emitter to
/// emit some compiler-provided functions and user-defined globals.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the Emitter field.
pub struct InitGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f>,
    pub context_name: String,
    pub err: &'g mut ErrorGen,
}
impl InitGenerator<'_, '_, '_, '_, '_, '_, '_> {
    pub fn run(&mut self, whamm: &mut Whamm) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_children();
        self.on_startup();
        // Generate globals and fns defined by `whamm` (this should modify the app_wasm)
        self.visit_whamm(whamm)
    }

    // Private helper functions
    fn visit_globals(&mut self, globals: &HashMap<String, Global>, script_name: &String) -> bool {
        let mut is_success = true;
        for (name, global) in globals.iter() {
            // do not inject globals into Wasm that are used/defined by the compiler
            if global.is_from_user() {
                if global.report {
                    //emit global and add the metadata to the report_var_metadata
                    match self.emitter.emit_report_global(
                        name.clone(),
                        global.ty.clone(),
                        &global.value,
                        script_name.clone(),
                    ) {
                        Err(e) => self.err.add_error(*e),
                        Ok(res) => is_success &= res,
                    }
                } else {
                    match self
                        .emitter
                        .emit_global(name.clone(), global.ty.clone(), &global.value)
                    {
                        Err(e) => self.err.add_error(*e),
                        Ok(res) => is_success &= res,
                    }
                }
            }
        }

        is_success
    }

    fn visit_stmts(&mut self, stmts: &mut [Statement]) -> bool {
        let mut is_success = true;
        stmts.iter_mut().for_each(|stmt| {
            is_success &= self.visit_stmt(stmt);
        });
        is_success
    }
    fn lib_fn_set(&mut self) {
        //add library functions to the symbol table - skips if not in the wasm module
        let lib_map_fns = [
            "create_i32_i32".to_string(),
            "create_i32_bool".to_string(),
            "create_i32_string".to_string(),
            "create_i32_tuple".to_string(),
            "create_i32_map".to_string(),
            "create_string_i32".to_string(),
            "create_string_bool".to_string(),
            "create_string_string".to_string(),
            "create_string_tuple".to_string(),
            "create_string_map".to_string(),
            "create_bool_i32".to_string(),
            "create_bool_bool".to_string(),
            "create_bool_string".to_string(),
            "create_bool_tuple".to_string(),
            "create_bool_map".to_string(),
            "create_tuple_i32".to_string(),
            "create_tuple_bool".to_string(),
            "create_tuple_string".to_string(),
            "create_tuple_tuple".to_string(),
            "create_tuple_map".to_string(),
            "insert_i32_i32".to_string(),
            "insert_map_i32i32i32tuple_i32".to_string(),
            "get_i32_i32".to_string(),
            "get_i32_from_i32i32i32tuple".to_string(),
            "add_report_map".to_string(),
            "output_report_maps".to_string(),
            "print_info".to_string(),
            "print_map".to_string(),
            "insert_i32_string".to_string(),
            "get_string_from_i32string".to_string(),
            "print_meta".to_string(),
        ];
        for lib_fn in lib_map_fns.iter() {
            let id_option = self.emitter.app_wasm.get_fid_by_name(lib_fn);
            let id = match id_option {
                Some(id_option) => id_option,
                None => std::u32::MAX,
            };
            match self.emitter.table.get_curr_scope_mut() {
                Some(scope) => {
                    if id != std::u32::MAX {
                        scope.put(lib_fn.clone(), id as usize);
                    }
                }
                _ => {
                    self.err.add_error(ErrorGen::get_unexpected_error(
                        true,
                        Some("No scope found in Visit Whamm".to_string()),
                        None,
                    ));
                }
            }
        }
    }
    fn on_startup(&mut self) {
        self.lib_fn_set();
        self.create_start();
    }
    fn create_start(&mut self) {
        match self.emitter.app_wasm.start {
            Some(_) => {
                println!("Start function already exists");
            }
            None => {
                //time to make a start fn
                println!("No start function found, creating one");
                match self.emitter.app_wasm.get_fid_by_name("_start") {
                    Some(_) => {
                        println!("start function is _start");
                    }
                    None => {
                        let start_fn = FunctionBuilder::new(&vec![], &vec![]);
                        let start_id = start_fn.finish(self.emitter.app_wasm);
                        self.emitter.app_wasm.start = Some(start_id);
                        self.emitter.app_wasm.set_fn_name(
                            start_id - self.emitter.app_wasm.num_import_func(),
                            "_start",
                        );
                    } //strcmp doesn't need to call add_export_fn so this probably doesnt either
                      //in app_wasm, not sure if need to have it in the ST
                }
            }
        }
    }
}
impl WhammVisitorMut<bool> for InitGenerator<'_, '_, '_, '_, '_, '_, '_> {
    fn visit_whamm(&mut self, whamm: &mut Whamm) -> bool {
        trace!("Entering: CodeGenerator::visit_whamm");
        self.context_name = "whamm".to_string();
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
        self.context_name = "".to_string();
        is_success
    }

    fn visit_script(&mut self, script: &mut Script) -> bool {
        trace!("Entering: CodeGenerator::visit_script");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        self.context_name += &format!(":{}", script.name.clone());
        let mut is_success = true;

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            is_success &= self.visit_fn(f);
        });
        // inject globals
        is_success &= self.visit_globals(&script.globals, &script.name);
        // visit providers
        script.providers.iter_mut().for_each(|(_name, provider)| {
            is_success &= self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        is_success
    }

    fn visit_provider(&mut self, provider: &mut Box<dyn Provider>) -> bool {
        trace!("Entering: CodeGenerator::visit_provider");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        self.context_name += &format!(":{}", provider.name());
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
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        is_success
    }

    fn visit_package(&mut self, package: &mut dyn Package) -> bool {
        trace!("Entering: CodeGenerator::visit_package");
        if let Err(e) = self.emitter.enter_scope() {
            self.err.add_error(*e)
        }
        let mut is_success = true;
        self.context_name += &format!(":{}", package.name());

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
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        is_success
    }

    fn visit_event(&mut self, event: &mut dyn Event) -> bool {
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
                is_success &= self.visit_fn(function);
            },
        );
        // do not inject globals into Wasm that are used/defined by the compiler
        // because they are statically-defined and folded away

        // 1. visit the BEFORE probes
        if let Some(probes) = event.probes_mut().get_mut(&"before".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }
        // 2. visit the ALT probes
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
        // 3. visit the AFTER probes
        if let Some(probes) = event.probes_mut().get_mut(&"after".to_string()) {
            probes.iter_mut().for_each(|probe| {
                is_success &= self.visit_probe(probe);
            });
        }

        trace!("Exiting: CodeGenerator::visit_event");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this event from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        is_success
    }

    fn visit_probe(&mut self, probe: &mut Box<dyn Probe>) -> bool {
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
            is_success &= self.visit_stmts(body);
        }

        trace!("Exiting: CodeGenerator::visit_probe");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
        is_success
    }

    fn visit_fn(&mut self, f: &mut Fn) -> bool {
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
            is_success &= self.visit_block(&mut f.body);
        }
        trace!("Exiting: CodeGenerator::visit_fn");
        if let Err(e) = self.emitter.exit_scope() {
            self.err.add_error(*e)
        }
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
                if let Some(args) = args {
                    let mut is_success = true;
                    args.iter_mut().for_each(|arg| {
                        is_success &= self.visit_expr(arg);
                    });
                    is_success
                } else {
                    // ignore, no arguments
                    true
                }
            }
            Expr::Primitive { val, .. } => self.visit_value(val),
            Expr::VarId { .. } => {
                // ignore, will not have a string to emit
                true
            }
            Expr::GetMap { map, key, .. } => {
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
                if let Err(e) = self.emitter.emit_string(val) {
                    self.err.add_error(*e)
                }
                true
            }
            Value::Tuple { vals, .. } => {
                let mut is_success = true;
                vals.iter_mut().for_each(|arg| {
                    is_success &= self.visit_expr(arg);
                });
                is_success
            }
            Value::Integer { .. } | Value::Boolean { .. } => {
                // ignore, will not have a string to emit
                true
            }
        }
    }
}
