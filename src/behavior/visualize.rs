use std::path::PathBuf;
use graphviz_rust::dot_structures::{Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex};
use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id, stmt};
use crate::behavior::tree::{ActionType, BehaviorTree, BehaviorVisitor, DecoratorType, Node as TreeNode};

pub fn visualize(tree: &BehaviorTree) {
    let mut visualizer = Visualizer {
        tree,
        graph: graph!(strict di id!(""))
    };
    if let Some(root) = tree.get_root() {
        visualizer.visit_root(root);
    }
}

const CONTROL_NODE_COLOR: &str = "black";
const DECORATOR_NODE_COLOR: &str = "green";
const ACTION_NODE_COLOR: &str = "red";

struct Visualizer<'a> {
    tree: &'a BehaviorTree,
    graph: Graph
}

impl Visualizer<'_> {

    // ===============
    // ==== NODES ====
    // ===============

    fn emit_node(&mut self, id: &usize, label: &str, color: &str) {
        self.graph.add_stmt(stmt!(
            node!(id;
                attr!("label", label),
                attr!("style", "filled"),
                attr!("color", color),
                attr!("fontcolor", "white")
            )
        ));
    }
    fn emit_control_node(&mut self, id: &usize, label: &str) {
        self.emit_node(id, label, CONTROL_NODE_COLOR);
    }
    fn emit_decorator_node(&mut self, id: &usize, label: &str) {
        self.emit_node(id, label, DECORATOR_NODE_COLOR);
    }
    fn emit_action_node(&mut self, id: &usize, label: &str) {
        self.emit_node(id, label, ACTION_NODE_COLOR);
    }

    // ===============
    // ==== EDGES ====
    // ===============

    fn emit_edge(&mut self, from: &usize, to: &usize) {
        self.graph.add_stmt(stmt!(
            edge!(node_id!(from) => node_id!(to))
        ));
    }
}
impl BehaviorVisitor<()> for Visualizer<'_> {
    fn visit_node(&mut self, node: &TreeNode) -> () {
        match node {
            TreeNode::Root { .. } => self.visit_root(node),
            TreeNode::Sequence { .. } => self.visit_sequence(node),
            TreeNode::Decorator { .. } => self.visit_decorator(node),
            TreeNode::Fallback { .. } => self.visit_fallback(node),
            TreeNode::Action { .. } => self.visit_action(node),
        };
    }

    fn visit_root(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Root { id, child } = node {
            self.emit_control_node(id, "root");

            if let Some(node) = self.tree.get_node(child.clone()) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_sequence(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Sequence { id, parent, children } = node {
            self.emit_control_node(id, "sequence");
            self.emit_edge(parent, id);

            for child in children {
                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
    }

    fn visit_decorator(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { ty, ..} = node {
            match ty {
                DecoratorType::IsInstr {..} => self.visit_is_instr(node),
                DecoratorType::IsProbeType {..} => self.visit_is_probe_type(node),
                DecoratorType::HasParams {..} => self.visit_has_params(node),
                DecoratorType::PredIs {..} => self.visit_pred_is(node),
                DecoratorType::ForEach {..} => self.visit_for_each(node),
            }
        } else {
            unreachable!()
        }
    }

    fn visit_fallback(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Fallback { id, parent, children } = node {
            self.emit_control_node(id, "fallback");
            self.emit_edge(parent, id);

            for child in children {
                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
    }

    fn visit_is_instr(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::IsInstr {instr_name} = ty {
                self.emit_decorator_node(id, &format!("IsInstr: {}", instr_name));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }

                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_is_probe_type(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::IsProbeType {probe_type} = ty {
                self.emit_decorator_node(id, &format!("IsProbeType: {}", probe_type));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }

                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_has_params(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::HasParams = ty {
                self.emit_decorator_node(id, "HasParams");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }

                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_pred_is(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::PredIs{ val } = ty {
                self.emit_decorator_node(id, &format!("PredIs: {}", val));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }

                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_for_each(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::ForEach{ target } = ty {
                self.emit_decorator_node(id, &format!("ForEach: {}", target));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }

                if let Some(node) = self.tree.get_node(child.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_action(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { ty, ..} = node {
            match ty {
                ActionType::EnterScope {..} => self.visit_enter_scope(node),
                ActionType::ExitScope {..} => self.visit_exit_scope(node),
                ActionType::Define {..} => self.visit_define(node),
                ActionType::EmitPred {..} => self.visit_emit_pred(node),
                ActionType::FoldPred {..} => self.visit_fold_pred(node),
                ActionType::Reset {..} => self.visit_reset(node),
                ActionType::SaveParams {..} => self.visit_save_params(node),
                ActionType::EmitParams {..} => self.visit_emit_params(node),
                ActionType::EmitBody {..} => self.visit_emit_body(node),
                ActionType::EmitOrig {..} => self.visit_emit_orig(node),
                ActionType::EmitIfElse {..} => self.visit_emit_if_else(node),
                ActionType::ForceSuccess {..} => self.visit_force_success(node),
            }
        } else {
            unreachable!()
        }
    }

    fn visit_enter_scope(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EnterScope{ scope_name } = ty {
                self.emit_action_node(id, &format!("EnterScope: {}", scope_name));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_exit_scope(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::ExitScope = ty {
                self.emit_action_node(id, "ExitScope");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_define(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::Define {var_name, ..} = ty {
                self.emit_action_node(id, &format!("Define: {}", var_name));
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_pred(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EmitPred = ty {
                self.emit_action_node(id, "EmitPred");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_fold_pred(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::FoldPred = ty {
                self.emit_action_node(id, "FoldPred");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_reset(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::Reset = ty {
                self.emit_action_node(id, "Reset");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_save_params(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::SaveParams = ty {
                self.emit_action_node(id, "SaveParams");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_params(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EmitParams = ty {
                self.emit_action_node(id, "EmitParams");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_body(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EmitBody = ty {
                self.emit_action_node(id, "EmitBody");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_orig(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EmitOrig = ty {
                self.emit_action_node(id, "EmitOrig");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if_else(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EmitIfElse { cond, conseq, alt} = ty {
                self.emit_action_node(id, "EmitIfElse");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
                // TODO make connections here
                //      possibly move to cond, seq, alt being behind a new type of node: Arguments
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_force_success(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::ForceSuccess = ty {
                self.emit_action_node(id, "ForceSuccess");
                if let Some(parent) = parent {
                    self.emit_edge(parent, id);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

