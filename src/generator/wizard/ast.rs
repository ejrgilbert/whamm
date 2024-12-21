use crate::parser::types::{Block, DataType, Expr, Global, Statement};
use std::collections::HashMap;
use crate::lang_features::report_vars::Metadata as ReportMetadata;

#[derive(Clone, Default)]
pub struct WizardScript {
    pub id: u8,
    pub fns: Vec<crate::parser::types::Fn>, // User-provided
    pub globals: HashMap<String, Global>,   // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
    /// The rules of the probes that have been used in the Script.
    pub probes: Vec<WizardProbe>,
}

#[derive(Clone)]
pub struct UnsharedVar {
    pub name: String,
    pub ty: DataType,
    pub is_report: bool,
    pub report_metadata: Option<ReportMetadata>
}

#[derive(Clone, Default)]
pub struct WizardProbe {
    pub rule: String,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub metadata: Metadata,
    pub unshared_to_alloc: Vec<UnsharedVar>,
    pub probe_number: i32,
}
impl WizardProbe {
    pub(crate) fn new(rule: String, probe_number: i32) -> Self {
        Self {
            rule,
            predicate: None,
            body: None,
            metadata: Metadata::default(),
            unshared_to_alloc: Vec::default(),
            probe_number,
        }
    }
    pub(crate) fn to_string(&self) -> String {
        format!("{}_{}", self.probe_number, self.rule)
    }
    pub(crate) fn add_unshared(&mut self, name: String, ty: DataType, is_report: bool, report_metadata: Option<ReportMetadata>) {
        self.unshared_to_alloc.push(
            UnsharedVar {
                name,
                ty,
                is_report,
                report_metadata
            });
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
