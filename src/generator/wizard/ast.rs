use crate::parser::types::{Block, DataType, Expr, Global, Statement};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct WizardScript {
    pub name: String,
    pub fns: Vec<crate::parser::types::Fn>, // User-provided
    pub globals: HashMap<String, Global>,   // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
    /// The rules of the probes that have been used in the Script.
    pub probes: Vec<WizardProbe>,
}

#[derive(Clone, Default)]
pub struct WizardProbe {
    pub rule: String,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub metadata: Metadata,
}
impl WizardProbe {
    pub(crate) fn new(rule: String) -> Self {
        Self {
            rule,
            predicate: None,
            body: None,
            metadata: Metadata::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Metadata {
    pub pred_args: Vec<(String, DataType)>,
    pub body_args: Vec<(String, DataType)>, // TODO pub num_reports: i32, // needed for `$alloc`
}
impl Metadata {
    pub fn push_pred_req(&mut self, var_name: String, var_type: DataType) {
        self.pred_args.push((var_name, var_type))
    }
    pub fn push_body_req(&mut self, var_name: String, var_type: DataType) {
        self.body_args.push((var_name, var_type))
    }
}
