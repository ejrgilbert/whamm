use std::fs;
use glob::{glob, Pattern};
use log::error;
use pest::iterators::Pair;
use pest::Parser;
use serde::{Deserialize, Serialize};
use termcolor::Buffer;
use wasmparser::TypeBounds;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{green, long_line, magenta_italics, white};
use crate::parser::rules::{matches_globs, print_provider_docs};
use crate::parser::types::{DataType, Expr, Location, ProbeRule, ProvidedGlobal, Rule, RulePart, WhammParser};
use crate::parser::whamm_parser::{handle_expr, handle_param, type_from_rule};

pub fn yml_to_providers(base_dir: &str) -> Vec<Provider> {
    let def = read_yml(base_dir);
    from_helper::<Provider, ProviderDef>(def.providers)
}

pub fn get_matches(rule: &ProbeRule, all_providers: &Vec<Provider>) -> Vec<Provider> {
    let mut matches: Vec<Provider> = vec![];
    for provider in all_providers.iter() {
        if let Some(prov) = provider.match_on(rule) {
            matches.push(*prov);
        }
    }

    if matches.is_empty() {
        todo!()
        // let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
        // return Err(Box::new(ErrorGen::get_parse_error(
        //     true,
        //     Some("Could not find any matches for the provider pattern".to_string()),
        //     loc,
        //     vec![],
        //     vec![],
        // )));
    }

    matches
}

// ===============================
// ==== TYPES FOR PROBE RULES ====
// ===============================

#[derive(Debug)]
pub struct Provider {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    docs: String,
    packages: Vec<Package>,
    type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    loc: Option<Location>
}
impl From<ProviderDef> for Provider {
    fn from(value: ProviderDef) -> Self {
        let bound_vars = from_helper::<BoundVar, BoundVarDef>(value.bound_vars);
        let bound_fns = from_helper::<BoundFunc, BoundFuncDef>(value.bound_fns);
        let packages = from_helper::<Package, PackageDef>(value.packages);
        Self {
            name: value.name.clone(),
            bound_vars,
            bound_fns,
            docs: value.docs.clone(),
            packages,
            type_bounds: vec![],
            loc: None
        }
    }
}
impl MatchOn for Provider {
    fn match_on(&self, probe_rule: &ProbeRule) -> Option<Box<Self>> {
        if let Some(RulePart {
            name: provider_patt,
            ty_info,
            loc,
            ..
        }) = &probe_rule.provider {
            let pkgs: Vec<Package> = match_helper(&self.name, &provider_patt, probe_rule, &self.packages)
                .into_iter().map(|b| *b).collect();
            return if !pkgs.is_empty() {
                Some(Box::new(Self {
                    name: self.name.clone(),
                    bound_vars: self.bound_vars.clone(),
                    bound_fns: self.bound_fns.clone(),
                    docs: self.docs.clone(),
                    packages: pkgs,
                    type_bounds: ty_info.clone(),
                    loc: loc.clone()
                }))
            } else {
                None
            }
        } else {
            todo!()
        }
    }
}
impl PrintInfo for Provider {
    fn print_info(&self, probe_rule: &ProbeRule, print_globals: bool, print_functions: bool, prov_buff: &mut Buffer, pkg_buff: &mut Buffer, evt_buffer: &mut Buffer,
                  tabs: &mut usize) {
        magenta_italics(true, self.name.clone(), prov_buff);
        white(true, " provider\n".to_string(), prov_buff);

        // Print the provider description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.docs),
            prov_buff,
        );
        print_bound_vars(&self.bound_vars, print_globals, prov_buff, tabs);
        print_bound_fns(&self.bound_fns, print_globals, prov_buff, tabs);
        *tabs -= 1;

        long_line(prov_buff);
        white(true, "\n\n".to_string(), prov_buff);

        if !self.packages.is_empty() {
            probe_rule.print_bold_package(pkg_buff);
            for pkg in self.packages.iter() {
                pkg.print_info(probe_rule, print_globals, print_functions, prov_buff, pkg_buff, evt_buffer, tabs);
            }
        }
    }
}

#[derive(Debug)]
pub struct Package {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    docs: String,
    events: Vec<Event>,
    type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    loc: Option<Location>
}
impl From<PackageDef> for Package {
    fn from(value: PackageDef) -> Self {
        let bound_vars = from_helper::<BoundVar, BoundVarDef>(value.bound_vars);
        let bound_fns = from_helper::<BoundFunc, BoundFuncDef>(value.bound_fns);
        let events = from_helper::<Event, EventDef>(value.events);
        Self {
            name: value.name.clone(),
            bound_vars,
            bound_fns,
            docs: value.docs.clone(),
            events,
            type_bounds: vec![],
            loc: None
        }
    }
}
impl MatchOn for Package {
    fn match_on(&self, probe_rule: &ProbeRule) -> Option<Box<Self>> {
        if let Some(RulePart {
            name: pkg_patt,
            ty_info,
            loc,
            ..
        }) = &probe_rule.package {
            let evts: Vec<Event> = match_helper(&self.name, &pkg_patt, probe_rule, &self.events)
                .into_iter().map(|b| *b).collect();
            return if !evts.is_empty() {
                Some(Box::new(Self {
                    name: self.name.clone(),
                    bound_vars: self.bound_vars.clone(),
                    bound_fns: self.bound_fns.clone(),
                    docs: self.docs.clone(),
                    events: evts,
                    type_bounds: ty_info.clone(),
                    loc: loc.clone()
                }))
            } else {
                None
            }
        } else {
            todo!()
        }
    }
}
impl PrintInfo for Package {
    fn print_info(&self, probe_rule: &ProbeRule, print_globals: bool, print_functions: bool, prov_buff: &mut Buffer, pkg_buff: &mut Buffer, evt_buffer: &mut Buffer,
                  tabs: &mut usize) {
        magenta_italics(true, self.name.clone(), pkg_buff);
        white(true, " package\n".to_string(), pkg_buff);

        // Print the package description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.docs),
            pkg_buff,
        );
        print_bound_vars(&self.bound_vars, print_globals, pkg_buff, tabs);
        print_bound_fns(&self.bound_fns, print_globals, pkg_buff, tabs);
        *tabs -= 1;

        long_line(pkg_buff);
        white(true, "\n\n".to_string(), pkg_buff);

        if !self.events.is_empty() {
            probe_rule.print_bold_event(evt_buffer);
            for evt in self.events.iter() {
                evt.print_info(probe_rule, print_globals, print_functions, prov_buff, pkg_buff, evt_buffer, tabs);
            }

            long_line(evt_buffer);
            white(true, "\n\n".to_string(), evt_buffer);
        }
    }
}

#[derive(Debug)]
pub struct Event {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    modes: Vec<Mode>,
    req_map: bool,      // TODO: Remove this...maybe make it request a list of libraries?
    docs: String,
    type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    loc: Option<Location>
}
impl From<EventDef> for Event {
    fn from(value: EventDef) -> Self {
        let bound_vars = from_helper::<BoundVar, BoundVarDef>(value.bound_vars);
        let bound_fns = from_helper::<BoundFunc, BoundFuncDef>(value.bound_fns);
        let modes = from_helper::<Mode, ModeDef>(value.supported_modes);
        Self {
            name: value.name.clone(),
            bound_vars,
            bound_fns,
            modes,
            req_map: value.req_map,
            docs: value.docs.clone(),
            type_bounds: vec![],
            loc: None
        }
    }
}
impl MatchOn for Event {
    fn match_on(&self, probe_rule: &ProbeRule) -> Option<Box<Self>> {
        if let Some(RulePart {
            name: evt_patt,
            ty_info,
            loc,
            ..
        }) = &probe_rule.event {
            let mds: Vec<Mode> = match_helper(&self.name, &evt_patt, probe_rule, &self.modes)
                .into_iter().map(|b| *b).collect();
            return if !mds.is_empty() {
                Some(Box::new(Self {
                    name: self.name.clone(),
                    bound_vars: self.bound_vars.clone(),
                    bound_fns: self.bound_fns.clone(),
                    docs: self.docs.clone(),
                    req_map: self.req_map,
                    modes: mds,
                    type_bounds: ty_info.clone(),
                    loc: loc.clone(),
                }))
            } else {
                None
            }
        } else {
            todo!()
        }
    }
}
impl PrintInfo for Event {
    fn print_info(&self, probe_rule: &ProbeRule, print_globals: bool, print_functions: bool, prov_buff: &mut Buffer, pkg_buff: &mut Buffer, evt_buffer: &mut Buffer,
                  tabs: &mut usize) {
        magenta_italics(true, self.name.clone(), evt_buffer);
        white(true, " event\n".to_string(), evt_buffer);

        // Print the event description
        *tabs += 1;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.docs),
            evt_buffer,
        );
        print_bound_vars(&self.bound_vars, print_globals, evt_buffer, tabs);
        print_bound_fns(&self.bound_fns, print_globals, evt_buffer, tabs);

        *tabs -= 1;

        if !self.modes.is_empty() {
            probe_rule.print_bold_mode(evt_buffer);
            for mode in self.modes.iter() {
                mode.print_info(probe_rule, print_globals, print_functions, prov_buff, pkg_buff, evt_buffer, tabs);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Mode {
    name: String,
    docs: String,
    type_bounds: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    loc: Option<Location>
}
impl From<ModeDef> for Mode {
    fn from(value: ModeDef) -> Self {
        Self {
            name: value.name.clone(),
            docs: value.docs.clone(),
            type_bounds: vec![],
            loc: None
        }
    }
}
impl MatchOn for Mode {
    fn match_on(&self, probe_rule: &ProbeRule) -> Option<Box<Self>> {
        if let Some(RulePart {
            name: md_patt,
            ty_info,
            loc,
            ..
        }) = &probe_rule.mode {
            if is_match(&self.name, &md_patt) {
                Some(Box::new(Self {
                    name: self.name.clone(),
                    docs: self.docs.clone(),
                    type_bounds: ty_info.clone(),
                    loc: loc.clone(),
                }))
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}
impl PrintInfo for Mode {
    fn print_info(&self, probe_rule: &ProbeRule, print_globals: bool, print_functions: bool, prov_buff: &mut Buffer, pkg_buff: &mut Buffer, evt_buffer: &mut Buffer, tabs: &mut usize) {
        magenta_italics(true, format!("    {}", self.name), evt_buffer);
        white(true, " mode\n".to_string(), evt_buffer);

        // Print the mode description
        *tabs += 2;
        white(
            false,
            format!("{}{}\n\n", " ".repeat(*tabs * 4), self.docs),
            evt_buffer,
        );
        // print_bound_vars(&self.bound_vars, print_globals, prov_buff, tabs);
        // print_bound_fns(&self.bound_fns, print_globals, prov_buff, tabs);

        *tabs -= 2;
    }
}

#[derive(Clone, Debug)]
pub struct BoundVar {
    name: String,
    docs: String,
    ty: DataType,
    derived_from: Option<Expr>
}
impl From<BoundVarDef> for BoundVar {
    fn from(value: BoundVarDef) -> Self {
        let ty = parse_helper::<DataType>("DataType", Rule::TYPE_YML, &value.ty, &type_from_rule);

        let derived_from = if let Some(derived_from) = value.derived_from {
            Some(parse_helper::<Expr>("Expr", Rule::expr, &derived_from, &handle_expr))
        } else {
            None
        };

        Self {
            name: value.name.to_owned(),
            docs: value.docs.to_owned(),
            ty,
            derived_from
        }
    }
}
impl BoundVar {
    fn print_info(&self, buff: &mut Buffer, tabs: &mut usize) {
        white(false, " ".repeat(*tabs * 4).to_string(), buff);
        green(true, self.name.to_string(), buff);
        white(true, ": ".to_string(), buff);
        self.ty.print(buff);

        *tabs += 1;
        white(
            false,
            format!("\n{}{}\n", " ".repeat(*tabs * 4), self.docs),
            buff,
        );
        *tabs -= 1;
    }
}

#[derive(Clone, Debug)]
pub struct BoundFunc {
    name: String,
    params: Vec<(Expr, DataType)>, // Expr::VarId -> DataType
    results: DataType,
    req_args: i32,      // TODO: Remove this...it's wasm opcode specific...
    docs: String
}
impl From<BoundFuncDef> for BoundFunc {
    fn from(value: BoundFuncDef) -> Self {
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
                err.fatal_report("YmlToProvider");
                params
            },
            Err(e) => {
                error!("Could not parse the token as function parameters: {}\n{:?}", e, value.params);
                panic!();
            }
        };
        let results = parse_helper::<DataType>("DataType", Rule::TYPE_YML, &value.results, &type_from_rule);

        Self {
            name: value.name.to_owned(),
            params,
            results,
            req_args: value.req_args,
            docs: value.docs.to_owned()
        }
    }
}
impl BoundFunc {
    fn print_info(&self, buffer: &mut Buffer, tabs: &mut usize) {
        green(true, " ".repeat(*tabs * 4).to_string(), buffer);

        green(true, self.name.to_string(), buffer);
        white(true, "(".to_string(), buffer);
        let mut is_first = true;
        for (param_name, param_ty) in self.params.iter() {
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
        self.results.print(buffer);

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
    fn print_info(&self, probe_rule: &ProbeRule, print_globals: bool, print_functions: bool, prov_buff: &mut Buffer, pkg_buff: &mut Buffer, evt_buffer: &mut Buffer,
                  tabs: &mut usize);
}
trait MatchOn {
    fn match_on(&self, probe_rule: &ProbeRule) -> Option<Box<Self>>;
}

// ===========================
// ==== UTILITY FUNCTIONS ====
// ===========================

fn print_bound_vars(vars: &Vec<BoundVar>, print_globals: bool, buff: &mut Buffer, tabs: &mut usize) {
    if print_globals && !vars.is_empty() {
        white(true, format!("{}GLOBALS:\n", " ".repeat(*tabs * 4)), buff);
        *tabs += 1;
        for var in vars.iter() {
            var.print_info(buff, tabs);
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buff);
    }
}

fn print_bound_fns(fns: &Vec<BoundFunc>, print_functions: bool, buff: &mut Buffer, tabs: &mut usize) {
    if print_functions && !fns.is_empty() {
        white(
            true,
            format!("{}FUNCTIONS:\n", " ".repeat(*tabs * 4)),
            buff,
        );
        *tabs += 1;
        for f in fns.iter() {
            f.print_info(buff, tabs);
        }
        *tabs -= 1;
        white(false, "\n".to_string(), buff);
    }
}

fn match_helper<T: MatchOn>(name: &str, pattern: &str, rule: &ProbeRule, to_check: &Vec<T>) -> Vec<Box<T>> {
    let mut matches = vec![];
    if is_match(name, pattern) {
        for item in to_check.iter() {
            if let Some(m) = item.match_on(rule) {
                matches.push(m);
            }
        }

        if matches.is_empty() {
            todo!()
            // let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
            // return Err(Box::new(ErrorGen::get_parse_error(
            //     true,
            //     Some("Could not find any matches for the provider pattern".to_string()),
            //     loc,
            //     vec![],
            //     vec![],
            // )));
        }
    }
    matches
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

fn parse_helper<T>(target: &str, parse_rule: Rule, token: &str, handler: &dyn Fn(Pair<Rule>) -> Result<T, Vec<WhammError>>) -> T {
    match WhammParser::parse(parse_rule, token) {
        Ok(mut pairs) => {
            if let Some(pair) = pairs.next() {
                let res = match handler(pair) {
                    Ok(res) => res,
                    Err(errs) => todo!()
                };
                res
            } else {
                todo!()
            }
        }
        Err(e) => {
            error!("Could not parse the token as a {target}: {token}\n{:?}", e);
            panic!();
        }
    }
}

fn from_helper<T: From<F>, F: Clone>(list: Vec<F>) -> Vec<T> {
    let mut new_list = vec![];
    for item in list.iter() {
        new_list.push(T::from(item.clone()));
    }
    new_list
}

// =====================
// ==== IR FOR YAML ====
// =====================

fn read_yml(base_dir_tmp: &str) -> YmlDefinition {
    let base_dir = base_dir_tmp.trim_end_matches("/");

    let mut yml_files = vec![];

    // push events first (sets up the anchors)
    println!("{base_dir}/providers/packages/events/*.yaml");
    for path in glob(&format!("{base_dir}/providers/packages/events/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    // push packages next (sets up the anchors)
    for path in glob(&format!("{base_dir}/providers/packages/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    // finally the providers
    for path in glob(&format!("{base_dir}/providers/*.yaml")).expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        println!("hi");
        println!("{unparsed_file}");
        yml_files.push(unparsed_file);
    }

    let mut all_yml = "".to_string();
    for yml in yml_files.iter() {
        all_yml += yml;
    }

    println!("{all_yml}");

    let def: YmlDefinition =
        serde_yml::from_str(&all_yml).expect("Could not read values.");
    println!("{:?}", def);

    def
}

#[derive(Debug, Serialize, Deserialize)]
struct YmlDefinition {
    providers: Vec<ProviderDef>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProviderDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    docs: String,
    packages: Vec<PackageDef>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PackageDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    docs: String,
    events: Vec<EventDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EventDef {
    name: String,
    bound_vars: Vec<BoundVarDef>,
    bound_fns: Vec<BoundFuncDef>,
    supported_modes: Vec<ModeDef>,
    req_map: bool,      // TODO: Remove this...maybe make it request a list of libraries?
    docs: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoundVarDef {
    name: String,
    docs: String,
    #[serde(rename = "type")]
    ty: String,
    derived_from: Option<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BoundFuncDef {
    name: String,
    params: String,
    results: String,
    req_args: i32,      // TODO: Remove this...it's wasm opcode specific...
    docs: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ModeDef {
    name: String,
    docs: String
}