use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::common::rule_tracker::RuleTracker;
use crate::generator::analysis_visitor::AnalysisVisitor;
use crate::generator::ast::{Probe, Script, StackReq, WhammParam, WhammParams};
use crate::lang_features::report_vars::{BytecodeLoc, LocationData, Metadata as ReportMetadata};
use crate::parser::provider_handler::{Event, ModeKind, Package, Probe as ParserProbe, Provider};
use crate::parser::types::{
    Annotation, BinOp, Block, CallKind, DataType, Definition, Expr, Location,
    Script as ParserScript, Statement, Value, Whamm, WhammVisitor,
};
use crate::verifier::types::{Record, SymbolTable};
use std::collections::{HashMap, HashSet};

const UNEXPECTED_ERR_MSG: &str =
    "MetadataCollector: Looks like you've found a bug...please report this behavior!";

/// Compute return type from a LibFn's results list. Reports an
/// unimplemented-error and returns None for the multi-result case
/// (which we don't support yet).
fn single_result_ty(
    results: &[DataType],
    qualified: &str,
    loc: &Option<Location>,
    err: &mut ErrorGen,
) -> Option<DataType> {
    match results.len() {
        0 => Some(DataType::empty_tuple()),
        1 => Some(results[0].clone()),
        _ => {
            err.add_unimplemented_error(
                &format!("We don't support functions with multiple return types: {qualified}"),
                loc,
            );
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Visiting {
    Predicate,
    Body,
    Init,
    Global,
    #[default]
    None,
}

#[derive(Default)]
pub(crate) struct UserLibs {
    pub(crate) funcs: HashMap<(String, String), bool>,
}
impl UserLibs {
    pub fn add(&mut self, lib_name: String, func_name: String, at_static: bool) {
        self.funcs
            .entry((lib_name, func_name))
            .and_modify(|orig| {
                // if there's any point that we use this library statically, remember!
                *orig = *orig || at_static
            })
            .or_insert(at_static);
    }
}

// Performs a pass on the AST to generate probe "metadata" that will be used
// while emitting. It will collect the required variables to pass to a probe
// (argN, localN, etc.) and can be extended to compute the memory space that
// must be allocated per probe (vars_to_alloc).
pub struct MetadataCollector<'a> {
    pub table: &'a mut SymbolTable,
    pub ast: Vec<Script>,

    // misc. trackers
    pub used_user_library_fns: UserLibs,
    pub used_user_library_mems: HashSet<String>,
    pub used_bound_fns: HashSet<(String, String)>,
    pub used_report_var_dts: HashSet<DataType>,
    pub check_strcmp: bool,
    pub strings_to_emit: Vec<String>,
    pub has_probe_state_init: bool,

    pub err: &'a mut ErrorGen,
    pub config: &'a Config,

    visiting: Visiting,
    rule_tracker: RuleTracker,
    curr_script: Script,
    script_num: u8,
    curr_probe: Probe,
    curr_mode: ModeKind,
    /// Stack of `is_static_call` flags for the lib calls we're currently
    /// nested inside. Length tells us if we're inside any lib call;
    /// `last()` tells us whether the innermost one is `@static`.
    /// (Lib name used to live here too but no consumer needed it after
    /// the issue #305 refactor — `CallKind::Lib` carries it now.)
    curr_user_lib: Vec<bool>,
    curr_lib_call_args: WhammParams,
}
impl<'a> MetadataCollector<'a> {
    pub(crate) fn new(
        table: &'a mut SymbolTable,
        err: &'a mut ErrorGen,
        config: &'a Config,
    ) -> Self {
        Self {
            table,
            err,
            config,
            ast: Default::default(),
            used_user_library_mems: Default::default(),
            used_user_library_fns: Default::default(),
            curr_user_lib: Default::default(),
            used_bound_fns: Default::default(),
            used_report_var_dts: Default::default(),
            check_strcmp: Default::default(),
            strings_to_emit: Default::default(),
            has_probe_state_init: Default::default(),
            visiting: Default::default(),
            rule_tracker: Default::default(),
            curr_script: Default::default(),
            script_num: Default::default(),
            curr_probe: Default::default(),
            curr_mode: Default::default(),
            curr_lib_call_args: Default::default(),
        }
    }

    fn visit_stmts(&mut self, stmts: &[Statement]) -> Vec<Statement> {
        let mut new_stmts = Vec::with_capacity(stmts.len());
        stmts.iter().for_each(|stmt| {
            if let Some(new_stmt) = self.visit_stmt_inner(stmt) {
                new_stmts.push(new_stmt);
            }
        });

        new_stmts
    }

    fn mark_expr_as_dynamic(&mut self) {
        // we only care about predicate expressions that are dynamic
        if matches!(self.visiting, Visiting::Predicate) {
            self.curr_probe.metadata.pred_is_dynamic = true;
            if let Some(true) = self.curr_user_lib.last() {
                self.err
                    .add_instr_error("Cannot use dynamic data in a static library call");
            }
        }
    }
    fn get_curr_params(&mut self) -> &mut WhammParams {
        if !self.curr_user_lib.is_empty() {
            return &mut self.curr_lib_call_args;
        }
        match self.visiting {
            Visiting::Global => &mut self.curr_script.req_globals,
            Visiting::Predicate => &mut self.curr_probe.metadata.pred_args,
            Visiting::Body => &mut self.curr_probe.metadata.body_args,
            Visiting::Init => &mut self.curr_probe.metadata.init_args,
            Visiting::None => {
                self.err.add_internal_error(
                    "Expected a set variant of 'Visiting', but found 'None'",
                    &None,
                );
                &mut self.curr_probe.metadata.pred_args
            }
        }
    }
    fn combine_req_args(&mut self, req_args: StackReq) {
        self.get_curr_params().req_args.combine(&req_args);
    }
    fn push_metadata(&mut self, name: &str, ty: &DataType) {
        if !self.curr_user_lib.is_empty() {
            self.curr_lib_call_args.push(
                WhammParam {
                    name: name.to_string(),
                    ty: ty.clone(),
                },
                &self.curr_mode,
            );
        }
        match self.visiting {
            Visiting::Global => self.curr_script.req_globals.push(
                WhammParam {
                    name: name.to_string(),
                    ty: ty.clone(),
                },
                &self.curr_mode,
            ),
            Visiting::Predicate => {
                self.curr_probe.metadata.push_pred_req(
                    name.to_string(),
                    ty.clone(),
                    &self.curr_mode,
                );
            }
            Visiting::Body => {
                self.curr_probe.metadata.push_body_req(
                    name.to_string(),
                    ty.clone(),
                    &self.curr_mode,
                );
            }
            Visiting::Init => {
                self.curr_probe.metadata.push_init_req(
                    name.to_string(),
                    ty.clone(),
                    &self.curr_mode,
                );
            }
            Visiting::None => {
                unreachable!("Expected a set variant of 'Visiting', but found 'None'");
            }
        }
    }
    /// Visits and rewrites expressions (if necessary)
    /// rewriting it only for @static lib calls right now
    fn visit_expr_inner(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::UnOp {
                op,
                expr,
                done_on,
                loc,
            } => Expr::UnOp {
                op: op.clone(),
                expr: Box::new(self.visit_expr_inner(expr)),
                done_on: done_on.clone(),
                loc: loc.clone(),
            },
            Expr::Ternary {
                cond,
                conseq,
                alt,
                ty,
                loc,
            } => Expr::Ternary {
                cond: Box::new(self.visit_expr_inner(cond)),
                conseq: Box::new(self.visit_expr_inner(conseq)),
                alt: Box::new(self.visit_expr_inner(alt)),
                ty: ty.clone(),
                loc: loc.clone(),
            },
            Expr::BinOp {
                lhs,
                rhs,
                op,
                done_on,
                loc,
            } => {
                self.check_strcmp = matches!(op, BinOp::EQ | BinOp::NE);
                let lhs = self.visit_expr_inner(lhs);
                let rhs = self.visit_expr_inner(rhs);
                if self.check_strcmp {
                    // if this flag is still true, we need the strcmp function!
                    self.used_bound_fns
                        .insert(("whamm".to_string(), "strcmp".to_string()));
                }
                self.check_strcmp = false;
                Expr::BinOp {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    op: op.clone(),
                    done_on: done_on.clone(),
                    loc: loc.clone(),
                }
            }
            Expr::Call {
                args,
                fn_target,
                kind,
                loc,
            } => {
                let fn_name = match &**fn_target {
                    Expr::VarId { name, .. } => name.clone(),
                    _ => {
                        self.err.add_internal_error(
                            &format!("{} Can only call functions.", UNEXPECTED_ERR_MSG),
                            fn_target.loc(),
                        );
                        "".to_string()
                    }
                };

                // Resolve the call's record + bookkeeping based on CallKind.
                // No ambient lookup (issue #305 is structurally impossible).
                let (def, ret_ty, req_args, context, is_lib, is_static_lib) = match kind {
                    CallKind::Lib {
                        rec_id,
                        lib_name,
                        annotation,
                    } => {
                        let Some(Record::LibFn { results, def, .. }) =
                            self.table.get_record(*rec_id).cloned()
                        else {
                            self.err.add_internal_error(
                                &format!("{UNEXPECTED_ERR_MSG} CallKind::Lib resolved to non-LibFn record"),
                                expr.loc(),
                            );
                            return expr.clone();
                        };
                        let is_static = annotation.as_ref().is_some_and(|a| a.is_static());
                        self.used_user_library_fns.add(
                            lib_name.clone(),
                            fn_name.to_string(),
                            is_static,
                        );
                        let Some(ret_ty) = single_result_ty(
                            &results,
                            &format!("{lib_name}.{fn_name}"),
                            expr.loc(),
                            self.err,
                        ) else {
                            return expr.clone();
                        };
                        (def, ret_ty, StackReq::None, None, true, is_static)
                    }
                    CallKind::TypeUtil {
                        rec_id,
                        receiver_var,
                        receiver_ty,
                        receiver_def,
                    } => {
                        let Some(Record::LibFn { results, def, .. }) =
                            self.table.get_record(*rec_id).cloned()
                        else {
                            self.err.add_internal_error(
                                &format!("{UNEXPECTED_ERR_MSG} CallKind::TypeUtil resolved to non-LibFn record"),
                                expr.loc(),
                            );
                            return expr.clone();
                        };
                        // Type-util-specific bookkeeping that the old
                        // ObjCall arm performed.
                        if fn_name == "contains" {
                            self.used_bound_fns
                                .insert(("whamm".to_string(), "strcmp".to_string()));
                            self.used_bound_fns
                                .insert(("whamm".to_string(), "strcontains".to_string()));
                        }
                        if matches!(receiver_ty, DataType::Str) {
                            self.used_bound_fns
                                .insert(("whamm".to_string(), "strcmp".to_string()));
                            if receiver_def.is_comp_defined() {
                                self.push_metadata(receiver_var, receiver_ty);
                            }
                        }
                        let Some(ret_ty) =
                            single_result_ty(&results, &fn_name, expr.loc(), self.err)
                        else {
                            return expr.clone();
                        };
                        (def, ret_ty, StackReq::None, None, false, false)
                    }
                    CallKind::Global { rec_id, context } => {
                        let Some(Record::Fn {
                            def,
                            ret_ty,
                            req_args,
                            ..
                        }) = self.table.get_record(*rec_id).cloned()
                        else {
                            self.err.add_internal_error(
                                &format!("{UNEXPECTED_ERR_MSG} CallKind::Global resolved to non-Fn record"),
                                expr.loc(),
                            );
                            return expr.clone();
                        };
                        (def, ret_ty, req_args, Some(context.clone()), false, false)
                    }
                    CallKind::Pending { .. } => {
                        self.err.add_internal_error(
                            &format!("{UNEXPECTED_ERR_MSG} unresolved call reached metadata collection: {fn_name}"),
                            expr.loc(),
                        );
                        return expr.clone();
                    }
                };

                self.check_strcmp &= matches!(ret_ty, DataType::Str);
                if matches!(def, Definition::CompilerDynamic) {
                    if let Some(context) = context {
                        // will need to emit this function!
                        self.used_bound_fns.insert((context, fn_name.clone()));
                    }
                } else if matches!(def, Definition::CompilerStatic) && fn_name == "memid" {
                    let target_lib = args.first().unwrap();
                    let Expr::VarId { name, .. } = target_lib else {
                        panic!("not supported")
                    };
                    self.used_user_library_mems.insert(name.clone());
                }
                if !matches!(self.visiting, Visiting::None) {
                    // will need to possibly define arguments!
                    self.combine_req_args(req_args.clone());
                }

                // For lib calls, record nesting depth in curr_user_lib so
                // inner expressions know they're inside a (possibly static)
                // bound call. This was previously the ObjCall arm's job.
                let is_nested_lib = is_lib && !self.curr_user_lib.is_empty();
                if is_lib {
                    self.curr_user_lib.push(is_static_lib);
                }
                let mut new_args = vec![];
                args.iter().for_each(|arg| {
                    new_args.push(self.visit_expr_inner(arg));
                });
                if is_lib {
                    self.curr_user_lib.pop();
                }

                let new_call = Expr::Call {
                    fn_target: fn_target.clone(),
                    args: new_args,
                    kind: kind.clone(),
                    loc: loc.clone(),
                };

                // Static-call lifting: replace this expression with a
                // synthesized one whose evaluation runs in a generated
                // helper function (only at the outermost static lib call).
                if is_static_lib {
                    if matches!(self.visiting, Visiting::Body) && self.config.as_monitor_module {
                        return if !is_nested_lib {
                            let new_expr = self.curr_probe.add_static_lib_call(
                                self.curr_lib_call_args.to_owned(),
                                new_call.clone(),
                            );
                            self.curr_lib_call_args = WhammParams::default();
                            new_expr
                        } else {
                            new_call
                        };
                    }
                } else if is_lib {
                    // Lib call without @static: surfaces as dynamic data.
                    self.mark_expr_as_dynamic();
                }

                if is_lib {
                    if matches!(self.visiting, Visiting::Predicate) && self.config.as_monitor_module
                    {
                        self.curr_probe
                            .metadata
                            .pred_args
                            .extend(self.curr_lib_call_args.clone());
                    }
                    self.curr_lib_call_args = WhammParams::default();
                }

                new_call
            }
            Expr::Primitive { val, loc } => {
                let (val, strcmp) = match val {
                    Value::Str { val: v, .. } => {
                        self.strings_to_emit.push(v.clone());
                        (val.clone(), true)
                    }
                    Value::Tuple { ty, vals } => {
                        let mut new_vals = vec![];
                        vals.iter().for_each(|val| {
                            new_vals.push(self.visit_expr_inner(val));
                        });
                        (
                            Value::Tuple {
                                vals: new_vals,
                                ty: ty.clone(),
                            },
                            false,
                        )
                    }
                    _ => (val.clone(), false), // nothing to do
                };
                self.check_strcmp = strcmp;
                Expr::Primitive {
                    val,
                    loc: loc.clone(),
                }
            }
            Expr::VarId { name, .. } => {
                let (def, ty, ..) = get_def(name, self.table);
                if matches!(def, Definition::CompilerDynamic | Definition::User) {
                    self.mark_expr_as_dynamic();
                }

                // check if bound, remember in metadata!
                self.check_strcmp &= matches!(ty, DataType::Str);

                if matches!(
                    def,
                    Definition::CompilerStatic | Definition::CompilerDynamic
                ) {
                    // For wei: Request all engine-provided vars!
                    // For B.R.: Only request dynamic data
                    // CompilerDerived vars are computed locally in the probe body from
                    // other engine vars; don't request them directly from the engine.
                    self.push_metadata(name, &ty);
                }
                expr.clone()
            }
            Expr::MapGet { map, key, loc } => {
                let (def, ty, ..) = get_def(map, self.table);
                if matches!(def, Definition::CompilerDynamic | Definition::User) {
                    self.mark_expr_as_dynamic();
                }
                if def.is_comp_defined() {
                    self.push_metadata(map, &ty);
                }
                let key = self.visit_expr_inner(key);

                Expr::MapGet {
                    map: map.clone(),
                    key: Box::new(key),
                    loc: loc.clone(),
                }
            }
            Expr::TupleGet { tuple, index, loc } => {
                let tuple = self.visit_expr_inner(tuple);
                Expr::TupleGet {
                    tuple: Box::new(tuple),
                    index: *index,
                    loc: loc.clone(),
                }
            }
        }
    }

    fn visit_stmt_inner(&mut self, stmt: &Statement) -> Option<Statement> {
        match stmt {
            Statement::VarDecl {
                name,
                ty,
                definition,
                modifiers,
                init,
                loc,
            } => {
                if modifiers.is_unshared {
                    let report_metadata = if modifiers.is_report {
                        // keep track of the used report var datatypes across the whole AST
                        self.used_report_var_dts.insert(ty.clone());
                        Some(ReportMetadata::new(
                            name.clone(),
                            ty.clone(),
                            &LocationData::Local {
                                script_id: self.script_num,
                                bytecode_loc: BytecodeLoc::new(0, 0), // (unused)
                                probe_id: self.curr_probe.to_string(self.config.as_monitor_module),
                            },
                        ))
                    } else {
                        None
                    };
                    self.curr_probe.add_unshared(
                        name.clone(),
                        ty.clone(),
                        modifiers.is_report,
                        report_metadata,
                        loc,
                    );

                    if let Some(init_expr) = init {
                        let v = self.visiting;
                        self.visiting = Visiting::Init;
                        let visited_init = self.visit_expr_inner(init_expr);
                        self.visiting = v;
                        self.has_probe_state_init = true;
                        let init_decl = Statement::VarDecl {
                            name: name.clone(),
                            ty: ty.clone(),
                            definition: *definition,
                            modifiers: modifiers.clone(),
                            loc: loc.clone(),
                            init: None,
                        };
                        let init_assign = Statement::Assign {
                            var_id: Expr::VarId {
                                name: name.clone(),
                                definition: *definition,
                                loc: None,
                            },
                            expr: visited_init,
                            loc: None,
                        };
                        self.curr_probe.add_init_logic(init_decl);
                        self.curr_probe.add_init_logic(init_assign);
                        return None;
                    }
                }
                Some(stmt.clone())
            }
            Statement::Assign { var_id, expr, loc } => {
                if let Expr::VarId {
                    name,
                    definition: var_def,
                    ..
                } = var_id
                {
                    let (def, _ty, loc) = get_def(name, self.table);
                    incr_times_set(name, self.table);
                    // Skip the WEI check for compiler-inserted derived-var assignments
                    // (builder_visitor marks these with Definition::CompilerDerived in the AST).
                    if *var_def != Definition::CompilerDerived
                        && def.is_comp_defined()
                        && self.config.as_monitor_module
                        && !self.config.enable_wei_alt
                    {
                        self.err.wei_error(
                            "Assigning to compiler-defined variables is not supported on Wizard target"
                                .to_string(),
                            &loc,
                        );
                    }
                }

                let var_id = self.visit_expr_inner(var_id);
                let expr = self.visit_expr_inner(expr);

                Some(Statement::Assign {
                    var_id,
                    expr,
                    loc: loc.clone(),
                })
            }
            Statement::Expr { expr, loc } => {
                if let Expr::Call {
                    kind: CallKind::Lib { annotation, .. },
                    ..
                } = expr
                {
                    if matches!(annotation, Some(Annotation::Init)) {
                        let prev_visiting = self.visiting;
                        self.visiting = Visiting::Init;
                        let visited_init = self.visit_expr_inner(expr);
                        self.visiting = prev_visiting;
                        self.has_probe_state_init = true;

                        let init = Statement::Expr {
                            expr: visited_init,
                            loc: None,
                        };
                        self.curr_probe.add_init_logic(init);
                        return None;
                    }
                }
                Some(Statement::Expr {
                    expr: self.visit_expr_inner(expr),
                    loc: loc.clone(),
                })
            }
            Statement::Return { expr, loc } => Some(Statement::Return {
                expr: self.visit_expr_inner(expr),
                loc: loc.clone(),
            }),
            Statement::If {
                cond,
                conseq:
                    Block {
                        stmts: conseq_stmts,
                        results: conseq_results,
                        loc: conseq_loc,
                    },
                alt:
                    Block {
                        stmts: alt_stmts,
                        results: alt_results,
                        loc: alt_loc,
                    },
                loc,
            } => Some(Statement::If {
                cond: self.visit_expr_inner(cond),
                conseq: Block {
                    stmts: self.visit_stmts(conseq_stmts),
                    results: conseq_results.clone(),
                    loc: conseq_loc.clone(),
                },
                alt: Block {
                    stmts: self.visit_stmts(alt_stmts),
                    results: alt_results.clone(),
                    loc: alt_loc.clone(),
                },
                loc: loc.clone(),
            }),
            Statement::SetMap { map, key, val, loc } => Some(Statement::SetMap {
                map: map.clone(),
                key: self.visit_expr_inner(key),
                val: self.visit_expr_inner(val),
                loc: loc.clone(),
            }),
            _ => Some(stmt.clone()),
        }
    }
}
impl AnalysisVisitor for MetadataCollector<'_> {
    fn enter_named_scope(&mut self, name: &str) {
        self.table.enter_named_scope(name);
    }

    fn exit_scope(&mut self) {
        self.table.exit_scope();
    }

    fn get_rule_tracker_mut(&mut self) -> &mut RuleTracker {
        &mut self.rule_tracker
    }
}

impl WhammVisitor<()> for MetadataCollector<'_> {
    fn visit_whamm(&mut self, whamm: &Whamm) {
        // visit scripts
        whamm.scripts.iter().for_each(|script| {
            self.curr_script = Script::default();
            self.visit_script(script);

            // copy over state from original script
            self.curr_script.id = script.id;
            self.curr_script.fns = script.fns.to_owned();
            self.curr_script.globals = script.globals.to_owned();
            self.curr_script.global_stmts = script.global_stmts.to_owned();
            self.ast.push(self.curr_script.clone());

            self.script_num += 1;
        });
    }

    fn visit_script(&mut self, script: &ParserScript) {
        self.table.enter_named_scope(&script.id.to_string());

        self.visiting = Visiting::Global;
        self.visit_stmts(&script.global_stmts);
        self.visiting = Visiting::None;

        // visit providers
        script.providers.iter().for_each(|(_name, provider)| {
            self.visit_provider(provider);
        });
        self.table.exit_scope();
    }

    fn visit_provider(&mut self, provider: &Provider) {
        self.do_visit_provider(provider);
    }

    fn visit_package(&mut self, package: &Package) {
        self.do_visit_package(package);
    }

    fn visit_event(&mut self, event: &Event) {
        self.table.enter_named_scope(&event.def.name);
        self.rule_tracker.push(&format!(":{}", event.def.name));

        event.probes.iter().for_each(|(_ty, probes)| {
            probes.iter().for_each(|probe| {
                self.rule_tracker.push(&format!(":{}", probe.kind.name()));
                self.curr_probe = Probe::new(
                    self.rule_tracker.get_owned(),
                    probe.id,
                    probe.scope_id,
                    self.curr_script.id,
                    probe.loc.clone(),
                );
                self.curr_probe.type_bounds = probe.type_bounds.clone();
                self.curr_mode = probe.kind.clone();
                self.visit_probe(probe);

                // copy over data from original probe
                self.curr_script.probes.push(self.curr_probe.clone());

                // reset per-probe track data
                self.rule_tracker.pop();
            });
        });
        self.table.exit_scope();
        self.rule_tracker.pop();
    }

    fn visit_probe(&mut self, probe: &ParserProbe) {
        let _ = self.table.enter_named_scope(&probe.kind.name()); // enter mode scope
        let _ = self.table.enter_named_scope(&probe.scope_id.to_string()); // enter probe scope
        self.rule_tracker.push(&format!(":{}", probe.kind.name()));
        if let Some(pred) = &probe.predicate {
            self.visiting = Visiting::Predicate;
            self.visit_expr(pred);
        }
        // compile which args have been requested
        self.curr_probe.metadata.pred_args.process_stack_reqs();
        if let Some(body) = &probe.body {
            self.visiting = Visiting::Body;
            self.visit_block(body);
            if probe.kind == ModeKind::Alt {
                // XXX: this is bad
                // always save all args for an alt probe
                self.combine_req_args(StackReq::All);
            }
            // TODO -- assign self.curr_probe.extend_body()
        }
        // compile which args have been requested
        self.curr_probe.metadata.body_args.process_stack_reqs();
        self.visiting = Visiting::None;

        self.table.exit_scope(); // exit the mode scope
        self.table.exit_scope(); // exit the probe scope
        self.rule_tracker.pop();
    }

    fn visit_block(&mut self, block: &Block) {
        let new_stmts = self.visit_stmts(&block.stmts);
        if matches!(self.visiting, Visiting::Body) {
            self.curr_probe.set_body(Some(Block {
                stmts: new_stmts,
                ..block.clone()
            }))
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        let new_expr = self.visit_expr_inner(expr);
        if matches!(self.visiting, Visiting::Predicate) {
            self.curr_probe.set_pred(Some(new_expr))
        }
    }
}

fn get_def(name: &str, table: &SymbolTable) -> (Definition, DataType, Option<Location>) {
    let var = table.lookup_var(name, false);
    if let Some(Record::Var { def, ty, loc, .. }) = var {
        (*def, ty.clone(), loc.clone())
    } else if let Some(Record::Library { .. }) = var {
        (Definition::User, DataType::Lib, None)
    } else {
        unreachable!("unexpected type");
    }
}

fn incr_times_set(name: &str, table: &mut SymbolTable) {
    let var = table.lookup_var_mut(name, false);
    if let Some(Record::Var { times_set, .. }) = var {
        *times_set += 1;
    } else {
        unreachable!("unexpected type");
    }
}
