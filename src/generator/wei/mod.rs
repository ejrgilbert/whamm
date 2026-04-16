use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::tag_handler::get_tag_for;
use crate::generator::ast::{Probe, Script, WhammParams};
use crate::generator::{create_curr_loc, emit_needed_funcs, GeneratingVisitor};
use crate::lang_features::alloc_vars::wei::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::report_vars::LocationData;
use crate::parser;
use crate::parser::types::{
    Block, CallKind, DataType, Expr, Location, RulePart, Statement, Value, WhammVisitorMut,
};
use crate::verifier::types::Record;
use std::collections::{HashMap, HashSet};
use wirm::ir::id::{FunctionID, LocalID};
use wirm::ir::types::DataType as WirmType;
use wirm::Module;

pub struct WeiGenerator<'a, 'lib, 'ir> {
    pub emitter: ModuleEmitter<'a, 'ir>,
    pub io_adapter: &'a mut IOAdapter,
    pub context_name: String,
    pub err: &'a mut ErrorGen,
    pub injected_funcs: &'a mut Vec<FunctionID>,
    pub config: &'a Config,
    pub used_exports_per_lib: HashMap<String, (bool, HashSet<String>)>,
    pub user_lib_modules: HashMap<String, (Option<String>, Module<'lib>)>,

    // tracking
    pub curr_script_id: u8,
    pub unshared_var_handler: &'a mut UnsharedVarHandler,
}

impl WeiGenerator<'_, '_, '_> {
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

    fn fold_stmts(&mut self, stmts: &mut Vec<Statement>) {
        crate::generator::folding::pass::fold_stmts(
            stmts,
            true,
            self.emitter.table,
            self.emitter.registry,
            &self.emitter.mem_allocator.emitted_strings,
            self.emitter.app_wasm,
            self.err,
        );
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
        if !script.req_globals.params.is_empty() {
            let mut list = String::new();
            for req in script.req_globals.params.iter() {
                list.push_str(&format!("- {}: {}\n", req.name, req.ty));
            }
            unimplemented!("`wei` does not support engine-provided global definitions yet. You requested:\n{list}")
        }
        self.visit_global_stmts(&mut script.global_stmts);
        // Merge probes with identical export-name signatures before visiting.
        // Two probes that would produce the same export name (same rule + same body/pred
        // param types + same alloc/static-lib structure) need their bodies combined into
        // a single export to avoid duplicate export names.
        // Probes that differ in any of those dimensions already produce unique names and
        // are left in separate groups.
        let mut probes = std::mem::take(&mut script.probes);
        // Sort by probe_number first so that merge groups and final probe order are
        // deterministic regardless of HashMap iteration order in earlier passes.
        probes.sort_by_key(|p| p.probe_number);
        script.probes = merge_overlapping_probes(probes);
        script.probes.iter_mut().for_each(|probe| {
            let probe_rule: crate::emitter::rewriting::rules::ProbeRule = (&probe.rule).into();
            assert!(
                self.emitter.table.enter_scope_via_rule(
                    &self.curr_script_id.to_string(),
                    &parser::types::ProbeRule {
                        provider: probe_rule.provider.clone(),
                        package: probe_rule.package.clone(),
                        event: probe_rule.event.clone(),
                        mode: Some(RulePart::new(
                            probe_rule.mode.as_ref().unwrap().name(),
                            None,
                        )),
                    },
                    probe.scope_id,
                ),
                "Failed to enter scope"
            );
            self.visit_probe(probe);
            self.emitter.table.exit_scope();
        });
        self.exit_scope();
    }

    fn visit_probe(&mut self, probe: &mut Probe) {
        self.set_curr_loc(create_curr_loc(self.curr_script_id, probe, true));

        let (pred_fid, pred_param_str, dynamic_pred) = if let Some(pred) = &mut probe.predicate {
            if probe.metadata.pred_is_dynamic {
                // dynamic analysis of the predicate will go here!
                // See: https://github.com/ejrgilbert/whamm/issues/163

                // for now, we push the dynamic predicate down into the probe body

                (None, "".to_string(), Some(pred))
            } else {
                let mut block = Block {
                    stmts: vec![Statement::Return {
                        expr: pred.clone(),
                        loc: None,
                    }],
                    results: None,
                    loc: None,
                };
                self.fold_stmts(&mut block.stmts);
                let (fid, str) = self.emitter.emit_special_func(
                    None,
                    &[],
                    &probe.metadata.pred_args,
                    None,
                    &[WirmType::I32],
                    &block,
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
                    stmts: vec![Statement::Return {
                        expr: lib_call.clone(),
                        loc: None,
                    }],
                    results: None,
                    loc: None,
                };
                self.fold_stmts(&mut block.stmts);

                // Recover the call's return type from the resolved CallKind.
                // (Was previously read off of ObjCall.results, which is gone
                // now that ObjCall is flattened — see issue #305 followup.)
                let ty = match lib_call {
                    Expr::Call {
                        kind: CallKind::Lib { rec_id, .. },
                        ..
                    } => match self.emitter.table.get_record(*rec_id) {
                        Some(Record::LibFn { results, .. }) => results
                            .first()
                            .cloned()
                            .unwrap_or_else(DataType::empty_tuple),
                        _ => {
                            self.err.add_internal_error(
                                "CallKind::Lib resolved to non-LibFn record",
                                lib_call.loc(),
                            );
                            DataType::AssumeGood
                        }
                    },
                    _ => {
                        self.err.add_internal_error(
                            "Expected a resolved Lib call here",
                            lib_call.loc(),
                        );
                        DataType::AssumeGood
                    }
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
                    &block,
                    true,
                    &probe.loc,
                    self.err,
                );
                all_lib_calls.push((fid, s, wirm_ty));
            });

        // create the probe body function
        let (mut body_fid, body_param_str) = if let Some(body) = &mut probe.body {
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

            self.fold_stmts(&mut body_block.stmts);
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
                pred.as_deref(),
                &[],
                body_block,
                false,
                &probe.loc,
                self.err,
            )
        } else {
            (None, "".to_string())
        };

        if body_fid.is_none() && !probe.init_logic.is_empty() {
            // emit an empty function for the probe body (ensures that the init logic runs!)
            body_fid = Some(*self.emitter.emit_empty_fn_with_alloc_param(&probe.loc));
        }

        let match_rule = self.create_wei_match_rule(
            &probe.rule.to_string(true),
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

impl GeneratingVisitor for WeiGenerator<'_, '_, '_> {
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
        if let Some((used_mem, used_fns)) = self.used_exports_per_lib.get(lib_name) {
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
                    *used_mem,
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
        // handle these first since importing libs affects lib mems (the constant pool for folding)
        self.handle_lib_imports(stmts);

        crate::generator::folding::pass::fold_stmts_slice(
            stmts,
            true,
            self.emitter.table,
            self.emitter.registry,
            &self.emitter.mem_allocator.emitted_strings,
            self.emitter.app_wasm,
            self.err,
        );
        // 1. create the emitting_func var, assign in self
        // 2. iterate over stmts and emit them! (will be different for Decl stmts)
        for stmt in stmts.iter_mut() {
            match stmt {
                Statement::VarDecl { init: None, .. } => {} // already handled
                Statement::VarDecl {
                    init: Some(init_expr),
                    name,
                    definition,
                    loc,
                    ..
                } => {
                    let assign = Statement::Assign {
                        var_id: Expr::VarId {
                            name: name.clone(),
                            definition: *definition,
                            loc: None,
                        },
                        expr: init_expr.clone(),
                        loc: loc.clone(),
                    };
                    self.emitter.emit_global_stmt(&assign, self.err);
                }
                Statement::LibImport { lib_name, loc, .. } => {
                    self.link_user_lib(lib_name, loc);
                }
                Statement::Assign { .. } | Statement::Expr { .. } => {
                    // assume this is a valid AST node since we've gone through validation
                    self.emitter.emit_global_stmt(stmt, self.err);
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

// =============================================================================
// Probe merging for the wei target
// =============================================================================

/// Groups probes by their export-name signature and merges each group into a
/// single probe. Groups are processed in insertion order so output is deterministic.
fn merge_overlapping_probes(probes: Vec<Probe>) -> Vec<Probe> {
    let mut order: Vec<String> = Vec::new();
    let mut groups: HashMap<String, Vec<Probe>> = HashMap::new();
    for probe in probes {
        let key = probe_merge_key(&probe);
        if !groups.contains_key(&key) {
            order.push(key.clone());
        }
        groups.entry(key).or_default().push(probe);
    }
    order
        .into_iter()
        .map(|key| merge_probe_group(groups.remove(&key).unwrap()))
        .collect()
}

/// Returns the key that determines whether two probes would produce the same
/// wei export name. Captures: rule string, body/pred/init param types, whether
/// a pred function is emitted, alloc presence, and static-lib-call count.
fn probe_merge_key(probe: &Probe) -> String {
    let fmt_params = |params: &HashSet<crate::generator::ast::WhammParam>| -> String {
        let mut v: Vec<String> = params
            .iter()
            .map(|p| format!("{}:{:?}", p.name, p.ty))
            .collect();
        v.sort_unstable();
        v.join(",")
    };

    // A separate predicate function is emitted only when there is a predicate
    // and it is not dynamic (dynamic preds are pushed into the body instead).
    let has_pred_fn = probe.predicate.is_some() && !probe.metadata.pred_is_dynamic;

    // Probes with non-trivial predicates produce unique export names based on
    // the pred function's FID, so they must never be merged with each other.
    // Include the probe's scope_id to make each predicated probe's key unique.
    if has_pred_fn {
        return format!(
            "{}|unique_pred:{}",
            probe.rule.to_string(true),
            probe.scope_id
        );
    }

    // Type bounds change which variables are in scope and thus which report vars
    // exist — probes with different type bounds must not be merged.
    let mut bounds: Vec<String> = probe
        .type_bounds
        .iter()
        .map(|(expr, ty)| {
            let name = if let Expr::VarId { name, .. } = expr {
                name.as_str()
            } else {
                "_"
            };
            format!("{name}:{ty:?}")
        })
        .collect();
    bounds.sort_unstable();
    let bounds_str = bounds.join(",");

    format!(
        "{}|body:{}|pred:false:|init:{}|alloc:{}|statics:{}|bounds:{}",
        probe.rule.to_string(true),
        fmt_params(&probe.metadata.body_args.params),
        fmt_params(&probe.metadata.init_args.params),
        !probe.unshared_to_alloc.is_empty(),
        probe.static_lib_calls.len(),
        bounds_str,
    )
}

/// Merges a group of probes with identical export-name signatures into one.
/// - Predicates are inlined as `if pred { body }` blocks so the merged probe
///   never has a top-level predicate.
/// - `@staticN` aliases in each probe's body are renumbered relative to the
///   accumulated `static_lib_calls` list so indices remain correct.
/// - Conflicting unshared-variable names are suffixed (`foo` → `foo_1`, etc.)
///   and all references in the body and init_logic are rewritten accordingly.
fn merge_probe_group(mut probes: Vec<Probe>) -> Probe {
    if probes.len() == 1 {
        return probes.swap_remove(0);
    }

    let first = &probes[0];
    let mut merged = Probe {
        rule: first.rule.clone(),
        probe_number: first.probe_number,
        scope_id: first.scope_id,
        script_id: first.script_id,
        loc: first.loc.clone(),
        ..Default::default()
    };

    let mut merged_body_stmts: Vec<Statement> = Vec::new();
    let mut used_unshared_names: HashSet<String> = HashSet::new();

    for mut probe in probes {
        // === 1. @static alias remapping ===
        // The merged probe's static_lib_calls list grows as we fold probes in.
        // This probe's @staticK references need to become @static(K + offset).
        let static_offset = merged.static_lib_calls.len();
        if static_offset > 0 && !probe.static_lib_calls.is_empty() {
            let renames: HashMap<String, String> = (0..probe.static_lib_calls.len())
                .map(|i| {
                    (
                        Probe::get_call_alias_for(i),
                        Probe::get_call_alias_for(i + static_offset),
                    )
                })
                .collect();
            if let Some(ref mut body) = probe.body {
                rename_vars_in_block(body, &renames);
            }
            for stmt in probe.init_logic.iter_mut() {
                rename_vars_in_stmt(stmt, &renames);
            }
        }
        merged.static_lib_calls.extend(probe.static_lib_calls);

        // === 2. Unshared-var deconfliction ===
        let mut unshared_renames: HashMap<String, String> = HashMap::new();
        for var in probe.unshared_to_alloc.iter_mut() {
            if used_unshared_names.contains(&var.name) {
                let base = var.name.clone();
                let mut n = 1u32;
                let new_name = loop {
                    let candidate = format!("{base}_{n}");
                    if !used_unshared_names.contains(&candidate) {
                        break candidate;
                    }
                    n += 1;
                };
                unshared_renames.insert(base, new_name.clone());
                used_unshared_names.insert(new_name.clone());
                var.name = new_name;
            } else {
                used_unshared_names.insert(var.name.clone());
            }
        }
        if !unshared_renames.is_empty() {
            if let Some(ref mut body) = probe.body {
                rename_vars_in_block(body, &unshared_renames);
            }
            for stmt in probe.init_logic.iter_mut() {
                rename_vars_in_stmt(stmt, &unshared_renames);
            }
        }

        // === 3. Accumulate unshared vars and init logic ===
        merged.unshared_to_alloc.extend(probe.unshared_to_alloc);
        merged.init_logic.extend(probe.init_logic);

        // === 4. Merge metadata ===
        // When a predicate is inlined the pred_args it needed must become body_args.
        if probe.predicate.is_some() {
            merged.metadata.body_args.extend(probe.metadata.pred_args);
        }
        merged.metadata.body_args.extend(probe.metadata.body_args);
        merged.metadata.init_args.extend(probe.metadata.init_args);
        // merged.metadata.pred_is_dynamic stays false (no top-level predicate)

        // === 5. Build merged body (inline predicates) ===
        match (probe.predicate, probe.body) {
            (Some(pred), Some(body)) => {
                merged_body_stmts.push(Statement::If {
                    cond: pred,
                    conseq: body,
                    alt: Block::default(),
                    loc: None,
                });
            }
            (None, Some(body)) => {
                merged_body_stmts.extend(body.stmts);
            }
            // (Some(pred), None) — pred with no body is a no-op; skip.
            // (None, None) — empty probe; nothing to add.
            _ => {}
        }
    }

    if !merged_body_stmts.is_empty() {
        merged.body = Some(Block {
            stmts: merged_body_stmts,
            results: None,
            loc: None,
        });
    }

    merged
}

// --- AST walkers for in-place variable renaming ---

fn rename_vars_in_block(block: &mut Block, renames: &HashMap<String, String>) {
    for stmt in block.stmts.iter_mut() {
        rename_vars_in_stmt(stmt, renames);
    }
}

fn rename_vars_in_stmt(stmt: &mut Statement, renames: &HashMap<String, String>) {
    match stmt {
        Statement::Assign { var_id, expr, .. } => {
            rename_vars_in_expr(var_id, renames);
            rename_vars_in_expr(expr, renames);
        }
        Statement::SetMap { map, key, val, .. } => {
            if let Some(new_name) = renames.get(map.as_str()) {
                *map = new_name.clone();
            }
            rename_vars_in_expr(key, renames);
            rename_vars_in_expr(val, renames);
        }
        Statement::Expr { expr, .. } => rename_vars_in_expr(expr, renames),
        Statement::Return { expr, .. } => rename_vars_in_expr(expr, renames),
        Statement::If {
            cond, conseq, alt, ..
        } => {
            rename_vars_in_expr(cond, renames);
            rename_vars_in_block(conseq, renames);
            rename_vars_in_block(alt, renames);
        }
        Statement::VarDecl { name, init, .. } => {
            if let Some(new_name) = renames.get(name.as_str()) {
                *name = new_name.clone();
            }
            if let Some(init_expr) = init {
                rename_vars_in_expr(init_expr, renames);
            }
        }
        Statement::LibImport { .. } => {}
    }
}

fn rename_vars_in_expr(expr: &mut Expr, renames: &HashMap<String, String>) {
    match expr {
        Expr::VarId { name, .. } => {
            if let Some(new_name) = renames.get(name.as_str()) {
                *name = new_name.clone();
            }
        }
        Expr::BinOp { lhs, rhs, .. } => {
            rename_vars_in_expr(lhs, renames);
            rename_vars_in_expr(rhs, renames);
        }
        Expr::UnOp { expr, .. } => rename_vars_in_expr(expr, renames),
        Expr::Ternary {
            cond, conseq, alt, ..
        } => {
            rename_vars_in_expr(cond, renames);
            rename_vars_in_expr(conseq, renames);
            rename_vars_in_expr(alt, renames);
        }
        Expr::Call {
            fn_target, args, ..
        } => {
            rename_vars_in_expr(fn_target, renames);
            for arg in args.iter_mut() {
                rename_vars_in_expr(arg, renames);
            }
        }
        Expr::MapGet { map, key, .. } => {
            if let Some(new_name) = renames.get(map.as_str()) {
                *map = new_name.clone();
            }
            rename_vars_in_expr(key, renames);
        }
        Expr::TupleGet { tuple, .. } => rename_vars_in_expr(tuple, renames),
        Expr::Primitive { .. } => {}
    }
}
