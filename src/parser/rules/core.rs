use std::collections::HashMap;
use termcolor::Buffer;
use crate::common::error::WhammError;
use crate::parser::rules::{Event, event_factory, FromStr, Mode, mode_factory, ModeInfo, NameOptions, Package, print_mode_docs, Probe};
use crate::parser::types::{Expr, Location, ProbeSpec, ProvidedFunction, ProvidedGlobal, Statement};

pub enum CorePackage {
    Default {
        docs: String,
        fns: Vec<ProvidedFunction>, // Comp-provided
        globals: HashMap<String, ProvidedGlobal>, // Comp-provided
        events: HashMap<String, Box<CoreEvent>>
    }
}
impl NameOptions for CorePackage {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "default".to_string()
        ]
    }
}
impl FromStr for CorePackage {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "default" => Self::default(loc),
            _ => panic!("unsupported CorePackage: {name}")
        }
    }
}
impl CorePackage {

    // ======================
    // ---- Constructors ----
    // ======================

    fn default(_loc: Option<Location>) -> Self {
        Self::Default {
            docs: "".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            events: HashMap::new()
        }
    }
}
impl Package for CorePackage {
    fn name(&self) -> String {
        match self {
            Self::Default{..} => {
                "".to_string()
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Default {docs, ..} => {
                docs
            }
        }
    }

    fn print_event_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Default{events, ..} => {
                for (.., event) in events.iter() {
                    crate::parser::rules::print_event_docs(event.as_ref(), print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Default{events, ..} => {
                for (.., event) in events.iter() {
                    event.print_mode_docs(print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Default { fns, ..} => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Default { globals, ..} => {
                globals
            }
        }
    }

    fn assign_matching_events(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>,
                              predicate: Option<Expr>,
                              body: Option<Vec<Statement>>) -> Result<(bool, bool), Box<WhammError>> {
        match self {
            Self::Default {events, ..} => {
                Ok(event_factory(events, probe_spec, loc, predicate, body)?)
            },
        }
    }
}

pub enum CoreEvent {
    Default {
        name: String,
        docs: String,
        fns: Vec<ProvidedFunction>, // Comp-provided
        globals: HashMap<String, ProvidedGlobal>, // Comp-provided
        probe_map: HashMap<String, Vec<Box<dyn Probe>>>
    }
}
impl NameOptions for CoreEvent {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "default".to_string()
        ]
    }
}
impl FromStr for CoreEvent {
    fn from_str(name: String, _loc: Option<Location>) -> Self {
        match name.as_str() {
            "default" => Self::Default {
                name: "".to_string(),
                docs: "".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                probe_map: HashMap::new()
            },
            _ => panic!("unsupported CoreEvent: {name}")
        }
    }
}
impl Event for CoreEvent {
    fn name(&self) -> &String {
        match self {
            Self::Default{name, ..} => {
                name
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Default{docs, ..} => {
                docs
            }
        }
    }

    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>> {
        match self {
            Self::Default{probe_map, ..} => {
                probe_map
            }
        }
    }

    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>> {
        match self {
            Self::Default{probe_map, ..} => {
                probe_map
            }
        }
    }

    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Default{probe_map, ..} => {
                for (.., probes) in probe_map.iter() {
                    for probe in probes.iter() {
                        probe.print_mode_docs(print_globals, print_functions, tabs, buffer);
                    }
                }
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Default{fns, ..} => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Default{globals, ..} => {
                globals
            }
        }
    }

    fn assign_matching_modes(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>,
                             predicate: Option<Expr>,
                             body: Option<Vec<Statement>>) -> Result<bool, Box<WhammError>> {
        let mut matched_modes = false;
        match self {
            Self::Default{ref mut probe_map, ..} => {
                let modes: Vec<Box<CoreMode>> = mode_factory(probe_spec, loc.clone())?;
                for mode in modes {
                    matched_modes = true;
                    probe_map.insert(mode.name(), vec![Box::new(CoreProbe::new(*mode, loc.clone(), predicate.clone(), body.clone()))]);
                }
            }
        }
        Ok(matched_modes)
    }
}

enum CoreMode {
    Begin (ModeInfo),
    End (ModeInfo)
}
impl NameOptions for CoreMode {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "begin".to_string(),
            "end".to_string()
        ]
    }
}
impl FromStr for CoreMode {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "begin" => Self::begin(loc),
            "end" => Self::end(loc),
            _ => panic!("unsupported CoreMode: {name}")
        }
    }
}
impl CoreMode {

    // ======================
    // ---- Constructors ----
    // ======================

    fn begin(loc: Option<Location>) -> Self {
        Self::Begin ( ModeInfo {
            docs: "Run this logic on application startup.".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            loc
        })
    }
    fn end(loc: Option<Location>) -> Self {
        Self::End ( ModeInfo {
            docs: "Run this logic when the application exits.".to_string(),
            fns: vec![],
            globals: HashMap::new(),
            loc
        })
    }
}
impl Mode for CoreMode {
    fn name(&self) -> String {
        match self {
            Self::Begin(..) => {
                "begin".to_string()
            },
            Self::End(..) => {
                "end".to_string()
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Begin(ModeInfo { docs, ..}) |
            Self::End(ModeInfo { docs, ..}) => {
                docs
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Begin(ModeInfo { fns, ..}) |
            Self::End(ModeInfo { fns, ..}) => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Begin(ModeInfo { globals, ..}) |
            Self::End(ModeInfo { globals, ..}) => {
                globals
            }
        }
    }
}

struct CoreProbe {
    pub mode: CoreMode,
    pub loc: Option<Location>,

    pub predicate: Option<Expr>,
    pub body: Option<Vec<Statement>>
}
impl Probe for CoreProbe {
    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        print_mode_docs(&self.mode, print_globals, print_functions, tabs, buffer);
    }
}
impl CoreProbe {
    fn new(mode: CoreMode, loc: Option<Location>, predicate: Option<Expr>, body: Option<Vec<Statement>>) -> Self {
        Self {
            mode,
            loc,
            predicate,
            body
        }
    }
}