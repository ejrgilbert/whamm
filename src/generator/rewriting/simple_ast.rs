use crate::parser::rules::{Event, Package, Probe, Provider};
use crate::parser::types as parser_types;
use parser_types::{Expr, Fn, Script, Statement, Whamm, WhammVisitor};
use std::collections::HashMap;

use crate::common::error::ErrorGen;
use crate::parser::rules::core::WhammModeKind;
use crate::parser::types::{BinOp, Block, DataType, UnOp, Value};
use log::trace;

/// This is a structure that saves a simplified variation of the activated
/// probe rules.
/// Note that this does not explicitly represent a "Script" (which was used in the
/// previous AST representation to keep instrumentation "units" together). This
/// design is important to support composable instrumentation that imposes an ordering
/// to the injected code. If multiple scripts probe the same point in an application,
/// the injection ordering should follow the ordering of the scripts as specified by the
/// user.
/// While this is not explicitly retained, there is still a convention that is followed by
/// construction of the BehaviorTree AST visitation logic that imposes this requirement. Since
/// the AST scripts are followed in-order (they're saved to an ordered Vec type), the collapsing
/// into this new AST representation retains the intended order as well.
/// Consider the following example (where <script0_body0> is used to refer to the contents of the probe for readability):
/// Script0 {
///     wasm:opcode:call:before {<script0_body0>}
///     wasm:opcode:call:before {<script0_body1>}
///     wasm:opcode:call:after {<script0_body2>}
/// }
/// Script1 {
///     wasm:opcode:call:before {<script1_body0>}
/// }
///
/// This will translate to the following structure in the `SimpleAstProbes` type:
/// {
///     "wasm" -> {
///         "opcode" -> {
///             "call" -> {
///                 "before" -> {
///                     <script0_body0>,
///                     <script0_body1>,
///                     <script1_body0>,
///                 }
///                 "after" -> {
///                     <script0_body2>
///                 }
///             }
///         }
///     }
/// }
/// The code generator will then take this and iterate over bodies to be injected at each probed point
/// in the application code in the same order as the scripts were passed to the `whamm!` tool.
///
/// Note: The Probes here are owned by this new AST structure. This is to simplify some logic in the
/// emitter. Holding on to a reference to the original AST complicates the lifetimes of building a new
/// AST representation since we're heavily reliant on traits. Since traits must have a static lifetime,
/// mixing that with a non-static lifetimes keeps from having simple factory code.
/// As a workaround, we know that the original AST isn't really needed at this point, so we have the new
/// AST representation own the Probes instead!
///
/// Note: This AST representation will only be used for bytecode rewriting, not when targeting Wizard.
pub type SimpleAstProbes =
    HashMap<String, HashMap<String, HashMap<String, HashMap<WhammModeKind, Vec<SimpleProbe>>>>>;
#[derive(Clone, Debug)]
pub struct SimpleProbe {
    pub script_id: u8,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub num_unshared: HashMap<DataType, i32>,
    pub maps_unshared: HashMap<DataType, (String, bool)>,
    pub probe_number: u32,
}
impl SimpleProbe {
    fn new(
        script_id: u8,
        probe: &dyn Probe,
        num_unshared: HashMap<DataType, i32>,
        maps_unshared: HashMap<DataType, (String, bool)>,
    ) -> Self {
        Self {
            script_id,
            predicate: probe.predicate().to_owned(),
            body: probe.body().to_owned(),
            num_unshared,
            maps_unshared,
            probe_number: probe.id(),
        }
    }
}

pub struct SimpleAST {
    pub global_stmts: Vec<Statement>,
    /// This points to probes defined in the `Whamm` AST node!
    pub probes: SimpleAstProbes,
}
impl Default for SimpleAST {
    fn default() -> Self {
        Self::new()
    }
}
impl SimpleAST {
    pub fn new() -> Self {
        Self {
            global_stmts: vec![],
            probes: HashMap::new(),
        }
    }
}

pub fn build_simple_ast(ast: &Whamm, err: &mut ErrorGen) -> SimpleAST {
    let mut simple_ast = SimpleAST::new();
    let mut visitor = SimpleASTBuilder {
        ast: &mut simple_ast,
        err,
        script_id: u8::MAX,
        curr_provider_name: "".to_string(),
        curr_package_name: "".to_string(),
        curr_event_name: "".to_string(),
        curr_unshared: HashMap::default(),
        maps_unshared: HashMap::default(),
    };
    visitor.visit_whamm(ast);

    simple_ast
}

pub struct SimpleASTBuilder<'a, 'b> {
    pub ast: &'a mut SimpleAST,
    pub err: &'b mut ErrorGen,

    script_id: u8,
    curr_provider_name: String,
    curr_package_name: String,
    curr_event_name: String,
    curr_unshared: HashMap<DataType, i32>,
    maps_unshared: HashMap<DataType, (String, bool)>,
}
impl SimpleASTBuilder<'_, '_> {
    // =======
    // = AST =
    // =======

    fn add_provider_to_ast(&mut self, provider_name: String) {
        if !self.ast.probes.contains_key(&provider_name) {
            self.ast
                .probes
                .insert(provider_name.clone(), HashMap::new());
        }
        self.curr_provider_name = provider_name;
    }

    fn add_package_to_ast(&mut self, package_name: String) {
        if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
            if !provider.contains_key(&package_name) {
                provider.insert(package_name.clone(), HashMap::new());
            }
        } else {
            unreachable!()
        }
        self.curr_package_name = package_name;
    }

    fn add_event_to_ast(&mut self, event_name: String) {
        if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
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

    fn add_probe_to_ast(
        &mut self,
        probe: &dyn Probe,
        num_unshared: HashMap<DataType, i32>,
        maps_unshared: HashMap<DataType, (String, bool)>,
    ) {
        if let Some(provider) = self.ast.probes.get_mut(&self.curr_provider_name) {
            if let Some(package) = provider.get_mut(&self.curr_package_name) {
                if let Some(event) = package.get_mut(&self.curr_event_name) {
                    if let Some(probes) = event.get_mut(&probe.mode()) {
                        probes.push(SimpleProbe::new(
                            self.script_id,
                            probe,
                            num_unshared,
                            maps_unshared,
                        ));
                    } else {
                        event.insert(
                            probe.mode(),
                            vec![SimpleProbe::new(
                                self.script_id,
                                probe,
                                num_unshared,
                                maps_unshared,
                            )],
                        );
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
}
impl WhammVisitor<()> for SimpleASTBuilder<'_, '_> {
    fn visit_whamm(&mut self, whamm: &Whamm) {
        trace!("Entering: BehaviorTreeBuilder::visit_whamm");

        // visit scripts
        whamm
            .scripts
            .iter()
            .for_each(|script| self.visit_script(script));

        trace!("Exiting: BehaviorTreeBuilder::visit_whamm");
        // Remove from `context_name`
    }

    fn visit_script(&mut self, script: &Script) {
        trace!("Entering: BehaviorTreeBuilder::visit_script");
        self.script_id = script.id;

        // NOTE: visit_globals() is no longer needed since initializing user-defined globals is done
        // in the init_generator (which doesn't traverse the behavior tree)
        // RATHER, we process and emit the statements that do anything with the global vars
        // (including declarations since that is an initialization action)
        self.ast.global_stmts = script.global_stmts.to_owned();

        script
            .providers
            .iter()
            .for_each(|(_name, provider)| self.visit_provider(provider));

        trace!("Exiting: BehaviorTreeBuilder::visit_script");
        self.script_id = u8::MAX
    }

    fn visit_provider(&mut self, provider: &Box<dyn Provider>) {
        trace!("Entering: BehaviorTreeBuilder::visit_provider");
        self.add_provider_to_ast(provider.name());

        provider
            .packages()
            .for_each(|package| self.visit_package(package));

        trace!("Exiting: BehaviorTreeBuilder::visit_provider");
    }

    fn visit_package(&mut self, package: &dyn Package) {
        trace!("Entering: BehaviorTreeBuilder::visit_package");
        // NOTE: Here we add a script's unit of instrumentation which retains
        // the script order as passed by the user during `whamm!` tool invocation.
        // This is guaranteed since we visit Scripts in order of the Vec and then
        // the in-unit order is retained as well since there is an ordering of the
        // Vec of probes contained by an Event.
        // Handle AST separately since we don't visit every package
        self.add_package_to_ast(package.name());
        package.events().for_each(|event| {
            self.visit_event(event);
        });

        trace!("Exiting: BehaviorTreeBuilder::visit_package");
    }

    fn visit_event(&mut self, event: &dyn Event) {
        trace!("Entering: BehaviorTreeBuilder::visit_event");
        self.add_event_to_ast(event.name());
        event.probes().iter().for_each(|(_mode, probe_list)| {
            probe_list.iter().for_each(|probe| {
                self.visit_probe(probe);
            });
        });
        trace!("Exiting: BehaviorTreeBuilder::visit_event");
    }

    fn visit_probe(&mut self, probe: &Box<dyn Probe>) {
        trace!("Entering: BehaviorTreeBuilder::visit_probe");
        //visit the statements in the probe and check for report_decls
        if probe.body().is_none() {
            self.add_probe_to_ast(probe.as_ref(), HashMap::default(), HashMap::default());
            return;
        }
        for stmt in &probe.body().as_ref().unwrap().stmts {
            self.visit_stmt(stmt);
        }
        self.add_probe_to_ast(
            probe.as_ref(),
            self.curr_unshared.to_owned(),
            self.maps_unshared.to_owned(),
        );
        self.curr_unshared = HashMap::default();
        self.maps_unshared = HashMap::default();
        trace!("Exiting: BehaviorTreeBuilder::visit_probe");
    }

    fn visit_fn(&mut self, _f: &Fn) {
        todo!()
    }

    fn visit_formal_param(&mut self, _param: &(Expr, DataType)) {
        todo!()
    }

    fn visit_block(&mut self, block: &Block) {
        trace!("Entering: BehaviorTreeBuilder::visit_block");
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        trace!("Exiting: BehaviorTreeBuilder::visit_block");
    }

    fn visit_stmt(&mut self, stmt: &Statement) {
        trace!("Entering: BehaviorTreeBuilder::visit_stmt");
        // for checking for report_decls
        match stmt {
            Statement::UnsharedDecl {
                decl, is_report, ..
            } => {
                if let Statement::Decl { ty, var_id, .. } = decl.as_ref() {
                    if matches!(ty, DataType::Map { .. }) {
                        let Expr::VarId { name, .. } = var_id else {
                            unreachable!(
                                "BUG: Expected Decl to have a VarId expression, but it didn't..."
                            )
                        };
                        self.maps_unshared
                            .insert(ty.clone(), (name.clone(), *is_report));
                    } else {
                        self.curr_unshared
                            .entry(ty.clone())
                            .and_modify(|count| {
                                *count += 1;
                            })
                            .or_insert(1);
                    }
                } else {
                    unreachable!()
                }
            }
            Statement::If { conseq, alt, .. } => {
                self.visit_block(conseq);
                self.visit_block(alt);
            }
            _ => {}
        }
        trace!("Exiting: BehaviorTreeBuilder::visit_stmt");
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
