use crate::behavior::tree::{ActionWithChildType, BehaviorTree, DecoratorType};

use crate::parser::types as parser_types;
use parser_types::{
    BinOp, DataType, Expr, Fn, Script, Statement, UnOp, Value,
    Whamm, WhammVisitor,
};
use std::collections::HashMap;
use crate::parser::rules::{Provider, Package, Event, Probe};

use crate::behavior::tree::DecoratorType::{HasAltCall, PredIs};
use crate::behavior::tree::ParamActionType;
use crate::common::error::ErrorGen;
use crate::parser::types::{Block, ProvidedGlobal};
use log::trace;
use regex::Regex;

// type SimpleAstProbes =
//     HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Box<dyn Probe>>>>>>;

pub struct SimpleAST {
    pub global_stmts: Vec<Statement>,
    // pub probes: SimpleAstProbes,
}

pub fn build_behavior_tree(ast: &Whamm, err: &mut ErrorGen) -> (BehaviorTree, SimpleAST) {
    let mut visitor = BehaviorTreeBuilder {
        tree: BehaviorTree::new(),
        ast: SimpleAST {
            global_stmts: vec![],
            // probes: HashMap::new(),
        },
        err,
        context_name: "".to_string(),
        curr_provider_name: "".to_string(),
        curr_package_name: "".to_string(),
        curr_event_name: "".to_string(),
    };
    visitor.visit_whamm(ast);

    (visitor.tree, visitor.ast)
}

pub struct BehaviorTreeBuilder<'a> {
    pub tree: BehaviorTree,
    pub ast: SimpleAST,
    pub err: &'a mut ErrorGen,

    pub context_name: String,
    curr_provider_name: String,
    curr_package_name: String,
    curr_event_name: String,
}
impl BehaviorTreeBuilder<'_> {
    // =======
    // = AST =
    // =======

    // fn add_provider_to_ast(&mut self, provider_name: String) {
    //     if !self.ast.probes.contains_key(&provider_name) {
    //         self.ast
    //             .probes
    //             .insert(provider_name.clone(), HashMap::new());
    //     }
    //     self.curr_provider_name = provider_name;
    // }
    // 
    // fn add_package_to_ast(&mut self, package_name: String) {
    //     if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
    //         if !provider.contains_key(&package_name) {
    //             provider.insert(package_name.clone(), HashMap::new());
    //         }
    //     } else {
    //         unreachable!()
    //     }
    //     self.curr_package_name = package_name;
    // }
    // 
    // fn add_event_to_ast(&mut self, event_name: String) {
    //     if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
    //         if let Some(package) = provider.get_mut(&self.curr_package_name) {
    //             if !package.contains_key(&event_name) {
    //                 package.insert(event_name.clone(), HashMap::new());
    //             }
    //         }
    //     } else {
    //         unreachable!()
    //     }
    //     self.curr_event_name = event_name;
    // }

    // fn add_probe_to_ast(&mut self, probe: &dyn Probe) {
    //     if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
    //         if let Some(package) = provider.get_mut(&self.curr_package_name) {
    //             if let Some(event) = package.get_mut(&self.curr_event_name) {
    //                 if let Some(probes) = event.get_mut(&probe.mode_name()) {
    //                     // probes.push((*probe).clone());
    //                     todo!()
    //                 } else {
    //                     // event.insert(probe.mode_name().clone(), vec![(*probe).clone()]);
    //                     todo!()
    //                 }
    //             }
    //         }
    //     } else {
    //         unreachable!()
    //     }
    // }

    // ================
    // = BehaviorTree =
    // ================

    fn visit_provided_globals(
        &mut self,
        globals: &HashMap<String, ProvidedGlobal>,
    ) {
        if !globals.is_empty() {
            self.tree.sequence(self.err);

            // visit globals
            for (_name, ProvidedGlobal {global, ..}) in globals.iter() {
                if global.is_comp_provided {
                    if let Expr::VarId { name, .. } = &global.var_name {
                        self.tree
                            .define(self.context_name.clone(), name.clone(), self.err);
                    }
                }
            }
            self.tree.exit_sequence(self.err);
        }
    }

    fn is_in_context(&self, pattern: &str) -> bool {
        let regex = Regex::new(pattern).unwrap();
        matches!(regex.captures(self.context_name.as_str()), Some(_caps))
    }

    fn visit_bytecode_package(&mut self, package: &dyn Package) {
        if package.has_events() {
            // Build events->globals HashMap
            let mut events = HashMap::new();
            for event in package.events() {
                let globals: Vec<String> = event.get_provided_globals().keys().cloned().collect();
                events.insert(event.name(), globals);
            }

            self.tree.action_with_child(
                ActionWithChildType::EnterPackage {
                    context: self.context_name.clone(),
                    package_name: package.name(),
                    events,
                },
                self.err,
            );
            if let Some(event) = package.events().next() {
                // just grab the first one and emit behavior (the decorator above is what
                // makes this apply to all events)
                self.visit_event(event);
            }
            self.tree.exit_action_with_child(self.err);
        }
    }

    fn visit_bytecode_event(&mut self, event: &dyn Event) {
        // Only create a sequence if there are multiple probes we're emitting
        if event.probes().len() > 1 {
            self.tree.sequence(self.err);
        }

        self.visit_probe_mode(event, "before");
        self.visit_probe_mode(event, "alt");
        self.visit_probe_mode(event, "after");

        if event.probes().len() > 1 {
            self.tree.exit_sequence(self.err);
        }
    }

    fn visit_probe_mode(&mut self, event: &dyn Event, ty: &str) {
        if let Some(probes) = event.probes().get(ty) {
            if let Some(probe) = probes.first() {
                // just grab the first one and emit behavior (the behavior includes a loop
                // over all probes of this type)
                self.visit_probe(probe.as_ref());
            }
        }
    }

    fn visit_bytecode_probe(&mut self, probe: &dyn Probe) {
        self.tree
            .sequence(self.err)
            .save_params(true, self.err)
            .fallback(self.err)
            .decorator(PredIs { val: true }, self.err)
            .sequence(self.err)
            .fallback(self.err)
            .decorator(
                DecoratorType::IsProbeMode {
                    probe_mode: "alt".to_string(),
                },
                self.err,
            )
            .remove_orig(self.err)
            .exit_decorator(self.err)
            .force_success(self.err)
            .exit_fallback(self.err)
            .emit_body(self.err)
            .emit_params(true, self.err)
            .fallback(self.err)
            .decorator(HasAltCall, self.err)
            .emit_alt_call(self.err)
            .exit_decorator(self.err)
            .force_success(self.err)
            .exit_fallback(self.err)
            .exit_sequence(self.err)
            .exit_decorator(self.err)
            .fallback(self.err)
            // before behavior
            .decorator(
                DecoratorType::IsProbeMode {
                    probe_mode: "before".to_string(),
                },
                self.err,
            );

        self.emit_bytecode_probe_before_body(probe);
        self.tree
            .exit_decorator(self.err)
            // alt behavior
            .decorator(
                DecoratorType::IsProbeMode {
                    probe_mode: "alt".to_string(),
                },
                self.err,
            );
        self.emit_bytecode_probe_alt_body(probe);
        self.tree
            .exit_decorator(self.err)
            // after behavior
            .decorator(
                DecoratorType::IsProbeMode {
                    probe_mode: "after".to_string(),
                },
                self.err,
            );
        self.emit_bytecode_probe_after_body(probe);
        self.tree
            .exit_decorator(self.err)
            // exit
            .exit_fallback(self.err)
            .exit_fallback(self.err)
            .exit_sequence(self.err);
    }

    fn emit_bytecode_probe_before_body(&mut self, _probe: &dyn Probe) {
        self.tree
            .parameterized_action(ParamActionType::EmitIf { cond: 0, conseq: 1 }, self.err)
            .emit_pred(self.err)
            .emit_body(self.err)
            .exit_parameterized_action(self.err);
    }

    fn emit_bytecode_probe_alt_body(&mut self, _probe: &dyn Probe) {
        self.tree
            .sequence(self.err)
            .remove_orig(self.err)
            .parameterized_action(
                ParamActionType::EmitIfElse {
                    cond: 0,
                    conseq: 1,
                    alt: 2,
                },
                self.err,
            )
            .emit_pred(self.err)
            .sequence(self.err)
            .emit_body(self.err)
            .fallback(self.err)
            .decorator(HasAltCall, self.err)
            .sequence(self.err) // TODO -- remove need for this (just have normal lib::<fn_name>() call syntax)
            // Emit alternate call before emitting parameters so that the location
            // of the alternate call is known to contextualize targeting the right place
            // for emitting the parameters.
            .emit_alt_call(self.err)
            .emit_params(true, self.err)
            .exit_sequence(self.err)
            .exit_decorator(self.err)
            .force_success(self.err)
            .exit_fallback(self.err)
            .exit_sequence(self.err)
            .sequence(self.err)
            // Emit original instruction before emitting parameters so that the location
            // of the original instruction is known to contextualize targeting the right place
            // for emitting the parameters.
            .emit_orig(self.err)
            .emit_params(true, self.err)
            .exit_sequence(self.err)
            .exit_parameterized_action(self.err)
            .exit_sequence(self.err);
    }

    fn emit_bytecode_probe_after_body(&mut self, _probe: &dyn Probe) {
        self.tree
            .parameterized_action(ParamActionType::EmitIf { cond: 0, conseq: 1 }, self.err)
            .emit_pred(self.err)
            .emit_body(self.err)
            .exit_parameterized_action(self.err);
    }
}
impl WhammVisitor<()> for BehaviorTreeBuilder<'_> {
    fn visit_whamm(&mut self, whamm: &Whamm) {
        trace!("Entering: BehaviorTreeBuilder::visit_whamm");
        self.context_name = "whamm".to_string();

        self.tree.sequence(self.err);
        // .enter_scope(self.context_name.clone());

        // visit globals
        self.visit_provided_globals(&whamm.globals);

        // visit scripts
        whamm
            .scripts
            .iter()
            .for_each(|script| self.visit_script(script));

        // self.tree.exit_scope();

        trace!("Exiting: BehaviorTreeBuilder::visit_whamm");
        self.tree.exit_sequence(self.err);
        // Remove from `context_name`
        self.context_name = "".to_string();
    }

    fn visit_script(&mut self, script: &Script) {
        trace!("Entering: BehaviorTreeBuilder::visit_script");
        self.context_name += &format!(":{}", script.name.clone());

        self.tree
            .enter_scope(self.context_name.clone(), script.name.clone(), self.err);

        // NOTE: visit_globals() is no longer needed since initializing user-defined globals is done
        // in the init_generator (which doesn't traverse the behavior tree)
        // RATHER, we process and emit the statements that do anything with the global vars
        // (including declarations since that is an initialization action)
        self.ast.global_stmts = script.global_stmts.to_owned();
        self.tree.emit_global_stmts(self.err);

        script
            .providers
            .iter()
            .for_each(|(_name, provider)| self.visit_provider(provider.as_ref()));

        self.tree.exit_scope(self.err);

        trace!("Exiting: BehaviorTreeBuilder::visit_script");
        // Remove from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
    }

    fn visit_provider(&mut self, provider: &dyn Provider) {
        trace!("Entering: BehaviorTreeBuilder::visit_provider");
        self.context_name += &format!(":{}", provider.name());
        // self.add_provider_to_ast(provider.name());

        self.tree
            .enter_scope(self.context_name.clone(), provider.name(), self.err);

        // visit globals
        self.visit_provided_globals(&provider.get_provided_globals());

        provider
            .packages()
            .for_each(|package| self.visit_package(package));

        self.tree.exit_scope(self.err);

        trace!("Exiting: BehaviorTreeBuilder::visit_provider");
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
    }

    fn visit_package(&mut self, package: &dyn Package) {
        trace!("Entering: BehaviorTreeBuilder::visit_package");
        self.context_name += &format!(":{}", package.name());

        if self.is_in_context(r"whamm:script([0-9]+):wasm:bytecode") {
            self.visit_bytecode_package(package);
        } else if let Some(loc) = &package.loc() {
            self.err.unexpected_error(
                true,
                Some(format!("Package not supported! {}", package.name())),
                Some(loc.line_col.clone()),
            )
        } else {
            self.err.unexpected_error(
                true,
                Some(format!("Package not supported! {}", package.name())),
                None,
            )
        };

        // Handle AST separately since we don't visit every package
        // self.add_package_to_ast(package.name());
        // package.events().for_each(|event| {
        //     self.add_event_to_ast(event.name());
        // 
        //     // Handle AST separately since we don't visit every probe
        //     // event.probes().iter().for_each(|(_mode, probe_list)| {
        //     //     probe_list
        //     //         .iter()
        //     //         .for_each(|probe| self.add_probe_to_ast(probe.as_ref()));
        //     // });
        // });

        trace!("Exiting: BehaviorTreeBuilder::visit_package");
        // Remove this package from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
    }

    fn visit_event(&mut self, event: &dyn Event) {
        trace!("Entering: BehaviorTreeBuilder::visit_event");
        self.context_name += &format!(":{}", event.name());
        // self.add_event_to_ast(event.name());

        if self.is_in_context(r"whamm:script([0-9]+):wasm:bytecode:(.*)") {
            self.visit_bytecode_event(event);
        } else if let Some(loc) = &event.loc() {
            self.err.unexpected_error(
                true,
                Some(format!("Event not supported! {}", event.name())),
                Some(loc.line_col.clone()),
            )
        } else {
            self.err.unexpected_error(
                true,
                Some(format!("Event not supported! {}", event.name())),
                None,
            )
        };

        trace!("Exiting: BehaviorTreeBuilder::visit_event");
        // Remove this event from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
    }

    fn visit_probe(&mut self, probe: &dyn Probe) {
        trace!("Entering: BehaviorTreeBuilder::visit_probe");
        self.context_name += &format!(":{}", probe.mode_name());
        // self.add_probe_to_ast(probe);

        self.tree.action_with_child(
            ActionWithChildType::EnterProbe {
                context: self.context_name.clone(),
                probe_mode: probe.mode_name(),
                global_names: probe.get_mode_provided_globals().keys().cloned().collect(),
            },
            self.err,
        );

        if self.is_in_context(r"whamm:script([0-9]+):wasm:bytecode:(.*)") {
            self.visit_bytecode_probe(probe);
        } else {
            self.err.unexpected_error(
                true,
                Some(format!("Probe not supported! {}", self.context_name)),
                None,
            );
        };

        trace!("Exiting: BehaviorTreeBuilder::visit_probe");
        self.tree.exit_action_with_child(self.err);
        // Remove this probe from `context_name`
        self.context_name = self.context_name[..self.context_name.rfind(':').unwrap()].to_string();
    }

    fn visit_fn(&mut self, _f: &Fn) {
        unreachable!()
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) {
        unreachable!()
    }

    fn visit_block(&mut self, _block: &Block) {
        unreachable!()
    }

    fn visit_stmt(&mut self, _assign: &Statement) {
        // Not visiting event/probe bodies
        unreachable!()
    }

    fn visit_expr(&mut self, _call: &Expr) {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_unop(&mut self, _unop: &UnOp) {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_binop(&mut self, _binop: &BinOp) {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_datatype(&mut self, _datatype: &DataType) {
        // Not visiting predicates/statements
        unreachable!()
    }

    fn visit_value(&mut self, _val: &Value) {
        // Not visiting predicates/statements
        unreachable!()
    }
}
