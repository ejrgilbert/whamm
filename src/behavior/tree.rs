use crate::common::error::ErrorGen;

const UNEXPECTED_ERR_MSG: &str =
    "BehaviorTree: Looks like you've found a bug...please report this behavior!";

#[derive(Debug)]
pub struct BehaviorTree {
    pub nodes: Vec<Node>,
    pub curr: usize, // indexes into this::nodes
}
impl Default for BehaviorTree {
    fn default() -> Self {
        BehaviorTree::new()
    }
}
impl BehaviorTree {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::Root { id: 0, child: 0 }],
            curr: 0,
        }
    }

    pub fn reset(&mut self) {
        self.curr = 0;
    }

    pub fn get_node(&self, idx: usize) -> Option<&Node> {
        self.nodes.get(idx)
    }

    pub fn get_node_mut(&mut self, idx: usize) -> Option<&mut Node> {
        self.nodes.get_mut(idx)
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.get_node(0)
    }

    pub fn get_curr(&self) -> Option<&Node> {
        self.get_node(self.curr)
    }

    pub fn get_curr_mut(&mut self) -> Option<&mut Node> {
        self.get_node_mut(self.curr)
    }

    // ==================
    // ==== Control =====
    // ==================

    pub fn sequence(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(
            Node::Sequence {
                id,
                parent: self.curr,
                children: vec![],
            },
            err,
        );
        self
    }

    pub fn exit_sequence(&mut self, err: &mut ErrorGen) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Sequence { parent, .. }) => self.curr = *parent,
            other => {
                err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Something went wrong, expected Sequence, but was: {:?}.", other)), None);
            }
        };
        self
    }

    pub fn fallback(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(
            Node::Fallback {
                id,
                parent: self.curr,
                children: vec![],
            },
            err,
        );
        self
    }

    pub fn exit_fallback(&mut self, err: &mut ErrorGen) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Fallback { parent, .. }) => self.curr = *parent,
            other => {
                err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Something went wrong, expected Fallback, but was: {:?}.", other)), None);
            }
        };
        self
    }

    pub fn decorator(&mut self, ty: DecoratorType, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(
            Node::Decorator {
                id,
                ty,
                parent: self.curr,
                child: 0,
            },
            err,
        );
        self
    }

    pub fn exit_decorator(&mut self, err: &mut ErrorGen) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Decorator { parent, .. }) => self.curr = *parent,
            other => {
                err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Something went wrong, expected Decorator, but was: {:?}.", other)), None);
            }
        };
        self
    }

    pub fn parameterized_action(&mut self, ty: ParamActionType, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(
            Node::ActionWithParams {
                id,
                parent: self.curr,
                ty,
                children: vec![],
            },
            err,
        );
        self
    }

    pub fn exit_parameterized_action(&mut self, err: &mut ErrorGen) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::ActionWithParams { parent, .. }) => self.curr = *parent,
            other => {
                err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Something went wrong, expected ParameterizedAction, but was: {:?}.", other)), None);
            }
        };
        self
    }

    // ==================
    // ==== Actions =====
    // ==================

    fn add_action_as_param(&mut self, idx: usize, id: usize, err: &mut ErrorGen) {
        if let Some(Node::ActionWithParams { ty, .. }) = self.get_curr_mut() {
            match ty {
                ParamActionType::EmitIf { cond, conseq } => {
                    if idx == 0 {
                        *cond = id;
                    } else if idx == 1 {
                        *conseq = id;
                    } else {
                        err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Unexpected index for parameterized action (EmitIf):  {:?}.", idx)), None);
                    }
                }
                ParamActionType::EmitIfElse { cond, conseq, alt } => {
                    if idx == 0 {
                        *cond = id;
                    } else if idx == 1 {
                        *conseq = id;
                    } else if idx == 2 {
                        *alt = id;
                    } else {
                        err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Unexpected index for parameterized action (EmitIfElse):  {:?}.", idx)), None);
                    }
                }
            }
        };
    }

    pub fn emit_global_stmts(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::EmitGlobalStmts,
            },
            err,
        );
        self
    }

    pub fn emit_body(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::EmitBody,
            },
            err,
        );
        self
    }

    pub fn emit_alt_call(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::EmitAltCall,
            },
            err,
        );
        self
    }

    pub fn emit_args(&mut self, force_success: bool, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::ArgAction {
                id,
                parent: self.curr,
                ty: ArgActionType::EmitArgs,
                force_success,
            },
            err,
        );
        self
    }

    pub fn save_args(&mut self, force_success: bool, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::ArgAction {
                id,
                parent: self.curr,
                ty: ArgActionType::SaveArgs,
                force_success,
            },
            err,
        );
        self
    }

    pub fn remove_orig(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::RemoveOrig,
            },
            err,
        );
        self
    }

    pub fn emit_orig(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::EmitOrig,
            },
            err,
        );
        self
    }

    pub fn emit_pred(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::EmitPred,
            },
            err,
        );
        self
    }

    pub fn force_success(&mut self, err: &mut ErrorGen) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(
            Node::Action {
                id,
                parent: self.curr,
                ty: ActionType::ForceSuccess,
            },
            err,
        );
        self
    }

    // ==================
    // ==== Base Fns ====
    // ==================

    pub fn put_child(&mut self, node: Node, err: &mut ErrorGen) -> Option<usize> {
        let mut assigned_id = None;
        let new_id = self.nodes.len();

        if let Some(curr) = self.get_curr_mut() {
            match curr {
                Node::Root { child, .. } => {
                    *child = new_id;
                    assigned_id = Some(new_id);
                }
                Node::Sequence { children, .. } => {
                    children.push(new_id);
                    assigned_id = Some(new_id);
                }
                Node::Decorator { child, .. } => {
                    *child = new_id;
                    assigned_id = Some(new_id);
                }
                Node::Fallback { children, .. } => {
                    children.push(new_id);
                    assigned_id = Some(new_id);
                }
                Node::ActionWithParams { children, .. } => {
                    let idx = children.len();
                    children.push(new_id);

                    self.add_action_as_param(idx, new_id, err);
                    assigned_id = Some(new_id);
                }
                _ => {
                    err.unexpected_error(
                        false,
                        Some(format!(
                            "{UNEXPECTED_ERR_MSG} Cannot add child to this Tree node type"
                        )),
                        None,
                    );
                }
            }
        }
        if assigned_id.is_some() {
            self.nodes.push(node);
        }
        assigned_id
    }

    pub fn put_child_and_enter(&mut self, node: Node, err: &mut ErrorGen) -> bool {
        if let Some(id) = self.put_child(node, err) {
            self.curr = id;
        }
        false
    }

    // For use as param passing (consider IfElse action)
    pub fn put_floating_child(&mut self, node: Node) -> usize {
        let new_id = self.nodes.len();
        self.nodes.push(node);
        new_id
    }

    pub fn exit_child(&mut self, err: &mut ErrorGen) {
        match self.get_curr_mut() {
            Some(Node::Sequence { parent, .. }) | Some(Node::Fallback { parent, .. }) => {
                self.curr = *parent
            }
            Some(Node::Decorator { parent, .. }) => self.curr = *parent,
            _ => {
                err.unexpected_error(false, Some(format!("{UNEXPECTED_ERR_MSG} Attempted to exit current scope, but there was no parent to exit into.")), None);
            }
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Root {
        id: usize,
        child: usize,
    },
    Sequence {
        id: usize,
        parent: usize,
        children: Vec<usize>,
    },
    Decorator {
        id: usize,
        ty: DecoratorType,
        parent: usize,
        child: usize,
    },
    Fallback {
        id: usize,
        parent: usize,
        children: Vec<usize>,
    },
    /// An action to perform on arguments to some Wasm function
    ArgAction {
        id: usize,
        ty: ArgActionType,
        parent: usize,
        force_success: bool,
    },
    ActionWithParams {
        id: usize,
        parent: usize,
        ty: ParamActionType,
        children: Vec<usize>,
    },
    Action {
        id: usize,
        parent: usize,
        ty: ActionType,
    },
}

#[derive(Debug)]
pub enum DecoratorType {
    IsProbeMode { probe_mode: String },
    HasAltCall,
    PredIs { val: bool },
}

#[derive(Debug)]
pub enum ActionType {
    EmitGlobalStmts,
    EmitPred,
    EmitBody,
    EmitAltCall,
    RemoveOrig,
    EmitOrig,
    ForceSuccess,
}

#[derive(Debug)]
pub enum ArgActionType {
    SaveArgs,
    EmitArgs,
}

#[derive(Debug)]
pub enum ParamActionType {
    EmitIf {
        cond: usize,
        conseq: usize,
    },
    EmitIfElse {
        cond: usize,
        conseq: usize,
        alt: usize,
    },
}

pub trait BehaviorVisitor<T> {
    // Abstracted visit fn
    fn visit_node(&mut self, node: &Node) -> T {
        match node {
            Node::Root { .. } => self.visit_root(node),
            Node::Sequence { .. } => self.visit_sequence(node),
            Node::Decorator { .. } => self.visit_decorator(node),
            Node::Fallback { .. } => self.visit_fallback(node),
            Node::ArgAction { .. } => self.visit_arg_action(node),
            Node::ActionWithParams { .. } => self.visit_action_with_args(node),
            Node::Action { .. } => self.visit_action(node),
        }
    }
    fn visit_root(&mut self, node: &Node) -> T;

    // Control nodes
    fn visit_sequence(&mut self, node: &Node) -> T;
    fn visit_decorator(&mut self, node: &Node) -> T {
        if let Node::Decorator { ty, .. } = node {
            match ty {
                DecoratorType::IsProbeMode { .. } => self.visit_is_probe_mode(node),
                DecoratorType::HasAltCall { .. } => self.visit_has_alt_call(node),
                DecoratorType::PredIs { .. } => self.visit_pred_is(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_fallback(&mut self, node: &Node) -> T;
    fn visit_arg_action(&mut self, node: &Node) -> T {
        if let Node::ArgAction { ty, .. } = node {
            match ty {
                ArgActionType::SaveArgs { .. } => self.visit_save_args(node),
                ArgActionType::EmitArgs { .. } => self.visit_emit_args(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_action_with_args(&mut self, node: &Node) -> T {
        if let Node::ActionWithParams { ty, .. } = node {
            match ty {
                ParamActionType::EmitIfElse { .. } => self.visit_emit_if_else(node),
                ParamActionType::EmitIf { .. } => self.visit_emit_if(node),
            }
        } else {
            unreachable!()
        }
    }

    // Decorator nodes
    fn visit_is_probe_mode(&mut self, node: &Node) -> T;
    fn visit_has_alt_call(&mut self, node: &Node) -> T;
    fn visit_pred_is(&mut self, node: &Node) -> T;

    // Argument action nodes
    fn visit_save_args(&mut self, node: &Node) -> T;
    fn visit_emit_args(&mut self, node: &Node) -> T;

    // Parameterized action nodes
    fn visit_emit_if_else(&mut self, node: &Node) -> T;
    fn visit_emit_if(&mut self, node: &Node) -> T;

    // Action nodes
    fn visit_action(&mut self, node: &Node) -> T {
        if let Node::Action { ty, .. } = node {
            match ty {
                ActionType::EmitGlobalStmts { .. } => self.visit_emit_global_stmts(node),
                ActionType::EmitPred { .. } => self.visit_emit_pred(node),
                ActionType::EmitBody { .. } => self.visit_emit_body(node),
                ActionType::EmitAltCall { .. } => self.visit_emit_alt_call(node),
                ActionType::RemoveOrig { .. } => self.visit_remove_orig(node),
                ActionType::EmitOrig { .. } => self.visit_emit_orig(node),
                ActionType::ForceSuccess { .. } => self.visit_force_success(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_emit_global_stmts(&mut self, node: &Node) -> T;
    fn visit_emit_pred(&mut self, node: &Node) -> T;
    fn visit_emit_body(&mut self, node: &Node) -> T;
    fn visit_emit_alt_call(&mut self, node: &Node) -> T;
    fn visit_remove_orig(&mut self, node: &Node) -> T;
    fn visit_emit_orig(&mut self, node: &Node) -> T;
    fn visit_force_success(&mut self, node: &Node) -> T;
}
