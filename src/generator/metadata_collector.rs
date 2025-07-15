use crate::api::instrument::Config;
use crate::common::error::ErrorGen;
use crate::generator::ast::{Probe, ReqArgs, Script};
use crate::lang_features::report_vars::{BytecodeLoc, Metadata as ReportMetadata};
use crate::parser::provider_handler::{Event, ModeKind, Package, Probe as ParserProbe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Location, Script as ParserScript, Statement, Value,
    Whamm, WhammVisitor,
};
use crate::verifier::types::{Record, SymbolTable};
use log::trace;
use std::collections::HashSet;

const UNEXPECTED_ERR_MSG: &str =
    "MetadataCollector: Looks like you've found a bug...please report this behavior!";

enum Visiting {
    Predicate,
    Body,
    None,
}

// Performs a pass on the AST to generate probe "metadata" that will be used
// while emitting. It will collect the required variables to pass to a probe
// (argN, localN, etc.) and can be extended to compute the memory space that
// must be allocated per probe (vars_to_alloc).
pub struct MetadataCollector<'a, 'b, 'c> {
    pub table: &'a mut SymbolTable,
    pub ast: Vec<Script>,

    // misc. trackers
    pub used_user_library_fns: HashSet<(String, String)>,
    curr_user_lib: Option<String>,
    pub used_bound_fns: HashSet<(String, String)>,
    pub used_report_var_dts: HashSet<DataType>,
    pub check_strcmp: bool,
    pub strings_to_emit: Vec<String>,

    visiting: Visiting,
    curr_rule: String,
    curr_script: Script,
    script_num: u8,
    curr_probe: Probe,

    pub err: &'b mut ErrorGen,
    pub config: &'c Config,
}
impl<'a, 'b, 'c> MetadataCollector<'a, 'b, 'c> {
    pub(crate) fn new(
        table: &'a mut SymbolTable,
        err: &'b mut ErrorGen,
        config: &'c Config,
    ) -> Self {
        Self {
            table,
            ast: Vec::default(),
            used_user_library_fns: HashSet::default(),
            curr_user_lib: None,
            used_bound_fns: HashSet::default(),
            used_report_var_dts: HashSet::default(),
            check_strcmp: false,
            strings_to_emit: Vec::default(),
            visiting: Visiting::None,
            curr_rule: "".to_string(),
            curr_script: Script::default(),
            script_num: 0,
            curr_probe: Probe::default(),
            err,
            config,
        }
    }

    fn visit_stmts(&mut self, stmts: &[Statement]) {
        stmts.iter().for_each(|stmt| {
            self.visit_stmt(stmt);
        })
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
        }
    }
    fn combine_req_args(&mut self, req_args: ReqArgs) {
        match self.visiting {
            Visiting::Predicate => {
                self.curr_probe
                    .metadata
                    .pred_args
                    .req_args
                    .combine(&req_args);
            }
            Visiting::Body => {
                self.curr_probe
                    .metadata
                    .body_args
                    .req_args
                    .combine(&req_args);
            }
            Visiting::None => {
                // error
                unreachable!("Expected a set variant of 'Visiting', but found 'None'");
            }
        }
    }
    fn push_metadata(&mut self, name: &str, ty: &DataType) {
        match self.visiting {
            Visiting::Predicate => {
                self.curr_probe
                    .metadata
                    .push_pred_req(name.to_string(), ty.clone());
            }
            Visiting::Body => {
                self.curr_probe
                    .metadata
                    .push_body_req(name.to_string(), ty.clone());
            }
            Visiting::None => {
                // error
                unreachable!("Expected a set variant of 'Visiting', but found 'None'");
            }
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
                self.visit_probe(probe);

                // copy over data from original probe
                self.curr_probe.predicate = probe.predicate.to_owned();
                self.curr_probe.body = probe.body.to_owned();
                self.curr_probe.body = probe.body.to_owned();
                self.curr_script.probes.push(self.curr_probe.clone());

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
        self.curr_probe.metadata.pred_args.process_req_args();
        if let Some(body) = &probe.body {
            self.visiting = Visiting::Body;
            self.visit_stmts(body.stmts.as_slice());
            if probe.kind == ModeKind::Alt {
                // XXX: this is bad
                // always save all args for an alt probe
                self.combine_req_args(ReqArgs::All);
            }
        }
        // compile which args have been requested
        self.curr_probe.metadata.body_args.process_req_args();
        self.visiting = Visiting::None;

        trace!("Exiting: CodeGenerator::visit_probe");
        self.table.exit_scope();
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_block(&mut self, block: &Block) {
        self.visit_stmts(&block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::LibImport { .. } => {
                // Nothing to do, we just want to track the libraries/functions that are **used**
            }
            Statement::Decl { .. } => {
                // ignore
            }
            Statement::UnsharedDeclInit { decl, init, .. } => {
                self.visit_stmt(decl);
                self.visit_stmt(init);
                self.curr_probe.add_init_logic(*init.clone());
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
            }
            Statement::Assign { var_id, expr, .. } => {
                if let Expr::VarId { name, .. } = var_id {
                    let (def, _ty, loc) = get_def(name, self.table);
                    if def.is_comp_defined()
                        && self.config.as_monitor_module
                        && !self.config.enable_wizard_alt
                    {
                        self.err.wizard_error(
                            "Assigning to compiler-defined variables is not supported on Wizard target"
                                .to_string(),
                            &loc,
                        );
                    }
                }

                self.visit_expr(var_id);
                self.visit_expr(expr);
            }
            Statement::Expr { expr, .. } | Statement::Return { expr, .. } => self.visit_expr(expr),
            Statement::If {
                cond, conseq, alt, ..
            } => {
                self.visit_expr(cond);
                self.visit_block(conseq);
                self.visit_block(alt);
            }
            Statement::SetMap { map, key, val, .. } => {
                self.visit_expr(map);
                self.visit_expr(key);
                self.visit_expr(val);
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::UnOp { expr, .. } => self.visit_expr(expr),
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                self.visit_expr(cond);
                self.visit_expr(conseq);
                self.visit_expr(alt);
            }
            Expr::BinOp { lhs, rhs, op, .. } => {
                self.check_strcmp = matches!(op, BinOp::EQ | BinOp::NE);
                self.visit_expr(lhs);
                self.visit_expr(rhs);
                if self.check_strcmp {
                    // if this flag is still true, we need the strcmp function!
                    self.used_bound_fns
                        .insert(("whamm".to_string(), "strcmp".to_string()));
                }
                self.check_strcmp = false;
            }
            Expr::LibCall { lib_name, call, .. } => {
                self.curr_user_lib = Some(lib_name.to_string());
                self.visit_expr(call);
                self.curr_user_lib = None;
            }
            Expr::Call {
                args, fn_target, ..
            } => {
                // is this a bound function?
                let fn_name = match &**fn_target {
                    Expr::VarId { name, .. } => name.clone(),
                    _ => {
                        unreachable!("{} Can only call functions.", UNEXPECTED_ERR_MSG);
                    }
                };

                let (def, ret_ty, req_args, context) = if let Some(lib_name) = &self.curr_user_lib {
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
                    let Some(exp_lib_name) = &self.curr_user_lib else {
                        panic!("Current user library is not set!")
                    };
                    assert_eq!(exp_lib_name, lib_name, "Library names should be equal!!");
                    self.used_user_library_fns
                        .insert((lib_name.clone(), fn_name.clone()));
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

                    (def, ret_ty, ReqArgs::None, None)
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

                self.check_strcmp = matches!(ret_ty, DataType::Str);
                if matches!(def, Definition::CompilerDynamic) {
                    if let Some(context) = context {
                        // will need to emit this function!
                        self.used_bound_fns.insert((context, fn_name));
                        // will need to possibly define arguments!
                        self.combine_req_args(req_args.clone());
                    }
                }

                args.iter().for_each(|arg| {
                    self.visit_expr(arg);
                });
            }
            Expr::Primitive { val, .. } => {
                match val {
                    Value::Str { val, .. } => {
                        self.strings_to_emit.push(val.clone());
                        return;
                    }
                    Value::Tuple { vals, .. } => vals.iter().for_each(|val| {
                        self.visit_expr(val);
                    }),
                    _ => {} // nothing to do
                }
                self.check_strcmp = false;
            }
            Expr::VarId { name, .. } => {
                let (def, ty, ..) = get_def(name, self.table);
                if matches!(def, Definition::CompilerDynamic | Definition::User) {
                    self.mark_expr_as_dynamic();
                }

                // check if bound, remember in metadata!
                self.check_strcmp = matches!(ty, DataType::Str);

                if def.is_comp_defined() {
                    // For Wizard: Request all!
                    // For B.R.: Only request dynamic data
                    self.push_metadata(name, &ty);
                }
            }
            Expr::MapGet { map, key, .. } => {
                self.visit_expr(map);
                self.visit_expr(key);
            }
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
