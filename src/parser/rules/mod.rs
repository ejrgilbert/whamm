pub mod core;
pub mod wasm;
use std::collections::HashMap;
use glob::Pattern;
use termcolor::Buffer;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{magenta_italics, white};
use crate::parser::rules::wasm::{WasmPackage};
use crate::parser::rules::core::{CorePackage};
use crate::parser::types::{Expr, Location, print_fns, print_global_vars, ProbeSpec, ProvidedFunction, ProvidedGlobal, SpecPart, Statement};


pub trait NameOptions {
    fn get_name_options() -> Vec<String>;
}
pub trait FromStr {
    fn from_str(name: String, loc: Option<Location>) -> Self;
}

// ==================
// ---- Provider ----
// ==================

pub trait Provider {
    fn name(&self) -> String;
    fn docs(&self) -> &String;
    fn print_package_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn print_event_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_packages(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<(bool, bool, bool), Box<WhammError>>;
}

/// 0: Box<Self> the matched provider instance
/// 1: bool, whether there were matched packages
/// 2: bool, whether there were matched events
/// 3: bool, whether there were matched modes
pub fn provider_factory<P: Provider + NameOptions + FromStr>(probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<(Vec<Box<P>>, bool, bool, bool), Box<WhammError>> {
    if let Some(SpecPart {name: provider_patt, loc: provider_loc}) = &probe_spec.provider {
        let matches = get_matches(P::get_name_options(), provider_patt);
        if matches.is_empty() {
            let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the provider pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        let mut matched_packages = false;
        let mut matched_events = false;
        let mut matched_modes = false;
        let mut providers = vec![];
        for m in matches {
            let mut provider = P::from_str(m, loc.clone());
            let (found_package, found_events, found_modes) = if let Some(SpecPart {loc: package_loc, .. }) = &probe_spec.package {
                provider.assign_matching_packages(probe_spec, package_loc.to_owned())?
            } else {
                (false, false, false)
            };
            matched_packages |= found_package;
            matched_events |= found_events;
            matched_modes |= found_modes;
            providers.push(Box::new(provider));
        }

        Ok((providers, matched_packages, matched_events, matched_modes))
    } else {
        Ok((vec![], false, false, false))
    }
}

pub fn print_provider_docs<P: Provider>(provider: &P, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
    let name = provider.name();
    let docs = provider.docs();

    if name.is_empty() {
        return;
    }
    magenta_italics(true, name.clone(), buffer);
    white(true, " provider\n".to_string(), buffer);

    // Print the provider description
    *tabs += 1;
    white(
        false,
        format!("{}{}\n\n", " ".repeat(*tabs * 4), docs),
        buffer,
    );

    // Print the globals
    if print_globals {
        let globals = provider.get_provided_globals();
        print_global_vars(tabs, &globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = provider.get_provided_fns();
        print_fns(tabs, &functions, buffer);
    }
    *tabs -= 1;
}

// =================
// ---- Package ----
// =================

pub trait Package {
    fn name(&self) -> String;
    fn docs(&self) -> &String;
    fn print_event_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_events(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<(bool, bool), Box<WhammError>>;
}
/// 0: Box<Self> the matched package instance
/// 2: bool, whether there were matched events
/// 3: bool, whether there were matched modes
fn package_factory<P: Package + NameOptions + FromStr>(probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<Vec<(Box<P>, bool, bool)>, Box<WhammError>> {
    if let Some(SpecPart {name: package_patt, loc: package_loc}) = &probe_spec.package {
        let matches = get_matches(P::get_name_options(), package_patt);
        if matches.is_empty() {
            let loc = package_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the package pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        let mut packages = vec![];
        for m in matches {
            let mut package = P::from_str(m, loc.clone());
            let (matched_events, matched_modes) = if let Some(SpecPart {loc: event_loc, .. }) = &probe_spec.event {
                package.assign_matching_events(probe_spec, event_loc.to_owned())?
            } else {
                (false, false)
            };
            packages.push((Box::new(package), matched_events, matched_modes));
        }

        Ok(packages)
    } else {
        Ok(vec![])
    }
}
fn print_package_docs<P>(package: &P, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer)
        where P: Package {
    let name = package.name();
    let docs = package.docs();

    if name.is_empty() {
        return;
    }
    magenta_italics(true, name.clone(), buffer);
    white(true, " package\n".to_string(), buffer);

    // Print the package description
    *tabs += 1;
    white(
        false,
        format!("{}{}\n\n", " ".repeat(*tabs * 4), docs),
        buffer,
    );

    // Print the globals
    if print_globals {
        let globals = package.get_provided_globals();
        print_global_vars(tabs, &globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = package.get_provided_fns();
        print_fns(tabs, &functions, buffer);
    }
    *tabs -= 1;
}

// ===============
// ---- Event ----
// ===============

pub trait Probe {
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
}
pub trait Event {
    fn name(&self) -> &String;
    fn docs(&self) -> &String;
    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>>;
    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>>;
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer);
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_modes(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<bool, Box<WhammError>>;
}

/// 0: Box<Self> the matched event instance
/// 3: bool, whether there were matched modes
fn event_factory<E: Event + NameOptions + FromStr>(probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<Vec<(Box<E>, bool)>, Box<WhammError>> {
    if let Some(SpecPart {name: event_patt, loc: event_loc}) = &probe_spec.event {
        let matches = get_matches(E::get_name_options(), event_patt);
        if matches.is_empty() {
            let loc = event_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the event pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        let mut events = vec![];
        for m in matches {
            let mut event = E::from_str(m, loc.clone());
            let matched_modes = if let Some(SpecPart {loc: mode_loc, .. }) = &probe_spec.mode {
                event.assign_matching_modes(probe_spec, mode_loc.to_owned())?
            } else {
                false
            };
            events.push((Box::new(event), matched_modes));
        }

        Ok(events)
    } else {
        Ok(vec![])
    }
}
fn print_event_docs<E: Event>(event: &E, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
    let name = event.name();
    let docs = event.docs();

    if name.is_empty() {
        return;
    }
    magenta_italics(true, name.clone(), buffer);
    white(true, " event\n".to_string(), buffer);

    // Print the event description
    *tabs += 1;
    white(
        false,
        format!("{}{}\n\n", " ".repeat(*tabs * 4), docs),
        buffer,
    );

    // Print the globals
    if print_globals {
        let globals = event.get_provided_globals();
        print_global_vars(tabs, &globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = event.get_provided_fns();
        print_fns(tabs, &functions, buffer);
    }
    *tabs -= 1;
}

// ==============
// ---- Mode ----
// ==============

pub trait Mode {
    fn name(&self) -> String;
    fn docs(&self) -> &String;
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
}

/// 0: Box<Self> the matched provider instance
fn mode_factory<M: Mode + NameOptions + FromStr>(probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<Vec<Box<M>>, Box<WhammError>> {
    if let Some(SpecPart {name: mode_patt, loc: mode_loc}) = &probe_spec.mode {
        let matches = get_matches(M::get_name_options(), mode_patt);
        if matches.is_empty() {
            let loc = mode_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the mode pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        let mut modes = vec![];
        for m in matches {
            let mode = M::from_str(m, loc.clone());
            modes.push(Box::new(mode));
        }

        Ok(modes)
    } else {
        Ok(vec![])
    }
}
fn print_mode_docs<M: Mode>(mode: &M, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
    let name = mode.name();
    let docs = mode.docs();

    if name.is_empty() {
        return;
    }
    magenta_italics(true, name.clone(), buffer);
    white(true, " mode\n".to_string(), buffer);

    // Print the mode description
    *tabs += 1;
    white(
        false,
        format!("{}{}\n\n", " ".repeat(*tabs * 4), docs),
        buffer,
    );

    // Print the globals
    if print_globals {
        let globals = mode.get_provided_globals();
        print_global_vars(tabs, &globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = mode.get_provided_fns();
        print_fns(tabs, &functions, buffer);
    }
    *tabs -= 1;
}

// ===================================
// ---- Base Provider Definitions ----
// ===================================

/// The base information needed for `WhammProvider`s, pulled out into a single struct.
pub struct ProviderInfo {
    // Statically defined, always the same
    pub docs: String,
    pub fns: Vec<ProvidedFunction>, // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>
}

/// The base providers provided by `whamm!`.
/// Custom providers can be created by following the conventions shown in this pattern.
/// TODO -- unsure how to enable custom providers, but trying to set up to ease supporting
///         this in the future. Now, the use of `WhammProvider` is hardcoded everywhere.
pub enum WhammProvider {
    Wasm {
        metadata: ProviderInfo,
        /// The packages of the probes that have been used in the Script.
        packages: HashMap<String, Box<WasmPackage>>,
    },
    Core {
        metadata: ProviderInfo,
        /// The packages of the probes that have been used in the Script.
        packages: HashMap<String, Box<CorePackage>>,
    }
}
impl NameOptions for WhammProvider {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "core".to_string(),
            "wasm".to_string()
        ]
    }
}
impl FromStr for WhammProvider {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "core" => Self::core(loc),
            "wasm" => Self::wasm(loc),
            _ => panic!("unsupported WhammProvider: {name}")
        }
    }
}
impl WhammProvider {
    fn core(loc: Option<Location>) -> Self {
        Self::Core {
            metadata: ProviderInfo {
                docs: "Provides the core probe definitions of `whamm`.".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc
            },
            packages: HashMap::new()
        }
    }
    fn wasm(loc: Option<Location>) -> Self {
        Self::Wasm {
            metadata: ProviderInfo {
                docs: "This provides various events to instrument that are specific to WebAssembly.".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc
            },
            packages: HashMap::new()
        }
    }
}
impl Provider for WhammProvider {
    fn name(&self) -> String {
        match self {
            Self::Core{..} => {
                "core".to_string()
            },
            Self::Wasm{..} => {
                "wasm".to_string()
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Core{metadata: ProviderInfo { docs, ..}, ..} |
            Self::Wasm{metadata: ProviderInfo { docs, ..}, ..} => {
                docs
            }
        }
    }

    fn print_package_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Core{packages, ..} => {
                for (.., package) in packages.iter() {
                    print_package_docs(package.as_ref(), print_globals, print_functions, tabs, buffer);
                }
            }
            Self::Wasm{packages, ..} => {
                for (.., package) in packages.iter() {
                    print_package_docs(package.as_ref(), print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn print_event_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Core{packages, ..} => {
                for (.., package) in packages.iter() {
                    package.print_event_docs(print_globals, print_functions, tabs, buffer);
                }
            }
            Self::Wasm{packages, ..} => {
                for (.., package) in packages.iter() {
                    package.print_event_docs(print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Core{packages, ..} => {
                for (.., package) in packages.iter() {
                    package.print_mode_docs(print_globals, print_functions, tabs, buffer);
                }
            }
            Self::Wasm{packages, ..} => {
                for (.., package) in packages.iter() {
                    package.print_mode_docs(print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Wasm{metadata: ProviderInfo {fns, ..}, ..} |
            Self::Core{metadata: ProviderInfo {fns, ..}, ..} => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Wasm{metadata: ProviderInfo {globals, ..}, ..} |
            Self::Core{metadata: ProviderInfo {globals, ..}, ..} => {
                globals
            }
        }
    }

    fn assign_matching_packages(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<(bool, bool, bool), Box<WhammError>> {
        let mut matched_packages = false;
        let mut matched_events = false;
        let mut matched_modes = false;
        match self {
            Self::Core {packages, ..} => {
                let matched: Vec<(Box<CorePackage>, bool, bool)> = package_factory(probe_spec, loc)?;
                for (package, found_match_for_event, found_match_for_mode) in matched {
                    matched_packages = true;
                    matched_events |= found_match_for_event;
                    matched_modes |= found_match_for_mode;
                    packages.insert(package.name(), package);
                }
            },
            Self::Wasm {packages, ..} => {
                let matched: Vec<(Box<WasmPackage>, bool, bool)> = package_factory(probe_spec, loc)?;
                for (package, found_match_for_event, found_match_for_mode) in matched {
                    matched_packages = true;
                    matched_events |= found_match_for_event;
                    matched_modes |= found_match_for_mode;
                    packages.insert(package.name(), package);
                }
            }
        }
        Ok((matched_packages, matched_events, matched_modes))
    }
}

/// The base information needed for `WhammMode`s, pulled out into a single struct.
pub struct ModeInfo {
    // Statically defined, always the same
    pub docs: String,
    pub fns: Vec<ProvidedFunction>, // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>
}

/// The base modes provided by `whamm!` for an Event, these can be changed if desired.
/// To do so, the type of enum for a Probe's possible modes will need to be changed.
/// This means the Event's probes HashMap will need to point to a custom Probe type.
pub enum WhammMode {
    Before (ModeInfo),
    After (ModeInfo),
    Alt (ModeInfo)
}
impl NameOptions for WhammMode {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "before".to_string(),
            "after".to_string(),
            "alt".to_string()
        ]
    }
}
impl FromStr for WhammMode {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "before" => Self::before(loc),
            "after" => Self::after(loc),
            "alt" => Self::alt(loc),
            _ => panic!("unsupported WhammMode: {name}")
        }
    }
}
impl WhammMode {

    // ======================
    // ---- Constructors ----
    // ======================
    
    fn before(loc: Option<Location>) -> Self {
        Self::Before ( ModeInfo {
            docs: "This mode will cause the instrumentation logic to run *before* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            loc
        })
    }
    fn after(loc: Option<Location>) -> Self {
        Self::After ( ModeInfo {
            docs: "This mode will cause the instrumentation logic to run *after* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            loc
        })
    }
    fn alt(loc: Option<Location>) -> Self {
        Self::Alt ( ModeInfo {
            docs: "This mode will cause the instrumentation logic to run *instead of* the \
                    probed event (if the predicate evaluates to `true`).".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            loc
        })
    }
}
impl Mode for WhammMode {
    fn name(&self) -> String {
        match self {
            Self::Before(..) => {
                "before".to_string()
            }
            Self::After(..) => {
                "after".to_string()
            }
            Self::Alt(..) => {
                "alt".to_string()
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Before(ModeInfo { docs, ..}) |
            Self::After(ModeInfo { docs, ..}) |
            Self::Alt(ModeInfo { docs, ..}) => {
                docs
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Before(ModeInfo { fns, ..}) |
            Self::After(ModeInfo { fns, ..}) |
            Self::Alt(ModeInfo { fns, ..}) => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Before(ModeInfo { globals, ..}) |
            Self::After(ModeInfo { globals, ..}) |
            Self::Alt(ModeInfo { globals, ..}) => {
                globals
            }
        }
    }
}

/// The base definition of a probe for `whamm!`.
/// This can be customized if desired.
pub struct WhammProbe {
    pub mode: WhammMode,
    pub loc: Option<Location>,

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>
}
impl Probe for WhammProbe {
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        print_mode_docs(&self.mode, print_globals, print_functions, tabs, buffer);
    }
}
impl WhammProbe {
    fn new(mode: WhammMode, loc: Option<Location>, predicate: Option<Expr>, body: Option<Vec<Statement>>) -> Self {
        Self {
            mode,
            loc,
            predicate,
            body
        }
    }
}

// ===================================
// Helper functions for glob matching.
// ===================================

pub fn get_matches(opts: Vec<String>, patt: &str) -> Vec<String> {
    let globs = get_globs(&patt.to_lowercase());

    let mut matches = vec![];
    for name in opts.iter() {
        if matches_globs(&name.to_lowercase(), &globs) {
            matches.push(name.to_owned());
        }
    }

    matches
}

pub fn get_globs(patt: &str) -> Vec<Pattern> {
    let mut globs = vec![];
    for p in patt.split('|') {
        globs.push(Pattern::new(p).unwrap());
    }

    globs
}

pub fn matches_globs(s: &str, globs: &[Pattern]) -> bool {
    for glob in globs.iter() {
        if glob.matches(s) {
            return true;
        }
    }
    false
}