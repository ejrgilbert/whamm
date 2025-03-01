use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::generator::ast::{Probe, ReqArgs, Script};
use crate::lang_features::report_vars::{BytecodeLoc, Metadata as ReportMetadata};
use crate::parser::rules::{Event, Package, Probe as ParserProbe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Location, Script as ParserScript, Statement, UnOp,
    Value, Whamm, WhammVisitor,
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
    pub used_provided_fns: HashSet<(String, String)>,
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
            used_provided_fns: HashSet::default(),
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
                self.err.unexpected_error(
                    true,
                    Some("Expected a set variant of 'Visiting', but found 'None'".to_string()),
                    None,
                );
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
                self.err.unexpected_error(
                    true,
                    Some("Expected a set variant of 'Visiting', but found 'None'".to_string()),
                    None,
                );
            }
        }
    }
    fn handle_special(&mut self, name: &str, prefix: &str) -> bool {
        if name.starts_with(prefix) && name[prefix.len()..].parse::<u32>().is_ok() {
            let (_, ty, _) = get_def(name, self.table, self.err);
            self.push_metadata(name, &ty);
            true
        } else {
            false
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
        self.table.exit_scope(self.err);
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) {
        trace!("Entering: CodeGenerator::visit_provider");
        self.table.enter_named_scope(&provider.name());
        self.set_curr_rule(provider.name());

        // visit the packages
        provider.packages().for_each(|package| {
            self.visit_package(package);
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        self.table.exit_scope(self.err);
    }

    fn visit_package(&mut self, package: &dyn Package) {
        trace!("Entering: CodeGenerator::visit_package");
        self.table.enter_named_scope(&package.name());
        self.append_curr_rule(format!(":{}", package.name()));

        // visit the events
        package.events().for_each(|event| {
            self.visit_event(event);
        });

        trace!("Exiting: CodeGenerator::visit_package");
        self.table.exit_scope(self.err);
        // Remove this package from `curr_rule`
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_event(&mut self, event: &dyn Event) {
        trace!("Entering: CodeGenerator::visit_event");
        self.table.enter_named_scope(&event.name());
        self.append_curr_rule(format!(":{}", event.name()));

        event.probes().iter().for_each(|(_ty, probes)| {
            probes.iter().for_each(|probe| {
                if !self.config.wizard {
                    // add the mode when not on the wizard target
                    self.append_curr_rule(format!(":{}", probe.mode().name()));
                }
                self.curr_probe = Probe::new(
                    self.get_curr_rule().clone(),
                    probe.id(),
                    self.curr_script.id,
                );
                self.visit_probe(probe);

                // copy over data from original probe
                self.curr_probe.predicate = probe.predicate().to_owned();
                self.curr_probe.body = probe.body().to_owned();
                self.curr_probe.body = probe.body().to_owned();
                self.curr_script.probes.push(self.curr_probe.clone());

                if !self.config.wizard {
                    // remove mode
                    let curr_rule = self.get_curr_rule();
                    let new_rule = curr_rule[..curr_rule.rfind(':').unwrap()].to_string();
                    self.set_curr_rule(new_rule);
                }
            });
        });

        trace!("Exiting: CodeGenerator::visit_event");
        self.table.exit_scope(self.err);
        let curr_rule = self.get_curr_rule();
        let new_rule = curr_rule[..curr_rule.rfind(':').unwrap()].to_string();
        self.set_curr_rule(new_rule);
    }

    fn visit_probe(&mut self, probe: &Box<dyn ParserProbe>) {
        trace!("Entering: CodeGenerator::visit_probe");
        self.table.enter_named_scope(&probe.mode().name());
        self.append_curr_rule(format!(":{}", probe.mode().name()));
        if let Some(pred) = probe.predicate() {
            self.visiting = Visiting::Predicate;
            self.visit_expr(pred);
        }
        // compile which args have been requested
        self.curr_probe.metadata.pred_args.process_req_args();
        if let Some(body) = probe.body() {
            self.visiting = Visiting::Body;
            self.visit_stmts(body.stmts.as_slice());
            if probe.mode().name() == "alt" {
                // XXX: this is bad
                // always save all args for an alt probe
                self.combine_req_args(ReqArgs::All);
            }
        }
        // compile which args have been requested
        self.curr_probe.metadata.body_args.process_req_args();
        self.visiting = Visiting::None;

        trace!("Exiting: CodeGenerator::visit_probe");
        self.table.exit_scope(self.err);
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_fn(&mut self, _f: &crate::parser::types::Fn) {
        unreachable!()
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) {
        unreachable!()
    }

    fn visit_block(&mut self, block: &Block) {
        self.visit_stmts(&block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Decl { .. } => {
                // ignore
            }
            Statement::UnsharedDecl {
                is_report,
                decl,
                loc,
            } => {
                if let Statement::Decl {
                    ty,
                    var_id: Expr::VarId { name, .. },
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
                    );
                } else {
                    self.err.unexpected_error(
                        true,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} Incorrect type for a UnsharedDecl's contents!"
                        )),
                        loc.clone().map(|l| l.line_col),
                    )
                }
            }
            Statement::Assign { var_id, expr, .. } => {
                if let Expr::VarId { name, .. } = var_id {
                    let (def, _ty, loc) = get_def(name, self.table, self.err);
                    if def.is_comp_provided()
                        && self.config.wizard
                        && !self.config.enable_wizard_alt
                    {
                        self.err.wizard_error(
                            true,
                            "Assigning to compiler-provided variables is not supported on Wizard target"
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
                    self.used_provided_fns
                        .insert(("whamm".to_string(), "strcmp".to_string()));
                }
                self.check_strcmp = false;
            }
            Expr::Call {
                args, fn_target, ..
            } => {
                // is this a provided function?
                let fn_name = match &**fn_target {
                    Expr::VarId { name, .. } => name.clone(),
                    _ => {
                        self.err.unexpected_error(
                            true,
                            Some(format!("{UNEXPECTED_ERR_MSG} Can only call functions.")),
                            None,
                        );
                        "".to_string()
                    }
                };
                let (
                    Some(Record::Fn {
                        def,
                        ret_ty,
                        req_args,
                        ..
                    }),
                    context,
                ) = self.table.lookup_fn_with_context(&fn_name, self.err)
                else {
                    self.err
                        .unexpected_error(true, Some("unexpected type".to_string()), None);
                    return;
                };
                self.check_strcmp = matches!(ret_ty, DataType::Str);
                if matches!(def, Definition::CompilerDynamic) {
                    // will need to emit this function!
                    self.used_provided_fns.insert((context, fn_name));
                    // will need to possibly define arguments!
                    self.combine_req_args(req_args.clone());
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
                let (def, ty, ..) = get_def(name, self.table, self.err);
                if matches!(def, Definition::CompilerDynamic | Definition::User) {
                    self.mark_expr_as_dynamic();
                }

                // handle argN special case
                if self.handle_special(name, "arg") {
                    return;
                }

                // handle immN special case
                if self.handle_special(name, "imm") {
                    return;
                }

                // check if provided, remember in metadata!
                self.check_strcmp = matches!(ty, DataType::Str);

                if def.is_comp_provided() {
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

    fn visit_unop(&mut self, _unop: &UnOp) {
        unreachable!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) {
        unreachable!()
    }

    fn visit_datatype(&mut self, _datatype: &DataType) {
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) {
        unreachable!()
    }
}

fn get_def(
    name: &str,
    table: &SymbolTable,
    err: &mut ErrorGen,
) -> (Definition, DataType, Option<Location>) {
    if let Some(Record::Var { def, ty, loc, .. }) = table.lookup_var(name, &None, err, false) {
        (def.clone(), ty.clone(), loc.clone())
    } else {
        err.unexpected_error(true, Some("unexpected type".to_string()), None);
        (Definition::User, DataType::Null, None)
    }
}
