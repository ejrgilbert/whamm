use crate::lang_features::report_vars::Metadata as ReportMetadata;
use crate::parser::types::{Block, DataType, Expr, Global, RulePart, Statement};
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
}

#[derive(Clone, Debug, Default)]
pub struct Probe {
    pub rule: ProbeRule,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub metadata: Metadata,
    pub unshared_to_alloc: Vec<UnsharedVar>,
    pub probe_number: u32,
    pub script_id: u8,

    // tracking
    pub body_fid: Option<u32>
}
impl Display for Probe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.probe_number, self.rule)
    }
}
impl Probe {
    pub(crate) fn new(rule_str: String, probe_number: u32, script_id: u8) -> Self {
        Self {
            rule: ProbeRule::from(rule_str),
            predicate: None,
            body: None,
            metadata: Metadata::default(),
            unshared_to_alloc: Vec::default(),
            probe_number,
            script_id,
            body_fid: None
        }
    }
    pub(crate) fn add_unshared(
        &mut self,
        name: String,
        ty: DataType,
        is_report: bool,
        report_metadata: Option<ReportMetadata>,
    ) {
        self.unshared_to_alloc.push(UnsharedVar {
            name,
            ty,
            is_report,
            report_metadata,
        });
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
            panic!("ProbeRule should either have all for subparts, or be missing the probe mode (for wizard)");
        }
    }
}
impl Display for ProbeRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.mode.name.is_empty() {
            f.write_str(&format!(
                "{}:{}:{}",
                self.provider.name,
                self.package.name,
                self.event.name
            ))
        } else {
            f.write_str(&format!(
                "{}:{}:{}:{}",
                self.provider.name,
                self.package.name,
                self.event.name,
                self.mode.name
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
        self.pred_args.push(WhammParam::new(var_name, var_type));
    }
    pub fn push_body_req(&mut self, var_name: String, var_type: DataType) {
        self.body_args.push(WhammParam::new(var_name, var_type));
    }
}

#[derive(Clone, Debug, Default)]
pub struct WhammParams {
    pub params: HashSet<WhammParam>,
    pub req_args: bool,
}
impl WhammParams {
    pub fn push(&mut self, param: WhammParam) {
        if matches!(param, WhammParam::Arg { .. }) {
            self.req_args = true;
        }
        self.params.insert(param);
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WhammParam {
    Pc,
    Fid,
    Fname,
    Imm { n: u32, ty: DataType },
    Arg { n: u32, ty: DataType },
    Local { n: u32, ty: DataType },
    AllocOffset,

    // calls
    TargetFnType,
    TargetFnName,
    TargetImpModule,

    // br_table
    Targets,
    NumTargets,
    DefaultTarget
}
impl WhammParam {
    pub fn new(var_name: String, var_type: DataType) -> Self {
        let mut obj = Self::from(var_name);
        obj.set_ty(var_type);

        obj
    }
    pub fn set_ty(&mut self, t: DataType) {
        match self {
            Self::Imm { ty, .. } | Self::Arg { ty, .. } | Self::Local { ty, .. } => *ty = t,
            Self::Pc | Self::Fid | Self::Fname | Self::AllocOffset |
            Self::TargetFnType | Self::TargetFnName | Self::TargetImpModule |
            Self::Targets | Self::NumTargets | Self::DefaultTarget => {
                assert_eq!(t, self.ty())
            }
        }
    }
    pub fn ty(&self) -> DataType {
        match self {
            Self::Pc => DataType::U32,
            Self::Fid => DataType::U32,
            Self::Fname => DataType::Str,
            Self::Imm { ty, .. } => ty.clone(),
            Self::Arg { ty, .. } => ty.clone(),
            Self::Local { ty, .. } => ty.clone(),
            Self::AllocOffset => DataType::U32,
            Self::TargetFnType => DataType::Str,
            Self::TargetFnName => DataType::Str,
            Self::TargetImpModule => DataType::Str,
            Self::Targets => DataType::Map {
                key_ty: Box::new(DataType::U32),
                val_ty: Box::new(DataType::U32)
            }, // TODO -- really want to request mapID though...
            Self::NumTargets => DataType::U32,
            Self::DefaultTarget => DataType::U32,
        }
    }
}
impl From<String> for WhammParam {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pc" => return Self::Pc,
            "fid" => return Self::Fid,
            "fname" => return Self::Fname,
            "target_fn_type" => return Self::TargetFnType,
            "target_fn_name" => return Self::TargetFnName,
            "target_imp_module" => return Self::TargetImpModule,
            "targets" => return Self::Targets,
            "num_targets" => return Self::NumTargets,
            "default_target" => return Self::DefaultTarget,
            _ => {}
        }

        // handle immN, argN, localN
        if let Some(n) = handle_special(&value, "imm".to_string()) {
            return Self::Imm {
                n,
                ty: DataType::Unknown,
            };
        }
        if let Some(n) = handle_special(&value, "arg".to_string()) {
            return Self::Arg {
                n,
                ty: DataType::Unknown,
            };
        }
        if let Some(n) = handle_special(&value, "local".to_string()) {
            return Self::Local {
                n,
                ty: DataType::Unknown,
            };
        }
        fn handle_special(value: &str, prefix: String) -> Option<u32> {
            if value.starts_with(&prefix) {
                if let Ok(n) = value[prefix.len()..].parse::<u32>() {
                    return Some(n);
                }
            }
            None
        }

        panic!("Invalid WhammParam request: {}", value);
    }
}
impl Display for WhammParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pc => f.write_str("pc"),
            Self::Fid => f.write_str("fid"),
            Self::Fname => f.write_str("fname"),
            Self::Imm { n, .. } => f.write_str(&format!("imm{n}")),
            Self::Arg { n, .. } => f.write_str(&format!("arg{n}")),
            Self::Local { n, .. } => f.write_str(&format!("local{n}")),
            // TODO -- unsure what to do for the alloc part...
            Self::AllocOffset => f.write_str("alloc"),
            Self::TargetFnType => f.write_str("target_fn_type"),
            Self::TargetFnName => f.write_str("target_fn_name"),
            Self::TargetImpModule => f.write_str("target_imp_module"),
            Self::Targets => f.write_str("targets"),
            Self::NumTargets => f.write_str("num_targets"),
            Self::DefaultTarget => f.write_str("default_target"),
        }
    }
}
