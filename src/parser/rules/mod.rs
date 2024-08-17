#![allow(clippy::borrowed_box)]
pub mod core;
pub mod wasm;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{magenta_italics, white};
use crate::parser::rules::core::{CorePackage, WhammMode, WhammModeKind, WhammProbe};
use crate::parser::rules::wasm::WasmPackage;
use crate::parser::types::{
    print_fns, print_global_vars, Block, DataType, Expr, Location, ProbeSpec, ProvidedFunction,
    ProvidedGlobal, SpecPart,
};
use glob::Pattern;
use std::collections::HashMap;
use termcolor::Buffer;

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
    fn has_packages(&self) -> bool;
    fn len_packages(&self) -> usize;
    fn packages(&self) -> Box<dyn Iterator<Item = &dyn Package> + '_>;
    fn packages_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn Package> + '_>;
    fn print_package_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn print_event_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_packages(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
        printing_info: bool,
    ) -> (bool, bool, bool);
}

/// 0: Box<Self> the matched provider instance
/// 1: bool, whether there were matched packages
/// 2: bool, whether there were matched events
/// 3: bool, whether there were matched modes
pub fn provider_factory<P: Provider + NameOptions + FromStr + 'static>(
    curr_providers: &mut HashMap<String, Box<dyn Provider>>,
    probe_spec: &ProbeSpec,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> Result<(bool, bool, bool, bool), Box<WhammError>> {
    if let Some(SpecPart {
        name: provider_patt,
        loc: provider_loc,
    }) = &probe_spec.provider
    {
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

        let mut matched_providers = false;
        let mut matched_packages = false;
        let mut matched_events = false;
        let mut matched_modes = false;
        for m in matches {
            matched_providers = true;
            let already_has = curr_providers.contains_key(&m.clone());
            let provider = curr_providers
                .entry(m.clone())
                .or_insert(Box::new(P::from_str(m.clone(), loc.clone())));

            let (found_package, found_events, found_modes) = if let Some(SpecPart {
                loc: package_loc,
                ..
            }) = &probe_spec.package
            {
                provider.assign_matching_packages(
                    probe_spec,
                    package_loc.to_owned(),
                    predicate.clone(),
                    body.clone(),
                    printing_info,
                )
            } else {
                (false, false, false)
            };
            if !printing_info && !found_modes && !already_has {
                // If this matched provider wasn't already present, we need to remove.
                // Otherwise, we'd have providers with no probes in them!
                // ONLY DO THIS IF NOT PRINTING INFO, this allows users to get information without
                // complete probe specs.
                curr_providers.remove(&m.clone());
            }
            matched_packages |= found_package;
            matched_events |= found_events;
            matched_modes |= found_modes;
        }
        if !matched_providers && probe_spec.provider.is_some() {
            let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the provider pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        if !matched_packages && probe_spec.package.is_some() {
            let loc = probe_spec
                .package
                .as_ref()
                .unwrap()
                .loc
                .as_ref()
                .map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the package pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        if !matched_events && probe_spec.event.is_some() {
            let loc = probe_spec
                .event
                .as_ref()
                .unwrap()
                .loc
                .as_ref()
                .map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the event pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        if !matched_modes && probe_spec.mode.is_some() {
            let loc = probe_spec
                .mode
                .as_ref()
                .unwrap()
                .loc
                .as_ref()
                .map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the mode pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }

        Ok((
            matched_providers,
            matched_packages,
            matched_events,
            matched_modes,
        ))
    } else {
        Ok((false, false, false, false))
    }
}

pub fn print_provider_docs(
    provider: &Box<dyn Provider>,
    print_globals: bool,
    print_functions: bool,
    tabs: &mut usize,
    buffer: &mut Buffer,
) {
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
        print_global_vars(tabs, globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = provider.get_provided_fns();
        print_fns(tabs, functions, buffer);
    }
    *tabs -= 1;
}

// =================
// ---- Package ----
// =================

pub trait Package {
    fn name(&self) -> String;
    fn loc(&self) -> &Option<Location>;
    fn docs(&self) -> &String;
    fn has_events(&self) -> bool;
    fn len_events(&self) -> usize;
    fn events(&self) -> Box<dyn Iterator<Item = &dyn Event> + '_>;
    fn events_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn Event> + '_>;
    fn print_event_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_events(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
        printing_info: bool,
    ) -> (bool, bool);
}

/// The base information needed for `Package`s, pulled out into a single struct.
pub struct PackageInfo {
    // Statically defined, always the same
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>,
    /// The events of the probes that have been used in the Script.
    pub events: HashMap<String, Box<dyn Event>>,
}

/// 0: Box<Self> the matched package instance
/// 2: bool, whether there were matched events
/// 3: bool, whether there were matched modes
fn package_factory<P: Package + NameOptions + FromStr + 'static>(
    curr_packages: &mut HashMap<String, Box<dyn Package>>,
    probe_spec: &ProbeSpec,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> (bool, bool, bool) {
    if let Some(SpecPart {
        name: package_patt, ..
    }) = &probe_spec.package
    {
        let matches = get_matches(P::get_name_options(), package_patt);
        if matches.is_empty() {
            return (false, false, false);
        }

        let mut matched_packages = false;
        let mut matched_events = false;
        let mut matched_modes = false;
        for m in matches {
            matched_packages = true;
            let already_has = curr_packages.contains_key(&m.clone());
            let package = curr_packages
                .entry(m.clone())
                .or_insert(Box::new(P::from_str(m.clone(), loc.clone())));

            let (found_match_for_event, found_match_for_mode) =
                if let Some(SpecPart { loc: event_loc, .. }) = &probe_spec.event {
                    package.assign_matching_events(
                        probe_spec,
                        event_loc.to_owned(),
                        predicate.clone(),
                        body.clone(),
                        printing_info,
                    )
                } else {
                    (false, false)
                };
            if !printing_info && !found_match_for_mode && !already_has {
                // If this matched package wasn't already present, we need to remove.
                // Otherwise, we'd have packages with no probes in them!
                // ONLY DO THIS IF NOT PRINTING INFO, this allows users to get information without
                // complete probe specs.
                curr_packages.remove(&m.clone());
            }
            matched_events |= found_match_for_event;
            matched_modes |= found_match_for_mode;
        }

        (matched_packages, matched_events, matched_modes)
    } else {
        (false, false, false)
    }
}
fn print_package_docs(
    package: &Box<dyn Package>,
    print_globals: bool,
    print_functions: bool,
    tabs: &mut usize,
    buffer: &mut Buffer,
) {
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
        print_global_vars(tabs, package.get_provided_globals(), buffer);
    }

    // Print the functions
    if print_functions {
        print_fns(tabs, package.get_provided_fns(), buffer);
    }
    *tabs -= 1;
}

// ===============
// ---- Event ----
// ===============

pub trait Probe {
    fn mode_name(&self) -> String;
    fn predicate(&self) -> &Option<Expr>;
    fn predicate_mut(&mut self) -> &mut Option<Expr>;
    fn body(&self) -> &Option<Block>;
    fn body_mut(&mut self) -> &mut Option<Block>;
    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn get_mode_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_mode_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction>;
    fn get_mode_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
}
pub trait Event {
    fn name(&self) -> String;
    fn loc(&self) -> &Option<Location>;
    fn supported_modes(&self) -> &Vec<WhammModeKind>;
    fn docs(&self) -> &String;
    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>>;
    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>>;
    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    );
    fn get_provided_fns(&self) -> &Vec<ProvidedFunction>;
    fn get_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
    fn assign_matching_modes(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> bool {
        let mut matched_modes = false;
        let modes: Vec<Box<WhammMode>> = mode_factory(&self.supported_modes(), probe_spec, loc.clone());
        let probes = self.probes_mut();
        for mode in modes {
            matched_modes = true;
            let modes = probes.entry(mode.name()).or_default();
            modes.push(Box::new(WhammProbe::new(
                *mode,
                loc.clone(),
                predicate.clone(),
                body.clone(),
            )));
        }
        matched_modes
    }
}

/// The base information needed for `Event`s, pulled out into a single struct.
pub struct EventInfo {
    // Statically defined, always the same
    pub supported_modes: Vec<WhammModeKind>,
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>,
    pub probe_map: HashMap<String, Vec<Box<dyn Probe>>>,
}

/// 0: Box<Self> the matched event instance
/// 3: bool, whether there were matched modes
fn event_factory<E: Event + NameOptions + FromStr + 'static>(
    curr_events: &mut HashMap<String, Box<dyn Event>>,
    probe_spec: &ProbeSpec,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> (bool, bool) {
    if let Some(SpecPart {
        name: event_patt, ..
    }) = &probe_spec.event
    {
        let matches = get_matches(E::get_name_options(), event_patt);
        if matches.is_empty() {
            return (false, false);
        }

        let mut matched_events = false;
        let mut matched_modes = false;
        for m in matches {
            matched_events = true;
            let already_has = curr_events.contains_key(&m.clone());
            let event = curr_events
                .entry(m.clone())
                .or_insert(Box::new(E::from_str(m.clone(), loc.clone())));

            let found_match_for_mode =
                if let Some(SpecPart { loc: mode_loc, .. }) = &probe_spec.mode {
                    event.assign_matching_modes(
                        probe_spec,
                        mode_loc.to_owned(),
                        predicate.clone(),
                        body.clone(),
                    )
                } else {
                    false
                };
            if !printing_info && !found_match_for_mode && !already_has {
                // If this matched package wasn't already present, we need to remove.
                // Otherwise, we'd have packages with no probes in them!
                // ONLY DO THIS IF NOT PRINTING INFO, this allows users to get information without
                // complete probe specs.
                curr_events.remove(&m.clone());
            }
            matched_modes |= found_match_for_mode;
        }

        (matched_events, matched_modes)
    } else {
        (false, false)
    }
}
fn print_event_docs(
    event: &Box<dyn Event>,
    print_globals: bool,
    print_functions: bool,
    tabs: &mut usize,
    buffer: &mut Buffer,
) {
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
        print_global_vars(tabs, globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = event.get_provided_fns();
        print_fns(tabs, functions, buffer);
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
    fn get_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction>;
    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal>;
}

/// 0: Box<Self> the matched provider instance
fn mode_factory<M: Mode + NameOptions + FromStr>(
    supported_modes: &[WhammModeKind],
    probe_spec: &ProbeSpec,
    loc: Option<Location>,
) -> Vec<Box<M>> {
    if let Some(SpecPart {
        name: mode_patt, ..
    }) = &probe_spec.mode
    {
        let mut name_options = vec![];
        for mode in supported_modes {
            name_options.push(mode.name());
        }
        
        let matches = get_matches(name_options, mode_patt);
        if matches.is_empty() {
            return vec![];
        }

        let mut modes = vec![];
        for m in matches {
            let mode = M::from_str(m, loc.clone());
            modes.push(Box::new(mode));
        }

        modes
    } else {
        vec![]
    }
}
fn print_mode_docs<M: Mode>(
    mode: &M,
    print_globals: bool,
    print_functions: bool,
    tabs: &mut usize,
    buffer: &mut Buffer,
) {
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
        print_global_vars(tabs, globals, buffer);
    }

    // Print the functions
    if print_functions {
        let functions = mode.get_provided_fns();
        print_fns(tabs, functions, buffer);
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
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>,
    /// The packages of the probes that have been used in the Script.
    pub packages: HashMap<String, Box<dyn Package>>,
}

pub enum WhammProviderKind {
    Core,
    Wasm,
}
impl WhammProviderKind {
    fn name(&self) -> String {
        match self {
            Self::Core => "core".to_string(),
            Self::Wasm => "wasm".to_string(),
        }
    }
}

/// The base providers provided by `whamm!`.
/// Custom providers can be created by following the conventions shown in this pattern.
/// TODO -- unsure how to enable custom providers, but trying to set up to ease supporting
///         this in the future. Now, the use of `WhammProvider` is hardcoded everywhere.

pub struct WhammProvider {
    pub(crate) kind: WhammProviderKind,
    info: ProviderInfo,
}
impl NameOptions for WhammProvider {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec!["core".to_string(), "wasm".to_string()]
    }
}
impl FromStr for WhammProvider {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "core" => Self::core(loc),
            "wasm" => Self::wasm(loc),
            _ => panic!("unsupported WhammProvider: {name}"),
        }
    }
}
impl WhammProvider {
    fn core(loc: Option<Location>) -> Self {
        Self {
            kind: WhammProviderKind::Core,
            info: ProviderInfo {
                docs: "Provides the core probe definitions of `whamm`.".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                packages: HashMap::new(),
            },
        }
    }
    fn wasm(loc: Option<Location>) -> Self {
        Self {
            kind: WhammProviderKind::Wasm,
            info: ProviderInfo {
                docs:
                    "This provides various events to instrument that are specific to WebAssembly."
                        .to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                packages: HashMap::new(),
            },
        }
    }
}
impl Provider for WhammProvider {
    fn name(&self) -> String {
        self.kind.name()
    }

    fn docs(&self) -> &String {
        &self.info.docs
    }

    fn has_packages(&self) -> bool {
        !self.info.packages.is_empty()
    }

    fn len_packages(&self) -> usize {
        self.info.packages.len()
    }

    fn packages(&self) -> Box<dyn Iterator<Item = &dyn Package> + '_> {
        Box::new(
            self.info
                .packages
                .values()
                .map(|p| p.as_ref() as &dyn Package),
        )
    }

    fn packages_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn Package> + '_> {
        Box::new(
            self.info
                .packages
                .values_mut()
                .map(|p| p.as_mut() as &mut dyn Package),
        )
    }

    fn print_package_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., package) in self.info.packages.iter() {
            print_package_docs(package, print_globals, print_functions, tabs, buffer);
        }
    }

    fn print_event_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., package) in self.info.packages.iter() {
            package.print_event_docs(print_globals, print_functions, tabs, buffer);
        }
    }

    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., package) in self.info.packages.iter() {
            package.print_mode_docs(print_globals, print_functions, tabs, buffer);
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        &self.info.fns
    }

    fn get_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction> {
        &mut self.info.fns
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        &self.info.globals
    }

    fn assign_matching_packages(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
        printing_info: bool,
    ) -> (bool, bool, bool) {
        match self {
            Self {
                kind: WhammProviderKind::Core,
                ..
            } => package_factory::<CorePackage>(
                &mut self.info.packages,
                probe_spec,
                loc,
                predicate,
                body,
                printing_info,
            ),
            Self {
                kind: WhammProviderKind::Wasm,
                ..
            } => package_factory::<WasmPackage>(
                &mut self.info.packages,
                probe_spec,
                loc,
                predicate,
                body,
                printing_info,
            ),
        }
    }
}

/// The base information needed for `WhammMode`s, pulled out into a single struct.
pub struct ModeInfo {
    // Statically defined, always the same
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>,
}

/// Expected inputs:
/// IdentifierName, common_name, docs: &str
#[macro_export]
macro_rules! for_each_mode {
($mac:ident) => { $mac! {
    Before, before, "This mode will cause the instrumentation logic to run *before* the \
                    probed event (if the predicate evaluates to `true`)."
    After, after, "This mode will cause the instrumentation logic to run *after* the \
                    probed event (if the predicate evaluates to `true`)."
    Alt, alt, "This mode will cause the instrumentation logic to run *instead of* the \
                    probed event (if the predicate evaluates to `true`)."
    
    // special modes
    SemanticAfter, semantic_after, "This mode will cause the instrumentation logic to run *semantically after*  the instrumented location, meaning it will find the point in the bytecode that will be executed *after* the point is reached (consider blocks and br* opcodes)."
    Entry, entry, "This mode will cause the instrumentation logic to run *on entry* to the instrumentation point (e.g. functions bodies, blocks, etc.)."
    Exit, exit, "This mode will cause the instrumentation logic to run *on exiting* to the instrumentation point (e.g. function bodies, blocks, etc.)."
    
    // core modes
    Begin, begin, "Run this logic on application startup."
    End, end, "Run this logic when the application exits."
}};}

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

/// Only specify the number of args since the arg type
/// isn't necessarily consistent based on just which opcode
/// we're at.
/// (Sometimes a specific opcode's arg0 is i32, sometimes it's not)
/// Expected inputs:
/// IdentifierName, common_name, num_args: i32, imms: Vec<DataType>, globals: HashMap<String, ProvidedGlobal>, fns: Vec<ProvidedFunction>, supported_modes: Vec<WhammModeKind>, docs: &str
#[macro_export]
macro_rules! for_each_opcode {
($mac:ident) => { $mac! {
    Unreachable, unreachable, 0, vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable"
    Nop, nop, 0, vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), "https://www.w3.org/TR/wasm-core-2/#syntax-instr-control"
    // TODO -- support blockty as a struct to read/manipulate (provided global?)
    //         Block { blockty: $crate::BlockType } => visit_block
    //         Loop { blockty: $crate::BlockType } => visit_loop
    //         If { blockty: $crate::BlockType } => visit_if
    Block, block, 0, vec![], HashMap::new(), vec![], WhammModeKind::block_type_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block"
    Loop, _loop, 0, vec![], HashMap::new(), vec![], WhammModeKind::block_type_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop"
    If, _if, 1, vec![], HashMap::new(), vec![], WhammModeKind::block_type_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else"
    Else, _else, 0, vec![], HashMap::new(), vec![], WhammModeKind::block_type_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else"
    // TryTable { try_table: $crate::TryTable } => visit_try_table
    // Throw { tag_index: u32 } => visit_throw
    // ThrowRef => visit_throw_ref
    // // Deprecated old instructions from the exceptions proposal
    // Try { blockty: $crate::BlockType } => visit_try
    // Catch { tag_index: u32 } => visit_catch
    // Rethrow { relative_depth: u32 } => visit_rethrow
    // Delegate { relative_depth: u32 } => visit_delegate
    // CatchAll => visit_catch_all
    End, end, 0, vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/end"
    // TODO
    Br, br, 0, vec![DataType::U32], HashMap::new(), vec![], WhammModeKind::default_modes_and_semantic_aft(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
    // // BrIf { relative_depth: u32 } => visit_br_if TODO
    // BrIf, br_if, 1, vec![DataType::U32], HashMap::new(), vec![], default_modes().push(WhammModeKind::SemanticAfter), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
    // // BrTable { targets: $crate::BrTable<'a> } => visit_br_table TODO
    // // can be any number of immediates! Just assume we have the immN used and check later while traversing the bytecode
    // // Can predicate on the number of immediates available using a global!
    // // TODO -- figure out how immN will work
    // BrTable, br_table, 1, vec![DataType::AssumeGood], get_br_table_globals(), vec![], "https://musteresel.github.io/posts/2020/01/webassembly-text-br_table-example.html"
    // // Return => visit_return TODO
    // Return, _return, 0, vec![], HashMap::new(), vec![], "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return"
    Call, call, 0, vec![DataType::U32], get_call_globals(), get_call_fns(), WhammModeKind::default_modes(), "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call"
    // CallIndirect { type_index: u32, table_index: u32 } => visit_call_indirect TODO
    // ReturnCall { function_index: u32 } => visit_return_call TODO
    // ReturnCallIndirect { type_index: u32, table_index: u32 } => visit_return_call_indirect TODO
    // Drop => visit_drop
    // Select => visit_select
    // TypedSelect { ty: $crate::ValType } => visit_typed_select
    // LocalGet { local_index: u32 } => visit_local_get TODO
    // LocalSet { local_index: u32 } => visit_local_set TODO
    // LocalTee { local_index: u32 } => visit_local_tee TODO
    // GlobalGet { global_index: u32 } => visit_global_get TODO
    // GlobalSet { global_index: u32 } => visit_global_set TODO
    // I32Load { memarg: $crate::MemArg } => visit_i32_load
    // I64Load { memarg: $crate::MemArg } => visit_i64_load
    // F32Load { memarg: $crate::MemArg } => visit_f32_load
    // F64Load { memarg: $crate::MemArg } => visit_f64_load
    // I32Load8S { memarg: $crate::MemArg } => visit_i32_load8_s
    // I32Load8U { memarg: $crate::MemArg } => visit_i32_load8_u TODO
    // I32Load16S { memarg: $crate::MemArg } => visit_i32_load16_s
    // I32Load16U { memarg: $crate::MemArg } => visit_i32_load16_u
    // I64Load8S { memarg: $crate::MemArg } => visit_i64_load8_s
    // I64Load8U { memarg: $crate::MemArg } => visit_i64_load8_u
    // I64Load16S { memarg: $crate::MemArg } => visit_i64_load16_s
    // I64Load16U { memarg: $crate::MemArg } => visit_i64_load16_u
    // I64Load32S { memarg: $crate::MemArg } => visit_i64_load32_s
    // I64Load32U { memarg: $crate::MemArg } => visit_i64_load32_u
    // I32Store { memarg: $crate::MemArg } => visit_i32_store
    // I64Store { memarg: $crate::MemArg } => visit_i64_store
    // F32Store { memarg: $crate::MemArg } => visit_f32_store
    // F64Store { memarg: $crate::MemArg } => visit_f64_store
    // I32Store8 { memarg: $crate::MemArg } => visit_i32_store8
    // I32Store16 { memarg: $crate::MemArg } => visit_i32_store16
    // I64Store8 { memarg: $crate::MemArg } => visit_i64_store8
    // I64Store16 { memarg: $crate::MemArg } => visit_i64_store16
    // I64Store32 { memarg: $crate::MemArg } => visit_i64_store32
    // MemorySize { mem: u32 } => visit_memory_size
    // MemoryGrow { mem: u32 } => visit_memory_grow
    // I32Const { value: i32 } => visit_i32_const TODO
    // I64Const { value: i64 } => visit_i64_const TODO
    // F32Const { value: $crate::Ieee32 } => visit_f32_const
    // F64Const { value: $crate::Ieee64 } => visit_f64_const
    // RefNull { hty: $crate::HeapType } => visit_ref_null
    // RefIsNull => visit_ref_is_null
    // RefFunc { function_index: u32 } => visit_ref_func
    // RefEq => visit_ref_eq
    // I32Eqz => visit_i32_eqz
    // I32Eq => visit_i32_eq
    // I32Ne => visit_i32_ne
    // I32LtS => visit_i32_lt_s
    // I32LtU => visit_i32_lt_u
    // I32GtS => visit_i32_gt_s
    // I32GtU => visit_i32_gt_u
    // I32LeS => visit_i32_le_s
    // I32LeU => visit_i32_le_u
    // I32GeS => visit_i32_ge_s
    // I32GeU => visit_i32_ge_u
    // I64Eqz => visit_i64_eqz
    // I64Eq => visit_i64_eq
    // I64Ne => visit_i64_ne
    // I64LtS => visit_i64_lt_s
    // I64LtU => visit_i64_lt_u
    // I64GtS => visit_i64_gt_s
    // I64GtU => visit_i64_gt_u
    // I64LeS => visit_i64_le_s
    // I64LeU => visit_i64_le_u
    // I64GeS => visit_i64_ge_s
    // I64GeU => visit_i64_ge_u
    // F32Eq => visit_f32_eq
    // F32Ne => visit_f32_ne
    // F32Lt => visit_f32_lt
    // F32Gt => visit_f32_gt
    // F32Le => visit_f32_le
    // F32Ge => visit_f32_ge
    // F64Eq => visit_f64_eq
    // F64Ne => visit_f64_ne
    // F64Lt => visit_f64_lt
    // F64Gt => visit_f64_gt
    // F64Le => visit_f64_le
    // F64Ge => visit_f64_ge
    // I32Clz => visit_i32_clz
    // I32Ctz => visit_i32_ctz
    // I32Popcnt => visit_i32_popcnt
    // I32Add => visit_i32_add
    // I32Sub => visit_i32_sub
    // I32Mul => visit_i32_mul
    // I32DivS => visit_i32_div_s
    // I32DivU => visit_i32_div_u
    // I32RemS => visit_i32_rem_s
    // I32RemU => visit_i32_rem_u
    // I32And => visit_i32_and
    // I32Or => visit_i32_or
    // I32Xor => visit_i32_xor
    // I32Shl => visit_i32_shl
    // I32ShrS => visit_i32_shr_s
    // I32ShrU => visit_i32_shr_u
    // I32Rotl => visit_i32_rotl
    // I32Rotr => visit_i32_rotr
    // I64Clz => visit_i64_clz
    // I64Ctz => visit_i64_ctz
    // I64Popcnt => visit_i64_popcnt
    // I64Add => visit_i64_add
    // I64Sub => visit_i64_sub
    // I64Mul => visit_i64_mul
    // I64DivS => visit_i64_div_s
    // I64DivU => visit_i64_div_u
    // I64RemS => visit_i64_rem_s
    // I64RemU => visit_i64_rem_u
    // I64And => visit_i64_and
    // I64Or => visit_i64_or
    // I64Xor => visit_i64_xor
    // I64Shl => visit_i64_shl
    // I64ShrS => visit_i64_shr_s
    // I64ShrU => visit_i64_shr_u
    // I64Rotl => visit_i64_rotl
    // I64Rotr => visit_i64_rotr
    // F32Abs => visit_f32_abs
    // F32Neg => visit_f32_neg
    // F32Ceil => visit_f32_ceil
    // F32Floor => visit_f32_floor
    // F32Trunc => visit_f32_trunc
    // F32Nearest => visit_f32_nearest
    // F32Sqrt => visit_f32_sqrt
    // F32Add => visit_f32_add
    // F32Sub => visit_f32_sub
    // F32Mul => visit_f32_mul
    // F32Div => visit_f32_div
    // F32Min => visit_f32_min
    // F32Max => visit_f32_max
    // F32Copysign => visit_f32_copysign
    // F64Abs => visit_f64_abs
    // F64Neg => visit_f64_neg
    // F64Ceil => visit_f64_ceil
    // F64Floor => visit_f64_floor
    // F64Trunc => visit_f64_trunc
    // F64Nearest => visit_f64_nearest
    // F64Sqrt => visit_f64_sqrt
    // F64Add => visit_f64_add
    // F64Sub => visit_f64_sub
    // F64Mul => visit_f64_mul
    // F64Div => visit_f64_div
    // F64Min => visit_f64_min
    // F64Max => visit_f64_max
    // F64Copysign => visit_f64_copysign
    // I32WrapI64 => visit_i32_wrap_i64
    // I32TruncF32S => visit_i32_trunc_f32_s
    // I32TruncF32U => visit_i32_trunc_f32_u
    // I32TruncF64S => visit_i32_trunc_f64_s
    // I32TruncF64U => visit_i32_trunc_f64_u
    // I64ExtendI32S => visit_i64_extend_i32_s
    // I64ExtendI32U => visit_i64_extend_i32_u
    // I64TruncF32S => visit_i64_trunc_f32_s
    // I64TruncF32U => visit_i64_trunc_f32_u
    // I64TruncF64S => visit_i64_trunc_f64_s
    // I64TruncF64U => visit_i64_trunc_f64_u
    // F32ConvertI32S => visit_f32_convert_i32_s
    // F32ConvertI32U => visit_f32_convert_i32_u
    // F32ConvertI64S => visit_f32_convert_i64_s
    // F32ConvertI64U => visit_f32_convert_i64_u
    // F32DemoteF64 => visit_f32_demote_f64
    // F64ConvertI32S => visit_f64_convert_i32_s
    // F64ConvertI32U => visit_f64_convert_i32_u
    // F64ConvertI64S => visit_f64_convert_i64_s
    // F64ConvertI64U => visit_f64_convert_i64_u
    // F64PromoteF32 => visit_f64_promote_f32
    // I32ReinterpretF32 => visit_i32_reinterpret_f32
    // I64ReinterpretF64 => visit_i64_reinterpret_f64
    // F32ReinterpretI32 => visit_f32_reinterpret_i32
    // F64ReinterpretI64 => visit_f64_reinterpret_i64
    // I32Extend8S => visit_i32_extend8_s
    // I32Extend16S => visit_i32_extend16_s
    // I64Extend8S => visit_i64_extend8_s
    // I64Extend16S => visit_i64_extend16_s
    // I64Extend32S => visit_i64_extend32_s
    //
    // // 0xFB prefixed operators
    // // Garbage Collection
    // // http://github.com/WebAssembly/gc
    // StructNew { struct_type_index: u32 } => visit_struct_new
    // StructNewDefault { struct_type_index: u32 } => visit_struct_new_default
    // StructGet { struct_type_index: u32, field_index: u32 } => visit_struct_get
    // StructGetS { struct_type_index: u32, field_index: u32 } => visit_struct_get_s
    // StructGetU { struct_type_index: u32, field_index: u32 } => visit_struct_get_u
    // StructSet { struct_type_index: u32, field_index: u32 } => visit_struct_set
    // ArrayNew { array_type_index: u32 } => visit_array_new
    // ArrayNewDefault { array_type_index: u32 } => visit_array_new_default
    // ArrayNewFixed { array_type_index: u32, array_size: u32 } => visit_array_new_fixed
    // ArrayNewData { array_type_index: u32, array_data_index: u32 } => visit_array_new_data
    // ArrayNewElem { array_type_index: u32, array_elem_index: u32 } => visit_array_new_elem
    // ArrayGet { array_type_index: u32 } => visit_array_get
    // ArrayGetS { array_type_index: u32 } => visit_array_get_s
    // ArrayGetU { array_type_index: u32 } => visit_array_get_u
    // ArraySet { array_type_index: u32 } => visit_array_set
    // ArrayLen => visit_array_len
    // ArrayFill { array_type_index: u32 } => visit_array_fill
    // ArrayCopy { array_type_index_dst: u32, array_type_index_src: u32 } => visit_array_copy
    // ArrayInitData { array_type_index: u32, array_data_index: u32 } => visit_array_init_data
    // ArrayInitElem { array_type_index: u32, array_elem_index: u32 } => visit_array_init_elem
    // RefTestNonNull { hty: $crate::HeapType } => visit_ref_test_non_null
    // RefTestNullable { hty: $crate::HeapType } => visit_ref_test_nullable
    // RefCastNonNull { hty: $crate::HeapType } => visit_ref_cast_non_null
    // RefCastNullable { hty: $crate::HeapType } => visit_ref_cast_nullable
    // BrOnCast { TODO
    //     relative_depth: u32,
    //     from_ref_type: $crate::RefType,
    //     to_ref_type: $crate::RefType
    // } => visit_br_on_cast
    // BrOnCastFail { TODO
    //     relative_depth: u32,
    //     from_ref_type: $crate::RefType,
    //     to_ref_type: $crate::RefType
    // } => visit_br_on_cast_fail
    // AnyConvertExtern => visit_any_convert_extern
    // ExternConvertAny => visit_extern_convert_any
    // RefI31 => visit_ref_i31
    // I31GetS => visit_i31_get_s
    // I31GetU => visit_i31_get_u
    //
    // // 0xFC operators
    // // Non-trapping Float-to-int Conversions
    // // https://github.com/WebAssembly/nontrapping-float-to-int-conversions
    // I32TruncSatF32S => visit_i32_trunc_sat_f32_s
    // I32TruncSatF32U => visit_i32_trunc_sat_f32_u
    // I32TruncSatF64S => visit_i32_trunc_sat_f64_s
    // I32TruncSatF64U => visit_i32_trunc_sat_f64_u
    // I64TruncSatF32S => visit_i64_trunc_sat_f32_s
    // I64TruncSatF32U => visit_i64_trunc_sat_f32_u
    // I64TruncSatF64S => visit_i64_trunc_sat_f64_s
    // I64TruncSatF64U => visit_i64_trunc_sat_f64_u
    //
    // // 0xFC prefixed operators
    // // bulk memory operations
    // // https://github.com/WebAssembly/bulk-memory-operations
    // MemoryInit { data_index: u32, mem: u32 } => visit_memory_init
    // DataDrop { data_index: u32 } => visit_data_drop
    // MemoryCopy { dst_mem: u32, src_mem: u32 } => visit_memory_copy
    // MemoryFill { mem: u32 } => visit_memory_fill
    // TableInit { elem_index: u32, table: u32 } => visit_table_init
    // ElemDrop { elem_index: u32 } => visit_elem_drop
    // TableCopy { dst_table: u32, src_table: u32 } => visit_table_copy
    //
    // // 0xFC prefixed operators
    // // reference-types
    // // https://github.com/WebAssembly/reference-types
    // TableFill { table: u32 } => visit_table_fill
    // TableGet { table: u32 } => visit_table_get
    // TableSet { table: u32 } => visit_table_set
    // TableGrow { table: u32 } => visit_table_grow
    // TableSize { table: u32 } => visit_table_size
    //
    // // OxFC prefixed operators
    // // memory control (experimental)
    // // https://github.com/WebAssembly/design/issues/1439
    // MemoryDiscard { mem: u32 } => visit_memory_discard
    //
    // // 0xFE prefixed operators
    // // threads
    // // https://github.com/WebAssembly/threads
    // MemoryAtomicNotify { memarg: $crate::MemArg } => visit_memory_atomic_notify
    // MemoryAtomicWait32 { memarg: $crate::MemArg } => visit_memory_atomic_wait32
    // MemoryAtomicWait64 { memarg: $crate::MemArg } => visit_memory_atomic_wait64
    // AtomicFence => visit_atomic_fence
    // I32AtomicLoad { memarg: $crate::MemArg } => visit_i32_atomic_load
    // I64AtomicLoad { memarg: $crate::MemArg } => visit_i64_atomic_load
    // I32AtomicLoad8U { memarg: $crate::MemArg } => visit_i32_atomic_load8_u
    // I32AtomicLoad16U { memarg: $crate::MemArg } => visit_i32_atomic_load16_u
    // I64AtomicLoad8U { memarg: $crate::MemArg } => visit_i64_atomic_load8_u
    // I64AtomicLoad16U { memarg: $crate::MemArg } => visit_i64_atomic_load16_u
    // I64AtomicLoad32U { memarg: $crate::MemArg } => visit_i64_atomic_load32_u
    // I32AtomicStore { memarg: $crate::MemArg } => visit_i32_atomic_store
    // I64AtomicStore { memarg: $crate::MemArg } => visit_i64_atomic_store
    // I32AtomicStore8 { memarg: $crate::MemArg } => visit_i32_atomic_store8
    // I32AtomicStore16 { memarg: $crate::MemArg } => visit_i32_atomic_store16
    // I64AtomicStore8 { memarg: $crate::MemArg } => visit_i64_atomic_store8
    // I64AtomicStore16 { memarg: $crate::MemArg } => visit_i64_atomic_store16
    // I64AtomicStore32 { memarg: $crate::MemArg } => visit_i64_atomic_store32
    // I32AtomicRmwAdd { memarg: $crate::MemArg } => visit_i32_atomic_rmw_add
    // I64AtomicRmwAdd { memarg: $crate::MemArg } => visit_i64_atomic_rmw_add
    // I32AtomicRmw8AddU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_add_u
    // I32AtomicRmw16AddU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_add_u
    // I64AtomicRmw8AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_add_u
    // I64AtomicRmw16AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_add_u
    // I64AtomicRmw32AddU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_add_u
    // I32AtomicRmwSub { memarg: $crate::MemArg } => visit_i32_atomic_rmw_sub
    // I64AtomicRmwSub { memarg: $crate::MemArg } => visit_i64_atomic_rmw_sub
    // I32AtomicRmw8SubU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_sub_u
    // I32AtomicRmw16SubU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_sub_u
    // I64AtomicRmw8SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_sub_u
    // I64AtomicRmw16SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_sub_u
    // I64AtomicRmw32SubU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_sub_u
    // I32AtomicRmwAnd { memarg: $crate::MemArg } => visit_i32_atomic_rmw_and
    // I64AtomicRmwAnd { memarg: $crate::MemArg } => visit_i64_atomic_rmw_and
    // I32AtomicRmw8AndU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_and_u
    // I32AtomicRmw16AndU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_and_u
    // I64AtomicRmw8AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_and_u
    // I64AtomicRmw16AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_and_u
    // I64AtomicRmw32AndU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_and_u
    // I32AtomicRmwOr { memarg: $crate::MemArg } => visit_i32_atomic_rmw_or
    // I64AtomicRmwOr { memarg: $crate::MemArg } => visit_i64_atomic_rmw_or
    // I32AtomicRmw8OrU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_or_u
    // I32AtomicRmw16OrU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_or_u
    // I64AtomicRmw8OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_or_u
    // I64AtomicRmw16OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_or_u
    // I64AtomicRmw32OrU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_or_u
    // I32AtomicRmwXor { memarg: $crate::MemArg } => visit_i32_atomic_rmw_xor
    // I64AtomicRmwXor { memarg: $crate::MemArg } => visit_i64_atomic_rmw_xor
    // I32AtomicRmw8XorU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_xor_u
    // I32AtomicRmw16XorU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_xor_u
    // I64AtomicRmw8XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_xor_u
    // I64AtomicRmw16XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_xor_u
    // I64AtomicRmw32XorU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_xor_u
    // I32AtomicRmwXchg { memarg: $crate::MemArg } => visit_i32_atomic_rmw_xchg
    // I64AtomicRmwXchg { memarg: $crate::MemArg } => visit_i64_atomic_rmw_xchg
    // I32AtomicRmw8XchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_xchg_u
    // I32AtomicRmw16XchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_xchg_u
    // I64AtomicRmw8XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_xchg_u
    // I64AtomicRmw16XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_xchg_u
    // I64AtomicRmw32XchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_xchg_u
    // I32AtomicRmwCmpxchg { memarg: $crate::MemArg } => visit_i32_atomic_rmw_cmpxchg
    // I64AtomicRmwCmpxchg { memarg: $crate::MemArg } => visit_i64_atomic_rmw_cmpxchg
    // I32AtomicRmw8CmpxchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw8_cmpxchg_u
    // I32AtomicRmw16CmpxchgU { memarg: $crate::MemArg } => visit_i32_atomic_rmw16_cmpxchg_u
    // I64AtomicRmw8CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw8_cmpxchg_u
    // I64AtomicRmw16CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw16_cmpxchg_u
    // I64AtomicRmw32CmpxchgU { memarg: $crate::MemArg } => visit_i64_atomic_rmw32_cmpxchg_u
    //
    // // Also 0xFE prefixed operators
    // // shared-everything threads
    // // https://github.com/WebAssembly/shared-everything-threads
    // GlobalAtomicGet { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_get
    // GlobalAtomicSet { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_set
    // GlobalAtomicRmwAdd { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_add
    // GlobalAtomicRmwSub { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_sub
    // GlobalAtomicRmwAnd { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_and
    // GlobalAtomicRmwOr { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_or
    // GlobalAtomicRmwXor { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_xor
    // GlobalAtomicRmwXchg { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_xchg
    // GlobalAtomicRmwCmpxchg { ordering: $crate::Ordering, global_index: u32 } => visit_global_atomic_rmw_cmpxchg
    // TableAtomicGet { ordering: $crate::Ordering, table_index: u32 } => visit_table_atomic_get
    // TableAtomicSet { ordering: $crate::Ordering, table_index: u32 } => visit_table_atomic_set
    // TableAtomicRmwXchg { ordering: $crate::Ordering, table_index: u32 } => visit_table_atomic_rmw_xchg
    // TableAtomicRmwCmpxchg { ordering: $crate::Ordering, table_index: u32 } => visit_table_atomic_rmw_cmpxchg
    // StructAtomicGet { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_get
    // StructAtomicGetS { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_get_s
    // StructAtomicGetU { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_get_u
    // StructAtomicSet { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_set
    // StructAtomicRmwAdd { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_add
    // StructAtomicRmwSub { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_sub
    // StructAtomicRmwAnd { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_and
    // StructAtomicRmwOr { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_or
    // StructAtomicRmwXor { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_xor
    // StructAtomicRmwXchg { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_xchg
    // StructAtomicRmwCmpxchg { ordering: $crate::Ordering, struct_type_index: u32, field_index: u32  } => visit_struct_atomic_rmw_cmpxchg
    // ArrayAtomicGet { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_get
    // ArrayAtomicGetS { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_get_s
    // ArrayAtomicGetU { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_get_u
    // ArrayAtomicSet { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_set
    // ArrayAtomicRmwAdd { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_add
    // ArrayAtomicRmwSub { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_sub
    // ArrayAtomicRmwAnd { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_and
    // ArrayAtomicRmwOr { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_or
    // ArrayAtomicRmwXor { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_xor
    // ArrayAtomicRmwXchg { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_xchg
    // ArrayAtomicRmwCmpxchg { ordering: $crate::Ordering, array_type_index: u32 } => visit_array_atomic_rmw_cmpxchg
    // RefI31Shared => visit_ref_i31_shared
    //
    // // 0xFD operators
    // // 128-bit SIMD
    // // - https://github.com/webassembly/simd
    // // - https://webassembly.github.io/simd/core/binary/instructions.html
    // V128Load { memarg: $crate::MemArg } => visit_v128_load
    // V128Load8x8S { memarg: $crate::MemArg } => visit_v128_load8x8_s
    // V128Load8x8U { memarg: $crate::MemArg } => visit_v128_load8x8_u
    // V128Load16x4S { memarg: $crate::MemArg } => visit_v128_load16x4_s
    // V128Load16x4U { memarg: $crate::MemArg } => visit_v128_load16x4_u
    // V128Load32x2S { memarg: $crate::MemArg } => visit_v128_load32x2_s
    // V128Load32x2U { memarg: $crate::MemArg } => visit_v128_load32x2_u
    // V128Load8Splat { memarg: $crate::MemArg } => visit_v128_load8_splat
    // V128Load16Splat { memarg: $crate::MemArg } => visit_v128_load16_splat
    // V128Load32Splat { memarg: $crate::MemArg } => visit_v128_load32_splat
    // V128Load64Splat { memarg: $crate::MemArg } => visit_v128_load64_splat
    // V128Load32Zero { memarg: $crate::MemArg } => visit_v128_load32_zero
    // V128Load64Zero { memarg: $crate::MemArg } => visit_v128_load64_zero
    // V128Store { memarg: $crate::MemArg } => visit_v128_store
    // V128Load8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load8_lane
    // V128Load16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load16_lane
    // V128Load32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load32_lane
    // V128Load64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_load64_lane
    // V128Store8Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store8_lane
    // V128Store16Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store16_lane
    // V128Store32Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store32_lane
    // V128Store64Lane { memarg: $crate::MemArg, lane: u8 } => visit_v128_store64_lane
    // V128Const { value: $crate::V128 } => visit_v128_const
    // I8x16Shuffle { lanes: [u8; 16] } => visit_i8x16_shuffle
    // I8x16ExtractLaneS { lane: u8 } => visit_i8x16_extract_lane_s
    // I8x16ExtractLaneU { lane: u8 } => visit_i8x16_extract_lane_u
    // I8x16ReplaceLane { lane: u8 } => visit_i8x16_replace_lane
    // I16x8ExtractLaneS { lane: u8 } => visit_i16x8_extract_lane_s
    // I16x8ExtractLaneU { lane: u8 } => visit_i16x8_extract_lane_u
    // I16x8ReplaceLane { lane: u8 } => visit_i16x8_replace_lane
    // I32x4ExtractLane { lane: u8 } => visit_i32x4_extract_lane
    // I32x4ReplaceLane { lane: u8 } => visit_i32x4_replace_lane
    // I64x2ExtractLane { lane: u8 } => visit_i64x2_extract_lane
    // I64x2ReplaceLane { lane: u8 } => visit_i64x2_replace_lane
    // F32x4ExtractLane { lane: u8 } => visit_f32x4_extract_lane
    // F32x4ReplaceLane { lane: u8 } => visit_f32x4_replace_lane
    // F64x2ExtractLane { lane: u8 } => visit_f64x2_extract_lane
    // F64x2ReplaceLane { lane: u8 } => visit_f64x2_replace_lane
    // I8x16Swizzle => visit_i8x16_swizzle
    // I8x16Splat => visit_i8x16_splat
    // I16x8Splat => visit_i16x8_splat
    // I32x4Splat => visit_i32x4_splat
    // I64x2Splat => visit_i64x2_splat
    // F32x4Splat => visit_f32x4_splat
    // F64x2Splat => visit_f64x2_splat
    // I8x16Eq => visit_i8x16_eq
    // I8x16Ne => visit_i8x16_ne
    // I8x16LtS => visit_i8x16_lt_s
    // I8x16LtU => visit_i8x16_lt_u
    // I8x16GtS => visit_i8x16_gt_s
    // I8x16GtU => visit_i8x16_gt_u
    // I8x16LeS => visit_i8x16_le_s
    // I8x16LeU => visit_i8x16_le_u
    // I8x16GeS => visit_i8x16_ge_s
    // I8x16GeU => visit_i8x16_ge_u
    // I16x8Eq => visit_i16x8_eq
    // I16x8Ne => visit_i16x8_ne
    // I16x8LtS => visit_i16x8_lt_s
    // I16x8LtU => visit_i16x8_lt_u
    // I16x8GtS => visit_i16x8_gt_s
    // I16x8GtU => visit_i16x8_gt_u
    // I16x8LeS => visit_i16x8_le_s
    // I16x8LeU => visit_i16x8_le_u
    // I16x8GeS => visit_i16x8_ge_s
    // I16x8GeU => visit_i16x8_ge_u
    // I32x4Eq => visit_i32x4_eq
    // I32x4Ne => visit_i32x4_ne
    // I32x4LtS => visit_i32x4_lt_s
    // I32x4LtU => visit_i32x4_lt_u
    // I32x4GtS => visit_i32x4_gt_s
    // I32x4GtU => visit_i32x4_gt_u
    // I32x4LeS => visit_i32x4_le_s
    // I32x4LeU => visit_i32x4_le_u
    // I32x4GeS => visit_i32x4_ge_s
    // I32x4GeU => visit_i32x4_ge_u
    // I64x2Eq => visit_i64x2_eq
    // I64x2Ne => visit_i64x2_ne
    // I64x2LtS => visit_i64x2_lt_s
    // I64x2GtS => visit_i64x2_gt_s
    // I64x2LeS => visit_i64x2_le_s
    // I64x2GeS => visit_i64x2_ge_s
    // F32x4Eq => visit_f32x4_eq
    // F32x4Ne => visit_f32x4_ne
    // F32x4Lt => visit_f32x4_lt
    // F32x4Gt => visit_f32x4_gt
    // F32x4Le => visit_f32x4_le
    // F32x4Ge => visit_f32x4_ge
    // F64x2Eq => visit_f64x2_eq
    // F64x2Ne => visit_f64x2_ne
    // F64x2Lt => visit_f64x2_lt
    // F64x2Gt => visit_f64x2_gt
    // F64x2Le => visit_f64x2_le
    // F64x2Ge => visit_f64x2_ge
    // V128Not => visit_v128_not
    // V128And => visit_v128_and
    // V128AndNot => visit_v128_andnot
    // V128Or => visit_v128_or
    // V128Xor => visit_v128_xor
    // V128Bitselect => visit_v128_bitselect
    // V128AnyTrue => visit_v128_any_true
    // I8x16Abs => visit_i8x16_abs
    // I8x16Neg => visit_i8x16_neg
    // I8x16Popcnt => visit_i8x16_popcnt
    // I8x16AllTrue => visit_i8x16_all_true
    // I8x16Bitmask => visit_i8x16_bitmask
    // I8x16NarrowI16x8S => visit_i8x16_narrow_i16x8_s
    // I8x16NarrowI16x8U => visit_i8x16_narrow_i16x8_u
    // I8x16Shl => visit_i8x16_shl
    // I8x16ShrS => visit_i8x16_shr_s
    // I8x16ShrU => visit_i8x16_shr_u
    // I8x16Add => visit_i8x16_add
    // I8x16AddSatS => visit_i8x16_add_sat_s
    // I8x16AddSatU => visit_i8x16_add_sat_u
    // I8x16Sub => visit_i8x16_sub
    // I8x16SubSatS => visit_i8x16_sub_sat_s
    // I8x16SubSatU => visit_i8x16_sub_sat_u
    // I8x16MinS => visit_i8x16_min_s
    // I8x16MinU => visit_i8x16_min_u
    // I8x16MaxS => visit_i8x16_max_s
    // I8x16MaxU => visit_i8x16_max_u
    // I8x16AvgrU => visit_i8x16_avgr_u
    // I16x8ExtAddPairwiseI8x16S => visit_i16x8_extadd_pairwise_i8x16_s
    // I16x8ExtAddPairwiseI8x16U => visit_i16x8_extadd_pairwise_i8x16_u
    // I16x8Abs => visit_i16x8_abs
    // I16x8Neg => visit_i16x8_neg
    // I16x8Q15MulrSatS => visit_i16x8_q15mulr_sat_s
    // I16x8AllTrue => visit_i16x8_all_true
    // I16x8Bitmask => visit_i16x8_bitmask
    // I16x8NarrowI32x4S => visit_i16x8_narrow_i32x4_s
    // I16x8NarrowI32x4U => visit_i16x8_narrow_i32x4_u
    // I16x8ExtendLowI8x16S => visit_i16x8_extend_low_i8x16_s
    // I16x8ExtendHighI8x16S => visit_i16x8_extend_high_i8x16_s
    // I16x8ExtendLowI8x16U => visit_i16x8_extend_low_i8x16_u
    // I16x8ExtendHighI8x16U => visit_i16x8_extend_high_i8x16_u
    // I16x8Shl => visit_i16x8_shl
    // I16x8ShrS => visit_i16x8_shr_s
    // I16x8ShrU => visit_i16x8_shr_u
    // I16x8Add => visit_i16x8_add
    // I16x8AddSatS => visit_i16x8_add_sat_s
    // I16x8AddSatU => visit_i16x8_add_sat_u
    // I16x8Sub => visit_i16x8_sub
    // I16x8SubSatS => visit_i16x8_sub_sat_s
    // I16x8SubSatU => visit_i16x8_sub_sat_u
    // I16x8Mul => visit_i16x8_mul
    // I16x8MinS => visit_i16x8_min_s
    // I16x8MinU => visit_i16x8_min_u
    // I16x8MaxS => visit_i16x8_max_s
    // I16x8MaxU => visit_i16x8_max_u
    // I16x8AvgrU => visit_i16x8_avgr_u
    // I16x8ExtMulLowI8x16S => visit_i16x8_extmul_low_i8x16_s
    // I16x8ExtMulHighI8x16S => visit_i16x8_extmul_high_i8x16_s
    // I16x8ExtMulLowI8x16U => visit_i16x8_extmul_low_i8x16_u
    // I16x8ExtMulHighI8x16U => visit_i16x8_extmul_high_i8x16_u
    // I32x4ExtAddPairwiseI16x8S => visit_i32x4_extadd_pairwise_i16x8_s
    // I32x4ExtAddPairwiseI16x8U => visit_i32x4_extadd_pairwise_i16x8_u
    // I32x4Abs => visit_i32x4_abs
    // I32x4Neg => visit_i32x4_neg
    // I32x4AllTrue => visit_i32x4_all_true
    // I32x4Bitmask => visit_i32x4_bitmask
    // I32x4ExtendLowI16x8S => visit_i32x4_extend_low_i16x8_s
    // I32x4ExtendHighI16x8S => visit_i32x4_extend_high_i16x8_s
    // I32x4ExtendLowI16x8U => visit_i32x4_extend_low_i16x8_u
    // I32x4ExtendHighI16x8U => visit_i32x4_extend_high_i16x8_u
    // I32x4Shl => visit_i32x4_shl
    // I32x4ShrS => visit_i32x4_shr_s
    // I32x4ShrU => visit_i32x4_shr_u
    // I32x4Add => visit_i32x4_add
    // I32x4Sub => visit_i32x4_sub
    // I32x4Mul => visit_i32x4_mul
    // I32x4MinS => visit_i32x4_min_s
    // I32x4MinU => visit_i32x4_min_u
    // I32x4MaxS => visit_i32x4_max_s
    // I32x4MaxU => visit_i32x4_max_u
    // I32x4DotI16x8S => visit_i32x4_dot_i16x8_s
    // I32x4ExtMulLowI16x8S => visit_i32x4_extmul_low_i16x8_s
    // I32x4ExtMulHighI16x8S => visit_i32x4_extmul_high_i16x8_s
    // I32x4ExtMulLowI16x8U => visit_i32x4_extmul_low_i16x8_u
    // I32x4ExtMulHighI16x8U => visit_i32x4_extmul_high_i16x8_u
    // I64x2Abs => visit_i64x2_abs
    // I64x2Neg => visit_i64x2_neg
    // I64x2AllTrue => visit_i64x2_all_true
    // I64x2Bitmask => visit_i64x2_bitmask
    // I64x2ExtendLowI32x4S => visit_i64x2_extend_low_i32x4_s
    // I64x2ExtendHighI32x4S => visit_i64x2_extend_high_i32x4_s
    // I64x2ExtendLowI32x4U => visit_i64x2_extend_low_i32x4_u
    // I64x2ExtendHighI32x4U => visit_i64x2_extend_high_i32x4_u
    // I64x2Shl => visit_i64x2_shl
    // I64x2ShrS => visit_i64x2_shr_s
    // I64x2ShrU => visit_i64x2_shr_u
    // I64x2Add => visit_i64x2_add
    // I64x2Sub => visit_i64x2_sub
    // I64x2Mul => visit_i64x2_mul
    // I64x2ExtMulLowI32x4S => visit_i64x2_extmul_low_i32x4_s
    // I64x2ExtMulHighI32x4S => visit_i64x2_extmul_high_i32x4_s
    // I64x2ExtMulLowI32x4U => visit_i64x2_extmul_low_i32x4_u
    // I64x2ExtMulHighI32x4U => visit_i64x2_extmul_high_i32x4_u
    // F32x4Ceil => visit_f32x4_ceil
    // F32x4Floor => visit_f32x4_floor
    // F32x4Trunc => visit_f32x4_trunc
    // F32x4Nearest => visit_f32x4_nearest
    // F32x4Abs => visit_f32x4_abs
    // F32x4Neg => visit_f32x4_neg
    // F32x4Sqrt => visit_f32x4_sqrt
    // F32x4Add => visit_f32x4_add
    // F32x4Sub => visit_f32x4_sub
    // F32x4Mul => visit_f32x4_mul
    // F32x4Div => visit_f32x4_div
    // F32x4Min => visit_f32x4_min
    // F32x4Max => visit_f32x4_max
    // F32x4PMin => visit_f32x4_pmin
    // F32x4PMax => visit_f32x4_pmax
    // F64x2Ceil => visit_f64x2_ceil
    // F64x2Floor => visit_f64x2_floor
    // F64x2Trunc => visit_f64x2_trunc
    // F64x2Nearest => visit_f64x2_nearest
    // F64x2Abs => visit_f64x2_abs
    // F64x2Neg => visit_f64x2_neg
    // F64x2Sqrt => visit_f64x2_sqrt
    // F64x2Add => visit_f64x2_add
    // F64x2Sub => visit_f64x2_sub
    // F64x2Mul => visit_f64x2_mul
    // F64x2Div => visit_f64x2_div
    // F64x2Min => visit_f64x2_min
    // F64x2Max => visit_f64x2_max
    // F64x2PMin => visit_f64x2_pmin
    // F64x2PMax => visit_f64x2_pmax
    // I32x4TruncSatF32x4S => visit_i32x4_trunc_sat_f32x4_s
    // I32x4TruncSatF32x4U => visit_i32x4_trunc_sat_f32x4_u
    // F32x4ConvertI32x4S => visit_f32x4_convert_i32x4_s
    // F32x4ConvertI32x4U => visit_f32x4_convert_i32x4_u
    // I32x4TruncSatF64x2SZero => visit_i32x4_trunc_sat_f64x2_s_zero
    // I32x4TruncSatF64x2UZero => visit_i32x4_trunc_sat_f64x2_u_zero
    // F64x2ConvertLowI32x4S => visit_f64x2_convert_low_i32x4_s
    // F64x2ConvertLowI32x4U => visit_f64x2_convert_low_i32x4_u
    // F32x4DemoteF64x2Zero => visit_f32x4_demote_f64x2_zero
    // F64x2PromoteLowF32x4 => visit_f64x2_promote_low_f32x4
    //
    // // Relaxed SIMD operators
    // // https://github.com/WebAssembly/relaxed-simd
    // I8x16RelaxedSwizzle => visit_i8x16_relaxed_swizzle
    // I32x4RelaxedTruncF32x4S => visit_i32x4_relaxed_trunc_f32x4_s
    // I32x4RelaxedTruncF32x4U => visit_i32x4_relaxed_trunc_f32x4_u
    // I32x4RelaxedTruncF64x2SZero => visit_i32x4_relaxed_trunc_f64x2_s_zero
    // I32x4RelaxedTruncF64x2UZero => visit_i32x4_relaxed_trunc_f64x2_u_zero
    // F32x4RelaxedMadd => visit_f32x4_relaxed_madd
    // F32x4RelaxedNmadd => visit_f32x4_relaxed_nmadd
    // F64x2RelaxedMadd => visit_f64x2_relaxed_madd
    // F64x2RelaxedNmadd => visit_f64x2_relaxed_nmadd
    // I8x16RelaxedLaneselect => visit_i8x16_relaxed_laneselect
    // I16x8RelaxedLaneselect => visit_i16x8_relaxed_laneselect
    // I32x4RelaxedLaneselect => visit_i32x4_relaxed_laneselect
    // I64x2RelaxedLaneselect => visit_i64x2_relaxed_laneselect
    // F32x4RelaxedMin => visit_f32x4_relaxed_min
    // F32x4RelaxedMax => visit_f32x4_relaxed_max
    // F64x2RelaxedMin => visit_f64x2_relaxed_min
    // F64x2RelaxedMax => visit_f64x2_relaxed_max
    // I16x8RelaxedQ15mulrS => visit_i16x8_relaxed_q15mulr_s
    // I16x8RelaxedDotI8x16I7x16S => visit_i16x8_relaxed_dot_i8x16_i7x16_s
    // I32x4RelaxedDotI8x16I7x16AddS => visit_i32x4_relaxed_dot_i8x16_i7x16_add_s
    //
    // // Typed Function references
    // CallRef { type_index: u32 } => visit_call_ref TODO
    // ReturnCallRef { type_index: u32 } => visit_return_call_ref TODO
    // RefAsNonNull => visit_ref_as_non_null
    // BrOnNull { relative_depth: u32 } => visit_br_on_null TODO
    // BrOnNonNull { relative_depth: u32 } => visit_br_on_non_null TODO
}};}

// ============================================
// ==== Getters for provided functionality ====
// ============================================

// (keeps the `for_each_opcode!` lines shorter)

pub fn get_call_globals() -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();

    // add in the extra globals (that aren't args or immediates)
    globals.insert(
        "target_fn_type".to_string(),
        ProvidedGlobal::new(
            "target_fn_type".to_string(),
            "The type of function being called at this call site. This constant will \
                            evaluate to either `local` or `import`."
                .to_string(),
            DataType::Str,
            true,
        ),
    );
    globals.insert(
        "target_imp_module".to_string(),
        ProvidedGlobal::new(
            "target_imp_module".to_string(),
            "The name of the module that the imported function comes from. \
                            To improve performance, pair with `target_fn_type == \"import\"` \
                            for faster short-circuiting."
                .to_string(),
            DataType::Str,
            true,
        ),
    );
    globals.insert(
        "target_fn_name".to_string(),
        ProvidedGlobal::new(
            "target_fn_name".to_string(),
            "The name of the imported function. \
                        To improve performance, pair with `target_fn_type == \"import\"` \
                        for faster short-circuiting."
                .to_string(),
            DataType::Str,
            true,
        ),
    );
    globals.insert(
        "arg[0:9]+".to_string(),
        ProvidedGlobal::new(
            "arg[0:9]+".to_string(),
            "The argument to the call at the specific index, e.g. [0:9]+.\
                Keep in mind, the number of arguments to a call changes based on the targeted function.".to_string(),
            DataType::AssumeGood,
            false
        )
    );

    globals
}

pub fn get_call_fns() -> Vec<ProvidedFunction> {
    vec![ProvidedFunction::new(
        "alt_call_by_id".to_string(),
        "Insert an alternate call (targeting the passed function ID) into the Wasm bytecode. Will also emit the original parameters onto the stack.".to_string(),
        vec![(
            Expr::VarId {
                is_comp_provided: true,
                name: "func_id".to_string(),
                loc: None,
            },
            DataType::I32,
        )],
        DataType::Tuple { ty_info: vec![] },
        true
    ), ProvidedFunction::new(
        "alt_call_by_name".to_string(),
        "Insert an alternate call (targeting the passed function name) into the Wasm bytecode. Will also emit the original parameters onto the stack.".to_string(),
        vec![(
            Expr::VarId {
                is_comp_provided: true,
                name: "func_name".to_string(),
                loc: None,
            },
            DataType::Str,
        )],
        DataType::Tuple { ty_info: vec![] },
        true
    )]
}

pub fn get_br_table_globals() -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();

    // add in the extra globals (that aren't args or immediates)
    globals.insert(
        "num_targets".to_string(),
        ProvidedGlobal::new(
            "num_targets".to_string(),
            "The number of target branches for this br_table instruction (correlates with the number of immediates, e.g. `immN`).\
            NOTE: This can be used in a predicate to ensure that the current br_table has the immN you need to interact with for the probe."
                .to_string(),
            DataType::Str,
            true,
        ),
    );
    globals.insert(
        "default_target".to_string(),
        ProvidedGlobal::new(
            "default_target".to_string(),
            "The default target of this br_table instruction.".to_string(),
            DataType::Str,
            true,
        ),
    );
    globals
}
