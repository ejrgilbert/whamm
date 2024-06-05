use crate::behavior::tree::{
    ActionType, ActionWithChildType, ArgActionType, BehaviorTree, BehaviorVisitor, DecoratorType,
    Node as TreeNode, ParamActionType,
};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id, stmt};
use graphviz_rust::dot_structures::{
    Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex,
};
use graphviz_rust::exec;
use graphviz_rust::printer::PrinterContext;
use std::io::Result;
use std::path::PathBuf;

pub fn visualization_to_file(tree: &BehaviorTree, path: PathBuf) -> Result<Vec<u8>> {
    let graph = visualize(tree);
    let p = path.to_str().unwrap();

    let res = exec(
        graph,
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output(p.to_string())],
    );
    if let Err(e) = &res {
        println!("{}", e);
    }
    res
}

fn visualize(tree: &BehaviorTree) -> Graph {
    let mut visualizer = Visualizer {
        tree,
        graph: graph!(strict di id!("")),
        is_param_action: false,
        param_label: None,
    };
    if let Some(root) = tree.get_root() {
        visualizer.visit_root(root);
    }

    visualizer.graph
}

const CONTROL_NODE_COLOR: &str = "dimgray";
const DECORATOR_NODE_COLOR: &str = "darkseagreen";
const ACTION_NODE_COLOR: &str = "indianred";
const SPECIAL_ACTION_NODE_COLOR: &str = "maroon";

struct Visualizer<'a> {
    tree: &'a BehaviorTree,
    graph: Graph,
    is_param_action: bool,
    param_label: Option<String>,
}

impl Visualizer<'_> {
    // ===============
    // ==== NODES ====
    // ===============

    fn emit_node(&mut self, id: &usize, label: &str, color: &str) {
        self.graph.add_stmt(stmt!(node!(id;
            attr!("label", label),
            attr!("style", "filled"),
            attr!("color", color),
            attr!("fontcolor", "white")
        )));
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
    fn emit_special_action_node(&mut self, id: &usize, label: &str) {
        self.emit_node(id, label, SPECIAL_ACTION_NODE_COLOR);
    }

    // ===============
    // ==== EDGES ====
    // ===============

    fn emit_labeled_edge(&mut self, from: &usize, to: &usize) {
        if let Some(label) = &self.param_label {
            self.graph
                .add_stmt(stmt!(edge!(node_id!(from) => node_id!(to);
                    attr!("label", label)
                )));
        }
    }

    fn emit_edge(&mut self, from: &usize, to: &usize) {
        if self.is_param_action {
            self.emit_labeled_edge(from, to);

            // reset
            self.is_param_action = false;
            self.param_label = None;
        } else {
            self.graph
                .add_stmt(stmt!(edge!(node_id!(from) => node_id!(to))));
        }
    }
}
impl BehaviorVisitor<()> for Visualizer<'_> {
    fn visit_root(&mut self, node: &TreeNode) {
        if let TreeNode::Root { id, child } = node {
            self.emit_control_node(id, "root");

            if let Some(node) = self.tree.get_node(*child) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_sequence(&mut self, node: &TreeNode) {
        if let TreeNode::Sequence {
            id,
            parent,
            children,
        } = node
        {
            self.emit_control_node(id, "sequence");
            self.emit_edge(parent, id);

            for child in children {
                if let Some(node) = self.tree.get_node(*child) {
                    self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
    }

    fn visit_fallback(&mut self, node: &TreeNode) {
        if let TreeNode::Fallback {
            id,
            parent,
            children,
        } = node
        {
            self.emit_control_node(id, "fallback");
            self.emit_edge(parent, id);

            for child in children {
                if let Some(node) = self.tree.get_node(*child) {
                    self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
    }

    fn visit_is_probe_mode(&mut self, node: &TreeNode) {
        if let TreeNode::Decorator {
            id,
            ty: DecoratorType::IsProbeMode { probe_mode },
            parent,
            child,
        } = node
        {
            self.emit_decorator_node(id, &format!("IsProbeMode_{}", probe_mode.replace(':', "_")));
            self.emit_edge(parent, id);

            if let Some(node) = self.tree.get_node(*child) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_has_alt_call(&mut self, node: &TreeNode) {
        if let TreeNode::Decorator {
            id,
            ty: DecoratorType::HasAltCall,
            parent,
            child,
        } = node
        {
            self.emit_decorator_node(id, "HasAltCall");
            self.emit_edge(parent, id);

            if let Some(node) = self.tree.get_node(*child) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_pred_is(&mut self, node: &TreeNode) {
        if let TreeNode::Decorator {
            id,
            ty: DecoratorType::PredIs { val },
            parent,
            child,
        } = node
        {
            self.emit_decorator_node(id, &format!("PredIs_{}", val));
            self.emit_edge(parent, id);

            if let Some(node) = self.tree.get_node(*child) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_save_params(&mut self, node: &TreeNode) {
        if let TreeNode::ArgAction {
            id,
            ty: ArgActionType::SaveParams,
            parent,
            ..
        } = node
        {
            self.emit_special_action_node(id, "SaveParams");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_params(&mut self, node: &TreeNode) {
        if let TreeNode::ArgAction {
            id,
            ty: ArgActionType::EmitParams,
            parent,
            ..
        } = node
        {
            self.emit_special_action_node(id, "EmitParams");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_enter_package(&mut self, node: &TreeNode) {
        if let TreeNode::ActionWithChild {
            id,
            ty:
                ActionWithChildType::EnterPackage {
                    package_name,
                    events,
                    ..
                },
            parent,
            child,
        } = node
        {
            let mut event_names = "".to_string();
            events.keys().for_each(|event_name| {
                if event_names.is_empty() {
                    event_names.push_str(event_name);
                } else {
                    event_names.push_str(&format!("OR{event_name}"));
                }
            });
            self.emit_special_action_node(
                id,
                &format!(
                    "EnterPackage_{}_{}",
                    package_name.replace(':', "_"),
                    event_names
                ),
            );
            self.emit_edge(parent, id);

            if let Some(node) = self.tree.get_node(*child) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_enter_probe(&mut self, node: &TreeNode) {
        if let TreeNode::ActionWithChild {
            id,
            ty,
            parent,
            child,
        } = node
        {
            if let ActionWithChildType::EnterProbe { probe_mode, .. } = ty {
                self.emit_special_action_node(
                    id,
                    &format!("EnterProbe_{}", probe_mode.replace(':', "_")),
                );
                self.emit_edge(parent, id);

                if let Some(node) = self.tree.get_node(*child) {
                    self.visit_node(node);
                }
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if_else(&mut self, node: &TreeNode) {
        if let TreeNode::ActionWithParams {
            id,
            parent,
            ty: ParamActionType::EmitIfElse { cond, conseq, alt },
            ..
        } = node
        {
            self.emit_special_action_node(id, "EmitIfElse");
            self.emit_edge(parent, id);

            self.is_param_action = true;
            self.param_label = Some("cond".to_string());
            if let Some(node) = self.tree.get_node(*cond) {
                self.visit_node(node);
            }
            self.is_param_action = true;
            self.param_label = Some("conseq".to_string());
            if let Some(node) = self.tree.get_node(*conseq) {
                self.visit_node(node);
            }
            self.is_param_action = true;
            self.param_label = Some("alt".to_string());
            if let Some(node) = self.tree.get_node(*alt) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if(&mut self, node: &TreeNode) {
        if let TreeNode::ActionWithParams {
            id,
            parent,
            ty: ParamActionType::EmitIf { cond, conseq },
            ..
        } = node
        {
            self.emit_special_action_node(id, "EmitIf");
            self.emit_edge(parent, id);

            self.is_param_action = true;
            self.param_label = Some("cond".to_string());
            if let Some(node) = self.tree.get_node(*cond) {
                self.visit_node(node);
            }
            self.is_param_action = true;
            self.param_label = Some("conseq".to_string());
            if let Some(node) = self.tree.get_node(*conseq) {
                self.visit_node(node);
            }
        } else {
            unreachable!()
        }
    }

    fn visit_enter_scope(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EnterScope { scope_name, .. },
            parent,
        } = node
        {
            self.emit_action_node(id, &format!("EnterScope_{}", scope_name.replace(':', "_")));
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_exit_scope(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::ExitScope,
            parent,
        } = node
        {
            self.emit_action_node(id, "ExitScope");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_define(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::Define { var_name, .. },
            parent,
        } = node
        {
            self.emit_action_node(id, &format!("Define_{}", var_name.replace(':', "_")));
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_global_stmts(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EmitGlobalStmts,
            parent,
        } = node
        {
            self.emit_special_action_node(id, "EmitGlobalStmts");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_pred(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EmitPred,
            parent,
        } = node
        {
            self.emit_action_node(id, "EmitPred");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_reset(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::Reset,
            parent,
        } = node
        {
            self.emit_action_node(id, "Reset");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_body(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EmitBody,
            parent,
        } = node
        {
            self.emit_action_node(id, "EmitBody");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_alt_call(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EmitAltCall,
            parent,
        } = node
        {
            self.emit_action_node(id, "EmitAltCall");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_remove_orig(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::RemoveOrig,
            parent,
        } = node
        {
            self.emit_action_node(id, "RemoveOrig");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_emit_orig(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::EmitOrig,
            parent,
        } = node
        {
            self.emit_action_node(id, "EmitOrig");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }

    fn visit_force_success(&mut self, node: &TreeNode) {
        if let TreeNode::Action {
            id,
            ty: ActionType::ForceSuccess,
            parent,
        } = node
        {
            self.emit_action_node(id, "ForceSuccess");
            self.emit_edge(parent, id);
        } else {
            unreachable!()
        }
    }
}
