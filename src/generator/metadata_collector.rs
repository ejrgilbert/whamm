use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::generator::ast::{Probe, Script, StackReq, WhammParam, WhammParams};
use crate::lang_features::report_vars::{BytecodeLoc, Metadata as ReportMetadata};
use crate::parser::provider_handler::{Event, ModeKind, Package, Probe as ParserProbe, Provider};
use crate::parser::types::{
    Annotation, BinOp, Block, DataType, Definition, Expr, Location, Script as ParserScript,
    Statement, Value, Whamm, WhammVisitor,
};
use crate::verifier::types::{Record, SymbolTable};
use log::trace;
use std::collections::{HashMap, HashSet};

const UNEXPECTED_ERR_MSG: &str =
    "MetadataCollector: Looks like you've found a bug...please report this behavior!";

#[derive(Clone, Copy, Default)]
enum Visiting {
    Predicate,
    Body,
    Init,
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
pub struct MetadataCollector<'a, 'b, 'c> {
    pub table: &'a mut SymbolTable,
    pub ast: Vec<Script>,

    // misc. trackers
    pub used_user_library_fns: UserLibs,
    pub used_bound_fns: HashSet<(String, String)>,
    pub used_report_var_dts: HashSet<DataType>,
    pub check_strcmp: bool,
    pub strings_to_emit: Vec<String>,
    pub has_probe_state_init: bool,

    pub err: &'b mut ErrorGen,
    pub config: &'c Config,

    visiting: Visiting,
    curr_rule: String,
    curr_script: Script,
    script_num: u8,
    curr_probe: Probe,
    curr_mode: ModeKind,
    curr_user_lib: Vec<(String, bool)>, // (lib_name, is_static_call)
    curr_lib_call_args: WhammParams,
}
impl<'a, 'b, 'c> MetadataCollector<'a, 'b, 'c> {
    pub(crate) fn new(
        table: &'a mut SymbolTable,
        err: &'b mut ErrorGen,
        config: &'c Config,
    ) -> Self {
        Self {
            table,
            err,
            config,
            ast: Default::default(),
            used_user_library_fns: Default::default(),
            curr_user_lib: Default::default(),
            used_bound_fns: Default::default(),
            used_report_var_dts: Default::default(),
            check_strcmp: Default::default(),
            strings_to_emit: Default::default(),
            has_probe_state_init: Default::default(),
            visiting: Default::default(),
            curr_rule: Default::default(),
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
            new_stmts.push(self.visit_stmt_inner(stmt));
        });

        new_stmts
    }

    fn set_curr_rule(&mut self, val: String) {
        self.curr_rule = val;
    }

    fn get_curr_rule(&self) -> &String {
        &self.curr_rule
    }

    fn append_curr_rule(&mut self, val: String) {
        self.curr_rule += &val;
    }

    fn mark_expr_as_dynamic(&mut self) {
        // we only care about predicate expressions that are dynamic
        if matches!(self.visiting, Visiting::Predicate) {
            self.curr_probe.metadata.pred_is_dynamic = true;
            if let Some((_, is_static)) = self.curr_user_lib.last() {
                if *is_static {
                    self.err
                        .add_instr_error("Cannot use dynamic data in a static library call");
                }
            }
        }
    }
    fn get_curr_params(&mut self) -> &mut WhammParams {
        if !self.curr_user_lib.is_empty() {
            return &mut self.curr_lib_call_args;
        }
        match self.visiting {
            Visiting::Predicate => &mut self.curr_probe.metadata.pred_args,
            Visiting::Body => &mut self.curr_probe.metadata.body_args,
            Visiting::Init => &mut self.curr_probe.metadata.init_args,
            Visiting::None => {
                unreachable!("Expected a set variant of 'Visiting', but found 'None'");
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
            Expr::LibCall {
                annotation,
                lib_name,
                call,
                results,
                loc,
            } => {
                let is_static = matches!(annotation, Some(Annotation::Static));
                let (is_nested, _) = if let Some(c) = self.curr_user_lib.first() {
                    (true, c.1)
                } else {
                    (false, false)
                };

                self.curr_user_lib.push((lib_name.to_string(), is_static));
                let new_call = Expr::LibCall {
                    annotation: annotation.clone(),
                    lib_name: lib_name.clone(),
                    call: Box::new(self.visit_expr_inner(call)),
                    results: results.clone(),
                    loc: loc.clone(),
                };
                self.curr_user_lib.pop();

                if is_static {
                    // this is a static library call, translate this into an optimize-able expression
                    // BUT ONLY IF we're not in a predicate that's targeting an engine, we want to rewrite this expression
                    if (matches!(self.visiting, Visiting::Body)) && self.config.as_monitor_module {
                        return if !is_nested {
                            // change this expression to something that I can use to pull the result of
                            // what I do to optimize this case.
                            // Definition will be put into symbol table by the strategy generator!
                            // (won't be a problem since we've already done type checking)
                            let new_expr = self.curr_probe.add_static_lib_call(
                                self.curr_lib_call_args.to_owned(),
                                new_call.clone(),
                            );
                            self.curr_lib_call_args = WhammParams::default();
                            new_expr
                        } else {
                            // we want to just evaluate nested body lib calls inside a single function
                            new_call
                        };
                    }
                } else {
                    // now we know that the call is not annotated as static
                    self.mark_expr_as_dynamic();
                }

                if matches!(self.visiting, Visiting::Predicate) && self.config.as_monitor_module {
                    // If I'm in the predicate and targeting an engine, I don't care about the lib call args
                    // Just merge these with the general requests for the entire predicate.
                    self.curr_probe
                        .metadata
                        .pred_args
                        .extend(self.curr_lib_call_args.clone());
                }

                self.curr_lib_call_args = WhammParams::default();
                new_call
            }
            Expr::Call {
                args,
                fn_target,
                loc,
            } => {
                // is this a bound function?
                let fn_name = match &**fn_target {
                    Expr::VarId { name, .. } => name.clone(),
                    _ => {
                        unreachable!("{} Can only call functions.", UNEXPECTED_ERR_MSG);
                    }
                };

                let (def, ret_ty, req_args, context) =
                    if let Some((lib_name, is_static)) = &self.curr_user_lib.last() {
                        let Some(Record::LibFn {
                            name, results, def, ..
                        }) = self.table.lookup_lib_fn(lib_name, &fn_name)
                        else {
                            panic!(
                                "Could not find library function for {}.{}",
                                lib_name, fn_name
                            )
                        };

                        // Track user library that's being used
                        self.used_user_library_fns.add(
                            lib_name.to_string(),
                            fn_name.to_string(),
                            *is_static,
                        );
                        let ret_ty = if results.len() > 1 {
                            panic!(
                                "We don't support functions with multiple return types: {}.{}",
                                lib_name, name
                            );
                        } else if results.is_empty() {
                            DataType::Tuple { ty_info: vec![] }
                        } else {
                            results.first().unwrap().clone()
                        };

                        (def, ret_ty, StackReq::None, None)
                    } else {
                        let (
                            Some(Record::Fn {
                                def,
                                ret_ty,
                                req_args,
                                ..
                            }),
                            context,
                        ) = self.table.lookup_fn_with_context(&fn_name)
                        else {
                            unreachable!("unexpected type");
                        };
                        (def, ret_ty.clone(), req_args.clone(), Some(context))
                    };

                self.check_strcmp &= matches!(ret_ty, DataType::Str);
                if matches!(def, Definition::CompilerDynamic) {
                    if let Some(context) = context {
                        // will need to emit this function!
                        self.used_bound_fns.insert((context, fn_name));
                        // will need to possibly define arguments!
                        self.combine_req_args(req_args.clone());
                    }
                }

                let mut new_args = vec![];
                args.iter().for_each(|arg| {
                    new_args.push(self.visit_expr_inner(arg));
                });
                Expr::Call {
                    fn_target: fn_target.clone(),
                    args: new_args,
                    loc: loc.clone(),
                }
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

                if def.is_comp_defined() {
                    // For wei: Request all!
                    // For B.R.: Only request dynamic data
                    self.push_metadata(name, &ty);
                }
                expr.clone()
            }
            Expr::MapGet { map, key, loc } => {
                let map = self.visit_expr_inner(map);
                let key = self.visit_expr_inner(key);

                Expr::MapGet {
                    map: Box::new(map),
                    key: Box::new(key),
                    loc: loc.clone(),
                }
            }
        }
    }

    fn visit_stmt_inner(&mut self, stmt: &Statement) -> Statement {
        match stmt {
            Statement::UnsharedDeclInit { decl, init, loc } => {
                let v = self.visiting;
                self.visiting = Visiting::Init;
                let decl = self.visit_stmt_inner(decl);
                let init = self.visit_stmt_inner(init);
                self.visiting = v;

                self.has_probe_state_init = true;
                self.curr_probe.add_init_logic(init.clone());
                Statement::UnsharedDeclInit {
                    decl: Box::new(decl),
                    init: Box::new(init),
                    loc: loc.clone(),
                }
            }
            Statement::UnsharedDecl {
                is_report, decl, ..
            } => {
                if let Statement::Decl {
                    ty,
                    var_id: Expr::VarId { name, loc, .. },
                    ..
                } = decl.as_ref()
                {
                    let report_metadata = if *is_report {
                        // keep track of the used report var datatypes across the whole AST
                        self.used_report_var_dts.insert(ty.clone());
                        // this needs to also add report_var_metadata (if is_report)!
                        let wasm_ty = if ty.to_wasm_type().len() > 1 {
                            unimplemented!()
                        } else {
                            *ty.to_wasm_type().first().unwrap()
                        };
                        Some(ReportMetadata::Local {
                            name: name.clone(),
                            whamm_ty: ty.clone(),
                            wasm_ty,
                            script_id: self.script_num,
                            bytecode_loc: BytecodeLoc::new(0, 0), // (unused)
                            probe_id: self.curr_probe.to_string(),
                        })
                    } else {
                        None
                    };
                    // change this to save off data to allocate
                    self.curr_probe.add_unshared(
                        name.clone(),
                        ty.clone(),
                        *is_report,
                        report_metadata,
                        loc,
                    );
                } else {
                    unreachable!(
                        "{} Incorrect type for a UnsharedDecl's contents!",
                        UNEXPECTED_ERR_MSG
                    )
                }
                stmt.clone()
            }
            Statement::Assign { var_id, expr, loc } => {
                if let Expr::VarId { name, .. } = var_id {
                    let (def, _ty, loc) = get_def(name, self.table);
                    if def.is_comp_defined()
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

                Statement::Assign {
                    var_id,
                    expr,
                    loc: loc.clone(),
                }
            }
            Statement::Expr { expr, loc } => Statement::Expr {
                expr: self.visit_expr_inner(expr),
                loc: loc.clone(),
            },
            Statement::Return { expr, loc } => Statement::Return {
                expr: self.visit_expr_inner(expr),
                loc: loc.clone(),
            },
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
            } => Statement::If {
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
            },
            Statement::SetMap { map, key, val, loc } => Statement::SetMap {
                map: self.visit_expr_inner(map),
                key: self.visit_expr_inner(key),
                val: self.visit_expr_inner(val),
                loc: loc.clone(),
            },
            _ => stmt.clone(),
        }
    }
}
impl WhammVisitor<()> for MetadataCollector<'_, '_, '_> {
    fn visit_whamm(&mut self, whamm: &Whamm) {
        trace!("Entering: CodeGenerator::visit_whamm");

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

        trace!("Exiting: CodeGenerator::visit_whamm");
    }

    fn visit_script(&mut self, script: &ParserScript) {
        trace!("Entering: CodeGenerator::visit_script");
        self.table.enter_named_scope(&script.id.to_string());

        self.visit_stmts(&script.global_stmts);

        // visit providers
        script.providers.iter().for_each(|(_name, provider)| {
            self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.table.exit_scope();
    }

    fn visit_provider(&mut self, provider: &Provider) {
        trace!("Entering: CodeGenerator::visit_provider");
        self.table.enter_named_scope(&provider.def.name);
        self.set_curr_rule(provider.def.name.clone());

        // visit the packages
        provider.packages.values().for_each(|package| {
            self.visit_package(package);
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        self.table.exit_scope();
    }

    fn visit_package(&mut self, package: &Package) {
        trace!("Entering: CodeGenerator::visit_package");
        self.table.enter_named_scope(&package.def.name);
        self.append_curr_rule(format!(":{}", package.def.name));

        // visit the events
        package.events.values().for_each(|event| {
            self.visit_event(event);
        });

        trace!("Exiting: CodeGenerator::visit_package");
        self.table.exit_scope();
        // Remove this package from `curr_rule`
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_event(&mut self, event: &Event) {
        trace!("Entering: CodeGenerator::visit_event");
        self.table.enter_named_scope(&event.def.name);
        self.append_curr_rule(format!(":{}", event.def.name));

        event.probes.iter().for_each(|(_ty, probes)| {
            probes.iter().for_each(|probe| {
                if !self.config.as_monitor_module {
                    // add the mode when not on the wizard target
                    self.append_curr_rule(format!(":{}", probe.kind.name()));
                }
                self.curr_probe = Probe::new(
                    self.get_curr_rule().clone(),
                    probe.id,
                    self.curr_script.id,
                    probe.loc.clone(),
                );
                self.curr_mode = probe.kind.clone();
                self.visit_probe(probe);

                // copy over data from original probe
                self.curr_script.probes.push(self.curr_probe.clone());

                // reset per-probe track data
                if !self.config.as_monitor_module {
                    // remove mode
                    let curr_rule = self.get_curr_rule();
                    let new_rule = curr_rule[..curr_rule.rfind(':').unwrap()].to_string();
                    self.set_curr_rule(new_rule);
                }
            });
        });

        trace!("Exiting: CodeGenerator::visit_event");
        self.table.exit_scope();
        let curr_rule = self.get_curr_rule();
        let new_rule = curr_rule[..curr_rule.rfind(':').unwrap()].to_string();
        self.set_curr_rule(new_rule);
    }

    fn visit_probe(&mut self, probe: &ParserProbe) {
        trace!("Entering: CodeGenerator::visit_probe");
        self.table.enter_named_scope(&probe.kind.name());
        self.append_curr_rule(format!(":{}", probe.kind.name()));
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

        trace!("Exiting: CodeGenerator::visit_probe");
        self.table.exit_scope();
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
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
    if let Some(Record::Var { def, ty, loc, .. }) = table.lookup_var(name, false) {
        (def.clone(), ty.clone(), loc.clone())
    } else {
        unreachable!("unexpected type");
    }
}
