use std::collections::HashMap;
use crate::behavior::tree::BehaviorVisitor;
use crate::behavior::tree::{BehaviorTree, Node};
use crate::generator::emitters::Emitter;
use crate::parser::types::Probe;

/// The second phase of instrumenting a Wasm module by actually emitting the
/// instrumentation code.
///
/// To do this, the generator traverses the BehaviorTree AST and calls the
/// passed emitter to emit instrumentation code.
/// This process should ideally be generic, made to perform a specific
/// instrumentation technique by the passed Emitter type.
pub struct InstrGenerator<'a> {
    pub emitter: Box<&'a mut dyn Emitter>,
    pub ast: HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Probe>>>>>,
    pub context_name: String,
    curr_provider_name: String,
    curr_package_name: String,
    curr_event_name: String
}
impl InstrGenerator<'_> {
    pub fn run(&mut self,
        behavior: &BehaviorTree
    ) -> bool {
        // Reset the emitter just in case
        self.emitter.reset_children();
        if let Some(root) = behavior.get_root() {
            // Traverse `behavior` tree and emit the probes held in `ast`
            return self.visit_root(root);
        }
        false
    }
    // ==================
    // = AST OPERATIONS =
    // ==================

    fn get_probes_from_ast(&mut self, name: &String) -> &Vec<Probe> {
        if let Some(provider) = self.ast.get_mut(&self.curr_provider_name) {
            if let Some(package) = provider.get_mut(&self.curr_package_name) {
                if let Some(event) = package.get_mut(&self.curr_event_name) {
                    if let Some(probes) = event.get(name) {
                        return probes;
                    }
                }
            }
        }
        unreachable!()
    }
}
impl BehaviorVisitor<bool> for InstrGenerator<'_> {
    fn visit_root(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_sequence(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_fallback(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_is_instr(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_is_probe_type(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_has_params(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_pred_is(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_for_each_probe(&mut self, node: &Node) -> bool {
        // Assumption: before probes push/pop from stack so it is equivalent to what it was originally
        // Assumption: after probes push/pop from stack so it is equivalent to what it was originally
        todo!()
    }

    fn visit_for_first_probe(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_if_else(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_if(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_enter_scope(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_exit_scope(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_define(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_pred(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_fold_pred(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_reset(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_save_params(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_params(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_body(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_emit_orig(&mut self, node: &Node) -> bool {
        todo!()
    }

    fn visit_force_success(&mut self, node: &Node) -> bool {
        todo!()
    }
}