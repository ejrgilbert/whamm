use crate::behavior::builder_visitor::SimpleAST;
use crate::behavior::tree::{ActionType, ArgActionType, BehaviorVisitor, DecoratorType};
use crate::behavior::tree::{BehaviorTree, Node};
use crate::common::error::ErrorGen;
use crate::emitter::rewriting::rules::{provider_factory, Arg, LocInfo, WhammProvider};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::rewriting::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::{Expr, Statement};
use log::warn;

fn get_loc_info<'a>(rule: &'a WhammProvider, emitter: &VisitingEmitter) -> Option<LocInfo<'a>> {
    // Pull the curr instr each time this is called to keep from having
    // long-lasting refs into self.emitter.
    emitter.get_loc_info(rule)
}

/// The second phase of instrumenting a Wasm module by actually emitting the
/// instrumentation code.
///
/// To do this, the generator traverses the BehaviorTree AST and calls the
/// passed emitter to emit instrumentation code.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the passed Emitter type.
pub struct InstrGenerator<'a, 'b, 'c, 'd, 'e> {
    pub tree: &'a BehaviorTree,
    pub emitter: VisitingEmitter<'b, 'c, 'd>,
    pub ast: SimpleAST,
    pub err: &'e mut ErrorGen,

    curr_instr_args: Vec<Arg>,
    curr_probe_mode: String,
    /// The current probe's body and predicate
    curr_probe: Option<(Option<Vec<Statement>>, Option<Expr>)>,
}
impl<'a, 'b, 'c, 'd, 'e> InstrGenerator<'a, 'b, 'c, 'd, 'e> {
    pub fn new(
        tree: &'a BehaviorTree,
        emitter: VisitingEmitter<'b, 'c, 'd>,
        ast: SimpleAST,
        err: &'e mut ErrorGen,
    ) -> Self {
        Self {
            tree,
            emitter,
            ast,
            err,
            curr_instr_args: vec![],
            curr_probe_mode: "".to_string(),
            curr_probe: None,
        }
    }

    pub fn configure_probe_mode(&mut self) -> bool {
        // TODO -- make the probe mode an enum!
        match self.curr_probe_mode.as_str() {
            "before" => {
                self.emitter.before()
            },
            "after" => {
                self.emitter.after()
            },
            "alt" => {
                self.emitter.alternate()
            },
            _ => return false,
        }
        true
    }

    pub fn run(&mut self, behavior: &BehaviorTree) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_children();

        // Here we do the following logic:
        // 1. initialize the emitter rules
        // 2. for each instruction (iterate over app_wasm)
        //    1. rules.process_instr() -> Vec<MatchedRules>
        //    2. for each matched probe
        //       1. enter the scope using (script_id, probe_spec)
        //       2. initialize the symbol table with the metadata at this program point
        //       3. create a new clone of the probe, fold the predicate
        //       4. traverse the behavior tree to emit code! (if predicate is not false)

        // Initialize the emitter rules
        let rules = provider_factory::<WhammProvider>(&self.ast.probes);

        // Iterate over each instruction in the application Wasm bytecode
        let mut is_success = true;
        let mut first_instr = true;
        while first_instr || self.emitter.next_instr() {
            first_instr = false;
            rules.iter().for_each(|rule| {
                // Check if any of the configured rules match this instruction in the application.
                if let Some(loc_info) = get_loc_info(rule, &self.emitter) {
                    if loc_info.num_alt_probes > 1 {
                        self.err
                            .multiple_alt_matches(self.emitter.curr_instr_name().as_str());
                    }
                    // This location has matched some rules, inject each matched probe!
                    loc_info.probes.iter().for_each(|(probe_spec, probe)| {
                        // Enter the scope for this matched probe
                        is_success = self
                            .emitter
                            .enter_scope_via_spec(&probe.script_id, probe_spec);

                        // Initialize the symbol table with the metadata at this program point
                        loc_info.static_data.iter().for_each(
                            |(static_var_name, static_var_val)| {
                                if let Err(e) = self.emitter.define(static_var_name, static_var_val)
                                {
                                    self.err.add_error(*e);
                                }
                            },
                        );

                        // Create a new clone of the probe, fold the predicate.
                        // NOTE: We make a clone so that the probe is reset for each instruction!
                        let (body_clone, mut pred_clone) =
                            (probe.body.clone(), probe.predicate.clone());
                        if let Some(pred) = &mut pred_clone {
                            // Fold predicate
                            is_success = self.emitter.fold_expr(pred);

                            // If the predicate evaluates to false, short-circuit!
                            if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                                if !pred_as_bool {
                                    // predicate is reduced to false, short-circuit!
                                    return;
                                }
                            }
                        }

                        self.curr_instr_args = loc_info.args.clone(); // must clone so that this lives long enough
                        self.curr_probe_mode = probe_spec.mode.as_ref().unwrap().name.clone();
                        self.curr_probe = Some((body_clone, pred_clone));

                        // Traverse the behavior tree to emit code (if the predicate is not false)
                        if let Some(root) = behavior.get_root() {
                            // Traverse `behavior` tree and emit the probes held in `ast`
                            is_success = self.visit_root(root);
                        } else {
                            warn!("The behavior tree was empty! Nothing to emit!");
                            is_success &= false;
                        }

                        // Now that we've emitted this probe, reset the symbol table's static/dynamic
                        // data defined for this instr
                        self.emitter.reset_table_data(&loc_info);
                    });
                }
            });
        }

        is_success
    }
}
impl BehaviorVisitor<bool> for InstrGenerator<'_, '_, '_, '_, '_> {
    fn visit_root(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Root { child, .. } = node {
            if let Some(node) = self.tree.get_node(*child) {
                is_success &= self.visit_node(node);
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_sequence(&mut self, node: &Node) -> bool {
        if let Node::Sequence { children, .. } = node {
            for child in children {
                let mut child_is_success = true;
                if let Some(node) = self.tree.get_node(*child) {
                    child_is_success &= self.visit_node(node);
                }
                if !&child_is_success {
                    // If the child was unsuccessful, don't execute the following children
                    // and return `false` (failure)
                    return child_is_success;
                }
            }
        } else {
            unreachable!()
        }
        true
    }

    fn visit_fallback(&mut self, node: &Node) -> bool {
        if let Node::Fallback { children, .. } = node {
            for child in children {
                let mut child_is_success = true;
                if let Some(node) = self.tree.get_node(*child) {
                    child_is_success &= self.visit_node(node);
                }
                if child_is_success {
                    // If that child was successful, don't execute the fallback
                    // and return `true` (success)
                    return child_is_success;
                }
            }
        } else {
            unreachable!()
        }
        // Never successfully executed a child
        false
    }

    fn visit_is_probe_mode(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator {
            ty: DecoratorType::IsProbeMode { probe_mode },
            child,
            ..
        } = node
        {
            if self.curr_probe_mode == *probe_mode {
                if let Some(node) = self.tree.get_node(*child) {
                    is_success &= self.visit_node(node);
                }
            } else {
                // If the decorator condition is false, return false
                return false;
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_has_alt_call(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator {
            ty: DecoratorType::HasAltCall,
            child,
            ..
        } = node
        {
            if self.emitter.has_alt_call() {
                // The current probe has a defined alt call, continue with behavior
                if let Some(node) = self.tree.get_node(*child) {
                    is_success &= self.visit_node(node);
                }
            } else {
                // If the decorator condition is false, return false
                return false;
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_pred_is(&mut self, node: &Node) -> bool {
        if let Node::Decorator {
            ty: DecoratorType::PredIs { val },
            child,
            ..
        } = node
        {
            if let Some((.., pred)) = &self.curr_probe {
                if let Some(pred) = pred {
                    if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
                        // predicate has been reduced to a boolean value
                        if pred_as_bool == *val {
                            // predicate is reduced to desired value, execute child node
                            // first, set the before/after/alt mode of the probe
                            self.configure_probe_mode();
                            if let Some(node) = self.tree.get_node(*child) {
                                return self.visit_node(node);
                            }
                        }
                    }
                } else {
                    // the predicate is not defined, it is automatically true
                    if *val {
                        // first, set the before/after/alt mode of the probe
                        self.configure_probe_mode();
                        if let Some(node) = self.tree.get_node(*child) {
                            return self.visit_node(node);
                        }
                    }
                }
            }
        } else {
            unreachable!()
        }
        false
    }

    fn visit_save_args(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::ArgAction {
            ty: ArgActionType::SaveArgs,
            force_success,
            ..
        } = node
        {
            if !self.curr_instr_args.is_empty() {
                // The current instruction has args, save them (before)
                self.emitter.before();
                is_success &= self.emitter.save_args(&self.curr_instr_args);
            } else {
                // If no args, return whatever was configured to do
                return *force_success;
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_args(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::ArgAction {
            ty: ArgActionType::EmitArgs,
            force_success,
            ..
        } = node
        {
            if !self.curr_instr_args.is_empty() {
                // The current instruction has args, emit them
                // NOTE: The emitter holds on to the saved arg info
                match self.emitter.emit_args() {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            } else {
                // If no args, return whatever was configured to do
                return *force_success;
            }
        } else {
            unreachable!()
        }
        is_success
    }

    // fn visit_emit_global_stmts(&mut self, node: &Node) -> bool {
    //     let mut is_success = true;
    //     if let Node::Action {
    //         ty: ActionType::EmitGlobalStmts,
    //         ..
    //     } = node
    //     {
    //         // NOTE -- this WILL NOT WORK for dfinity or microservice applications...they are stateless
    //         //     will need to instrument ALL entrypoints for that to work :/
    //         // TODO -- fix above noted issue by inserting a new 'start' function if it doesn't exist
    //         if !self.ast.global_stmts.is_empty() {
    //             match self.emitter.emit_global_stmts(&mut self.ast.global_stmts) {
    //                 Err(e) => self.err.add_error(*e),
    //                 Ok(res) => is_success &= res,
    //             }
    //         }
    //     } else {
    //         unreachable!()
    //     }
    //     is_success
    // }

    fn visit_emit_body(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitBody,
            ..
        } = node
        {
            if let Some((Some(ref mut body), ..)) = self.curr_probe {
                match self.emitter.emit_body(body) {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_alt_call(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitAltCall,
            ..
        } = node
        {
            match self.emitter.emit_alt_call() {
                Err(e) => self.err.add_error(*e),
                Ok(res) => is_success &= res,
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_orig(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitOrig,
            ..
        } = node
        {
            is_success &= self.emitter.emit_orig();
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_force_success(&mut self, node: &Node) -> bool {
        if let Node::Action {
            ty: ActionType::ForceSuccess,
            ..
        } = node
        {
            true
        } else {
            unreachable!()
        }
    }

    fn visit_emit_probe_as_if(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitProbeAsIf,
            ..
        } = node
        {
            if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
                match self.emitter.emit_if(pred, body) {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_probe_as_if_else(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitProbeAsIfElse,
            ..
        } = node
        {
            self.configure_probe_mode();
            if let Some((Some(ref mut body), Some(ref mut pred))) = self.curr_probe {
                match self.emitter.emit_if_with_orig_as_else(pred, body) {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }
}
