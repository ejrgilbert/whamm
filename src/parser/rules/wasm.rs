use crate::for_each_opcode;
use crate::parser::rules::{
    event_factory, get_br_table_globals, get_call_fns, get_call_globals, mode_factory, Event, EventInfo, FromStr, Mode,
    NameOptions, Package, PackageInfo, Probe, WhammMode, WhammProbe,
};
use crate::parser::types::{
    Block, DataType, Expr, Location, ProbeSpec, ProvidedFunction, ProvidedGlobal,
};
use std::collections::HashMap;
use termcolor::Buffer;

pub enum WasmPackageKind {
    Opcode,
}
impl WasmPackageKind {
    fn name(&self) -> String {
        match self {
            Self::Opcode => "opcode".to_string(),
        }
    }
}

pub struct WasmPackage {
    kind: WasmPackageKind,
    info: PackageInfo,
}
impl NameOptions for WasmPackage {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec!["opcode".to_string()]
    }
}
impl FromStr for WasmPackage {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "opcode" => Self::opcode(loc),
            _ => panic!("unsupported WasmPackage: {name}"),
        }
    }
}
impl WasmPackage {
    // ======================
    // ---- Constructors ----
    // ======================

    fn opcode(loc: Option<Location>) -> Self {
        Self {
            kind: WasmPackageKind::Opcode,
            info: PackageInfo {
                docs: "This package within the wasm provider contains enables the \
                    instrumentation of WebAssembly bytecode instructions."
                    .to_string(),
                fns: vec![],
                globals: HashMap::from([(
                    "wasm_bytecode_loc".to_string(),
                    ProvidedGlobal::new(
                        "wasm_bytecode_loc".to_string(),
                        "A unique identifier tied to the probe's location in the Wasm bytecode."
                            .to_string(),
                        DataType::I32,
                        true,
                    ),
                )]),
                loc,
                events: HashMap::new(),
            },
        }
    }
}
impl Package for WasmPackage {
    // ==========================
    // ---- Instance Methods ----
    // ==========================

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
                kind: WasmPackageKind::Opcode,
                ..
            } => event_factory::<OpcodeEvent>(
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

macro_rules! define_opcode {
    ($($op:ident, $name:ident, $num_args:expr, $imms:expr, $globals:expr, $fns:expr, $docs:expr)*) => {
        /// Instructions as defined [here].
        ///
        /// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
        #[derive(Debug)]
        pub enum OpcodeEventKind {
            $(
                $op {
                    num_args: u32,
                    imms: Vec<DataType>,
                },
            )*
        }
        impl OpcodeEventKind {
            pub fn name(&self) -> String {
                match self {
                    $(
                        Self::$op {..} => stringify!($name).to_string(),
                    )*
                }
            }
                        /// Only specify the number of args since the arg type
                        /// isn't necessarily consistent based on just which opcode
                        /// we're at.
                        /// (Sometimes a specific opcode's arg0 is i32, sometimes it's not)
            fn get_num_args(&self) -> &u32 {
                match self {
                    $(
                        Self::$op {num_args, ..}
                    )|* => &num_args,
                }
            }

            fn get_imms(&self) -> &Vec<DataType> {
                match self {
                    $(
                        Self::$op {imms, ..}
                    )|* => &imms,
                }
                            }
            // ======================
            // ---- Constructors ----
            // ======================

            $(
            pub fn $name() -> Self {
                Self::$op {
                    num_args: $num_args,
                    imms: $imms,
                }
                            }
            )*
        }
        pub struct OpcodeEvent {
            info: EventInfo,
            kind: OpcodeEventKind,
        }
        impl NameOptions for OpcodeEvent {
            fn get_name_options() -> Vec<String> {
                vec![
                    $(stringify!($name).to_string()),*
                ]
            }
        }
        impl FromStr for OpcodeEvent {
            fn from_str(name: String, loc: Option<Location>) -> Self {
                match name.as_str() {
                    $(stringify!($name) => Self::$name(loc),)*
                     _ => panic!("unsupported OpcodeEvent: {name}"),
                }
            }
        }
        impl OpcodeEvent {
            // =========================
            // ---- Globals Helpers ----
            // =========================

            fn init_globals(kind: &OpcodeEventKind) -> HashMap<String, ProvidedGlobal> {
                let mut globals = HashMap::new();
                Self::gen_args(&mut globals, *kind.get_num_args());
                Self::gen_immediates(&mut globals, kind.get_imms());

                globals
            }

            fn gen_args(globals: &mut HashMap<String, ProvidedGlobal>, args: u32) {
                for i in 0..args {
                    let name = format!("arg{}", i);
                    globals.insert(
                        name.clone(),
                        ProvidedGlobal::new(
                            name.to_string(),
                            format!("The argument to the opcode at index {}.", i),
                            DataType::AssumeGood,
                            false,
                        ),
                    );
                }
            }

            fn gen_immediates(globals: &mut HashMap<String, ProvidedGlobal>, imms: &[DataType]) {
                for (idx, ty) in imms.iter().enumerate() {
                    let name = format!("imm{}", idx);
                    globals.insert(
                        name.clone(),
                        ProvidedGlobal::new(
                            name.to_string(),
                            format!("The immediate to the opcode at index {}.", idx),
                            ty.clone(),
                            true,
                        ),
                    );
                }
            }

            // ======================
            // ---- Constructors ----
            // ======================

            $(
            fn $name(loc: Option<Location>) -> Self {
                let kind = OpcodeEventKind::$name();
                let mut globals = Self::init_globals(&kind);
                // todo -- add configured globals
                globals.extend($globals);
                Self {
                    kind,
                    info: EventInfo {
                        docs: $docs.to_string(),
                        fns: $fns,
                        globals,
                        loc,
                        probe_map: HashMap::new()
                    }
                }
            }
            )*
        }
    }
}
for_each_opcode!(define_opcode);

impl Event for OpcodeEvent {
    fn name(&self) -> String {
        self.kind.name()
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

    fn assign_matching_modes(
        &mut self,
        probe_spec: &ProbeSpec,
        loc: Option<Location>,
        predicate: Option<Expr>,
        body: Option<Block>,
    ) -> bool {
        let mut matched_modes = false;
        let probes = self.probes_mut();
        let modes: Vec<Box<WhammMode>> = mode_factory(probe_spec, loc.clone());
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
