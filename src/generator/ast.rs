use crate::emitter::rewriting::rules::StackVal;
use crate::lang_features::report_vars::Metadata as ReportMetadata;
use crate::parser::provider_handler::ModeKind;
use crate::parser::types::{
    Block, DataType, Definition, Expr, Global, Location, RulePart, Statement,
};
use itertools::Itertools;
use log::error;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::process::exit;

#[derive(Clone, Default)]
pub struct Script {
    pub id: u8,
    pub fns: Vec<crate::parser::types::Fn>, // User-provided
    pub globals: HashMap<String, Global>,   // User-provided, should be VarId
    pub global_stmts: Vec<Statement>,
    /// The rules of the probes that have been used in the Script.
    pub probes: Vec<Probe>,
}

#[derive(Clone, Debug)]
pub struct UnsharedVar {
    pub name: String,
    pub ty: DataType,
    pub is_report: bool,
    pub report_metadata: Option<ReportMetadata>,
    pub loc: Option<Location>,
}

#[derive(Clone, Debug, Default)]
pub struct Probe {
    pub rule: ProbeRule,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub metadata: Metadata,
    pub unshared_to_alloc: Vec<UnsharedVar>,
    pub static_lib_calls: Vec<(WhammParams, Expr)>,
    pub init_logic: Vec<Statement>,
    pub probe_number: u32,
    pub scope_id: usize,
    pub script_id: u8,
    pub loc: Option<Location>,
}
impl Display for Probe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.probe_number, self.rule)
    }
}
impl Probe {
    pub(crate) fn new(
        rule_str: String,
        probe_number: u32,
        scope_id: usize,
        script_id: u8,
        loc: Location,
    ) -> Self {
        Self {
            rule: ProbeRule::from(rule_str),
            probe_number,
            scope_id,
            script_id,
            loc: Some(loc),
            ..Default::default()
        }
    }
    pub(crate) fn set_pred(&mut self, pred: Option<Expr>) {
        self.predicate = pred;
    }
    pub(crate) fn set_body(&mut self, body: Option<Block>) {
        self.body = body;
    }
    /// Adds a new static library call and returns what can be used
    /// to refer to the lib call result
    pub(crate) fn add_static_lib_call(&mut self, params: WhammParams, call: Expr) -> Expr {
        let i = self.static_lib_calls.len();
        self.static_lib_calls.push((params, call));

        Expr::VarId {
            definition: Definition::CompilerStatic,
            name: Self::get_call_alias_for(i),
            loc: None,
        }
    }
    pub(crate) fn get_call_alias_for(i: usize) -> String {
        format!("@static{i}")
    }
    pub(crate) fn add_unshared(
        &mut self,
        name: String,
        ty: DataType,
        is_report: bool,
        report_metadata: Option<ReportMetadata>,
        loc: &Option<Location>,
    ) {
        self.unshared_to_alloc.push(UnsharedVar {
            name,
            ty,
            is_report,
            report_metadata,
            loc: loc.clone(),
        });
    }
    pub(crate) fn add_init_logic(&mut self, stmt: Statement) {
        self.init_logic.push(stmt);
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProbeRule {
    pub provider: RulePart,
    pub package: RulePart,
    pub event: RulePart,
    pub mode: RulePart,
}
impl From<String> for ProbeRule {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(':').collect();

        if parts.len() == 3 {
            Self {
                provider: RulePart::new(parts[0].to_owned(), None),
                package: RulePart::new(parts[1].to_owned(), None),
                event: RulePart::new(parts[2].to_owned(), None),
                mode: RulePart::new("".to_string(), None),
            }
        } else if parts.len() == 4 {
            Self {
                provider: RulePart::new(parts[0].to_owned(), None),
                package: RulePart::new(parts[1].to_owned(), None),
                event: RulePart::new(parts[2].to_owned(), None),
                mode: RulePart::new(parts[3].to_owned(), None),
            }
        } else {
            panic!(
                "ProbeRule should either have all four subparts, or be missing the probe mode (for wei): {value}"
            );
        }
    }
}
impl Display for ProbeRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.mode.name.is_empty() {
            f.write_str(&format!(
                "{}:{}:{}",
                self.provider.name, self.package.name, self.event.name
            ))
        } else {
            f.write_str(&format!(
                "{}:{}:{}:{}",
                self.provider.name, self.package.name, self.event.name, self.mode.name
            ))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub pred_is_dynamic: bool,
    // These are hashsets to avoid requesting duplicate data
    pub pred_args: WhammParams,
    pub init_args: WhammParams,
    pub body_args: WhammParams,
}
impl Metadata {
    pub fn push_pred_req(&mut self, var_name: String, var_type: DataType, mode: &ModeKind) {
        Self::push_req(&mut self.pred_args, var_name, var_type, mode);
    }
    pub fn push_body_req(&mut self, var_name: String, var_type: DataType, mode: &ModeKind) {
        Self::push_req(&mut self.body_args, var_name, var_type, mode);
    }
    pub fn push_init_req(&mut self, var_name: String, var_type: DataType, mode: &ModeKind) {
        Self::push_req(&mut self.init_args, var_name, var_type, mode);
    }
    fn push_req(params: &mut WhammParams, var_name: String, var_type: DataType, mode: &ModeKind) {
        params.push(
            WhammParam {
                name: var_name,
                ty: var_type,
            },
            mode,
        );
    }
}

#[derive(Clone, Debug, Default)]
pub enum StackReq {
    #[default]
    None,
    FirstN {
        n: u32,
    },
    All,
}
impl StackReq {
    pub fn new(n: i32) -> Self {
        if n == -1 {
            Self::All
        } else if n == 0 {
            Self::None
        } else {
            Self::FirstN { n: n as u32 }
        }
    }
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn matches(&self, num_args: usize) -> bool {
        // Check if the requested args is within the bounds of the available args
        match self {
            Self::None | Self::All => true,
            Self::FirstN { n } => *n < num_args as u32,
        }
    }

    /// Make `self` request the most of the two `ReqArgs` instances.
    pub fn combine(&mut self, other: &Self) {
        match self {
            Self::All => {} // already max
            Self::None => match other {
                Self::None => {} // equal amount
                Self::FirstN { .. } | Self::All => *self = other.clone(),
            },
            Self::FirstN { n: my_n } => match other {
                Self::None => {} // less than self
                Self::FirstN { n: other_n } => {
                    let mut cmp_n = *other_n;
                    *self = Self::FirstN {
                        n: *my_n.max(&mut cmp_n),
                    }
                }
                Self::All => *self = other.clone(), // other is max
            },
        }
    }
    pub fn of(&self, args: Vec<StackVal>) -> Vec<StackVal> {
        if args.is_empty() {
            return vec![];
        }
        match self {
            Self::None => vec![],
            Self::All => args,
            Self::FirstN { n } => {
                if *n == 0 {
                    vec![args.first().unwrap().clone()]
                } else {
                    args.as_slice()[0..*n as usize + 1].to_vec()
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct WhammParams {
    pub params: HashSet<WhammParam>,
    pub req_args: StackReq,
    pub req_results: StackReq,

    requested_args: Vec<u32>,
    requested_results: Vec<u32>,
}
impl WhammParams {
    pub fn extend(&mut self, other: WhammParams) {
        self.params.extend(other.params);
        self.req_args.combine(&other.req_args);
        self.req_results.combine(&other.req_results);
        self.requested_args.extend(&other.requested_args);
        self.requested_results.extend(&other.requested_results);
    }
    pub fn push(&mut self, param: WhammParam, mode: &ModeKind) {
        if let Some(n) = param.n_for("arg") {
            self.requested_args.push(n);
        }
        if let Some(n) = param.n_for("res") {
            if !matches!(mode, ModeKind::After) {
                error!("we haven't supported bound resN variables in non-after probes yet!");
                exit(1)
            }
            self.requested_results.push(n);
        }
        self.params.insert(param);
    }

    /// This gets called at the end of a probe visit to figure out
    /// which stack values to save off!!
    /// (see MetadataCollector::visit_probe)
    pub fn process_stack_reqs(&mut self) {
        Self::process_stack_req(&mut self.requested_args, &mut self.req_args);
        Self::process_stack_req(&mut self.requested_results, &mut self.req_results);
    }

    fn process_stack_req(requested: &mut [u32], stack_req: &mut StackReq) {
        if requested.is_empty() {
            // not requesting any stack vals, but could have been set externally!
            // so tentatively do the combination here
            stack_req.combine(&StackReq::None);
            return;
        }
        let sorted = requested.iter().sorted();

        let mut first_n = 0;
        for (i, req_n) in sorted.into_iter().enumerate() {
            if i as u32 == *req_n {
                // is requesting the first N thus far
                first_n = *req_n;
            } else {
                // is requesting out of order...
                // tentatively do the combination here in case hardcoded elsewhere
                stack_req.combine(&StackReq::All);
                return;
            }
        }

        // has requested the first N reqs without skipping any!
        // tentatively do the combination here in case hardcoded elsewhere
        stack_req.combine(&StackReq::FirstN { n: first_n });
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WhammParam {
    pub(crate) name: String,
    pub ty: DataType,
}
impl WhammParam {
    pub fn n_for(&self, prefix: &str) -> Option<u32> {
        if self.name.starts_with(prefix) {
            if let Ok(n) = self.name[prefix.len()..].parse::<u32>() {
                return Some(n);
            }
        }
        None
    }
}

// TODO -- create a default implementation!
pub trait AstVisitor<T> {
    fn visit_ast(&mut self, ast: &[Script]) -> T;
    fn visit_script(&mut self, script: &Script) -> T;
    fn visit_probe(&mut self, probe: &Probe) -> T;
    fn visit_metadata(&mut self, metadata: &Metadata) -> T;
    fn visit_whamm_param(&mut self, param: &WhammParam) -> T;
    fn visit_fn(&mut self, f: &crate::parser::types::Fn) -> T;
    fn visit_formal_param(&mut self, param: &(Expr, DataType)) -> T;
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_stmt(&mut self, stmt: &Statement) -> T;
    fn visit_datatype(&mut self, datatype: &DataType) -> T;
}
