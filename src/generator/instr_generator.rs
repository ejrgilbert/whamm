use crate::behavior::builder_visitor::SimpleAST;
use crate::behavior::tree::{
    ActionType, ArgActionType, BehaviorVisitor, DecoratorType, ParamActionType,
};
use crate::behavior::tree::{BehaviorTree, Node};
use crate::common::error::ErrorGen;
use crate::emitter::rewriting::rules::{provider_factory, LocInfo, WhammProvider};
use crate::emitter::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::{Expr, Statement};
use log::warn;
use walrus::ValType;

const UNEXPECTED_ERR_MSG: &str =
    "InstrGenerator: Looks like you've found a bug...please report this behavior!";
fn get_loc_info<'a>(
    rule: &'a WhammProvider,
    emitter: &Box<&mut dyn Emitter>,
) -> Option<LocInfo<'a>> {
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
pub struct InstrGenerator<'a, 'b> {
    pub tree: &'a BehaviorTree,
    pub emitter: Box<&'b mut dyn Emitter>,
    pub ast: SimpleAST,
    pub err: &'a mut ErrorGen,

    // pub context_name: String,
    // pub curr_provider_name: String,
    // pub curr_package_name: String,
    // pub curr_event_name: String,
    curr_instr_args: Vec<ValType>,
    curr_probe_mode: String,
    /// The current probe's body and predicate
    curr_probe: Option<(Option<Vec<Statement>>, Option<Expr>)>,
}
impl<'a, 'b> InstrGenerator<'a, 'b> {
    pub fn new(
        tree: &'a BehaviorTree,
        emitter: Box<&'b mut dyn Emitter>,
        ast: SimpleAST,
        err: &'a mut ErrorGen,
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

        // Initialize the instr visitor
        if let Err(e) = self.emitter.init_instr_iter() {
            self.err.add_error(*e)
        }

        // Iterate over each instruction in the application Wasm bytecode
        let mut is_success = true;
        let mut first_instr = true;
        while first_instr || self.emitter.has_next_instr() {
            if first_instr {
                is_success = self.emitter.init_first_instr();
            }
            if !first_instr {
                is_success = self.emitter.next_instr();
            }
            first_instr = false;

            rules.iter().for_each(|rule| {
                // Check if any of the configured rules match this instruction in the application.
                if let Some(loc_info) = get_loc_info(rule, &self.emitter) {
                    if loc_info.num_alt_probes > 1 {
                        self.err
                            .multiple_alt_matches(self.emitter.curr_instr_name());
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
                        // todo -- how do i do dynamic data

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

    // fn set_context_info(&mut self, context: &str) {
    //     // Set the current context info for probe lookup
    //     self.context_name = context.to_string();
    //
    //     let mut spec_split = context.split(':');
    //     if let Some(_whamm) = spec_split.next() {
    //         if let Some(_script) = spec_split.next() {
    //             if let Some(provider) = spec_split.next() {
    //                 self.curr_provider_name = provider.to_string();
    //                 if let Some(package) = spec_split.next() {
    //                     self.curr_package_name = package.to_string();
    //                     if let Some(event) = spec_split.next() {
    //                         self.curr_event_name = event.to_string();
    //                         if let Some(mode) = spec_split.next() {
    //                             self.curr_probe_mode = mode.to_string()
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    fn emit_cond(&mut self, cond: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(*cond) {
            // emit the branch conditional
            self.emitter.emit_condition();
            is_success &= self.visit_node(node);
        } else {
            self.err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Node to define conditional logic node does not exist!"
                )),
                None,
            );
        }
        is_success
    }

    fn emit_conseq(&mut self, conseq: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(*conseq) {
            // emit the consequent logic
            self.emitter.emit_consequent();
            is_success &= self.visit_node(node);
        } else {
            self.err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Node to define consequent logic node does not exist!"
                )),
                None,
            );
        }
        is_success
    }

    fn emit_alt(&mut self, alt: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(*alt) {
            // emit the alternate logic
            self.emitter.emit_alternate();
            is_success &= self.visit_node(node);
        } else {
            self.err.unexpected_error(
                true,
                Some(format!(
                    "{UNEXPECTED_ERR_MSG} Node to define alternate logic node does not exist!"
                )),
                None,
            );
        }
        is_success
    }
}
impl BehaviorVisitor<bool> for InstrGenerator<'_, '_> {
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
                            if let Some(node) = self.tree.get_node(*child) {
                                return self.visit_node(node);
                            }
                        }
                    }
                } else {
                    // the predicate is not defined, it is automatically true
                    if *val {
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
                // The current instruction has args, save them
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

    // fn visit_enter_package(&mut self, node: &Node) -> bool {
    //     let mut is_success = true;
    //     if let Node::ActionWithChild { ty, child, .. } = node {
    //         if let ActionWithChildType::EnterPackage {
    //             context,
    //             package_name,
    //             events,
    //         } = ty
    //         {
    //             if package_name == "opcode" {
    //                 // Perform 'opcode' package logic
    //
    //                 // Initialize the instr visitor
    //                 let instrs_of_interest: Vec<String> = events.keys().cloned().collect();
    //                 if let Err(e) = self.emitter.init_instr_iter() {
    //                     self.err.add_error(*e)
    //                 }
    //
    //                 // enter 'opcode' scope
    //                 if !self.emitter.enter_named_scope(package_name) {
    //                     self.err.unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find the specified scope by name: `{}`", package_name)), None);
    //                 }
    //                 self.set_context_info(context);
    //
    //                 let mut first_instr = true;
    //                 while first_instr || self.emitter.has_next_instr() {
    //                     if first_instr {
    //                         self.emitter.init_first_instr();
    //                     }
    //                     if !&first_instr {
    //                         self.emitter.next_instr();
    //                     }
    //
    //                     let instr_ty = match self.emitter.curr_instr_type().as_str() {
    //                         // Handle some special-cases
    //                         "V128Bitselect" => "v128_bitselect".to_string(),
    //                         "I8x16Swizzle" => "i8x16_swizzle".to_string(),
    //                         "I8x16Shuffle" => "i8x16_shuffle".to_string(),
    //                         other => other.to_case(Case::Snake),
    //                     };
    //
    //                     // is this an instruction of-interest?
    //                     if let Some(globals) = events.get(&instr_ty) {
    //                         // enter this event's scope
    //                         if !self.emitter.enter_named_scope(&instr_ty) {
    //                             self.err.unexpected_error(true, Some(format!("{UNEXPECTED_ERR_MSG} Could not find the specified scope by name: `{}`", instr_ty)), None);
    //                         }
    //                         self.curr_event_name = instr_ty.clone();
    //
    //                         // define this instruction type's compiler variables
    //                         for global in globals {
    //                             match self.emitter.define_compiler_var(&self.context_name, global) {
    //                                 Err(e) => self.err.add_error(*e),
    //                                 Ok(res) => is_success &= res,
    //                             }
    //                         }
    //
    //                         // continue with logic
    //                         if let Some(node) = self.tree.get_node(*child) {
    //                             is_success &= self.visit_node(node);
    //                         }
    //
    //                         // exit this event's scope
    //                         if let Err(e) = self.emitter.exit_scope() {
    //                             self.err.add_error(*e)
    //                         }
    //                     }
    //                     first_instr = false;
    //                 }
    //
    //                 if let Err(e) = self.emitter.exit_scope() {
    //                     self.err.add_error(*e)
    //                 }
    //             }
    //         }
    //     } else {
    //         unreachable!()
    //     }
    //     is_success
    // }


    // fn visit_enter_probe(&mut self, node: &Node) -> bool {
    //     let mut is_success = true;
    //     if let Node::ActionWithChild {
    //         ty:
    //             ActionWithChildType::EnterProbe {
    //                 probe_mode,
    //                 global_names,
    //                 ..
    //             },
    //         child,
    //         ..
    //     } = node
    //     {
    //         // enter probe's scope
    //         if !self.emitter.enter_named_scope(probe_mode) {
    //             self.err.unexpected_error(
    //                 true,
    //                 Some(format!(
    //                     "{UNEXPECTED_ERR_MSG} Could not find the specified scope by name: `{}`",
    //                     probe_mode
    //                 )),
    //                 None,
    //             );
    //         }
    //         self.curr_probe_mode = probe_mode.clone();
    //
    //         // define this probe's compiler variables
    //         for global in global_names {
    //             match self.emitter.define_compiler_var(&self.context_name, global) {
    //                 Err(e) => self.err.add_error(*e),
    //                 Ok(res) => is_success &= res,
    //             }
    //         }
    //         if probe_mode == "before" || probe_mode == "after" {
    //             // Perform 'before' and 'after' probe logic
    //             // Must pull the probe by index due to Rust calling constraints...
    //             let probe_list_len = self
    //                 .ast
    //                 .get_probes_from_ast(
    //                     &self.curr_provider_name,
    //                     &self.curr_package_name,
    //                     &self.curr_event_name,
    //                     probe_mode,
    //                 )
    //                 .len();
    //             for i in Vec::from_iter(0..probe_list_len).iter() {
    //                 if let Some(probe) = self.ast.get_probe_at_idx(
    //                     &self.curr_provider_name,
    //                     &self.curr_package_name,
    //                     &self.curr_event_name,
    //                     probe_mode,
    //                     i,
    //                 ) {
    //                     // make a clone of the current probe per instruction traversal
    //                     // this will reset the clone pred/body for each instruction!
    //                     let (body_cloned, mut pred_cloned) =
    //                         ((*probe).body.clone(), (*probe).predicate.clone());
    //                     if let Some(pred) = &mut pred_cloned {
    //                         // Fold predicate
    //                         is_success &= self.emitter.fold_expr(pred);
    //
    //                         // If the predicate evaluates to false, short-circuit!
    //                         if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
    //                             // predicate has been reduced to a boolean value
    //                             if !pred_as_bool {
    //                                 // predicate is reduced to `false` short-circuit!
    //                                 if let Err(e) = self.emitter.exit_scope() {
    //                                     self.err.add_error(*e)
    //                                 }
    //                                 return true;
    //                             }
    //                         }
    //                     }
    //
    //                     self.curr_probe = Some((body_cloned, pred_cloned));
    //                 }
    //
    //                 // Process the instructions for this probe!
    //                 if let Some(node) = self.tree.get_node(*child) {
    //                     is_success &= self.visit_node(node);
    //                 }
    //             }
    //         } else if probe_mode == "alt" {
    //             // Perform 'alt' probe logic
    //             let probe_list = self.ast.get_probes_from_ast(
    //                 &self.curr_provider_name,
    //                 &self.curr_package_name,
    //                 &self.curr_event_name,
    //                 probe_mode,
    //             );
    //             if probe_list.len() > 1 {
    //                 warn!("There is more than one probe for probe type '{}'. So only emitting first probe, ignoring rest.", probe_mode)
    //             }
    //             // make a clone of the first probe per instruction traversal
    //             // this will reset the clone pred/body for each instruction!
    //             if let Some(probe) = probe_list.first() {
    //                 let (body_cloned, mut pred_cloned) =
    //                     ((*probe).body.clone(), (*probe).predicate.clone());
    //                 if let Some(pred) = &mut pred_cloned {
    //                     // Fold predicate
    //                     is_success &= self.emitter.fold_expr(pred);
    //
    //                     // If the predicate evaluates to false, short-circuit!
    //                     if let Some(pred_as_bool) = ExprFolder::get_single_bool(pred) {
    //                         // predicate has been reduced to a boolean value
    //                         if !pred_as_bool {
    //                             // predicate is reduced to `false` short-circuit!
    //                             if let Err(e) = self.emitter.exit_scope() {
    //                                 self.err.add_error(*e)
    //                             }
    //                             return true;
    //                         }
    //                     }
    //                 }
    //                 self.curr_probe = Some((body_cloned, pred_cloned));
    //             }
    //
    //             // Process the instructions for this single probe!
    //             if let Some(node) = self.tree.get_node(*child) {
    //                 is_success &= self.visit_node(node);
    //             }
    //         } else {
    //             unreachable!()
    //         }
    //         if let Err(e) = self.emitter.exit_scope() {
    //             self.err.add_error(*e)
    //         }
    //     }
    //     is_success
    // }

    fn visit_emit_if_else(&mut self, node: &Node) -> bool {
        if let Node::ActionWithParams {
            ty: ParamActionType::EmitIfElse { cond, conseq, alt },
            ..
        } = node
        {
            self.emitter.emit_if_else();
            self.emit_cond(cond);
            self.emit_conseq(conseq);
            self.emit_alt(alt);
            self.emitter.finish_branch();
            true
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if(&mut self, node: &Node) -> bool {
        if let Node::ActionWithParams {
            ty: ParamActionType::EmitIf { cond, conseq },
            ..
        } = node
        {
            self.emitter.emit_if();
            self.emit_cond(cond);
            self.emit_conseq(conseq);
            self.emitter.finish_branch();
            true
        } else {
            unreachable!()
        }
    }

    fn visit_emit_global_stmts(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitGlobalStmts,
            ..
        } = node
        {
            // NOTE -- this WILL NOT WORK for dfinity or microservice applications...they are stateless
            //     will need to instrument ALL entrypoints for that to work :/
            // TODO -- fix above noted issue by inserting a new 'start' function if it doesn't exist
            if !self.ast.global_stmts.is_empty() {
                match self.emitter.emit_global_stmts(&mut self.ast.global_stmts) {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_pred(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitPred,
            ..
        } = node
        {
            if let Some((.., Some(ref mut pred))) = self.curr_probe {
                match self.emitter.emit_expr(pred) {
                    Err(e) => self.err.add_error(*e),
                    Ok(res) => is_success &= res,
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_body(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::EmitBody,
            ..
        } = node
        {
            if let Some((Some(ref mut body), ..)) = self.curr_probe {
                if self.curr_probe_mode == "after" {
                    // tell the emitter to point to location after instruction-of-interest
                    self.emitter.incr_loc_pointer();
                }
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

    fn visit_remove_orig(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {
            ty: ActionType::RemoveOrig,
            ..
        } = node
        {
            is_success &= self.emitter.remove_orig();
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
}
