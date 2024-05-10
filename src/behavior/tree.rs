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
            parent: Some(self.curr),
            child: 0,
        });
        self
    }

    pub fn exit_decorator(&mut self) -> &mut Self {
        match self.get_curr_mut() {
            Some(Node::Decorator {parent, ..}) => {
                if let Some(parent) = parent {
                    self.curr = parent.clone()
                } else {
                    error!("Attempted to exit decorator, but there is no parent");
                }
            },
            other => {
                error!("Something went wrong, expected Decorator, but was: {:?}", other)
            }
        };
        self
    }

    // ==================
    // ==== Actions =====
    // ==================

    pub fn define(&mut self, context: String, var_name: String) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
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
            parent: Some(self.curr),
            ty: ActionType::EmitBody
        });
        self
    }

    pub fn emit_if_else(&mut self, cond: usize, conseq: usize, alt: usize) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
            ty: ActionType::EmitIfElse {
                cond,
                conseq,
                alt
            }
        });
        self
    }

    pub fn emit_params(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
            ty: ActionType::EmitParams
        });
        self
    }

    pub fn enter_scope(&mut self, scope_name: String) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
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
            parent: Some(self.curr),
            ty: ActionType::ExitScope
        });
        self
    }

    pub fn fold_pred(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
            ty: ActionType::FoldPred
        });
        self
    }

    pub fn force_success(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
            ty: ActionType::ForceSuccess
        });
        self
    }

    pub fn save_params(&mut self) -> &mut Self {
        let id = self.nodes.len();
        self.put_child(Node::Action {
            id,
            parent: Some(self.curr),
            ty: ActionType::SaveParams
        });
        self
    }

    // ==================
    // ==== Base Fns ====
    // ==================

    pub fn put_child(&mut self, node: Node) -> bool {
        let mut added = false;
        let new_id = self.nodes.len();

        if let Some(curr) = self.get_curr_mut() {
            match curr {
                Node::Root { child, .. } => {
                    *child = new_id;
                    added = true;
                }
                Node::Sequence { children, .. } => {
                    children.push(new_id);
                    added = true;
                }
                Node::Decorator { child, .. } => {
                    *child = new_id;
                    added = true;
                }
                Node::Fallback { children, .. } => {
                    children.push(new_id);
                    added = true;
                }
                _ => {
                    error!("Cannot add child to this Tree node type");
                }
            }
        }
        if added {
            self.nodes.push(node);
        }
        added
    }

    pub fn put_child_and_enter(&mut self, node: Node) -> bool {
        if self.put_child(node) {
            self.curr += 1;
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
                if let Some(parent) = parent {
                    self.curr = parent.clone()
                } else {
                    error!("Attempted to exit decorator, but there is no parent");
                }
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
        parent: Option<usize>,
        child: usize
    },
    Fallback {
        id: usize,
        parent: usize,
        children: Vec<usize>
    },
    Action {
        id: usize,
        parent: Option<usize>,
        ty: ActionType
    }
}

#[derive(Debug)]
pub enum DecoratorType {
    IsInstr {
        instr_name: String
    },
    IsProbeType {
        probe_type: String
    },
    HasParams,
    PredIs {
        val: bool
    },
    ForEach {
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
    EmitOrig,
    EmitIfElse {
        cond: usize,
        conseq: usize,
        alt: usize
    },
    ForceSuccess
}

pub trait BehaviorVisitor<T> {
    // Abstracted visit fn
    fn visit_node(&mut self, node: &Node) -> T;
    fn visit_root(&mut self, node: &Node) -> T;

    // Control nodes
    fn visit_sequence(&mut self, node: &Node) -> T;
    fn visit_decorator(&mut self, node: &Node) -> T;
    fn visit_fallback(&mut self, node: &Node) -> T;

    // Decorator nodes
    fn visit_is_instr(&mut self, node: &Node) -> T;
    fn visit_is_probe_type(&mut self, node: &Node) -> T;
    fn visit_has_params(&mut self, node: &Node) -> T;
    fn visit_pred_is(&mut self, node: &Node) -> T;
    fn visit_for_each(&mut self, node: &Node) -> T;

    // Action nodes
    fn visit_action(&mut self, action: &Node) -> T;
    fn visit_enter_scope(&mut self, node: &Node) -> T;
    fn visit_exit_scope(&mut self, node: &Node) -> T;
    fn visit_define(&mut self, node: &Node) -> T;
    fn visit_emit_pred(&mut self, node: &Node) -> T;
    fn visit_fold_pred(&mut self, node: &Node) -> T;
    fn visit_reset(&mut self, node: &Node) -> T;
    fn visit_save_params(&mut self, node: &Node) -> T;
    fn visit_emit_params(&mut self, node: &Node) -> T;
    fn visit_emit_body(&mut self, node: &Node) -> T;
    fn visit_emit_orig(&mut self, node: &Node) -> T;
    fn visit_emit_if_else(&mut self, node: &Node) -> T;
    fn visit_force_success(&mut self, node: &Node) -> T;
}
