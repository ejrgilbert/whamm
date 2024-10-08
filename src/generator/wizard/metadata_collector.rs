use log::trace;
use crate::generator::wizard::ast::{Metadata, WizardProbe, WizardScript};
use crate::common::error::ErrorGen;
use crate::common::instr::Config;
use crate::emitter::report_var_metadata::LocationData;
use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types::{BinOp, Block, DataType, Expr, ProvidedFunction, Script, Statement, UnOp, Value, Whamm, WhammVisitor};
use crate::verifier::types::{Record, SymbolTable};

enum Visiting {
    Predicate,
    Body,
    None
}

// Performs a pass on the AST to generate probe "metadata" that will be used
// while emitting. It will collect the required variables to pass to a probe
// (argN, localN, etc.) and can be extended to compute the memory space that
// must be allocated per probe (vars_to_alloc).
pub struct WizardProbeMetadataCollector<'a, 'b, 'c> {
    table: &'a mut SymbolTable,
    pub wizard_ast: Vec<WizardScript>,

    visiting: Visiting,
    curr_rule: String,
    curr_script: WizardScript,
    curr_probe: WizardProbe,

    // vars_to_alloc: Vec<(String, DataType)>, // TODO (once we have 'local' variables)

    err: &'b mut ErrorGen,
    pub config: &'c Config
}
impl<'a, 'b, 'c> WizardProbeMetadataCollector<'a, 'b, 'c> {
    pub(crate) fn new(table: &'a mut SymbolTable,
                      err: &'b mut ErrorGen,
                      config: &'c Config) -> Self {
        Self {
            table,
            wizard_ast: Vec::default(),
            visiting: Visiting::None,
            curr_rule: "".to_string(),
            curr_script: WizardScript::default(),
            curr_probe: WizardProbe::default(),
            err,
            config
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
}
impl WhammVisitor<()> for WizardProbeMetadataCollector<'_, '_, '_> {
    fn visit_whamm(&mut self, whamm: &Whamm) -> () {
        trace!("Entering: CodeGenerator::visit_whamm");

        // visit scripts
        whamm.scripts.iter().for_each(|script| {
            self.curr_script = WizardScript::default();
            self.visit_script(script);

            self.wizard_ast.push(self.curr_script.clone())
        });

        trace!("Exiting: CodeGenerator::visit_whamm");
    }

    fn visit_script(&mut self, script: &Script) -> () {
        trace!("Entering: CodeGenerator::visit_script");
        self.table.enter_scope(self.err);

        // visit providers
        script.providers.iter().for_each(|(_name, provider)| {
            self.visit_provider(provider);
        });

        trace!("Exiting: CodeGenerator::visit_script");
        self.table.exit_scope(self.err);
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) -> () {
        trace!("Entering: CodeGenerator::visit_provider");
        self.table.enter_scope(self.err);
        self.set_curr_rule(format!("{}", provider.name()));

        // visit the packages
        provider.packages().for_each(|package| {
            self.visit_package(package);
        });

        trace!("Exiting: CodeGenerator::visit_provider");
        self.table.exit_scope(self.err);
    }

    fn visit_package(&mut self, package: &dyn Package) -> () {
        trace!("Entering: CodeGenerator::visit_package");
        self.table.enter_scope(self.err);
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

    fn visit_event(&mut self, event: &dyn Event) -> () {
        trace!("Entering: CodeGenerator::visit_event");
        self.table.enter_scope(self.err);
        self.append_curr_rule(format!(":{}", event.name()));

        event.probes().iter().for_each(|(ty, probes)| {
            probes.iter().for_each(|probe| {
                self.curr_probe = WizardProbe::new(self.get_curr_rule().clone());
                self.visit_probe(probe);

                self.curr_script.probes.push(self.curr_probe.clone());
            });
        });

        trace!("Exiting: CodeGenerator::visit_event");
        self.table.exit_scope(self.err);
        // Remove this event from `context_name`
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) -> () {
        trace!("Entering: CodeGenerator::visit_probe");
        self.table.enter_scope(self.err);
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
        // Remove this probe from `context_name`
        let curr_rule = self.get_curr_rule();
        self.set_curr_rule(curr_rule[..curr_rule.rfind(':').unwrap()].to_string());
    }

    fn visit_fn(&mut self, _f: &crate::parser::types::Fn) -> () {
        unreachable!()
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) -> () {
        unreachable!()
    }

    fn visit_block(&mut self, block: &Block) -> () {
        self.visit_stmts(&block.stmts)
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> () {
        match stmt {
            Statement::Decl { .. } |
            Statement::ReportDecl { .. } => {
                // ignore
                return
            }
            Statement::Assign {var_id, expr, ..} => {
                if let Expr::VarId {definition, loc, .. } = var_id {
                    if definition.is_comp_provided() {
                        if !self.config.enable_wizard_alt {
                            self.err.wizard_error(
                                true,
                                "Assigning to compiler-provided variables is not supported on Wizard".to_string(),
                                loc
                            );
                        }
                    }
                }

                self.visit_expr(var_id);
                self.visit_expr(expr);
            }
            Statement::Expr { expr, .. }
            | Statement::Return { expr, .. } => self.visit_expr(expr),
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

    fn visit_expr(&mut self, expr: &Expr) -> () {
        match expr {
            Expr::UnOp { expr, .. } => self.visit_expr(expr),
            Expr::Ternary {
                cond, conseq, alt, ..
            } => {
                self.visit_expr(cond);
                self.visit_expr(conseq);
                self.visit_expr(alt);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                self.visit_expr(lhs);
                self.visit_expr(rhs);
            }
            Expr::Call { args, .. } => {
                args.iter().for_each(|arg| {
                    self.visit_expr(arg);
                });
            }
            Expr::Primitive { .. } => return,
            Expr::VarId { definition, name, .. } => {
                // check if provided, remember in metadata!
                if definition.is_comp_provided() {
                    let Some(Record::Var { ty, .. }) = self.table.lookup_var_mut(&name, &None, self.err) else {
                        self.err.unexpected_error(true, Some("unexpected type".to_string()), None);
                        return
                    };

                    match self.visiting {
                        Visiting::Predicate => {
                            self.curr_probe.metadata.push_pred_req(name.clone(), ty.clone());
                        },
                        Visiting::Body => {
                            self.curr_probe.metadata.push_body_req(name.clone(), ty.clone());
                        }
                        Visiting::None => {
                            // error
                            self.err.unexpected_error(true, Some("Expected a set variant of 'Visiting', but found 'None'".to_string()), None);
                        }
                    }
                }
            }
            Expr::MapGet { map, key, .. } => {
                self.visit_expr(map);
                self.visit_expr(key);
            }
        }
    }

    fn visit_unop(&mut self, _unop: &UnOp) -> () {
        unreachable!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) -> () {
        unreachable!()
    }

    fn visit_datatype(&mut self, _datatype: &DataType) -> () {
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) -> () {
        unreachable!()
    }
}