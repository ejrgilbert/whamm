use log::error;

#[derive(Debug)]
pub struct BehaviorTree {
    pub nodes: Vec<Node>,
    pub curr: usize,     // indexes into this::nodes
}
impl BehaviorTree {
    pub fn new() -> Self {
        Self {
            nodes: vec![ Node::Root {
                id: 0,
                child: 0
            }],
            curr: 0
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

    pub fn get_root(&self) -> Option<&Node>{
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

    pub fn sequence(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(Node::Sequence {
            id,
            parent: self.curr,
            children: vec![],
        });
        self
    }

    pub fn exit_sequence(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Sequence {parent, ..}) => {
                self.curr = parent.clone()
            },
            other => {
                error!("Something went wrong, expected Sequence, but was: {:?}", other)
            }
        };
        self
    }

    pub fn fallback(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(Node::Fallback {
            id,
            parent: self.curr,
            children: vec![],
        });
        self
    }

    pub fn exit_fallback(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Fallback {parent, ..}) => {
                self.curr = parent.clone()
            },
            other => {
                error!("Something went wrong, expected Fallback, but was: {:?}", other)
            }
        };
        self
    }

    pub fn decorator(&mut self, ty: DecoratorType) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(Node::Decorator {
            id,
            ty,
            parent: self.curr,
            child: 0,
        });
        self
    }

    pub fn exit_decorator(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Decorator {parent, ..}) => {
                self.curr = parent.clone()
            },
            other => {
                error!("Something went wrong, expected Decorator, but was: {:?}", other)
            }
        };
        self
    }

    pub fn action_with_child(&mut self, ty: ActionWithChildType) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(Node::ActionWithChild {
            id,
            parent: self.curr,
            ty,
            child: 0,
        });
        self
    }

    pub fn exit_action_with_child(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::ActionWithChild {parent, ..}) => {
                self.curr = parent.clone()
            },
            other => {
                error!("Something went wrong, expected ActionWithChild, but was: {:?}", other)
            }
        };
        self
    }

    pub fn parameterized_action(&mut self, ty: ParamActionType) -> &mut Self {
        let id = self.nodes.len();
        self.put_child_and_enter(Node::ParameterizedAction {
            id,
            parent: self.curr,
            ty,
            children: vec![],
        });
        self
    }

    pub fn exit_parameterized_action(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::ParameterizedAction {parent, ..}) => {
                self.curr = parent.clone()
            },
            other => {
                error!("Something went wrong, expected ParameterizedAction, but was: {:?}", other)
            }
        };
        self
    }

    // ==================
    // ==== Actions =====
    // ==================

    fn add_action_as_param(&mut self, idx: usize, id: usize) {
        match self.get_curr_mut() {
            Some(Node::ParameterizedAction {ty, ..}) => {
                match ty {
                    ParamActionType::EmitIf { cond, conseq } => {
                        if idx == 0 {
                            *cond = id;
                        } else if idx == 1 {
                            *conseq = id;
                        } else {
                            error!("Unexpected index for parameterized action (EmitIf): {}", idx);
                        }
                    },
                    ParamActionType::EmitIfElse { cond, conseq, alt } => {
                        if idx == 0 {
                            *cond = id;
                        } else if idx == 1 {
                            *conseq = id;
                        }else if idx == 2 {
                            *alt = id;
                        } else {
                            error!("Unexpected index for parameterized action (EmitIfElse): {}", idx);
                        }
                    }
                }
            },
            _ => {}
        };
    }

    pub fn define(&mut self, context: String, var_name: String) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::Define {
                context,
                var_name
            }
        });
        self
    }

    pub fn emit_body(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EmitBody
        });
        self
    }

    pub fn emit_alt_call(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EmitAltCall
        });
        self
    }

    pub fn emit_params(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EmitParams
        });
        self
    }

    pub fn remove_orig(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::RemoveOrig
        });
        self
    }

    pub fn emit_orig(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EmitOrig
        });
        self
    }

    pub fn emit_pred(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EmitPred
        });
        self
    }

    pub fn enter_scope(&mut self, scope_name: String) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::EnterScope {
                scope_name
            }
        });
        self
    }

    pub fn exit_scope(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::ExitScope
        });
        self
    }

    pub fn fold_pred(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::FoldPred
        });
        self
    }

    pub fn force_success(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::ForceSuccess
        });
        self
    }

    pub fn save_params(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: self.curr,
            ty: ActionType::SaveParams
        });
        self
    }

    // ==================
    // ==== Base Fns ====
    // ==================

    pub fn put_child(&mut self, node: Node) -> Option<usize> {
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
                Node::ActionWithChild { child, .. } => {
                    *child = new_id;
                    assigned_id = Some(new_id);
                }
                Node::ParameterizedAction { children, .. } => {
                    let idx = children.len();
                    children.push(new_id);

                    self.add_action_as_param(idx, new_id);
                    assigned_id = Some(new_id);
                }
                _ => {
                    error!("Cannot add child to this Tree node type");
                }
            }
        }
        if assigned_id.is_some() {
            self.nodes.push(node);
        }
        assigned_id
    }

    pub fn put_child_and_enter(&mut self, node: Node) -> bool {
        if let Some(id) = self.put_child(node) {
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

    pub fn exit_child(&mut self) {
        match self.get_curr_mut() {
            Some(Node::Sequence {parent, ..}) |
            Some(Node::Fallback {parent, ..}) => {
                self.curr = parent.clone()
            },
            Some(Node::Decorator {parent, ..}) => {
                self.curr = parent.clone()
            }
            _ => {
                error!("Attempted to exit current scope, but there was no parent to exit into.")
            }
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Root {
        id: usize,
        child: usize
    },
    Sequence {
        id: usize,
        parent: usize,
        children: Vec<usize>
    },
    Decorator {
        id: usize,
        ty: DecoratorType,
        parent: usize,
        child: usize
    },
    Fallback {
        id: usize,
        parent: usize,
        children: Vec<usize>
    },
    ActionWithChild {
        id: usize,
        ty: ActionWithChildType,
        parent: usize,
        child: usize
    },
    ParameterizedAction {
        id: usize,
        parent: usize,
        ty: ParamActionType,
        children: Vec<usize>
    },
    Action {
        id: usize,
        parent: usize,
        ty: ActionType
    }
}

#[derive(Debug)]
pub enum DecoratorType {
    IsInstr {
        instr_names: Vec<String>
    },
    IsProbeType {
        probe_type: String
    },
    HasParams,
    HasAltCall,
    PredIs {
        val: bool
    },
    /// Iterates over all probes of the specified name in the list.
    ForEachProbe {
        target: String
    },
    /// Only pulls the first probe of the specified name from the list.
    ForFirstProbe {
        target: String
    }
}

#[derive(Debug)]
pub enum ActionType {
    EnterScope {
        scope_name: String
    },
    ExitScope,
    Define {
        context: String,
        var_name: String
    },
    EmitPred,
    FoldPred,
    Reset,
    SaveParams,
    EmitParams,
    EmitBody,
    EmitAltCall,
    RemoveOrig,
    EmitOrig,
    ForceSuccess
}

#[derive(Debug)]
pub enum ActionWithChildType {
    EnterPackage {
        package_name: String
    },
}

#[derive(Debug)]
pub enum ParamActionType {
    EmitIf {
        cond: usize,
        conseq: usize
    },
    EmitIfElse {
        cond: usize,
        conseq: usize,
        alt: usize
    }
}

pub trait BehaviorVisitor<T> {
    // Abstracted visit fn
    fn visit_node(&mut self, node: &Node) -> T {
        match node {
            Node::Root { .. } => self.visit_root(node),
            Node::Sequence { .. } => self.visit_sequence(node),
            Node::Decorator { .. } => self.visit_decorator(node),
            Node::Fallback { .. } => self.visit_fallback(node),
            Node::ActionWithChild { .. } => self.visit_action_with_child(node),
            Node::ParameterizedAction { .. } => self.visit_parameterized_action(node),
            Node::Action { .. } => self.visit_action(node),
        }
    }
    fn visit_root(&mut self, node: &Node) -> T;

    // Control nodes
    fn visit_sequence(&mut self, node: &Node) -> T;
    fn visit_decorator(&mut self, node: &Node) -> T {
        if let Node::Decorator { ty, ..} = node {
            match ty {
                DecoratorType::IsInstr {..} => self.visit_is_instr(node),
                DecoratorType::IsProbeType {..} => self.visit_is_probe_type(node),
                DecoratorType::HasAltCall {..} => self.visit_has_alt_call(node),
                DecoratorType::HasParams {..} => self.visit_has_params(node),
                DecoratorType::PredIs {..} => self.visit_pred_is(node),
                DecoratorType::ForEachProbe {..} => self.visit_for_each_probe(node),
                DecoratorType::ForFirstProbe {..} => self.visit_for_first_probe(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_fallback(&mut self, node: &Node) -> T;
    fn visit_action_with_child(&mut self, node: &Node) -> T {
        if let Node::ActionWithChild { ty, ..} = node {
            match ty {
                ActionWithChildType::EnterPackage {..} => self.visit_enter_package(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_parameterized_action(&mut self, node: &Node) -> T {
        if let Node::ParameterizedAction { ty, ..} = node {
            match ty {
                ParamActionType::EmitIfElse {..} => self.visit_emit_if_else(node),
                ParamActionType::EmitIf {..} => self.visit_emit_if(node)
            }
        } else {
            unreachable!()
        }
    }

    // Decorator nodes
    fn visit_is_instr(&mut self, node: &Node) -> T;
    fn visit_is_probe_type(&mut self, node: &Node) -> T;
    fn visit_has_alt_call(&mut self, node: &Node) -> T;
    fn visit_has_params(&mut self, node: &Node) -> T;
    fn visit_pred_is(&mut self, node: &Node) -> T;
    fn visit_for_each_probe(&mut self, node: &Node) -> T;
    fn visit_for_first_probe(&mut self, node: &Node) -> T;

    // Action with child nodes
    fn visit_enter_package(&mut self, node: &Node) -> T;

    // Parameterized action nodes
    fn visit_emit_if_else(&mut self, node: &Node) -> T;
    fn visit_emit_if(&mut self, node: &Node) -> T;

    // Action nodes
    fn visit_action(&mut self, node: &Node) -> T {
        if let Node::Action { ty, ..} = node {
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
                ActionType::EmitAltCall {..} => self.visit_emit_alt_call(node),
                ActionType::RemoveOrig {..} => self.visit_remove_orig(node),
                ActionType::EmitOrig {..} => self.visit_emit_orig(node),
                ActionType::ForceSuccess {..} => self.visit_force_success(node),
            }
        } else {
            unreachable!()
        }
    }
    fn visit_enter_scope(&mut self, node: &Node) -> T;
    fn visit_exit_scope(&mut self, node: &Node) -> T;
    fn visit_define(&mut self, node: &Node) -> T;
    fn visit_emit_pred(&mut self, node: &Node) -> T;
    fn visit_fold_pred(&mut self, node: &Node) -> T;
    fn visit_reset(&mut self, node: &Node) -> T;
    fn visit_save_params(&mut self, node: &Node) -> T;
    fn visit_emit_params(&mut self, node: &Node) -> T;
    fn visit_emit_body(&mut self, node: &Node) -> T;
    fn visit_emit_alt_call(&mut self, node: &Node) -> T;
    fn visit_remove_orig(&mut self, node: &Node) -> T;
    fn visit_emit_orig(&mut self, node: &Node) -> T;
    fn visit_force_success(&mut self, node: &Node) -> T;
}
