#![allow(clippy::borrowed_box)]
pub mod core;
pub mod wasm;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::terminal::{magenta_italics, white};
use crate::generator::ast::ReqArgs;
use crate::parser::rules::core::{CorePackage, WhammMode, WhammModeKind, WhammProbe};
use crate::parser::rules::wasm::{OpcodeCategory, WasmPackage};
use crate::parser::types::{
    print_fns, print_global_vars, Block, DataType, Definition, Expr, Location, ProbeRule,
    ProvidedFunction, ProvidedGlobal, RulePart, Value,
};
use glob::Pattern;
use std::collections::HashMap;
use termcolor::Buffer;

pub trait NameOptions {
    fn get_name_options() -> Vec<String>;
}
pub trait FromStrWithLoc {
    fn from_str(name: &str, ty_info: Vec<(Expr, DataType)>, loc: Option<Location>) -> Self;
}
pub trait FromStr {
    fn from_str(name: &str) -> Self;
}

// ==================
// ---- Provider ----
// ==================

pub trait Provider {
    fn name(&self) -> String;
    fn docs(&self) -> &String;
    fn requires_map_lib(&self) -> bool;
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
    fn print_event_and_mode_docs(
        &self,
        probe_rule: &ProbeRule,
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
        id: &mut u32,
        probe_rule: &ProbeRule,
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
pub fn provider_factory<P: Provider + NameOptions + FromStrWithLoc + 'static>(
    curr_providers: &mut HashMap<String, Box<dyn Provider>>,
    id: &mut u32,
    probe_rule: &ProbeRule,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> Result<(bool, bool, bool, bool), Box<WhammError>> {
    if let Some(RulePart {
        name: provider_patt,
        ty_info,
        loc: provider_loc,
        ..
    }) = &probe_rule.provider
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
                .or_insert(Box::new(P::from_str(&m, ty_info.to_owned(), loc.clone())));

            let (found_package, found_events, found_modes) = if let Some(RulePart {
                loc: package_loc,
                ..
            }) = &probe_rule.package
            {
                provider.assign_matching_packages(
                    id,
                    probe_rule,
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
                // complete probe rules.
                curr_providers.remove(&m.clone());
            }
            matched_packages |= found_package;
            matched_events |= found_events;
            matched_modes |= found_modes;
        }
        if !matched_providers && probe_rule.provider.is_some() {
            let loc = provider_loc.as_ref().map(|loc| loc.line_col.clone());
            return Err(Box::new(ErrorGen::get_parse_error(
                true,
                Some("Could not find any matches for the provider pattern".to_string()),
                loc,
                vec![],
                vec![],
            )));
        }
        if !matched_packages && probe_rule.package.is_some() {
            let loc = probe_rule
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
        if !matched_events && probe_rule.event.is_some() {
            let loc = probe_rule
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
        if !matched_modes && probe_rule.mode.is_some() {
            let loc = probe_rule
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
    fn requires_map_lib(&self) -> bool;
    fn docs(&self) -> &String;
    fn has_events(&self) -> bool;
    fn len_events(&self) -> usize;
    fn events(&self) -> Box<dyn Iterator<Item = &dyn Event> + '_>;
    fn events_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn Event> + '_>;
    fn print_event_and_mode_docs(
        &self,
        probe_rule: &ProbeRule,
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
        id: &mut u32,
        probe_rule: &ProbeRule,
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
fn package_factory<P: Package + NameOptions + FromStrWithLoc + 'static>(
    curr_packages: &mut HashMap<String, Box<dyn Package>>,
    id: &mut u32,
    probe_rule: &ProbeRule,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> (bool, bool, bool) {
    if let Some(RulePart {
        name: package_patt,
        ty_info,
        ..
    }) = &probe_rule.package
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
                .or_insert(Box::new(P::from_str(&m, ty_info.to_owned(), loc.clone())));

            let (found_match_for_event, found_match_for_mode) =
                if let Some(RulePart { loc: event_loc, .. }) = &probe_rule.event {
                    package.assign_matching_events(
                        id,
                        probe_rule,
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
                // complete probe rules.
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
    fn id(&self) -> u32;
    fn mode(&self) -> WhammModeKind;
    fn predicate(&self) -> &Option<Expr>;
    fn predicate_mut(&mut self) -> &mut Option<Expr>;
    fn body(&self) -> &Option<Block>;
    fn body_mut(&mut self) -> &mut Option<Block>;
    fn print_mode_docs(
        &self,
        alias: Option<&String>,
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
    fn ty_info(&self) -> &Vec<(Expr, DataType)>;
    fn loc(&self) -> &Option<Location>;
    fn requires_map_lib(&self) -> bool;
    fn supported_modes(&self) -> &HashMap<String, WhammModeKind>;
    fn docs(&self) -> &String;
    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>>;
    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>>;
    fn print_mode_docs(
        &self,
        probe_rule: &ProbeRule,
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
        id: &mut u32,
        probe_rule: &ProbeRule,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> bool {
        let mut matched_modes = false;
        let modes: Vec<Box<WhammMode>> =
            mode_factory(self.supported_modes(), probe_rule, loc.clone());
        let probes = self.probes_mut();
        for mode in modes {
            matched_modes = true;
            let modes = probes.entry(mode.name()).or_default();
            modes.push(Box::new(WhammProbe::new(
                *id,
                *mode,
                loc.clone(),
                predicate.clone(),
                body.clone(),
            )));

            // prep for the next probe to add
            *id += 1;
        }
        matched_modes
    }
}

/// The base information needed for `Event`s, pulled out into a single struct.
pub struct EventInfo {
    // Statically defined, always the same (supports aliasing)
    // alias -> kind
    pub supported_modes: HashMap<String, WhammModeKind>,
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided
    pub requires_map_lib: bool,

    // Tied to the user script
    pub ty_info: Vec<(Expr, DataType)>,
    pub loc: Option<Location>,
    pub probe_map: HashMap<String, Vec<Box<dyn Probe>>>,
}

/// 0: Box<Self> the matched event instance
/// 3: bool, whether there were matched modes
fn event_factory<E: Event + NameOptions + FromStrWithLoc + 'static>(
    curr_events: &mut HashMap<String, Box<dyn Event>>,
    id: &mut u32,
    probe_rule: &ProbeRule,
    loc: Option<Location>,
    predicate: Option<Expr>,
    body: Option<Block>,
    printing_info: bool,
) -> (bool, bool) {
    if let Some(RulePart {
        name: event_patt,
        ty_info,
        ..
    }) = &probe_rule.event
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
            let event = curr_events.entry(m.clone()).or_insert(Box::new(E::from_str(
                &m,
                ty_info.to_owned(),
                loc.clone(),
            )));

            let found_match_for_mode =
                if let Some(RulePart { loc: mode_loc, .. }) = &probe_rule.mode {
                    event.assign_matching_modes(
                        id,
                        probe_rule,
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
                // complete probe rules.
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
fn mode_factory<M: Mode + NameOptions + FromStrWithLoc>(
    supported_modes: &HashMap<String, WhammModeKind>,
    probe_rule: &ProbeRule,
    loc: Option<Location>,
) -> Vec<Box<M>> {
    if let Some(RulePart {
        name: mode_patt,
        ty_info,
        ..
    }) = &probe_rule.mode
    {
        let mut name_options = vec![];
        for (alias, ..) in supported_modes {
            name_options.push(alias.clone());
        }

        let matches = get_matches(name_options, mode_patt);
        if matches.is_empty() {
            return vec![];
        }

        let mut modes = vec![];
        for m in matches {
            let mode_kind = supported_modes.get(&m).unwrap();
            let mode = M::from_str(&mode_kind.name(), ty_info.to_owned(), loc.clone());
            modes.push(Box::new(mode));
        }

        modes
    } else {
        vec![]
    }
}
fn print_mode_docs<M: Mode>(
    alias: Option<&String>,
    mode: &M,
    print_globals: bool,
    print_functions: bool,
    tabs: &mut usize,
    buffer: &mut Buffer,
) {
    let name = match alias {
        Some(alias) => alias.clone(),
        None => mode.name(),
    };
    let docs = mode.docs();

    if name.is_empty() {
        return;
    }
    magenta_italics(true, format!("    {}", name), buffer);
    white(true, " mode\n".to_string(), buffer);

    // Print the mode description
    *tabs += 2;
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
    *tabs -= 2;
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
impl FromStrWithLoc for WhammProvider {
    fn from_str(name: &str, _ty_info: Vec<(Expr, DataType)>, loc: Option<Location>) -> Self {
        match name {
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
        let mut globals = HashMap::new();
        globals.insert(
            "fid".to_string(),
            ProvidedGlobal::new(
                "fid".to_string(),
                "The ID of the function the probe is located in (zero-based indexing).".to_string(),
                DataType::U32,
                None,
                true,
            ),
        );
        globals.insert(
            "fname".to_string(),
            ProvidedGlobal::new(
                "fname".to_string(),
                "The name of the function the probe is located in. Empty string if not defined."
                    .to_string(),
                DataType::Str,
                None,
                true,
            ),
        );
        globals.insert(
            "pc".to_string(),
            ProvidedGlobal::new(
                "pc".to_string(),
                "The instruction offset of the probe within the function (zero-based indexing)."
                    .to_string(),
                DataType::U32,
                None,
                true,
            ),
        );
        // Don't think we need this right now...
        // globals.insert(
        //     "wasm_bytecode_loc".to_string(),
        //     ProvidedGlobal::new(
        //         "wasm_bytecode_loc".to_string(),
        //         "A unique identifier tied to the probe's location in the Wasm bytecode."
        //             .to_string(),
        //         DataType::U32,
        //         true,
        //     )
        // );
        Self {
            kind: WhammProviderKind::Wasm,
            info: ProviderInfo {
                docs:
                    "This provides various events to instrument that are specific to WebAssembly."
                        .to_string(),
                fns: vec![],
                globals,
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
    fn requires_map_lib(&self) -> bool {
        false
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

    fn print_event_and_mode_docs(
        &self,
        probe_rule: &ProbeRule,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., package) in self.info.packages.iter() {
            package.print_event_and_mode_docs(
                probe_rule,
                print_globals,
                print_functions,
                tabs,
                buffer,
            );
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
        id: &mut u32,
        probe_spec: &ProbeRule,
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
                id,
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
                id,
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
                   probed event (if the predicate evaluates to `true`). \
                   For block-structured opcodes, the probe will be injected after the 'end' of the block."
    Alt, alt, "This mode will cause the instrumentation logic to run *instead of* the \
               probed event (if the predicate evaluates to `true`)."

    // special modes
    SemanticAfter, semantic_after, "This mode will cause the instrumentation logic to run \
                                    *semantically after* the instrumented location, meaning \
                                    it will find the target point in the bytecode that will be executed \
                                    *after* the point is reached (consider blocks and br* opcodes)."
    Entry, entry, "This mode will cause the instrumentation logic to run *on entry* to the \
                   instrumentation point (e.g. functions bodies, blocks, etc.)."
    Exit, exit, "This mode will cause the instrumentation logic to run *on exiting* to the \
                 instrumentation point (e.g. function bodies, blocks, etc.)."
    BlockAlt, block_alt, "This mode will cause the instrumentation logic to run *instead of* the\
                          probed block (if the predicate evaluates to `true`)."
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
/// Specify an Option for immediates, Some(vec![]) means we have none, None means we don't know how many there are.
/// Expected inputs:
/// IdentifierName, common_name, category: OpcodeCategory, num_args: Option<Vec<DataType>>, imms: Vec<DataType>, globals: HashMap<String, ProvidedGlobal>, fns: Vec<ProvidedFunction>, supported_modes: Vec<WhammModeKind>, req_map: bool, docs: &str
#[macro_export]
macro_rules! for_each_opcode {
($mac:ident) => { $mac! {
    Unreachable, Misc, unreachable, Some(vec![]), vec![], HashMap::new(), vec![], HashMap::from([(WhammModeKind::Before.name(), WhammModeKind::Before), (WhammModeKind::Alt.name(), WhammModeKind::Alt)]), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable"
    Nop, Misc, nop, Some(vec![]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://www.w3.org/TR/wasm-core-2/#syntax-instr-control"
    // TODO -- support blockty as a struct to read/manipulate (provided global?)
    //         Block { blockty: $crate::BlockType } => visit_block
    //         Loop { blockty: $crate::BlockType } => visit_loop
    //         If { blockty: $crate::BlockType } => visit_if
    Block, Control, block, None, vec![], HashMap::new(), vec![], OpcodeEvent::block_type_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block"
    Loop, Control, _loop, None, vec![], HashMap::new(), vec![], OpcodeEvent::block_type_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop"
    If, Control, _if, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], OpcodeEvent::block_type_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else"
    Else, Control, _else, Some(vec![]), vec![], HashMap::new(), vec![], OpcodeEvent::block_type_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else"
    // TODO -- support pulling immediates
    //         TryTable { try_table: $crate::TryTable } => visit_try_table
    TryTable, Table, try_table, Some(vec![]), vec![], HashMap::new(), vec![], OpcodeEvent::block_type_modes(), false, "https://github.com/WebAssembly/exception-handling/blob/e7c7c313d26f6b0fe8f1bda33cd6ab5e9edd838b/proposals/exception-handling/Exceptions.md#try-blocks"
    Throw, Exn, throw, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/exception-handling/blob/e7c7c313d26f6b0fe8f1bda33cd6ab5e9edd838b/proposals/exception-handling/Exceptions.md#throwing-an-exception"
    // TODO -- support exnref
    ThrowRef, Exn, throw_ref, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/exception-handling/blob/e7c7c313d26f6b0fe8f1bda33cd6ab5e9edd838b/proposals/exception-handling/Exceptions.md#rethrowing-an-exception"
    End, Control, end, Some(vec![]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes_no_alt(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/end"
    Br, Control, br, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], OpcodeEvent::branching_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
    BrIf, Control, br_if, Some(vec![DataType::I32]), vec![(DataType::U32, 1)], HashMap::new(), vec![], OpcodeEvent::branching_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
    BrTable, Control, br_table, Some(vec![DataType::U32]), vec![(DataType::U32, -1)], get_br_table_globals(), vec![], OpcodeEvent::branching_modes(), true, "https://musteresel.github.io/posts/2020/01/webassembly-text-br_table-example.html"
    Return, Control, _return, Some(vec![]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return"

    Call, Control, call, None, vec![(DataType::U32, 1)], get_call_globals(), get_call_fns(), WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call"
    CallIndirect, Control, call_indirect, None, vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call"
    ReturnCall, Control, return_call, None, vec![(DataType::U32, 1)], get_call_globals(), get_call_fns(), WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call"
    ReturnCallIndirect, Control, return_call_indirect, None, vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call"

    Drop, Misc, drop, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Drop"
    Select, Control, select, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select"
    // TODO -- support pulling the type!
    //         TypedSelect { ty: $crate::ValType } => visit_typed_select
    TypedSelect, Control, typed_select, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select"
    LocalGet, Local, local_get, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_get"
    LocalSet, Local, local_set, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_set"
    LocalTee, Local, local_tee, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee"
    GlobalGet, Global, global_get, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_get"
    GlobalSet, Global, global_set, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_set"
    I32Load, Load, i32_load, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load, Load, i64_load, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    F32Load, Load, f32_load, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I32Load8S, Load, i32_load8_s, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I32Load8U, Load, i32_load8_u, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I32Load16S, Load, i32_load16_s, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I32Load16U, Load, i32_load16_u, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load8S, Load, i64_load8_s, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load8U, Load, i64_load8_u, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load16S, Load, i64_load16_s, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load16U, Load, i64_load16_u, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load32S, Load, i64_load32_s, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I64Load32U, Load, i64_load32_u, Some(vec![DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
    I32Store, Store, i32_store, Some(vec![DataType::I32, DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I64Store, Store, i64_store, Some(vec![DataType::I64, DataType::U32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    F32Store, Store, f32_store, Some(vec![DataType::F32, DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    F64Store, Store, f64_store, Some(vec![DataType::F64, DataType::U32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I32Store8, Store, i32_store8, Some(vec![DataType::I32, DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I32Store16, Store, i32_store16, Some(vec![DataType::I32, DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I64Store8, Store, i64_store8, Some(vec![DataType::I64, DataType::U32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I64Store16, Store, i64_store16, Some(vec![DataType::I64, DataType::U32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    I64Store32, Store, i64_store32, Some(vec![DataType::I64, DataType::U32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
    MemorySize, Memory, memory_size, Some(vec![]), vec![(DataType::U32, 1)], vec![], vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Size"
    MemoryGrow, Memory, memory_grow, Some(vec![DataType::U32]), vec![(DataType::U32, 1)], vec![], vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Grow"
    I32Const, Const, i32_const, Some(vec![]), vec![(DataType::I32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const"
    I64Const, Const, i64_const, Some(vec![]), vec![(DataType::I64, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const"
    F32Const, Const, f32_const, Some(vec![]), vec![(DataType::F32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const"
    F64Const, Const, f64_const, Some(vec![]), vec![(DataType::F64, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const"
    // TODO -- support pulling heaptype
    //     RefNull { hty: $crate::HeapType } => visit_ref_null
    RefNull, Gc, ref_null, Some(vec![]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    // TODO -- support argN
    RefIsNull, Gc, ref_is_null, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    RefFunc, Gc, ref_func, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    RefEq, Gc, ref_eq, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    I32Eqz, Compare, i32_eqz, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    I32Eq, Compare, i32_eq, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    I32Ne, Compare, i32_ne, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Not_equal"
    I32LtS, Compare, i32_lt_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    I32LtU, Compare, i32_lt_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    I32GtS, Compare, i32_gt_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    I32GtU, Compare, i32_gt_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    I32LeS, Compare, i32_le_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    I32LeU, Compare, i32_le_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    I32GeS, Compare, i32_ge_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"
    I32GeU, Compare, i32_ge_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"

    I64Eqz, Compare, i64_eqz, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    I64Eq, Compare, i64_eq, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    I64Ne, Compare, i64_ne, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Not_equal"
    I64LtS, Compare, i64_lt_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    I64LtU, Compare, i64_lt_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    I64GtS, Compare, i64_gt_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    I64GtU, Compare, i64_gt_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    I64LeS, Compare, i64_le_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    I64LeU, Compare, i64_le_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    I64GeS, Compare, i64_ge_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"
    I64GeU, Compare, i64_ge_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"

    F32Eq, Compare, f32_eq, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    F32Ne, Compare, f32_ne, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Not_equal"
    F32Lt, Compare, f32_lt, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    F32Gt, Compare, f32_gt, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    F32Le, Compare, f32_le, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    F32Ge, Compare, f32_ge, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"

    F64Eq, Compare, f64_eq, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Equal"
    F64Ne, Compare, f64_ne, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Not_equal"
    F64Lt, Compare, f64_lt, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_than"
    F64Gt, Compare, f64_gt, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_than"
    F64Le, Compare, f64_le, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Less_or_equal"
    F64Ge, Compare, f64_ge, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Greater_or_equal"

    I32Clz, Arith, i32_clz, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Count_leading_zeros"
    I32Ctz, Arith, i32_ctz, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Count_trailing_zeros"
    I32Popcnt, Arith, i32_popcnt, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Population_count"
    I32Add, Arith, i32_add, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Addition"
    I32Sub, Arith, i32_sub, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Subtraction"
    I32Mul, Arith, i32_mul, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Multiplication"
    I32DivS, Arith, i32_div_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    I32DivU, Arith, i32_div_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    I32RemS, Arith, i32_rem_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Remainder"
    I32RemU, Arith, i32_rem_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Remainder"
    I32And, Arith, i32_and, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/AND"
    I32Or, Arith, i32_or, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/OR"
    I32Xor, Arith, i32_xor, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/XOR"
    I32Shl, Arith, i32_shl, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Left_shift"
    I32ShrS, Arith, i32_shr_s, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_shift"
    I32ShrU, Arith, i32_shr_u, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_shift"
    I32Rotl, Arith, i32_rotl, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Left_rotate"
    I32Rotr, Arith, i32_rotr, Some(vec![DataType::I32, DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_rotate"

    I64Clz, Arith, i64_clz, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Count_leading_zeros"
    I64Ctz, Arith, i64_ctz, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Count_trailing_zeros"
    I64Popcnt, Arith, i64_popcnt, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Population_count"
    I64Add, Arith, i64_add, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Addition"
    I64Sub, Arith, i64_sub, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Subtraction"
    I64Mul, Arith, i64_mul, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Multiplication"
    I64DivS, Arith, i64_div_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    I64DivU, Arith, i64_div_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    I64RemS, Arith, i64_rem_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Remainder"
    I64RemU, Arith, i64_rem_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Remainder"
    I64And, Arith, i64_and, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/AND"
    I64Or, Arith, i64_or, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/OR"
    I64Xor, Arith, i64_xor, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/XOR"
    I64Shl, Arith, i64_shl, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Left_shift"
    I64ShrS, Arith, i64_shr_s, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_shift"
    I64ShrU, Arith, i64_shr_u, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_shift"
    I64Rotl, Arith, i64_rotl, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Left_rotate"
    I64Rotr, Arith, i64_rotr, Some(vec![DataType::I64, DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Right_rotate"

    F32Abs, Arith, f32_abs, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Absolute"
    F32Neg, Arith, f32_neg, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Negate"
    F32Ceil, Arith, f32_ceil, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Ceil"
    F32Floor, Arith, f32_floor, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Floor"
    F32Trunc, Arith, f32_trunc, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_float"
    F32Nearest, Arith, f32_nearest, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Nearest"
    F32Sqrt, Arith, f32_sqrt, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Square_root"
    F32Add, Arith, f32_add, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Addition"
    F32Sub, Arith, f32_sub, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Subtraction"
    F32Mul, Arith, f32_mul, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Multiplication"
    F32Div, Arith, f32_div, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    F32Min, Arith, f32_min, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Min"
    F32Max, Arith, f32_max, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Max"
    F32Copysign, Arith, f32_copysign, Some(vec![DataType::F32, DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Copy_sign"

    F64Abs, Arith, f64_abs, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Absolute"
    F64Neg, Arith, f64_neg, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Negate"
    F64Ceil, Arith, f64_ceil, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Ceil"
    F64Floor, Arith, f64_floor, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Floor"
    F64Trunc, Arith, f64_trunc, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_float"
    F64Nearest, Arith, f64_nearest, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Nearest"
    F64Sqrt, Arith, f64_sqrt, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Square_root"
    F64Add, Arith, f64_add, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Addition"
    F64Sub, Arith, f64_sub, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Subtraction"
    F64Mul, Arith, f64_mul, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Multiplication"
    F64Div, Arith, f64_div, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Division"
    F64Min, Arith, f64_min, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Min"
    F64Max, Arith, f64_max, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Max"
    F64Copysign, Arith, f64_copysign, Some(vec![DataType::F64, DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Copy_sign"

    I32WrapI64, Convert, i32_wrap_i64, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Wrap"
    I32TruncF32S, Convert, i32_trunc_f32_s, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"
    I32TruncF32U, Convert, i32_trunc_f32_u, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"
    I32TruncF64S, Convert, i32_trunc_f64_s, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"
    I32TruncF64U, Convert, i32_trunc_f64_u, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"

    I64ExtendI32S, Convert, i64_extend_i32_s, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I64ExtendI32U, Convert, i64_extend_i32_u, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I64TruncF32S, Convert, i64_trunc_f32_s, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"
    I64TruncF32U, Convert, i64_trunc_f32_u, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Truncate_float_to_int"

    F32ConvertI32S, Convert, f32_convert_i32_s, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F32ConvertI32U, Convert, f32_convert_i32_u, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F32ConvertI64S, Convert, f32_convert_i64_s, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F32ConvertI64U, Convert, f32_convert_i64_u, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F32DemoteF64, Convert, f32_demote_f64, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Demote"

    F64ConvertI32S, Convert, f64_convert_i32_s, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F64ConvertI32U, Convert, f64_convert_i32_u, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F64ConvertI64S, Convert, f64_convert_i64_s, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F64ConvertI64U, Convert, f64_convert_i64_u, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Convert"
    F64PromoteF32, Convert, f64_promote_f32, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Promote"

    I32ReinterpretF32, Convert, i32_reinterpret_f32, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Reinterpret"
    I64ReinterpretF64, Convert, i64_reinterpret_f64, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Reinterpret"
    F32ReinterpretI32, Convert, f32_reinterpret_i32, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Reinterpret"
    F64ReinterpretI64, Convert, f64_reinterpret_i64, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Reinterpret"

    I32Extend8S, Convert, i32_extend8_s, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I32Extend16S, Convert, i32_extend16_s, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I64Extend8S, Convert, i64_extend8_s, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I64Extend16S, Convert, i64_extend16_s, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"
    I64Extend32S, Convert, i64_extend32_s, Some(vec![DataType::I64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Extend"

    // 0xFB prefixed operators
    // Garbage Collection
    // http://github.com/WebAssembly/gc
    StructNew, Gc, struct_new, None, vec![(DataType::U32, 1)], get_struct_globals(true, false), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    StructNewDefault, Gc, struct_new_default, Some(vec![]), vec![(DataType::U32, 1)], get_struct_globals(false, false), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- there's no support for this arg type at the moment
    StructGet, Gc, struct_get, None, vec![(DataType::U32, 2)], get_struct_globals(true, true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- there's no support for this arg type at the moment
    StructGetS, Gc, struct_get_s, None, vec![(DataType::U32, 2)], get_struct_globals(true, true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- there's no support for this arg type at the moment
    StructGetU, Gc, struct_get_u, None, vec![(DataType::U32, 2)], get_struct_globals(true, true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- there's no support for this arg type at the moment
    StructSet, Gc, struct_set, None, vec![(DataType::U32, 2)], get_struct_globals(true, true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"

    ArrayNew, Gc, array_new, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayNewDefault, Gc, array_new_default, Some(vec![DataType::I32]), vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayNewFixed, Gc, array_new_fixed, None, vec![(DataType::U32, 2)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayNewData, Gc, array_new_data, Some(vec![DataType::I32, DataType::I32]), vec![(DataType::U32, 2)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayNewElem, Gc, array_new_elem, Some(vec![DataType::I32, DataType::I32]), vec![(DataType::U32, 2)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- there's no support for this arg type at the moment
    ArrayGet, Gc, array_get, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayGetS, Gc, array_get_s, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayGetU, Gc, array_get_u, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArraySet, Gc, array_set, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayLen, Gc, array_len, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayFill, Gc, array_fill, None, vec![(DataType::U32, 1)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayCopy, Gc, array_copy, None, vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayInitData, Gc, array_init_data, None, vec![(DataType::U32, 2)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    ArrayInitElem, Gc, array_init_elem, None, vec![(DataType::U32, 2)], get_array_globals(true), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"

    // TODO -- support immN and argN types
    // RefTestNonNull, RefTestNullable
    RefTest, Gc, ref_test, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support immN and argN types
    // RefCastNonNull, RefCastNullable
    RefCast, Gc, ref_cast, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support immN and argN types
    BrOnCast, Gc, br_on_cast, None, vec![(DataType::U32, 1), (DataType::Unknown, 2)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    BrOnCastFail, Gc, br_on_cast_fail, None, vec![(DataType::U32, 1), (DataType::Unknown, 2)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support argN types
    AnyConvertExtern, Gc, any_convert_extern, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support argN types
    ExternConvertAny, Gc, extern_convert_any, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    RefI31, Gc, ref_i31, Some(vec![DataType::I32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support argN types
    I31GetS, Gc, i31_get_s, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"
    // TODO -- support argN types
    I31GetU, Gc, i31_get_u, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/gc/MVP.md"

    // 0xFC operators
    // Non-trapping Float-to-int Conversions
    // https://github.com/WebAssembly/nontrapping-float-to-int-conversions
    I32TruncSatF32S, Convert, i32_trunc_sat_f32_s, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I32TruncSatF32U, Convert, i32_trunc_sat_f32_u, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I32TruncSatF64S, Convert, i32_trunc_sat_f64_s, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I32TruncSatF64U, Convert, i32_trunc_sat_f64_u, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"

    I64TruncSatF32S, Convert, i64_trunc_sat_f32_s, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I64TruncSatF32U, Convert, i64_trunc_sat_f32_u, Some(vec![DataType::F32]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I64TruncSatF64S, Convert, i64_trunc_sat_f64_s, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    I64TruncSatF64U, Convert, i64_trunc_sat_f64_u, Some(vec![DataType::F64]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"

    // 0xFC prefixed operators
    // bulk memory operations
    // https://github.com/WebAssembly/bulk-memory-operations
    MemoryInit, Memory, memory_init, Some(vec![]), vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    MemoryCopy, Memory, memory_copy, Some(vec![DataType::I32, DataType::I32, DataType::I32]), vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy"
    MemoryFill, Memory, memory_fill, Some(vec![DataType::I32, DataType::I32, DataType::I32]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Fill"
    DataDrop, Memory, data_drop, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"

    ElemDrop, Memory, elem_drop, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://pengowray.github.io/wasm-ops/"
    TableCopy, Table, table_copy, Some(vec![DataType::I32, DataType::I32, DataType::I32]), vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    TableInit, Table, table_init, Some(vec![DataType::I32, DataType::I32, DataType::I32]), vec![(DataType::U32, 2)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"

    // 0xFC prefixed operators
    // reference-types
    // https://github.com/WebAssembly/reference-types
    TableFill, Table, table_fill, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    TableGet, Table, table_get, Some(vec![DataType::I32]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    TableSet, Table, table_set, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    TableGrow, Table, table_grow, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    TableSize, Table, table_size, Some(vec![]), vec![(DataType::U32, 1)], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"

    // 0xFE prefixed operators
    // threads
    // https://github.com/WebAssembly/threads
    MemoryAtomicNotify, Atomic, memory_atomic_notify, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(None), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    MemoryAtomicWait32, Atomic, memory_atomic_wait32, Some(vec![DataType::I32, DataType::I32, DataType::I64]), vec![], get_memarg_globals(None), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    MemoryAtomicWait64, Atomic, memory_atomic_wait64, Some(vec![DataType::I32, DataType::I64, DataType::I64]), vec![], get_memarg_globals(None), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    AtomicFence, Atomic, atomic_fence, Some(vec![]), vec![], HashMap::new(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicLoad, Atomic, i32_atomic_load, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicLoad, Atomic, i64_atomic_load, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicLoad8U, Atomic, i32_atomic_load8_u, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicLoad16U, Atomic, i32_atomic_load16_u, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicLoad8U, Atomic, i64_atomic_load8_u, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicLoad16U, Atomic, i64_atomic_load16_u, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicLoad32U, Atomic, i64_atomic_load32_u, Some(vec![DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicStore, Atomic, i32_atomic_store, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicStore8, Atomic, i32_atomic_store8, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicStore16, Atomic, i32_atomic_store16, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicStore, Atomic, i64_atomic_store, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicStore8, Atomic, i64_atomic_store8, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicStore16, Atomic, i64_atomic_store16, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicStore32, Atomic, i64_atomic_store32, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwAdd, Atomic, i32_atomic_rmw_add, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8AddU, Atomic, i32_atomic_rmw8_add_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16AddU, Atomic, i32_atomic_rmw16_add_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwAdd, Atomic, i64_atomic_rmw_add, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8AddU, Atomic, i64_atomic_rmw8_add_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16AddU, Atomic, i64_atomic_rmw16_add_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32AddU, Atomic, i64_atomic_rmw32_add_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwSub, Atomic, i32_atomic_rmw_sub, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8SubU, Atomic, i32_atomic_rmw8_sub_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16SubU, Atomic, i32_atomic_rmw16_sub_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwSub, Atomic, i64_atomic_rmw_sub, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8SubU, Atomic, i64_atomic_rmw8_sub_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16SubU, Atomic, i64_atomic_rmw16_sub_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32SubU, Atomic, i64_atomic_rmw32_sub_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwAnd, Atomic, i32_atomic_rmw_and, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8AndU, Atomic, i32_atomic_rmw8_and_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16AndU, Atomic, i32_atomic_rmw16_and_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwAnd, Atomic, i64_atomic_rmw_and, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8AndU, Atomic, i64_atomic_rmw8_and_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16AndU, Atomic, i64_atomic_rmw16_and_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32AndU, Atomic, i64_atomic_rmw32_and_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwOr, Atomic, i32_atomic_rmw_or, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8OrU, Atomic, i32_atomic_rmw8_or_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16OrU, Atomic, i32_atomic_rmw16_or_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwOr, Atomic, i64_atomic_rmw_or, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8OrU, Atomic, i64_atomic_rmw8_or_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16OrU, Atomic, i64_atomic_rmw16_or_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32OrU, Atomic, i64_atomic_rmw32_or_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwXor, Atomic, i32_atomic_rmw_xor, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8XorU, Atomic, i32_atomic_rmw8_xor_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16XorU, Atomic, i32_atomic_rmw16_xor_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwXor, Atomic, i64_atomic_rmw_xor, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8XorU, Atomic, i64_atomic_rmw8_xor_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16XorU, Atomic, i64_atomic_rmw16_xor_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32XorU, Atomic, i64_atomic_rmw32_xor_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwXchg, Atomic, i32_atomic_rmw_xchg, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8XchgU, Atomic, i32_atomic_rmw8_xchg_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16XchgU, Atomic, i32_atomic_rmw16_xchg_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwXchg, Atomic, i64_atomic_rmw_xchg, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8XchgU, Atomic, i64_atomic_rmw8_xchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16XchgU, Atomic, i64_atomic_rmw16_xchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32XchgU, Atomic, i64_atomic_rmw32_xchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    I32AtomicRmwCmpxchg, Atomic, i32_atomic_rmw_cmpxchg, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw8CmpxchgU, Atomic, i32_atomic_rmw8_cmpxchg_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I32AtomicRmw16CmpxchgU, Atomic, i32_atomic_rmw16_cmpxchg_u, Some(vec![DataType::I32, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmwCmpxchg, Atomic, i64_atomic_rmw_cmpxchg, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(8)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw8CmpxchgU, Atomic, i64_atomic_rmw8_cmpxchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(1)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw16CmpxchgU, Atomic, i64_atomic_rmw16_cmpxchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(2)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"
    I64AtomicRmw32CmpxchgU, Atomic, i64_atomic_rmw32_cmpxchg_u, Some(vec![DataType::I64, DataType::I32]), vec![], get_memarg_globals(Some(4)), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md"

    // Typed Function references
    CallRef, Control, call_ref, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    ReturnCallRef, Control, return_call_ref, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/reference-types/blob/master/proposals/reference-types/Overview.md"
    RefAsNonNull, Convert, ref_as_non_null, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/function-references/Overview.md"
    BrOnNull, Control, br_on_null, None, vec![], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/function-references/Overview.md"
    BrOnNonNull, Control, br_on_non_null, None, vec![(DataType::U32, 1)], get_unknown_args_globals(), vec![], WhammModeKind::default_modes(), false, "https://github.com/WebAssembly/gc/blob/main/proposals/function-references/Overview.md"

    // Also 0xFE prefixed operators
    // shared-everything threads
    // https://github.com/WebAssembly/shared-everything-threads
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
}};}

// ============================================
// ==== Getters for provided functionality ====
// ============================================

// (keeps the `for_each_opcode!` lines shorter)
const UNKNOWN_ARGS: &str = "arg[0:9]+";
pub const UNKNOWN_IMMS: &str = "imm[0:9]+";

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
            None,
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
            None,
            true,
        ),
    );
    globals.insert(
        "target_fn_name".to_string(),
        ProvidedGlobal::new(
            "target_fn_name".to_string(),
            "The function name of the call target. \
                        Local functions get this from the custom section, imports from the import name. \
                        To improve performance for imports, pair with `target_fn_type == \"import\"` \
                        for faster short-circuiting."
                .to_string(),
            DataType::Str,
            None,
            true,
        ),
    );
    globals.extend(get_unknown_args_globals());

    globals
}

pub fn get_struct_globals(
    include_args: bool,
    include_field: bool,
) -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();
    globals.insert(
        "tid".to_string(),
        ProvidedGlobal::new(
            "tid".to_string(),
            "The type ID of this struct.".to_string(),
            DataType::U32,
            None,
            true,
        ),
    );

    if include_field {
        globals.insert(
            "field_idx".to_string(),
            ProvidedGlobal::new(
                "field_idx".to_string(),
                "The index of the struct field being accessed.".to_string(),
                DataType::U32,
                None,
                true,
            ),
        );
    }

    if include_args {
        globals.extend(get_unknown_args_globals());
    }

    globals
}

pub fn get_array_globals(include_args: bool) -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();
    globals.insert(
        "tid".to_string(),
        ProvidedGlobal::new(
            "tid".to_string(),
            "The type ID of this array.".to_string(),
            DataType::U32,
            None,
            true,
        ),
    );

    if include_args {
        globals.extend(get_unknown_args_globals());
    }

    globals
}

pub fn get_unknown_args_globals() -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();

    globals.insert(
        UNKNOWN_ARGS.to_string(),
        ProvidedGlobal::new(
            UNKNOWN_ARGS.to_string(),
            "The argument to the call at the specific index, e.g. [0:9]+.\
                Keep in mind, the number of arguments to a call changes based on the targeted function.".to_string(),
            DataType::Unknown,
            None,
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
                definition: Definition::CompilerDynamic,
                name: "func_id".to_string(),
                loc: None,
            },
            DataType::I32,
        )],
        DataType::Tuple { ty_info: vec![] },
        true,
        ReqArgs::All
    ), ProvidedFunction::new(
        "alt_call_by_name".to_string(),
        "Insert an alternate call (targeting the passed function name) into the Wasm bytecode. Will also emit the original parameters onto the stack.".to_string(),
        vec![(
            Expr::VarId {
                definition: Definition::CompilerDynamic,
                name: "func_name".to_string(),
                loc: None,
            },
            DataType::Str,
        )],
        DataType::Tuple { ty_info: vec![] },
        true,
        ReqArgs::All
    )]
}

pub fn get_memarg_globals(data_size: Option<u8>) -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();
    globals.insert(
        "align".to_string(),
        ProvidedGlobal::new(
            "align".to_string(),
            "The alignment of the load.".to_string(),
            DataType::U32,
            None,
            true,
        ),
    );
    globals.insert(
        "offset".to_string(),
        ProvidedGlobal::new(
            "offset".to_string(),
            "The static offset of the load's target address.".to_string(),
            DataType::U64,
            None,
            true,
        ),
    );
    globals.insert(
        "memory".to_string(),
        ProvidedGlobal::new(
            "memory".to_string(),
            "The ID of memory to load from.".to_string(),
            DataType::U32,
            None,
            true,
        ),
    );
    globals.insert(
        "effective_addr".to_string(),
        ProvidedGlobal::new(
            "effective_addr".to_string(),
            "The address in memory that will be loaded from, shorthand for: `arg0 + offset`"
                .to_string(),
            DataType::U32,
            None,
            false,
        ),
    );
    if let Some(data_size) = data_size {
        globals.insert(
            "data_size".to_string(),
            ProvidedGlobal::new(
                "data_size".to_string(),
                "The number of bytes that the memory operation is on, e.g. i32.load8_u is 1"
                    .to_string(),
                DataType::U32,
                Some(Value::gen_u8(data_size)),
                false,
            ),
        );
    }

    globals
}

pub fn get_br_table_globals() -> HashMap<String, ProvidedGlobal> {
    let mut globals = HashMap::new();

    // add in the extra globals (that aren't args or immediates)
    globals.insert(
        UNKNOWN_IMMS.to_string(),
        ProvidedGlobal::new(
            UNKNOWN_IMMS.to_string(),
            "The immediate to the opcode at the specific index, e.g. [0:9]+, not including the default target.\
            Keep in mind, the number of immediates on the br_table is specific to the instruction.".to_string(),
            DataType::U32,
            None,
            true
        )
    );
    globals.insert(
        "num_targets".to_string(),
        ProvidedGlobal::new(
            "num_targets".to_string(),
            "The number of NON-DEFAULT target branches for this br_table instruction (correlates with the number of immediates, e.g. `immN`).\
             This means the total number of targets is really num_targets + 1, to include the default target. \
             NOTE: This can be used in a predicate to ensure that the current br_table has the immN you need to interact with for the probe."
                .to_string(),
            DataType::U32,
            None,
            true,
        ),
    );
    globals.insert(
        "targets".to_string(),
        ProvidedGlobal::new(
            "targets".to_string(),
            "The NON-DEFAULT target branches for this br_table instruction represented as a map. \
             The map follows the pattern: [0->imm0, 1->imm1, .. N->immN]."
                .to_string(),
            DataType::Map {
                key_ty: Box::new(DataType::U32),
                val_ty: Box::new(DataType::U32),
            },
            None,
            false,
        ),
    );
    globals.insert(
        "default_target".to_string(),
        ProvidedGlobal::new(
            "default_target".to_string(),
            "The default target of this br_table instruction.".to_string(),
            DataType::U32,
            None,
            true,
        ),
    );
    globals
}
