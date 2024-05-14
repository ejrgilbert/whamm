use std::collections::HashMap;
use log::{error, warn};
use crate::behavior::tree::{ActionType, ActionWithChildType, BehaviorVisitor, DecoratorType, ParamActionType};
use crate::behavior::tree::{BehaviorTree, Node};
use crate::generator::emitters::Emitter;
use crate::generator::types::ExprFolder;
use crate::parser::types::Probe;

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
    pub ast: HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Probe>>>>>,

    pub context_name: String,
    pub curr_provider_name: String,
    pub curr_package_name: String,
    pub curr_event_name: String,
    pub curr_probe_name: String,
    pub curr_probe: usize
}
impl InstrGenerator<'_, '_> {
    pub fn run(&mut self,
        behavior: &BehaviorTree
    ) -> bool {
        // Reset the symbol table in the emitter just in case
        self.emitter.reset_children();
        if let Some(root) = behavior.get_root() {
            // Traverse `behavior` tree and emit the probes held in `ast`
            return self.visit_root(root);
        }
        warn!("The behavior tree was empty! Nothing to emit!");
        false
    }
    // ==================
    // = AST OPERATIONS =
    // ==================

    fn get_probes_from_ast(&self, name: &String) -> &Vec<Probe> {
        if let Some(provider) = self.ast.get(&self.curr_provider_name) {
            if let Some(package) = provider.get(&self.curr_package_name) {
                if let Some(event) = package.get(&self.curr_event_name) {
                    if let Some(probes) = event.get(name) {
                        return probes;
                    }
                }
            }
        }
        unreachable!()
    }

    fn get_probes_from_ast_mut(&mut self, name: &String) -> &mut Vec<Probe> {
        // let provider_name = self.curr_provider_name.clone();
        // let package_name = self.curr_package_name.clone();
        // let event_name = self.curr_event_name.clone();
        if let Some(provider) = self.ast.get_mut(&self.curr_provider_name) {
            if let Some(package) = provider.get_mut(&self.curr_package_name) {
                if let Some(event) = package.get_mut(&self.curr_event_name) {
                    if let Some(probes) = event.get_mut(name) {
                        return probes;
                    }
                }
            }
        }
        unreachable!()
    }

    fn get_curr_probe(&self) -> Option<&Probe> {
        self.get_probes_from_ast(&self.curr_probe_name)
            .get(self.curr_probe)
    }

    fn get_curr_probe_mut(&mut self) -> Option<&mut Probe> {
        let probe_name = self.curr_probe_name.clone();
        let curr_probe_idx = self.curr_probe.clone();
        self.get_probes_from_ast_mut(&probe_name)
            .get_mut(curr_probe_idx)
    }

    fn emit_cond(&mut self, cond: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(cond.clone()) {
            // emit the branch conditional
            self.emitter.emit_condition();
            is_success &= self.visit_node(node);
        } else {
            error!("Node to define conditional logic node does not exist!");
        }
        is_success
    }

    fn emit_conseq(&mut self, conseq: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(conseq.clone()) {
            // emit the consequent logic
            self.emitter.emit_consequent();
            is_success &= self.visit_node(node);
        } else {
            error!("Node to define consequent logic node does not exist!");
        }
        is_success
    }

    fn emit_alt(&mut self, alt: &usize) -> bool {
        let mut is_success = true;
        if let Some(node) = self.tree.get_node(alt.clone()) {
            // emit the alternate logic
            self.emitter.emit_alternate();
            is_success &= self.visit_node(node);
        } else {
            error!("Node to define alternate logic node does not exist!");
        }
        is_success
    }
}
impl BehaviorVisitor<bool> for InstrGenerator<'_, '_> {
    fn visit_root(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Root { child, .. } = node {
            if let Some(node) = self.tree.get_node(child.clone()) {
                is_success &= self.visit_node(node);
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_sequence(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Sequence { children, .. } = node {
            for child in children {
                if let Some(node) = self.tree.get_node(child.clone()) {
                    is_success &= self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_fallback(&mut self, node: &Node) -> bool {
        if let Node::Fallback { children, .. } = node {
            for child in children {
                let mut child_is_success = true;
                if let Some(node) = self.tree.get_node(child.clone()) {
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

    fn visit_is_instr(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator {ty, child, ..} = node {
            if let DecoratorType::IsInstr {instr_names} = ty {
                if self.emitter.curr_instr_is_of_type(instr_names) {
                    // If the current instruction is of-interest, continue with the behavior tree logic
                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_is_probe_type(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator { ty, child, .. } = node {
            if let DecoratorType::IsProbeType {probe_type} = ty {
                if self.curr_probe_name == *probe_type {
                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_has_alt_call(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator { ty, child, .. } = node {
            if let DecoratorType::HasAltCall = ty {
                if self.emitter.has_alt_call() {
                    // The current probe has a defined alt call, continue with behavior
                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_has_params(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator { ty, child, .. } = node {
            if let DecoratorType::HasParams = ty {
                if self.emitter.has_params() {
                    // The current instruction has parameters, continue with behavior
                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_pred_is(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator {ty, child, ..} = node {
            if let DecoratorType::PredIs{ val } = ty {
                if let Some(probe) = self.get_curr_probe() {
                    if let Some(pred) = &probe.predicate {
                        if let Some(pred_as_bool) = ExprFolder::get_single_bool(&pred) {
                            // predicate has been reduced to a boolean value
                            if pred_as_bool == *val {
                                // predicate is reduced to desired value, execute child node
                                if let Some(node) = self.tree.get_node(child.clone()) {
                                    is_success &= self.visit_node(node);
                                }
                            }
                        }
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_for_each_probe(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
        // Assumption: after probes push/pop from stack so it is equivalent to what it was originally

        if let Node::Decorator { ty, child, .. } = node {
            if let DecoratorType::ForEachProbe { target } = ty {
                self.curr_probe_name = target.clone();
                for i in Vec::from_iter(0..self.get_probes_from_ast(target).len()).iter() {
                    self.curr_probe = i.clone();

                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_for_first_probe(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Decorator {ty, child, .. } = node {
            if let DecoratorType::ForFirstProbe { target } = ty {
                if self.get_probes_from_ast(target).len() > 1 {
                    warn!("There is more than one probe for probe type '{}'. So only emitting first probe, ignoring rest.", target)
                }
                self.curr_probe_name = target.clone();
                self.curr_probe = 0;

                // Process the instructions for this single probe!
                if let Some(node) = self.tree.get_node(child.clone()) {
                    is_success &= self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_enter_package(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::ActionWithChild { ty, child, .. } = node {
            let ActionWithChildType::EnterPackage { package_name } = ty;
            if package_name == "bytecode" {
                // Process first instruction!
                if let Some(node) = self.tree.get_node(child.clone()) {
                    is_success &= self.visit_node(node);
                }

                // Process the rest of the instructions
                while self.emitter.has_next_instr() {
                    self.emitter.next_instr();
                    if let Some(node) = self.tree.get_node(child.clone()) {
                        is_success &= self.visit_node(node);
                    }
                }
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_if_else(&mut self, node: &Node) -> bool {
        if let Node::ParameterizedAction {ty, .. } = node {
            if let ParamActionType::EmitIfElse { cond, conseq, alt } = ty {
                self.emitter.emit_if_else();
                self.emit_cond(cond);
                self.emit_conseq(conseq);
                self.emit_alt(alt);
                self.emitter.finish_branch();
                return true;
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if(&mut self, node: &Node) -> bool {
        if let Node::ParameterizedAction { ty, .. } = node {
            if let ParamActionType::EmitIf { cond, conseq } = ty {
                self.emitter.emit_if();
                self.emit_cond(cond);
                self.emit_conseq(conseq);
                self.emitter.finish_branch();
                return true;
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_enter_scope(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action { ty, ..} = node {
            if let ActionType::EnterScope{ scope_name } = ty {
                is_success &= self.emitter.enter_named_scope(scope_name);
                if is_success {
                    // Set the current context info for probe lookup
                    self.context_name = scope_name.clone();

                    let mut spec_split = scope_name.split(":");
                    if let Some(_whamm) = spec_split.next() {
                        if let Some(_whammy) = spec_split.next() {
                            if let Some(provider) = spec_split.next() {
                                self.curr_provider_name = provider.to_string();
                                if let Some(package) = spec_split.next() {
                                    self.curr_package_name = package.to_string();
                                    if let Some(event) = spec_split.next() {
                                        self.curr_event_name = event.to_string();
                                        if let Some(probe) = spec_split.next() {
                                            self.curr_probe_name = probe.to_string()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_exit_scope(&mut self, node: &Node) -> bool {
        let is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::ExitScope = ty {
                self.emitter.exit_scope();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_define(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::Define {var_name, ..} = ty {
                is_success &= self.emitter.define_compiler_var(&self.context_name, var_name);
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_pred(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::EmitPred = ty {
                if let Some(probe) = self.get_curr_probe() {
                    // This clone is because of borrowing self as mutable AND immutable at the same time
                    // TODO -- remove the need for this clone
                    if let Some(pred) = &mut probe.predicate.clone() {
                        is_success &= self.emitter.emit_expr(pred);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_fold_pred(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action { ty, ..} = node {
            if let ActionType::FoldPred = ty {
                if let Some(probe) = self.get_curr_probe_mut() {
                    // This clone is because of borrowing self as mutable AND immutable at the same time
                    // TODO -- remove the need for this clone
                    if let Some(pred) = &mut probe.predicate.clone() {
                        is_success &= self.emitter.fold_expr(pred);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_reset(&mut self, node: &Node) -> bool {
        let is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::Reset = ty {
                self.emitter.reset_children();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_save_params(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::SaveParams = ty {
                is_success &= self.emitter.save_params();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_params(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action { ty, ..} = node {
            if let ActionType::EmitParams = ty {
                is_success &= self.emitter.emit_params();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_body(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::EmitBody = ty {
                if let Some(probe) = self.get_curr_probe() {
                    // This clone is because of borrowing self as mutable AND immutable at the same time
                    // TODO -- remove the need for this clone
                    if let Some(body) = &mut probe.body.clone() {
                        is_success &= self.emitter.emit_body(body);
                    }
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_alt_call(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::EmitAltCall = ty {
                is_success &= self.emitter.emit_alt_call();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_remove_orig(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::EmitOrig = ty {
                is_success &= self.emitter.remove_orig();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_emit_orig(&mut self, node: &Node) -> bool {
        let mut is_success = true;
        if let Node::Action {ty, ..} = node {
            if let ActionType::EmitOrig = ty {
                is_success &= self.emitter.emit_orig();
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
        is_success
    }

    fn visit_force_success(&mut self, node: &Node) -> bool {
        if let Node::Action {ty, ..} = node {
            if let ActionType::ForceSuccess = ty {
                return true;
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}