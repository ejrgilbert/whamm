use crate::emitter::rewriting::rules::Arg;
use crate::lang_features::report_vars::Metadata as ReportMetadata;
use crate::parser::types::{Block, DataType, Expr, Global, Location, RulePart, Statement};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

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
    pub init_logic: Vec<Statement>,
    pub probe_number: u32,
    pub script_id: u8,
    pub loc: Option<Location>,
}
impl Display for Probe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.probe_number, self.rule)
    }
}
impl Probe {
    pub(crate) fn new(rule_str: String, probe_number: u32, script_id: u8, loc: Location) -> Self {
        Self {
            rule: ProbeRule::from(rule_str),
            predicate: None,
            body: None,
            metadata: Metadata::default(),
            unshared_to_alloc: Vec::default(),
            init_logic: Vec::default(),
            probe_number,
            script_id,
            loc: Some(loc),
        }
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
            panic!("ProbeRule should either have all four subparts, or be missing the probe mode (for wizard): {value}");
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
    pub body_args: WhammParams,
}
impl Metadata {
    pub fn push_pred_req(&mut self, var_name: String, var_type: DataType) {
        self.pred_args.push(WhammParam {
            name: var_name,
            ty: var_type,
        });
    }
    pub fn push_body_req(&mut self, var_name: String, var_type: DataType) {
        self.body_args.push(WhammParam {
            name: var_name,
            ty: var_type,
        });
    }
}

#[derive(Clone, Debug, Default)]
pub enum ReqArgs {
    #[default]
    None,
    FirstN {
        n: u32,
    },
    All,
}
impl ReqArgs {
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
            ReqArgs::All => {} // already max
            ReqArgs::None => match other {
                ReqArgs::None => {} // equal amount
                ReqArgs::FirstN { .. } | ReqArgs::All => *self = other.clone(),
            },
            ReqArgs::FirstN { n: my_n } => match other {
                ReqArgs::None => {} // less than self
                ReqArgs::FirstN { n: other_n } => {
                    let mut cmp_n = *other_n;
                    *self = ReqArgs::FirstN {
                        n: *my_n.max(&mut cmp_n),
                    }
                }
                ReqArgs::All => *self = other.clone(), // other is max
            },
        }
    }
    pub fn of(&self, args: Vec<Arg>) -> Vec<Arg> {
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
    pub req_args: ReqArgs,

    requested_args: Vec<u32>,
}
impl WhammParams {
    pub fn push(&mut self, param: WhammParam) {
        if let Some(n) = param.n_for("arg") {
            self.requested_args.push(n);
        }
        self.params.insert(param);
    }

    /// This gets called at the end of a probe visit to figure out
    /// which args to save off!!
    /// (see MetadataCollector::visit_probe)
    pub fn process_req_args(&mut self) {
        if self.requested_args.is_empty() {
            // not requesting any args, but could have been set externally!
            // so tentatively do the combination here
            self.req_args.combine(&ReqArgs::None);
            return;
        }
        let sorted_args = self.requested_args.iter().sorted();

        let mut first_n = 0;
        for (i, arg_n) in sorted_args.into_iter().enumerate() {
            if i as u32 == *arg_n {
                // is requesting the first N thus far
                first_n = *arg_n;
            } else {
                // is requesting out of order...
                // tentatively do the combination here in case hardcoded elsewhere
                self.req_args.combine(&ReqArgs::All);
                return;
            }
        }

        // has requested the first N args without skipping any!
        // tentatively do the combination here in case hardcoded elsewhere
        self.req_args.combine(&ReqArgs::FirstN { n: first_n });
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
