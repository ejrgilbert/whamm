use std::collections::HashSet;
use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::generator::wizard::ast::{WizardProbe, WizardScript};
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{
    BinOp, Block, DataType, Definition, Expr, Location, Script, Statement, UnOp, Value, Whamm,
    WhammVisitor,
};
use crate::verifier::types::{Record, SymbolTable};
use log::trace;

const UNEXPECTED_ERR_MSG: &str =
    "WizardProbeMetadataCollector: Looks like you've found a bug...please report this behavior!";

enum Visiting {
    Predicate,
    Body,
    None,
}

// Performs a pass on the AST to generate probe "metadata" that will be used
// while emitting. It will collect the required variables to pass to a probe
// (argN, localN, etc.) and can be extended to compute the memory space that
// must be allocated per probe (vars_to_alloc).
pub struct WizardProbeMetadataCollector<'a, 'b, 'c> {
    table: &'a mut SymbolTable,
    pub wizard_ast: Vec<WizardScript>,
    pub used_provided_fns: HashSet<(String, String)>,
    pub check_strcmp: bool,
    pub strings_to_emit: Vec<String>,

    visiting: Visiting,
    curr_rule: String,
    curr_script: WizardScript,
    curr_probe: WizardProbe,
    probe_count: i32,

    err: &'b mut ErrorGen,
    pub config: &'c Config,
}
impl<'a, 'b, 'c> WizardProbeMetadataCollector<'a, 'b, 'c> {
    pub(crate) fn new(
        table: &'a mut SymbolTable,
        err: &'b mut ErrorGen,
        config: &'c Config,
    ) -> Self {
        Self {
            table,
            wizard_ast: Vec::default(),
            used_provided_fns: HashSet::default(),
            check_strcmp: false,
            strings_to_emit: Vec::default(),
            visiting: Visiting::None,
            curr_rule: "".to_string(),
            curr_script: WizardScript::default(),
            curr_probe: WizardProbe::default(),
            probe_count: 0,
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
            // TODO -- this assumes we'll always use I32
            self.push_metadata(name, &DataType::I32);
            true
        } else {
            false
        }
    }
}
impl WhammVisitor<()> for WizardProbeMetadataCollector<'_, '_, '_> {
    fn visit_whamm(&mut self, whamm: &Whamm) {
        trace!("Entering: CodeGenerator::visit_whamm");

        // visit scripts
        whamm.scripts.iter().for_each(|script| {
            self.curr_script = WizardScript::default();
            self.visit_script(script);

            // copy over state from original script
            self.curr_script.name = script.name.to_owned();
            self.curr_script.fns = script.fns.to_owned();
            self.curr_script.globals = script.globals.to_owned();
            self.curr_script.global_stmts = script.global_stmts.to_owned();
            self.wizard_ast.push(self.curr_script.clone())
        });

        trace!("Exiting: CodeGenerator::visit_whamm");
    }

    fn visit_script(&mut self, script: &Script) {
        trace!("Entering: CodeGenerator::visit_script");
        self.table.enter_named_scope(&script.name);

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
                self.curr_probe = WizardProbe::new(self.get_curr_rule().clone(), self.probe_count);
                self.visit_probe(probe);

                // copy over data from original probe
                self.curr_probe.predicate = probe.predicate().to_owned();
                self.curr_probe.body = probe.body().to_owned();
                self.curr_script.probes.push(self.curr_probe.clone());

                self.probe_count += 1;
            });
        });

        trace!("Exiting: CodeGenerator::visit_event");
        self.table.exit_scope(self.err);
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) {
        trace!("Entering: CodeGenerator::visit_probe");
        self.table.enter_named_scope(&probe.mode().name());
        self.append_curr_rule(format!(":{}", probe.mode().name()));
        if let Some(pred) = probe.predicate() {
            self.visiting = Visiting::Predicate;
            self.visit_expr(pred);
        }
        if let Some(body) = probe.body() {
            self.visiting = Visiting::Body;
            self.visit_stmts(body.stmts.as_slice());
        }
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
                loc
            } => {
                if let Statement::Decl {ty, var_id: Expr::VarId {name, ..}, ..} = decl.as_ref() {
                    // change this to save off data to allocate
                    self.curr_probe.add_unshared(name.clone(), ty.clone(), *is_report);
                } else {
                    self.err.unexpected_error(
                        true,
                        Some(format!("{UNEXPECTED_ERR_MSG} Incorrect type for a UnsharedDecl's contents!")),
                        loc.clone().map(|l| l.line_col)
                    )
                }
            }
            Statement::Assign { var_id, expr, .. } => {
                if let Expr::VarId { name, .. } = var_id {
                    let (def, _ty, loc) = get_def(name, self.table, self.err);
                    if def.is_comp_provided() && !self.config.enable_wizard_alt {
                        self.err.wizard_error(
                            true,
                            "Assigning to compiler-provided variables is not supported on Wizard"
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
                    self.used_provided_fns.insert(("whamm".to_string(), "strcmp".to_string()));
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
                let (Some(Record::Fn { def, ret_ty, .. }), context) =
                    self.table.lookup_fn_with_context(&fn_name, self.err)
                else {
                    self.err
                        .unexpected_error(true, Some("unexpected type".to_string()), None);
                    return;
                };
                self.check_strcmp = matches!(ret_ty, DataType::Str);
                if matches!(def, Definition::CompilerDynamic) {
                    // will need to emit this function!
                    self.used_provided_fns.insert((context, fn_name));
                }

                args.iter().for_each(|arg| {
                    self.visit_expr(arg);
                });
            }
            Expr::Primitive { val, .. } => {
                if let Value::Str { val, .. } = val {
                    self.strings_to_emit.push(val.clone());
                } else {
                    self.check_strcmp = false;
                }
            }
            Expr::VarId { name, .. } => {
                // handle argN special case
                if self.handle_special(name, "arg") {
                    return;
                }

                // handle immN special case
                if self.handle_special(name, "imm") {
                    return;
                }

                // check if provided, remember in metadata!
                let (def, ty, ..) = get_def(name, self.table, self.err);
                self.check_strcmp = matches!(ty, DataType::Str);

                if def.is_comp_provided() {
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
