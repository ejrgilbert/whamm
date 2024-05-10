use std::io::Result;
use std::path::PathBuf;
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::exec;
use graphviz_rust::dot_structures::{Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex};
use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id, stmt};
use graphviz_rust::printer::PrinterContext;
use crate::behavior::tree::{ActionType, BehaviorTree, BehaviorVisitor, DecoratorType, Node as TreeNode, ParamActionType};

pub fn visualization_to_file(tree: &BehaviorTree, path: PathBuf) -> Result<Vec<u8>> {
    let graph = visualize(tree);
    let p = path.to_str().unwrap();

    let res = exec(
        graph,
        &mut PrinterContext::default(),
        vec![Format::Svg.into(), CommandArg::Output(p.to_string())]
    );
    match &res {
        Err(e) => {
            println!("{}", e.to_string());
        }
        _ => {}
    }
    res
}

fn visualize(tree: &BehaviorTree) -> Graph {
    let mut visualizer = Visualizer {
        tree,
        graph: graph!(strict di id!("")),
        is_param_action: false,
        param_label: None
    };
    if let Some(root) = tree.get_root() {
        visualizer.visit_root(root);
    }

    visualizer.graph
}

const CONTROL_NODE_COLOR: &str = "dimgray";
const DECORATOR_NODE_COLOR: &str = "darkseagreen";
const ACTION_NODE_COLOR: &str = "indianred";
const PARAM_ACTION_NODE_COLOR: &str = "maroon";

struct Visualizer<'a> {
    tree: &'a BehaviorTree,
    graph: Graph,
    is_param_action: bool,
    param_label: Option<String>
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
    fn emit_param_action_node(&mut self, id: &usize, label: &str) {
        self.emit_node(id, label, PARAM_ACTION_NODE_COLOR);
    }

    // ===============
    // ==== EDGES ====
    // ===============

    fn emit_labeled_edge(&mut self, from: &usize, to: &usize) {
        if let Some(label) = &self.param_label {
            self.graph.add_stmt(stmt!(
                edge!(node_id!(from) => node_id!(to);
                    attr!("label", label)
                )
            ));
        }
    }

    fn emit_edge(&mut self, from: &usize, to: &usize) {
        if self.is_param_action {
            self.emit_labeled_edge(from, to);

            // reset
            self.is_param_action = false;
            self.param_label = None;
        } else {
            self.graph.add_stmt(stmt!(
                edge!(node_id!(from) => node_id!(to))
            ));
        }
    }
}
impl BehaviorVisitor<()> for Visualizer<'_> {
    fn visit_node(&mut self, node: &TreeNode) -> () {
        match node {
            TreeNode::Root { .. } => self.visit_root(node),
            TreeNode::Sequence { .. } => self.visit_sequence(node),
            TreeNode::Decorator { .. } => self.visit_decorator(node),
            TreeNode::Fallback { .. } => self.visit_fallback(node),
            TreeNode::ParameterizedAction { .. } => self.visit_parameterized_action(node),
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

    fn visit_parameterized_action(&mut self, node: &TreeNode) -> () {
        if let TreeNode::ParameterizedAction { ty, ..} = node {
            match ty {
                ParamActionType::EmitIfElse {..} => self.visit_emit_if_else(node),
                ParamActionType::EmitIf {..} => self.visit_emit_if(node)
            }
        } else {
            unreachable!()
        }
    }

    fn visit_is_instr(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Decorator { id, ty, parent, child } = node {
            if let DecoratorType::IsInstr {instr_names} = ty {
                let mut names = "".to_string();
                for name in instr_names {
                    if names.is_empty() {
                        names.push_str(name);
                    } else {
                        names.push_str(&format!("OR{name}"));
                    }
                }
                self.emit_decorator_node(id, &format!("IsInstr_{}", names));
                self.emit_edge(parent, id);

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
                self.emit_decorator_node(id, &format!("IsProbeType_{}", probe_type.replace(":", "_")));
                self.emit_edge(parent, id);

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
                self.emit_edge(parent, id);

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
                self.emit_decorator_node(id, &format!("PredIs_{}", val));
                self.emit_edge(parent, id);

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
                self.emit_decorator_node(id, &format!("ForEach_{}", target.replace(":", "_")));
                self.emit_edge(parent, id);

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

    fn visit_emit_if_else(&mut self, node: &TreeNode) -> () {
        if let TreeNode::ParameterizedAction { id, parent, ty, .. } = node {
            if let ParamActionType::EmitIfElse { cond, conseq, alt } = ty {
                self.emit_param_action_node(id, "EmitIfElse");
                self.emit_edge(parent, id);

                self.is_param_action = true;
                self.param_label = Some("cond".to_string());
                if let Some(node) = self.tree.get_node(cond.clone()) {
                    self.visit_node(node);
                }
                self.is_param_action = true;
                self.param_label = Some("conseq".to_string());
                if let Some(node) = self.tree.get_node(conseq.clone()) {
                    self.visit_node(node);
                }
                self.is_param_action = true;
                self.param_label = Some("alt".to_string());
                if let Some(node) = self.tree.get_node(alt.clone()) {
                    self.visit_node(node);
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn visit_emit_if(&mut self, node: &TreeNode) -> () {
        if let TreeNode::ParameterizedAction { id, parent, ty, .. } = node {
            if let ParamActionType::EmitIf { cond, conseq } = ty {
                self.emit_param_action_node(id, "EmitIf");
                self.emit_edge(parent, id);

                self.is_param_action = true;
                self.param_label = Some("cond".to_string());
                if let Some(node) = self.tree.get_node(cond.clone()) {
                    self.visit_node(node);
                }
                self.is_param_action = true;
                self.param_label = Some("conseq".to_string());
                if let Some(node) = self.tree.get_node(conseq.clone()) {
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
                ActionType::ForceSuccess {..} => self.visit_force_success(node),
            }
        } else {
            unreachable!()
        }
    }

    fn visit_enter_scope(&mut self, node: &TreeNode) -> () {
        if let TreeNode::Action { id, ty, parent} = node {
            if let ActionType::EnterScope{ scope_name } = ty {
                self.emit_action_node(id, &format!("EnterScope_{}", scope_name.replace(":", "_")));
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_action_node(id, &format!("Define_{}", var_name.replace(":", "_")));
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
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
                self.emit_edge(parent, id);
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

