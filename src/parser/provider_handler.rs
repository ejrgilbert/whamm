#![allow(clippy::too_many_arguments)]

use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{cyan, green, long_line, magenta_italics, white};
use crate::generator::ast::StackReq;
use crate::parser::types::{
    Block, DataType, Definition, Expr, Fn as WhammFn, FnId, Location, ProbeRule, Rule, RulePart,
    WhammParser,
};
use crate::parser::whamm_parser::{expr_from_pair, handle_param, type_from_rule};
use glob::Pattern;
use log::error;
use pest::Parser;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::process::exit;
use termcolor::Buffer;

pub fn yml_to_providers(def_yamls: &[String]) -> Result<Vec<ProviderDef>, Box<ErrorGen>> {
    let def = read_yml(def_yamls);
    from_helper::<ProviderDef, ProviderYml>(def.providers)
}

pub fn get_matches(
    rule: &ProbeRule,
    all_providers: &[ProviderDef],
    err: &mut ErrorGen,
) -> Vec<ProviderDef> {
    let mut err_ctxt = ErrCtxt::default();
    let mut matches: Vec<ProviderDef> = vec![];
    for provider in all_providers.iter() {
        if let Ok(prov) = provider.match_on(rule, &mut err_ctxt) {
            matches.push(*prov);
        }
    }

    if matches.is_empty() {
        // only return an error if there were no matches!
        if let Some(e) = err_ctxt.get_most_specific() {
            err.add_error(e);
        } else {
            // shouldn't happen, panic
            error!("Got no matches, but without an error");
            exit(1)
        }
    }

    matches
}

// ===============================
// ==== TYPES FOR PROBE RULES ====
// ===============================

#[derive(Debug)]
pub struct Provider {
    pub(crate) def: Def,
    pub(crate) type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) packages: HashMap<String, Package>,
    next_id: u32,
}
impl Provider {
    pub fn new(def: Def, rule: &ProbeRule) -> Self {
        if let Some(prov_rule) = &rule.provider {
            Self {
                def,
                type_bounds: prov_rule.ty_info.clone(),
                packages: HashMap::new(),
                next_id: 0,
            }
        } else {
            Self {
                def,
                type_bounds: vec![],
                packages: HashMap::new(),
                next_id: 0,
            }
        }
    }
    pub fn add_probes(
        &mut self,
        loc: Location,
        matched_pkgs: &[PackageDef],
        rule: &ProbeRule,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) {
        for matched_pkg in matched_pkgs.iter() {
            let pkg = self
                .packages
                .entry(matched_pkg.def.name.clone())
                .or_insert(Package::new(matched_pkg.def.clone(), rule));

            pkg.add_probes(
                loc.clone(),
                &matched_pkg.events,
                rule,
                predicate.clone(),
                body.clone(),
                &mut self.next_id,
            );
        }
    }
}

#[derive(Debug)]
pub struct Package {
    pub(crate) def: Def,
    pub(crate) type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) events: HashMap<String, Event>,
}
impl Package {
    pub fn new(def: Def, rule: &ProbeRule) -> Self {
        if let Some(pkg_rule) = &rule.package {
            Self {
                def,
                type_bounds: pkg_rule.ty_info.clone(),
                events: HashMap::new(),
            }
        } else {
            Self {
                def,
                type_bounds: vec![],
                events: HashMap::new(),
            }
        }
    }
    pub fn add_probes(
        &mut self,
        loc: Location,
        matched_evts: &[EventDef],
        rule: &ProbeRule,
        predicate: Option<Expr>,
        body: Option<Block>,
        next_id: &mut u32,
    ) {
        for matched_evt in matched_evts.iter() {
            let evt = self
                .events
                .entry(matched_evt.def.name.clone())
                .or_insert(Event::new(matched_evt.def.clone(), rule));

            evt.add_probes(
                loc.clone(),
                &matched_evt.modes,
                predicate.clone(),
                body.clone(),
                next_id,
            );
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub(crate) def: Def,
    pub(crate) type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    pub(crate) probes: HashMap<ModeKind, Vec<Probe>>,
}
impl Event {
    pub fn new(def: Def, rule: &ProbeRule) -> Self {
        if let Some(evt_rule) = &rule.event {
            Self {
                def,
                type_bounds: evt_rule.ty_info.clone(),
                probes: HashMap::new(),
            }
        } else {
            Self {
                def,
                type_bounds: vec![],
                probes: HashMap::new(),
            }
        }
    }
    pub fn add_probes(
        &mut self,
        loc: Location,
        matched_modes: &[ModeDef],
        predicate: Option<Expr>,
        body: Option<Block>,
        next_id: &mut u32,
    ) {
        // TODO -- type_bounds for all of the hierarchy should be local to the PROBE...not to the prov/pkg/event...or it gets messed up for other probes...
        for matched_mode in matched_modes.iter() {
            let probes = self.probes.entry(matched_mode.kind.clone()).or_default();

            probes.push(Probe {
                id: *next_id,
                scope_id: 0,
                kind: matched_mode.kind.clone(),
                def: matched_mode.def.clone(),
                predicate: predicate.clone(),
                body: body.clone(),
                loc: loc.clone(),
            });
            *next_id += 1;
        }
    }
}

// ===================================
// ==== TYPES FOR PROBE RULE DEFS ====
// ===================================

#[derive(Clone, Debug, Default)]
pub struct Def {
    pub name: String,
    pub bound_vars: Vec<BoundVar>,
    pub bound_fns: Vec<BoundFunc>,
    docs: String,
}

#[derive(Debug)]
pub struct ProviderDef {
    pub def: Def,
    pub packages: Vec<PackageDef>,
}
impl CheckedFrom<ProviderYml> for ProviderDef {
    fn from(value: ProviderYml) -> Result<Self, Box<ErrorGen>> {
        let bound_vars = from_helper::<BoundVar, BoundVarYml>(value.bound_vars)?;
        let bound_fns = from_helper::<BoundFunc, BoundFuncYml>(value.bound_fns)?;
        let packages = from_helper::<PackageDef, PackageYml>(value.packages)?;
        Ok(Self {
            def: Def {
                name: value.name.clone(),
                bound_vars,
                bound_fns,
                docs: value.docs.clone(),
            },
            packages,
        })
    }
}
impl MatchOn for ProviderDef {
    fn match_on(&self, probe_rule: &ProbeRule, err_ctxt: &mut ErrCtxt) -> Result<Box<Self>, ()> {
        if let Some(RulePart {
            name: provider_patt,
            loc,
            ..
        }) = &probe_rule.provider
        {
            match match_helper(
                &self.def.name,
                "provider",
                provider_patt,
                loc,
                probe_rule,
                &self.packages,
                err_ctxt,
            ) {
                Ok(pkgs_res) => {
                    let packages: Vec<PackageDef> = pkgs_res.into_iter().map(|b| *b).collect();
                    if packages.is_empty() && probe_rule.package.is_some() {
                        // if there's a further match pattern to consider, this isn't a match!
                        // (consider wasm:begin and wasm:end)
                        err_ctxt.on_provider = Some(ErrorGen::get_parse_error(
                            Some(format!(
                                "Could not find any matches for the specified provider pattern: {provider_patt}"
                            )),
                            loc.as_ref().map(|l| l.line_col.clone()),
                            vec![],
                            vec![],
                        ));
                        Err(())
                    } else {
                        Ok(Box::new(Self {
                            def: self.def.clone(),
                            packages,
                        }))
                    }
                }
                Err(e) => {
                    err_ctxt.on_provider = Some(*e);
                    Err(())
                }
            }
        } else {
            // shouldn't happen, panic
            error!("No provider pattern in the rule!");
            exit(1)
        }
    }
}
impl PrintInfo for ProviderDef {
    fn print_info(
        &self,
        probe_rule: &ProbeRule,
        print_vars: bool,
        print_functions: bool,
        prov_buff: &mut Buffer,
        pkg_buff: &mut Buffer,
        evt_buffer: &mut Buffer,
        tabs: &mut usize,
    ) {
        magenta_italics(true, self.def.name.clone(), prov_buff);
        white(true, " provider\n".to_string(), prov_buff);

        // Print the provider description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.def.docs),
            prov_buff,
        );
        print_bound_vars(&self.def.bound_vars, print_vars, prov_buff, tabs);
        print_bound_fns(&self.def.bound_fns, print_functions, prov_buff, tabs);
        *tabs -= 1;

        long_line(prov_buff);
        white(true, "\n\n".to_string(), prov_buff);

        if !self.packages.is_empty() {
            probe_rule.print_bold_package(pkg_buff);
            for pkg in self.packages.iter() {
                pkg.print_info(
                    probe_rule,
                    print_vars,
                    print_functions,
                    prov_buff,
                    pkg_buff,
                    evt_buffer,
                    tabs,
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct PackageDef {
    def: Def,
    events: Vec<EventDef>,
}
impl CheckedFrom<PackageYml> for PackageDef {
    fn from(value: PackageYml) -> Result<Self, Box<ErrorGen>> {
        let bound_vars = from_helper::<BoundVar, BoundVarYml>(value.bound_vars)?;
        let bound_fns = from_helper::<BoundFunc, BoundFuncYml>(value.bound_fns)?;
        let events = from_helper::<EventDef, EventYml>(value.events)?;
        Ok(Self {
            def: Def {
                name: value.name.clone(),
                bound_vars,
                bound_fns,
                docs: value.docs.clone(),
            },
            events,
        })
    }
}
impl MatchOn for PackageDef {
    fn match_on(&self, probe_rule: &ProbeRule, err_ctxt: &mut ErrCtxt) -> Result<Box<Self>, ()> {
        if let Some(RulePart {
            name: pkg_patt,
            loc,
            ..
        }) = &probe_rule.package
        {
            match match_helper(
                &self.def.name,
                "package",
                pkg_patt,
                loc,
                probe_rule,
                &self.events,
                err_ctxt,
            ) {
                Ok(evts_res) => {
                    let evts: Vec<EventDef> = evts_res.into_iter().map(|b| *b).collect();
                    if evts.is_empty() {
                        if probe_rule.event.is_some() {
                            // if there's a further match pattern to consider, this isn't a match!
                            // (consider wasm:begin and wasm:end)
                            err_ctxt.on_package = Some(ErrorGen::get_parse_error(
                                Some(format!(
                                    "Could not find any matches for the specified package pattern: {pkg_patt}"
                                )),
                                loc.as_ref().map(|l| l.line_col.clone()),
                                vec![],
                                vec![],
                            ));
                            Err(())
                        } else {
                            // otherwise, we just have a shortened match rule! (wasm:begin, wasm:report)
                            Ok(Box::new(Self {
                                def: self.def.clone(),
                                events: vec![EventDef {
                                    def: Def::default(),
                                    modes: vec![ModeDef {
                                        def: Def::default(),
                                        alias: None,
                                        kind: ModeKind::Null,
                                    }],
                                }],
                            }))
                        }
                    } else {
                        Ok(Box::new(Self {
                            def: self.def.clone(),
                            events: evts,
                        }))
                    }
                }
                Err(e) => {
                    err_ctxt.on_package = Some(*e);
                    Err(())
                }
            }
        } else {
            error!("No package pattern in the rule!");
            exit(1)
        }
    }
}
impl PrintInfo for PackageDef {
    fn print_info(
        &self,
        probe_rule: &ProbeRule,
        print_vars: bool,
        print_functions: bool,
        prov_buff: &mut Buffer,
        pkg_buff: &mut Buffer,
        evt_buffer: &mut Buffer,
        tabs: &mut usize,
    ) {
        magenta_italics(true, self.def.name.clone(), pkg_buff);
        white(true, " package\n".to_string(), pkg_buff);

        // Print the package description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.def.docs),
            pkg_buff,
        );
        print_bound_vars(&self.def.bound_vars, print_vars, pkg_buff, tabs);
        print_bound_fns(&self.def.bound_fns, print_functions, pkg_buff, tabs);
        *tabs -= 1;

        long_line(pkg_buff);
        white(true, "\n\n".to_string(), pkg_buff);

        if !self.events.is_empty() {
            probe_rule.print_bold_event(evt_buffer);
            for evt in self.events.iter() {
                evt.print_info(
                    probe_rule,
                    print_vars,
                    print_functions,
                    prov_buff,
                    pkg_buff,
                    evt_buffer,
                    tabs,
                );
            }

            long_line(evt_buffer);
            white(true, "\n\n".to_string(), evt_buffer);
        }
    }
}

#[derive(Debug)]
pub struct EventDef {
    def: Def,
    modes: Vec<ModeDef>,
}
impl CheckedFrom<EventYml> for EventDef {
    fn from(value: EventYml) -> Result<Self, Box<ErrorGen>> {
        let bound_vars = from_helper::<BoundVar, BoundVarYml>(value.bound_vars)?;
        let bound_fns = from_helper::<BoundFunc, BoundFuncYml>(value.bound_fns)?;
        let modes = from_helper::<ModeDef, ModeYml>(value.supported_modes)?;
        Ok(Self {
            def: Def {
                name: value.name.clone(),
                docs: value.docs.clone(),
                bound_vars,
                bound_fns,
            },
            modes,
        })
    }
}
impl MatchOn for EventDef {
    fn match_on(&self, probe_rule: &ProbeRule, err_ctxt: &mut ErrCtxt) -> Result<Box<Self>, ()> {
        if let Some(RulePart {
            name: evt_patt,
            loc,
            ..
        }) = &probe_rule.event
        {
            match match_helper(
                &self.def.name,
                "event",
                evt_patt,
                loc,
                probe_rule,
                &self.modes,
                err_ctxt,
            ) {
                Ok(mds_res) => {
                    let mds: Vec<ModeDef> = mds_res.into_iter().map(|b| *b).collect();
                    if mds.is_empty() {
                        if probe_rule.mode.is_some() {
                            // if there's a further match pattern to consider, this isn't a match!
                            // (consider wasm:begin and wasm:end)
                            err_ctxt.on_event = Some(ErrorGen::get_parse_error(
                                Some(format!(
                                    "Could not find any matches for the specified event pattern: {evt_patt}"
                                )),
                                loc.as_ref().map(|l| l.line_col.clone()),
                                vec![],
                                vec![],
                            ));
                            Err(())
                        } else {
                            // otherwise, we just have a shortened match rule! (wasm:func:entry)
                            Ok(Box::new(Self {
                                def: self.def.clone(),
                                modes: vec![ModeDef {
                                    def: Def::default(),
                                    alias: None,
                                    kind: ModeKind::Null,
                                }],
                            }))
                        }
                    } else {
                        // we had mode matches
                        Ok(Box::new(Self {
                            def: self.def.clone(),
                            modes: mds,
                        }))
                    }
                }
                Err(e) => {
                    err_ctxt.on_event = Some(*e);
                    Err(())
                }
            }
        } else {
            error!("The rule must contain an event pattern.");
            exit(1);
        }
    }
}
impl PrintInfo for EventDef {
    fn print_info(
        &self,
        probe_rule: &ProbeRule,
        print_vars: bool,
        print_functions: bool,
        prov_buff: &mut Buffer,
        pkg_buff: &mut Buffer,
        evt_buffer: &mut Buffer,
        tabs: &mut usize,
    ) {
        if self.def.name.is_empty() {
            return;
        }
        magenta_italics(true, self.def.name.clone(), evt_buffer);
        white(true, " event\n".to_string(), evt_buffer);

        // Print the event description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.def.docs),
            evt_buffer,
        );
        print_bound_vars(&self.def.bound_vars, print_vars, evt_buffer, tabs);
        print_bound_fns(&self.def.bound_fns, print_functions, evt_buffer, tabs);

        *tabs -= 1;

        if !self.modes.is_empty() {
            probe_rule.print_bold_mode(evt_buffer);
            for mode in self.modes.iter() {
                if !matches!(mode.kind, ModeKind::Null) {
                    mode.print_info(
                        probe_rule,
                        print_vars,
                        print_functions,
                        prov_buff,
                        pkg_buff,
                        evt_buffer,
                        tabs,
                    );
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum ModeKind {
    Before,
    After,
    Alt,
    SemanticAfter,
    BlockAlt,
    Entry,
    Exit,

    // For skipping the mode
    #[default]
    Null,
}
impl From<String> for ModeKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "before" => Self::Before,
            "after" => Self::After,
            "alt" => Self::Alt,
            "semantic_after" => Self::SemanticAfter,
            "block_alt" => Self::BlockAlt,
            "entry" => Self::Entry,
            "exit" => Self::Exit,
            "<no-mode>" => Self::Null,
            _ => {
                error!("unable to match mode kind: {value}");
                exit(1);
            }
        }
    }
}
impl Display for ModeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
impl ModeKind {
    pub fn name(&self) -> String {
        match self {
            Self::Before => "before".to_string(),
            Self::After => "after".to_string(),
            Self::Alt => "alt".to_string(),
            Self::SemanticAfter => "semantic_after".to_string(),
            Self::BlockAlt => "block_alt".to_string(),
            Self::Entry => "entry".to_string(),
            Self::Exit => "exit".to_string(),
            Self::Null => "<no-mode>".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModeDef {
    def: Def,
    alias: Option<String>,
    kind: ModeKind,
}
impl CheckedFrom<ModeYml> for ModeDef {
    fn from(value: ModeYml) -> Result<Self, Box<ErrorGen>> {
        Ok(Self {
            def: Def {
                name: value.name.clone(),
                bound_vars: vec![],
                bound_fns: vec![],
                docs: value.docs.clone(),
            },
            alias: value.alias_to.clone(),
            kind: ModeKind::from(value.name),
        })
    }
}
impl MatchOn for ModeDef {
    fn match_on(&self, probe_rule: &ProbeRule, err_ctxt: &mut ErrCtxt) -> Result<Box<Self>, ()> {
        if let Some(RulePart {
            name: md_patt, loc, ..
        }) = &probe_rule.mode
        {
            let match_on = if let Some(alias) = &self.alias {
                alias.clone()
            } else {
                self.def.name.clone()
            };
            if is_match(&match_on, md_patt) {
                Ok(Box::new(self.clone()))
            } else {
                err_ctxt.on_mode = Some(ErrorGen::get_parse_error(
                    Some(format!(
                        "Could not find any matches for the specified mode pattern: {md_patt}"
                    )),
                    loc.as_ref().map(|l| l.line_col.clone()),
                    vec![],
                    vec![],
                ));
                Err(())
            }
        } else {
            error!("No mode pattern in the rule!");
            exit(1);
        }
    }
}
impl PrintInfo for ModeDef {
    fn print_info(
        &self,
        _probe_rule: &ProbeRule,
        print_vars: bool,
        print_functions: bool,
        prov_buff: &mut Buffer,
        _pkg_buff: &mut Buffer,
        evt_buffer: &mut Buffer,
        tabs: &mut usize,
    ) {
        magenta_italics(true, format!("    {}", self.def.name), evt_buffer);
        white(true, " mode\n".to_string(), evt_buffer);

        // Print the mode description
        *tabs += 2;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.def.docs),
            evt_buffer,
        );
        print_bound_vars(&self.def.bound_vars, print_vars, prov_buff, tabs);
        print_bound_fns(&self.def.bound_fns, print_functions, prov_buff, tabs);

        *tabs -= 2;
    }
}

#[derive(Clone, Debug)]
pub struct BoundVar {
    pub name: String,
    docs: String,
    pub ty: DataType,
    pub lifetime: Definition,
    pub derived_from: Option<Expr>,
}
impl CheckedFrom<BoundVarYml> for BoundVar {
    fn from(value: BoundVarYml) -> Result<Self, Box<ErrorGen>> {
        let ty = parse_helper::<DataType>("DataType", Rule::TYPE_YML, &value.ty, &type_from_rule);

        let derived_from = value.derived_from.map(|derived_from| {
            parse_helper::<Expr>("Expr", Rule::assignment_rhs, &derived_from, &expr_from_pair)
        });

        Ok(Self {
            name: value.name.to_owned(),
            docs: value.docs.to_owned(),
            ty,
            derived_from,
            lifetime: Definition::from(value.lifetime.as_str()),
        })
    }
}
impl BoundVar {
    pub fn print_info(&self, buff: &mut Buffer, tabs: &mut usize) {
        white(false, " ".repeat(*tabs * 4).to_string(), buff);
        green(true, self.name.to_string(), buff);
        white(true, ": ".to_string(), buff);
        self.ty.print(buff);
        if let Some(expr) = &self.derived_from {
            white(true, " ==> ".to_string(), buff);
            cyan(true, format!("{expr}"), buff);
        }

        *tabs += 1;
        white(
            false,
            format!("\n{}{}\n", " ".repeat(*tabs * 4), self.docs),
            buff,
        );
        *tabs -= 1;
    }
}

trait CheckedFrom<F>
where
    Self: Sized,
{
    fn from(value: F) -> Result<Self, Box<ErrorGen>>;
}

#[derive(Clone, Debug)]
pub struct BoundFunc {
    pub func: WhammFn,
    pub req_args: StackReq, // TODO: Remove this...it's wasm opcode specific...
    docs: String,
}
impl CheckedFrom<BoundFuncYml> for BoundFunc {
    fn from(value: BoundFuncYml) -> Result<Self, Box<ErrorGen>> {
        let params = match WhammParser::parse(Rule::fn_params, &value.params) {
            Ok(mut pairs) => {
                let mut err = ErrorGen::new("".to_string(), "".to_string(), 15);
                let mut params = vec![];
                let mut next = pairs.next();
                while let Some(n) = &next {
                    if matches!(n.as_rule(), Rule::param) {
                        if let Some(param) = handle_param(n.clone().into_inner(), &mut err) {
                            params.push(param)
                        }
                        next = pairs.next();
                    } else {
                        break;
                    }
                }
                if err.has_errors {
                    return Err(Box::new(err));
                } else {
                    params
                }
            }
            Err(e) => {
                error!(
                    "Could not parse the token as function parameters: {}\n{:?}",
                    e, value.params
                );
                exit(1);
            }
        };
        let results =
            parse_helper::<DataType>("DataType", Rule::TYPE_YML, &value.results, &type_from_rule);

        Ok(Self {
            func: WhammFn {
                def: Definition::from(value.lifetime.as_str()),
                name: FnId {
                    name: value.name.to_owned(),
                    loc: None,
                },
                params,
                results,
                body: Block::default(),
            },
            req_args: StackReq::new(value.req_args),
            docs: value.docs.to_owned(),
        })
    }
}
impl BoundFunc {
    fn print_info(&self, buffer: &mut Buffer, tabs: &mut usize) {
        green(true, " ".repeat(*tabs * 4).to_string(), buffer);

        green(true, self.func.name.name.to_string(), buffer);
        white(true, "(".to_string(), buffer);
        let mut is_first = true;
        for (param_name, param_ty) in self.func.params.iter() {
            if !is_first {
                white(true, ", ".to_string(), buffer);
            }
            if let Expr::VarId { name, .. } = param_name {
                green(true, name.to_string(), buffer);
                white(true, ": ".to_string(), buffer);
                param_ty.print(buffer);
            }
            is_first = false;
        }
        white(true, ")".to_string(), buffer);

        white(true, " -> ".to_string(), buffer);
        self.func.results.print(buffer);

        green(true, "\n".to_string(), buffer);
        *tabs += 1;
        white(
            false,
            format!("{}{}\n", " ".repeat(*tabs * 4), self.docs),
            buffer,
        );
        *tabs -= 1;
    }
}

pub trait PrintInfo {
    fn print_info(
        &self,
        probe_rule: &ProbeRule,
        print_vars: bool,
        print_functions: bool,
        prov_buff: &mut Buffer,
        pkg_buff: &mut Buffer,
        evt_buffer: &mut Buffer,
        tabs: &mut usize,
    );
}
trait MatchOn {
    fn match_on(&self, probe_rule: &ProbeRule, err_ctxt: &mut ErrCtxt) -> Result<Box<Self>, ()>;
}

#[derive(Clone, Debug)]
pub struct Probe {
    // The ID of the probe (in order of placement in script)
    pub id: u32,
    // The ID used to identify this probe's scope in the symbol table
    pub scope_id: usize,
    pub kind: ModeKind,
    pub def: Def,
    pub predicate: Option<Expr>,
    pub body: Option<Block>,
    pub loc: Location,
}

// ===========================
// ==== UTILITY FUNCTIONS ====
// ===========================

fn print_bound_vars(vars: &[BoundVar], print_vars: bool, buff: &mut Buffer, tabs: &mut usize) {
    if print_vars && !vars.is_empty() {
        white(true, format!("{}VARIABLES:\n", " ".repeat(*tabs * 4)), buff);
        *tabs += 1;
        for var in vars.iter() {
            var.print_info(buff, tabs);
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buff);
    }
}

fn print_bound_fns(fns: &[BoundFunc], print_functions: bool, buff: &mut Buffer, tabs: &mut usize) {
    if print_functions && !fns.is_empty() {
        white(true, format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)), buff);
        *tabs += 1;
        for f in fns.iter() {
            f.print_info(buff, tabs);
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buff);
    }
}

fn match_helper<T: MatchOn>(
    name: &str,
    ctxt: &str,
    pattern: &str,
    loc: &Option<Location>,
    rule: &ProbeRule,
    to_check: &[T],
    err_ctxt: &mut ErrCtxt,
) -> Result<Vec<Box<T>>, Box<WhammError>> {
    let mut matches = vec![];
    if is_match(name, pattern) {
        for item in to_check.iter() {
            if let Ok(m) = item.match_on(rule, err_ctxt) {
                matches.push(m);
            }
        }
    } else {
        // create an error here
        return Err(Box::new(ErrorGen::get_parse_error(
            Some(format!(
                "Could not find any matches for the specified {ctxt} pattern: {pattern}"
            )),
            loc.as_ref().map(|l| l.line_col.clone()),
            vec![],
            vec![],
        )));
    }
    Ok(matches)
}

fn is_match(name: &str, patt: &str) -> bool {
    let globs = get_globs(&patt.to_lowercase());

    for glob in globs.iter() {
        if glob.matches(&name.to_lowercase()) {
            return true;
        }
    }
    false
}

fn get_globs(patt: &str) -> Vec<Pattern> {
    let mut globs = vec![];
    for p in patt.split('|') {
        globs.push(Pattern::new(p).unwrap());
    }

    globs
}

type RuleHandler<T> = dyn Fn(Pair<Rule>) -> Result<T, Vec<WhammError>>;
fn parse_helper<T>(target: &str, parse_rule: Rule, token: &str, handler: &RuleHandler<T>) -> T {
    match WhammParser::parse(parse_rule, token) {
        Ok(mut pairs) => {
            if let Some(pair) = pairs.next() {
                handler(pair).unwrap_or_else(|errs| {
                    error!("Could not parse the token correctly");
                    for e in errs.iter() {
                        error!("{:?}", e)
                    }
                    exit(1)
                })
            } else {
                error!("Could not parse the token correctly");
                exit(1)
            }
        }
        Err(e) => {
            error!("Could not parse the token as a {target}: {token}\n{:?}", e);
            exit(1)
        }
    }
}

fn from_helper<T: CheckedFrom<F>, F: Clone>(list: Vec<F>) -> Result<Vec<T>, Box<ErrorGen>> {
    let mut new_list = vec![];
    for item in list.iter() {
        new_list.push(T::from(item.clone())?);
    }
    Ok(new_list)
}

#[derive(Default)]
struct ErrCtxt {
    on_provider: Option<WhammError>,
    on_package: Option<WhammError>,
    on_event: Option<WhammError>,
    on_mode: Option<WhammError>,
}
impl ErrCtxt {
    // fn has_error(&self) -> bool {
    //     return self.on_provider.is_some() || self.on_package.is_some() || self.on_event.is_some() || self.on_mode.is_some()
    // }
    fn get_most_specific(&self) -> Option<WhammError> {
        if self.on_mode.is_some() {
            self.on_mode.clone()
        } else if self.on_event.is_some() {
            self.on_event.clone()
        } else if self.on_package.is_some() {
            self.on_package.clone()
        } else if self.on_provider.is_some() {
            self.on_provider.clone()
        } else {
            None
        }
    }
}

// =====================
// ==== IR FOR YAML ====
// =====================

fn read_yml(def_yamls: &[String]) -> YmlDefinition {
    let mut all_yml = "".to_string();
    for yml in def_yamls.iter() {
        all_yml += yml;
    }
    let def: YmlDefinition = serde_yml::from_str(&all_yml).expect("Could not read values.");

    def
}

#[derive(Debug, Serialize, Deserialize)]
struct YmlDefinition {
    providers: Vec<ProviderYml>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProviderYml {
    name: String,
    bound_vars: Vec<BoundVarYml>,
    bound_fns: Vec<BoundFuncYml>,
    docs: String,
    packages: Vec<PackageYml>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PackageYml {
    name: String,
    bound_vars: Vec<BoundVarYml>,
    bound_fns: Vec<BoundFuncYml>,
    docs: String,
    events: Vec<EventYml>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EventYml {
    name: String,
    bound_vars: Vec<BoundVarYml>,
    bound_fns: Vec<BoundFuncYml>,
    supported_modes: Vec<ModeYml>,
    docs: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoundVarYml {
    name: String,
    docs: String,
    #[serde(rename = "type")]
    ty: String,
    derived_from: Option<String>,
    lifetime: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoundFuncYml {
    name: String,
    params: String,
    results: String,
    req_args: i32, // TODO: Remove this...it's wasm opcode specific...
    docs: String,
    lifetime: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ModeYml {
    name: String,
    docs: String,
    alias_to: Option<String>,
}
