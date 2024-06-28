pub mod core;
pub mod wasm;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{magenta_italics, white};
use crate::parser::rules::core::CorePackage;
use crate::parser::rules::wasm::WasmPackage;
use crate::parser::types::{
    print_fns, print_global_vars, Expr, Location, ProbeSpec, ProvidedFunction, ProvidedGlobal,
    SpecPart, Statement,
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
        body: Option<Vec<Statement>>,
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
    body: Option<Vec<Statement>>,
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
                )
            } else {
                (false, false, false)
            };
            if !found_modes && !already_has {
                // If this matched provider wasn't already present, we need to remove.
                // Otherwise, we'd have providers with no probes in them!
                curr_providers.remove(&m.clone());
            }
            matched_packages |= found_package;
            matched_events |= found_events;
            matched_modes |= found_modes;
        }
        if !matched_providers {
            let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the provider pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        if !matched_packages {
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
        if !matched_events {
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
        if !matched_modes {
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
    provider: &dyn Provider,
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
        body: Option<Vec<Statement>>,
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
    body: Option<Vec<Statement>>,
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
                    )
                } else {
                    (false, false)
                };
            if !found_match_for_mode && !already_has {
                // If this matched package wasn't already present, we need to remove.
                // Otherwise, we'd have packages with no probes in them!
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
    package: &dyn Package,
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
    fn body(&self) -> &Option<Vec<Statement>>;
    fn body_mut(&mut self) -> &mut Option<Vec<Statement>>;
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
        body: Option<Vec<Statement>>,
    ) -> bool;
}

/// The base information needed for `Event`s, pulled out into a single struct.
pub struct EventInfo {
    // Statically defined, always the same
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
    body: Option<Vec<Statement>>,
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
            if !found_match_for_mode && !already_has {
                // If this matched package wasn't already present, we need to remove.
                // Otherwise, we'd have packages with no probes in them!
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
    event: &dyn Event,
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
    probe_spec: &ProbeSpec,
    loc: Option<Location>,
) -> Vec<Box<M>> {
    if let Some(SpecPart {
        name: mode_patt, ..
    }) = &probe_spec.mode
    {
        let matches = get_matches(M::get_name_options(), mode_patt);
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
    kind: WhammProviderKind,
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
            print_package_docs(&**package, print_globals, print_functions, tabs, buffer);
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
        body: Option<Vec<Statement>>,
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

pub enum WhammModeKind {
    Before,
    After,
    Alt,
}
impl WhammModeKind {
    fn name(&self) -> String {
        match self {
            Self::Before => "before".to_string(),
            Self::After => "after".to_string(),
            Self::Alt => "alt".to_string(),
        }
    }
}

/// The base modes provided by `whamm!` for an Event, these can be changed if desired.
/// To do so, the type of enum for a Probe's possible modes will need to be changed.
/// This means the Event's probes HashMap will need to point to a custom Probe type.
pub struct WhammMode {
    kind: WhammModeKind,
    info: ModeInfo,
}
impl NameOptions for WhammMode {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec!["before".to_string(), "after".to_string(), "alt".to_string()]
    }
}
impl FromStr for WhammMode {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "before" => Self::before(loc),
            "after" => Self::after(loc),
            "alt" => Self::alt(loc),
            _ => panic!("unsupported WhammMode: {name}"),
        }
    }
}
impl WhammMode {
    // ======================
    // ---- Constructors ----
    // ======================

    fn before(loc: Option<Location>) -> Self {
        Self {
            kind: WhammModeKind::Before,
            info: ModeInfo {
                docs: "This mode will cause the instrumentation logic to run *before* the \
                    probed event (if the predicate evaluates to `true`)."
                    .to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
            },
        }
    }
    fn after(loc: Option<Location>) -> Self {
        Self {
            kind: WhammModeKind::After,
            info: ModeInfo {
                docs: "This mode will cause the instrumentation logic to run *after* the \
                    probed event (if the predicate evaluates to `true`)."
                    .to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
            },
        }
    }
    fn alt(loc: Option<Location>) -> Self {
        Self {
            kind: WhammModeKind::Alt,
            info: ModeInfo {
                docs: "This mode will cause the instrumentation logic to run *instead of* the \
                    probed event (if the predicate evaluates to `true`)."
                    .to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
            },
        }
    }
}
impl Mode for WhammMode {
    fn name(&self) -> String {
        self.kind.name()
    }

    fn docs(&self) -> &String {
        &self.info.docs
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
}

/// The base definition of a probe for `whamm!`.
/// This can be customized if desired.
pub struct WhammProbe {
    pub mode: WhammMode,
    pub loc: Option<Location>,

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>,
}
impl Probe for WhammProbe {
    fn mode_name(&self) -> String {
        self.mode.name()
    }
    fn predicate(&self) -> &Option<Expr> {
        &self.predicate
    }
    fn predicate_mut(&mut self) -> &mut Option<Expr> {
        &mut self.predicate
    }

    fn body(&self) -> &Option<Vec<Statement>> {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Option<Vec<Statement>> {
        &mut self.body
    }

    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        print_mode_docs(&self.mode, print_globals, print_functions, tabs, buffer);
    }

    fn get_mode_provided_fns(&self) -> &Vec<ProvidedFunction> {
        self.mode.get_provided_fns()
    }

    fn get_mode_provided_fns_mut(&mut self) -> &mut Vec<ProvidedFunction> {
        self.mode.get_provided_fns_mut()
    }

    fn get_mode_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        self.mode.get_provided_globals()
    }
}
impl WhammProbe {
    fn new(
        mode: WhammMode,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Vec<Statement>>,
    ) -> Self {
        Self {
            mode,
            loc,
            predicate,
            body,
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
