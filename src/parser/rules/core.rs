use crate::parser::rules::{
    event_factory, print_mode_docs, Event, EventInfo, FromStr, Mode, ModeInfo,
    NameOptions, Package, PackageInfo, Probe,
};
use crate::parser::types::{Block, Expr, Location, ProbeSpec, ProvidedFunction, ProvidedGlobal};
use std::collections::HashMap;
use termcolor::Buffer;
use crate::for_each_mode;

pub enum CorePackageKind {
    Default,
}
impl CorePackageKind {
    fn name(&self) -> String {
        match self {
            Self::Default => "".to_string(),
        }
    }
}

pub struct CorePackage {
    kind: CorePackageKind,
    info: PackageInfo,
}
impl NameOptions for CorePackage {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec!["default".to_string()]
    }
}
impl FromStr for CorePackage {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "default" => Self::default(loc),
            _ => panic!("unsupported CorePackage: {name}"),
        }
    }
}
impl CorePackage {
    // ======================
    // ---- Constructors ----
    // ======================

    fn default(_loc: Option<Location>) -> Self {
        Self {
            kind: CorePackageKind::Default,
            info: PackageInfo {
                docs: "".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc: None,
                events: HashMap::new(),
            },
        }
    }
}
impl Package for CorePackage {
    fn name(&self) -> String {
        self.kind.name()
    }

    fn loc(&self) -> &Option<Location> {
        &self.info.loc
    }

    fn docs(&self) -> &String {
        &self.info.docs
    }

    fn has_events(&self) -> bool {
        !self.info.events.is_empty()
    }

    fn len_events(&self) -> usize {
        self.info.events.len()
    }

    fn events(&self) -> Box<dyn Iterator<Item = &dyn Event> + '_> {
        Box::new(self.info.events.values().map(|e| e.as_ref() as &dyn Event))
    }

    fn events_mut(&mut self) -> Box<dyn Iterator<Item = &mut dyn Event> + '_> {
        Box::new(
            self.info
                .events
                .values_mut()
                .map(|e| e.as_mut() as &mut dyn Event),
        )
    }

    fn print_event_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., event) in self.info.events.iter() {
            crate::parser::rules::print_event_docs(
                event,
                print_globals,
                print_functions,
                tabs,
                buffer,
            );
        }
    }

    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., event) in self.info.events.iter() {
            event.print_mode_docs(print_globals, print_functions, tabs, buffer);
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

    fn assign_matching_events(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
        printing_info: bool,
    ) -> (bool, bool) {
        match self {
            Self {
                kind: CorePackageKind::Default,
                ..
            } => event_factory::<CoreEvent>(
                &mut self.info.events,
                probe_spec,
                loc,
                predicate,
                body,
                printing_info,
            ),
        }
    }
}

pub enum CoreEventKind {
    Default,
}
impl CoreEventKind {
    fn name(&self) -> String {
        match self {
            Self::Default => "".to_string(),
        }
    }
}

pub struct CoreEvent {
    kind: CoreEventKind,
    info: EventInfo,
}
impl NameOptions for CoreEvent {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec!["default".to_string()]
    }
}
impl FromStr for CoreEvent {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "default" => Self::default(loc),
            _ => panic!("unsupported CoreEvent: {name}"),
        }
    }
}
impl CoreEvent {
    // ======================
    // ---- Constructors ----
    // ======================

    fn default(_loc: Option<Location>) -> Self {
        Self {
            kind: CoreEventKind::Default,
            info: EventInfo {
                supported_modes: vec![WhammModeKind::Begin, WhammModeKind::End],
                docs: "".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc: None,
                probe_map: HashMap::new(),
            },
        }
    }
}
impl Event for CoreEvent {
    fn name(&self) -> String {
        self.kind.name()
    }

    fn supported_modes(&self) -> &Vec<WhammModeKind> {
        &self.info.supported_modes
    }

    fn loc(&self) -> &Option<Location> {
        &self.info.loc
    }

    fn docs(&self) -> &String {
        &self.info.docs
    }

    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>> {
        &self.info.probe_map
    }

    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>> {
        &mut self.info.probe_map
    }

    fn print_mode_docs(
        &self,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        for (.., probes) in self.info.probe_map.iter() {
            if let Some(probe) = probes.iter().next() {
                // only print out the docs for some probe type one time!
                probe.print_mode_docs(print_globals, print_functions, tabs, buffer);
            }
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

    // fn assign_matching_modes(
    //     &mut self,
    //     probe_spec: &ProbeSpec,
    //     loc: Option<Location>,
    //     predicate: Option<Expr>,
    //     body: Option<Block>,
    // ) -> bool {
    //     let mut matched_modes = false;
    //     let modes: Vec<Box<WhammMode>> = mode_factory(&self.info.supported_modes, probe_spec, loc.clone());
    //     let probes = self.probes_mut();
    //     for mode in modes {
    //         matched_modes = true;
    //         let modes = probes.entry(mode.name()).or_default();
    //         modes.push(Box::new(WhammProbe::new(
    //             *mode,
    //             loc.clone(),
    //             predicate.clone(),
    //             body.clone(),
    //         )));
    //     }
    //     matched_modes
    // }
}

// The supported modes
macro_rules! define_mode {
($($mode:ident, $name:ident, $docs:expr)*) => {
    /// The modes available to use as instrumentation rules.
    #[derive(Debug)]
    pub enum WhammModeKind {
        $(
            $mode,
        )*
    }
    
    impl WhammModeKind {
        pub fn name(&self) -> String {
            match self {
                $(
                    Self::$mode {..} => stringify!($name).to_string(),
                )*
            }
        }

        pub fn default_modes() -> Vec<Self> {
            vec![Self::Before, Self::After, Self::Alt]
        }
        pub fn default_modes_and_semantic_aft() -> Vec<Self> {
            let mut defaults = Self::default_modes();
            defaults.push(Self::SemanticAfter);
            defaults
        }
        pub fn block_type_modes() -> Vec<Self> {
            vec![
                Self::Before,
                Self::After,
                Self::Alt,
                Self::SemanticAfter,
                Self::Entry,
                Self::Exit
            ]
        }
        
        pub fn all_modes() -> Vec<Self> {
            vec![
                $(
                    Self::$mode,
                )*
            ]
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
            vec![
                $(stringify!($name).to_string()),*
            ]
        }
    }
    impl FromStr for WhammMode {
        fn from_str(name: String, loc: Option<Location>) -> Self {
            match name.as_str() {
                $(stringify!($name) => Self::$name(loc),)*
                 _ => panic!("unsupported WhammMode: {name}"),
            }
        }
    }
    impl WhammMode {
        // ======================
        // ---- Constructors ----
        // ======================

        $(
        fn $name(loc: Option<Location>) -> Self {
            Self {
                kind: WhammModeKind::$mode,
                info: ModeInfo {
                    docs: $docs.to_string(),
                    fns: vec![],
                    globals: HashMap::new(),
                    loc
                }
            }
        }
        )*
    }
};}
for_each_mode!(define_mode);

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
    pub body: Option<Block>,
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

    fn body(&self) -> &Option<Block> {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Option<Block> {
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
    pub(crate) fn new(
        mode: WhammMode,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> Self {
        Self {
            mode,
            loc,
            predicate,
            body,
        }
    }
}
