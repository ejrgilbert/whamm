use crate::behavior::tree::{ActionWithChildType, BehaviorTree, DecoratorType};

use std::collections::HashMap;
use crate::parser::types as parser_types;
use parser_types::{DataType, Whammy, Whamm, WhammVisitor, Expr, Fn, Event, Package, Op, Probe, Provider, Statement, Value};

use log::{debug, error, trace};
use regex::Regex;
use crate::behavior::tree::ParamActionType;
use crate::behavior::tree::DecoratorType::{HasParams, PredIs};
use crate::parser::types::Global;

pub fn build_behavior_tree(ast: &Whamm) -> (BehaviorTree, HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Probe>>>>>) {
    let mut visitor = BehaviorTreeBuilder::new();
    visitor.visit_whamm(ast);

    debug!("{:#?}", visitor.ast);
    (visitor.tree, visitor.ast)
}

pub struct BehaviorTreeBuilder {
    pub tree: BehaviorTree,
    // TODO -- should also generate a Consolidated AST:
    pub ast: HashMap<String, //     <-- provider
                     HashMap<String, //     <-- package
                             HashMap<String, //     <-- event
                                     HashMap<String, //     <-- probe name
                                         Vec<Probe>
                                     >
                             >
                     >
             >,
    pub context_name: String,
    curr_provider_name: String,
    curr_package_name: String,
    curr_event_name: String
}
impl BehaviorTreeBuilder {
    pub fn new() -> Self {
        Self {
            tree: BehaviorTree::new(),
            ast: HashMap::new(),
            context_name: "".to_string(),
            curr_provider_name: "".to_string(),
            curr_package_name: "".to_string(),
            curr_event_name: "".to_string()
        }
    }
}
impl BehaviorTreeBuilder {
    // =======
    // = AST =
    // =======

    fn add_provider_to_ast(&mut self, provider_name: String) {
        if !self.ast.contains_key(&provider_name) {
            self.ast.insert(provider_name.clone(), HashMap::new());
        }
        self.curr_provider_name = provider_name;
    }

    fn add_package_to_ast(&mut self, package_name: String) {
        if let Some(provider) = self.ast.get_mut(&self.curr_provider_name) {
            if !provider.contains_key(&package_name) {
                provider.insert(package_name.clone(), HashMap::new());
            }
        } else {
            unreachable!()
        }
        self.curr_package_name = package_name;
    }

    fn add_event_to_ast(&mut self, event_name: String) {
        if let Some(provider) = self.ast.get_mut(&self.curr_provider_name) {
            if let Some(package) = provider.get_mut(&self.curr_package_name) {
                if !package.contains_key(&event_name) {
                    package.insert(event_name.clone(), HashMap::new());
                }
            }
        } else {
            unreachable!()
        }
        self.curr_event_name = event_name;
    }

    fn add_probe_to_ast(&mut self, probe: &Probe) {
        if let Some(provider) = self.ast.get_mut(&self.curr_provider_name) {
            if let Some(package) = provider.get_mut(&self.curr_package_name) {
                if let Some(event) = package.get_mut(&self.curr_event_name) {
                    if let Some(probes) = event.get_mut(&probe.name) {
                        probes.push((*probe).clone());
                    } else {
                        event.insert(probe.name.clone(), vec![(*probe).clone()]);
                    }
                }


            }
        } else {
            unreachable!()
        }
    }

    // ================
    // = BehaviorTree =
    // ================

    fn visit_globals(&mut self, globals: &HashMap<String, Global>) {
        if globals.len() > 0 {
            self.tree.sequence();

            // visit globals
            for (_name, global) in globals.iter() {
                if global.is_comp_provided {
                    if let Expr::VarId { name } = &global.var_name {
                        self.tree.define(self.context_name.clone(),
                                         name.clone());
                    }
                }
            }
            self.tree.exit_sequence();
        }
    }

    fn is_in_context(&self, pattern: &str) -> bool {
        let regex = Regex::new(pattern).unwrap();
        if let Some(_caps) = regex.captures(self.context_name.as_str()) {
            true
        } else {
            false
        }
    }

    fn visit_bytecode_package(&mut self, package: &Package) {
        if package.events.len() > 0 {
            self.tree.action_with_child(ActionWithChildType::EnterPackage {
                    package_name: package.name.clone()
                })
                .decorator(DecoratorType::IsInstr {
                    instr_names: package.events.keys().cloned().collect(),
                });
            for (_name, event) in package.events.iter() {
                // just grab the first one and emit behavior (the decorator above is what
                // makes this apply to all events)
                self.visit_event(event);
                break;
            }
            self.tree.exit_decorator();
            self.tree.exit_action_with_child();
        }
    }

    fn visit_bytecode_event(&mut self, event: &Event) {
        // self.tree.decorator(DecoratorType::IsInstr {
        //         instr_name: event.name.clone()
        //     }).sequence()
        //     .enter_scope(self.context_name.clone());
        self.tree.sequence()
            .enter_scope(self.context_name.clone());

        // Define globals
        self.visit_globals(&event.globals);

        self.visit_probe_ty(event, "before");
        self.visit_probe_ty(event, "alt");
        self.visit_probe_ty(event, "after");

        self.tree.exit_scope();
        self.tree.exit_sequence();
    }

    fn visit_probe_ty(&mut self, event: &Event, ty: &str) {
        if let Some(probes) = event.probe_map.get(ty) {
            if let Some(probe) = probes.get(0) {
                // just grab the first one and emit behavior (the behavior includes a loop
                // over all probes of this type)
                self.visit_probe(probe);
            }
        }
    }

    fn visit_bytecode_probe(&mut self, probe: &Probe) {
        self.tree.fold_pred()
            .fallback()
                .decorator(PredIs {
                    val: false
                })
                    .force_success()
                    .exit_decorator()
                .sequence()
                    .decorator(HasParams)
                        .save_params()
                    .exit_decorator()
                    .fallback()
                        .decorator(PredIs {
                            val: true
                        })
                            .sequence()
                                .emit_body()
                                .fallback()
                                    .decorator(HasParams)
                                        .emit_params()
                                        .exit_decorator()
                                    .force_success()
                                    .exit_fallback()
                            .exit_sequence()
                        .exit_decorator()
                            .fallback()
                            // before behavior
                            .decorator(DecoratorType::IsProbeType {
                                probe_type: "before".to_string()
                            });

        self.emit_bytecode_probe_before_body(probe);
        self.tree.exit_decorator()
            // alt behavior
            .decorator(DecoratorType::IsProbeType {
                probe_type: "alt".to_string()
            });
        self.emit_bytecode_probe_alt_body(probe);
        self.tree.exit_decorator()
            // after behavior
            .decorator(DecoratorType::IsProbeType {
                probe_type: "after".to_string()
            });
        self.emit_bytecode_probe_after_body(probe);
        self.tree.exit_decorator()
            // exit
            .exit_fallback()
            .exit_fallback()
            .exit_sequence()
            .exit_scope()
            .exit_fallback();
    }

    fn emit_bytecode_probe_before_body(&mut self, _probe: &Probe) {
        self.tree.parameterized_action(ParamActionType::EmitIf {
            cond: 0,
            conseq: 1
        })
            .emit_pred()
            .emit_body()
            .exit_parameterized_action();
    }

    fn emit_bytecode_probe_alt_body(&mut self, _probe: &Probe) {
        self.tree.sequence()
            .remove_orig()
            .parameterized_action(ParamActionType::EmitIfElse {
                cond: 0,
                conseq: 1,
                alt: 2
            })
                .emit_pred()
                .sequence()
                    .emit_body()
                    .emit_alt_call() // TODO -- remove need for this
                    .exit_sequence()
                .sequence()
                    .decorator(HasParams)
                        .emit_params()
                        .exit_decorator()
                    .emit_orig()
                    .exit_sequence()
                .exit_parameterized_action()
            .exit_sequence();
    }

    fn emit_bytecode_probe_after_body(&mut self, _probe: &Probe) {
        self.tree.parameterized_action(ParamActionType::EmitIf {
            cond: 0,
            conseq: 1
        })
            .emit_pred()
            .emit_body()
            .exit_parameterized_action();
    }
}
impl WhammVisitor<()> for BehaviorTreeBuilder {
    fn visit_whamm(&mut self, whamm: &Whamm) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_whamm");
        self.context_name  = "whamm".to_string();

        self.tree.sequence()
            .enter_scope(self.context_name.clone());

        // visit globals
        self.visit_globals(&whamm.globals);

        // visit whammys
        whamm.whammys.iter().for_each(| whammy | self.visit_whammy(whammy));

        self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_whamm");
        self.tree.exit_sequence();
        // Remove from `context_name`
        self.context_name = "".to_string();
    }

    fn visit_whammy(&mut self, whammy: &Whammy) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_whammy");
        self.context_name += &format!(":{}", whammy.name.clone());

        self.tree.enter_scope(self.context_name.clone());

        // visit globals
        self.visit_globals(&whammy.globals);

        whammy.providers.iter().for_each(| (_name, provider) | {
            self.visit_provider(provider)
        });

        self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_whammy");
        // Remove from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
    }

    fn visit_provider(&mut self, provider: &Provider) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_provider");
        self.context_name += &format!(":{}", provider.name.clone());
        self.add_provider_to_ast(provider.name.clone());

        self.tree.enter_scope(self.context_name.clone());

        // visit globals
        self.visit_globals(&provider.globals);

        provider.packages.iter().for_each(| (_name, package) | {
            self.visit_package(package)
        });

        self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_provider");
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
    }

    fn visit_package(&mut self, package: &Package) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_package");
        self.context_name += &format!(":{}", package.name.clone());
        self.add_package_to_ast(package.name.clone());

        self.tree.enter_scope(self.context_name.clone());

        if self.is_in_context(r"whamm:whammy([0-9]+):wasm:bytecode") {
            self.visit_bytecode_package(package);
        } else {
            error!("Unsupported package: {}", package.name);
        };

        self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_package");
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
    }

    fn visit_event(&mut self, event: &Event) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_event");
        self.context_name += &format!(":{}", event.name.clone());
        self.add_event_to_ast(event.name.clone());

        if self.is_in_context(r"whamm:whammy([0-9]+):wasm:bytecode:(.*)") {
            self.visit_bytecode_event(event);
        } else {
            error!("Unsupported event: {}", event.name);
        };

        trace!("Exiting: BehaviorTreeBuilder::visit_event");
        // Remove this event from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
    }

    fn visit_probe(&mut self, probe: &Probe) -> () {
        trace!("Entering: BehaviorTreeBuilder::visit_probe");
        self.context_name += &format!(":{}", probe.name.clone());
        self.add_probe_to_ast(probe);

        if probe.name == "alt" {
            self.tree.decorator(DecoratorType::ForFirstProbe {
                target: probe.name.clone()
            });
        } else {
            self.tree.decorator(DecoratorType::ForEachProbe {
                target: probe.name.clone()
            });
        }
        self.tree.sequence()
                .enter_scope(self.context_name.clone());

        // visit globals
        self.visit_globals(&probe.globals);

        if self.is_in_context(r"whamm:whammy([0-9]+):wasm:bytecode:(.*)") {
            self.visit_bytecode_probe(probe);
        } else {
            error!("Unsupported probe: {}", self.context_name);
        };

        self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_probe");
        self.tree.exit_sequence()
            .exit_decorator();
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(":").unwrap()].to_string();
    }

    fn visit_fn(&mut self, _f: &Fn) -> () {
        unreachable!()
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) -> () {
        unreachable!()
    }

    fn visit_stmt(&mut self, _assign: &Statement) -> () {
        // Not visiting event/probe bodies
        unreachable!()
    }

    fn visit_expr(&mut self, _call: &Expr) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_op(&mut self, _op: &Op) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_datatype(&mut self, _datatype: &DataType) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) -> () {
        // Not visiting predicates/statements
        unreachable!()
    }
}