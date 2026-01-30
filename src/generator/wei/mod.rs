use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::tag_handler::get_tag_for;
use crate::generator::ast::{Probe, Script, WhammParams};
use crate::generator::{GeneratingVisitor, create_curr_loc, emit_needed_funcs};
use crate::lang_features::alloc_vars::wei::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::report_vars::LocationData;
use crate::parser::types::{Block, DataType, Expr, Location, Statement, Value, WhammVisitorMut};
use crate::verifier::types::Record;
use log::trace;
use std::collections::{HashMap, HashSet};
use wirm::Module;
use wirm::ir::id::{FunctionID, LocalID};
use wirm::ir::types::DataType as WirmType;

pub struct WeiGenerator<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'l, 'm> {
    pub emitter: ModuleEmitter<'a, 'b, 'c, 'd, 'e, 'f, 'g>,
    pub io_adapter: &'h mut IOAdapter,
    pub context_name: String,
    pub err: &'i mut ErrorGen,
    pub injected_funcs: &'j mut Vec<FunctionID>,
    pub config: &'k Config,
    pub used_fns_per_lib: HashMap<String, HashSet<String>>,
    pub user_lib_modules: HashMap<String, (Option<String>, Module<'l>)>,

    // tracking
    pub curr_script_id: u8,
    pub unshared_var_handler: &'m mut UnsharedVarHandler,
}

impl WeiGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    pub fn run(
        &mut self,
        mut ast: Vec<Script>,
        used_bound_funcs: HashSet<(String, String)>,
        used_report_dts: HashSet<DataType>,
        strings_to_emit: Vec<String>,
        has_probe_state_init: bool,
    ) {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_table();
        self.emitter.setup_module(false, has_probe_state_init);
        emit_needed_funcs(used_bound_funcs, &mut self.emitter, self.injected_funcs);
        self.emitter.emit_strings(strings_to_emit);
        self.visit_ast(&mut ast);

        self.emit_end_func(&ast, used_report_dts);
    }

    fn emit_end_func(&mut self, ast: &[Script], used_report_dts: HashSet<DataType>) {
        self.emitter
            .emit_end_fn(ast, used_report_dts, self.io_adapter, self.err);
    }

    // Visit the AST
    fn visit_ast(&mut self, ast: &mut [Script]) {
        for script in ast.iter_mut() {
            self.visit_script(script);
        }
    }

    fn visit_script(&mut self, script: &mut Script) {
        trace!("Entering: CodeGenerator::visit_script");
        self.enter_named_scope(&script.id.to_string());
        self.set_context_name(script.id.to_string());
        self.curr_script_id = script.id;

        self.set_curr_loc(LocationData::Global {
            script_id: script.id,
        });

        // visit fns
        script.fns.iter_mut().for_each(|f| {
            self.visit_fn(f);
        });
        // inject globals
        self.visit_globals(&script.globals);
        // visit global statements
        self.visit_global_stmts(&mut script.global_stmts);
        // visit probes
        script.probes.iter_mut().for_each(|probe| {
            self.visit_probe(probe);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.exit_scope();
    }

    fn visit_probe(&mut self, probe: &mut Probe) {
        self.set_curr_loc(create_curr_loc(self.curr_script_id, probe));

        let (pred_fid, pred_param_str, dynamic_pred) = if let Some(pred) = &mut probe.predicate {
            if probe.metadata.pred_is_dynamic {
                // dynamic analysis of the predicate will go here!
                // See: https://github.com/ejrgilbert/whamm/issues/163

                // for now, we push the dynamic predicate down into the probe body

                (None, "".to_string(), Some(pred))
            } else {
                let mut block = Block {
                    stmts: vec![Statement::Expr {
                        expr: pred.clone(),
                        loc: None,
                    }],
                    results: None,
                    loc: None,
                };
                let (fid, str) = self.emitter.emit_special_func(
                    None,
                    &[],
                    &probe.metadata.pred_args,
                    None,
                    &[WirmType::I32],
                    &mut block,
                    true,
                    &probe.loc,
                    self.err,
                );
                (fid, str, None)
            }
        } else {
            (None, "".to_string(), None)
        };

        // create the probe's $alloc method
        let (alloc_fid, alloc_param_str) = self.unshared_var_handler.emit_alloc_func(
            &mut probe.unshared_to_alloc,
            &probe.metadata.init_args,
            &mut probe.init_logic,
            &mut self.emitter,
            self.err,
        );

        // create any @static evaluation functions
        let mut all_lib_calls = vec![];
        probe
            .static_lib_calls
            .iter()
            .for_each(|(params, lib_call)| {
                let mut block = Block {
                    stmts: vec![Statement::Expr {
                        expr: lib_call.clone(),
                        loc: None,
                    }],
                    results: None,
                    loc: None,
                };

                let ty = if let Expr::LibCall { results, .. } = lib_call {
                    results.as_ref().unwrap().clone()
                } else {
                    self.err.add_internal_error(
                        "Results of a library call should have been set by the type checker!",
                        lib_call.loc(),
                    );
                    DataType::AssumeGood
                };
                let wirm_ty = match ty.to_wasm_type().first() {
                    Some(ty) => *ty,
                    None => {
                        self.err.add_internal_error(
                            &format!(
                                "Should have been able to convert the type to a Wasm type: {ty}"
                            ),
                            lib_call.loc(),
                        );
                        return;
                    }
                };

                let (fid, s) = self.emitter.emit_special_func(
                    None,
                    &[],
                    params,
                    None,
                    std::slice::from_ref(&wirm_ty),
                    &mut block,
                    true,
                    &probe.loc,
                    self.err,
                );
                all_lib_calls.push((fid, s, wirm_ty));
            });

        // create the probe body function
        let (body_fid, body_param_str) = if let Some(body) = &mut probe.body {
            let alloc_local = if alloc_fid.is_some() {
                Some(LocalID(0))
            } else {
                None
            };
            let (pred, body_block) = match (self.config.no_pred, self.config.no_body) {
                // as normal
                (false, false) => (dynamic_pred, body),
                // empty if statement
                (false, true) => (dynamic_pred, &mut Block::default()),
                // unpredicated body
                (true, false) => (None, body),
                // empty function
                (true, true) => (None, &mut Block::default()),
            };

            // since we're only supporting 'no_bundle' when 'no_body' and 'no_pred' are also true
            // we can simplify this to just not requesting any arguments...shouldn't even have a
            // function body!
            if self.config.no_bundle {
                assert!(pred.is_none());
                assert!(body_block.stmts.is_empty());
            }
            let no_params = WhammParams::default();
            let mut params = probe.metadata.body_args.clone();
            if pred.is_some() {
                // need to request predicate params now that we're pushing down into the body
                params.extend(probe.metadata.pred_args.clone());
            }

            self.emitter.emit_special_func(
                if self.config.no_bundle {
                    None
                } else {
                    alloc_local
                },
                &all_lib_calls,
                if self.config.no_bundle {
                    &no_params
                } else {
                    &params
                },
                pred,
                &[],
                body_block,
                false,
                &probe.loc,
                self.err,
            )
        } else {
            (None, "".to_string())
        };

        let match_rule = self.create_wei_match_rule(
            &probe.rule.to_string(),
            (pred_fid, &pred_param_str),
            (alloc_fid, &alloc_param_str),
            &all_lib_calls,
            &body_param_str,
        );
        if let Some(fid) = body_fid {
            self.emitter.app_wasm.exports.add_export_func_with_tag(
                match_rule,
                fid,
                get_tag_for(&None),
            );
        } else {
            self.err.add_probe_warn(
                "Are you sure you meant to emit a probe with no body?",
                &probe.loc.clone(),
            );
        }
    }

    fn create_wei_match_rule(
        &self,
        probe_name: &str,
        pred_fid: (Option<u32>, &str),
        alloc: (Option<u32>, &str),
        all_lib_calls: &[(Option<u32>, String, WirmType)],
        body_params: &str,
    ) -> String {
        let pred_part = if let (Some(pred_fid), pred_params) = pred_fid {
            format!("/ {} /", call(pred_fid, pred_params))
        } else {
            "".to_string()
        };
        let alloc_part = if let (Some(alloc_fid), alloc_params) = alloc {
            call(alloc_fid, alloc_params) + ", "
        } else {
            "".to_string()
        };
        let mut lib_calls_part = String::new();
        all_lib_calls
            .iter()
            .for_each(|(fid, params, _)| lib_calls_part += &(call(fid.unwrap(), params) + ", "));

        let body_part = alloc_part + &lib_calls_part + body_params;

        fn call(fid: u32, params: &str) -> String {
            format!("${fid}({params})")
        }

        format!("{probe_name} {pred_part} ({body_part})")
    }
}

impl GeneratingVisitor for WeiGenerator<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    // TODO -- these are all duplicates, try to factor out
    fn add_internal_error(&mut self, message: &str, loc: &Option<Location>) {
        self.err.add_internal_error(message, loc);
    }
    fn emit_string(&mut self, val: &mut Value) -> bool {
        self.emitter.emit_string(val)
    }

    fn emit_func(&mut self, f: &mut crate::parser::types::Fn) -> Option<FunctionID> {
        self.emitter.emit_fn("TODO", f)
    }

    fn emit_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_global(name, ty, value, self.err)
    }

    fn emit_report_global(
        &mut self,
        name: String,
        ty: DataType,
        value: &Option<Value>,
    ) -> Option<FunctionID> {
        self.emitter.emit_report_global(name, ty, value, self.err)
    }

    fn link_user_lib(&mut self, lib_name: &str, loc: &Option<Location>) {
        // Perform import now! (we'll be in the right table scope at this point)
        if let Some(used_fns) = self.used_fns_per_lib.get(lib_name) {
            let Some((lib_name_import_override, lib_wasm)) = self.user_lib_modules.get(lib_name)
            else {
                panic!("Could not find wasm module for library '{lib_name}'");
            };
            self.injected_funcs.extend(
                crate::lang_features::libraries::linking::import_lib::link_user_lib(
                    self.emitter.app_wasm,
                    loc,
                    lib_wasm,
                    lib_name.to_string(),
                    lib_name_import_override,
                    used_fns,
                    self.emitter.table,
                ),
            );
        }
    }

    fn add_injected_func(&mut self, fid: FunctionID) {
        self.injected_funcs.push(fid);
    }

    fn get_context_name_mut(&mut self) -> &mut String {
        &mut self.context_name
    }

    fn get_context_name(&self) -> &String {
        &self.context_name
    }

    fn set_curr_loc(&mut self, loc: LocationData) {
        self.emitter.report_vars.curr_location = loc;
    }

    fn enter_named_scope(&mut self, name: &str) {
        self.emitter.table.enter_named_scope(name);
    }

    fn enter_scope(&mut self) {
        self.emitter.enter_scope();
    }

    fn exit_scope(&mut self) {
        self.emitter.exit_scope();
    }
    fn lookup_var_mut(&mut self, name: &str) -> Option<&mut Record> {
        self.emitter.table.lookup_var_mut(name, true)
    }

    fn visit_global_stmts(&mut self, stmts: &mut [Statement]) -> bool {
        // 1. create the emitting_func var, assign in self
        // 2. iterate over stmts and emit them! (will be different for Decl stmts)
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::Decl { .. } | Statement::UnsharedDecl { .. } => {} // already handled
                Statement::LibImport { lib_name, loc, .. } => {
                    self.link_user_lib(lib_name, loc);
                }
                Statement::Assign { .. } | Statement::Expr { .. } => {
                    // assume this is a valid AST node since we've gone through validation
                    self.emitter.emit_global_stmt(stmt, self.err);
                }
                Statement::UnsharedDeclInit { init, .. } => {
                    self.emitter.emit_global_stmt(init, self.err);
                }
                _ => {
                    self.err.add_unimplemented_error(&format!("We have not added support for this statement type in the script global scope: {stmt:?}"), stmt.loc());
                    return false;
                }
            }
        }
        true
    }
}
