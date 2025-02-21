use crate::for_each_opcode;
use crate::parser::rules::{
    event_factory, get_array_globals, get_br_table_globals, get_call_fns, get_call_globals,
    get_memarg_globals, get_struct_globals, get_unknown_args_globals, Event, EventInfo,
    FromStrWithLoc, NameOptions, OpcodeCategory::*, Package, PackageInfo, Probe, WhammModeKind, UNKNOWN_ARGS,
};
use crate::parser::types::{
    Block, DataType, Expr, Location, ProbeRule, ProvidedFunction, ProvidedGlobal,
};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem::discriminant;
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
impl FromStrWithLoc for WasmPackage {
    fn from_str(name: &str, _ty_info: Vec<(Expr, DataType)>, loc: Option<Location>) -> Self {
        match name {
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
        let drop_args = ProvidedFunction::new(
            "drop_args".to_string(),
            "Drop the incoming arguments to the instrumented opcode.".to_string(),
            vec![],
            DataType::Tuple {
                // returns nothing (empty tuple)
                ty_info: vec![],
            },
            true,
        );

        Self {
            kind: WasmPackageKind::Opcode,
            info: PackageInfo {
                docs: "This package within the wasm provider contains enables the \
                    instrumentation of WebAssembly bytecode instructions."
                    .to_string(),
                fns: vec![drop_args],
                globals: HashMap::new(),
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
    fn requires_map_lib(&self) -> bool {
        false
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

    fn print_event_and_mode_docs(
        &self,
        probe_rule: &ProbeRule,
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
            event.print_mode_docs(probe_rule, print_globals, print_functions, tabs, buffer);
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
        id: &mut u32,
        probe_rule: &ProbeRule,
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
                id,
                probe_rule,
                loc,
                predicate,
                body,
                printing_info,
            ),
        }
    }
}

pub enum OpcodeCategory {
    Const,
    Misc,
    Control,
    Local,
    Global,
    Table,
    Load,
    Store,
    Memory,
    Arith,
    Compare,
    Convert,
    Exn,
    Simd,
    Ref,
    Gc,
    Atomic
}
impl Display for OpcodeCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Const => "const",
            Control => "control",
            Local => "local",
            Global => "global",
            Table => "table",
            Load => "load",
            Store => "store",
            Memory => "memory",
            Arith => "arith",
            Compare => "compare",
            Convert => "convert",
            Exn => "exn",
            Simd => "simd",
            Ref => "ref",
            Gc => "gc",
            Atomic => "atomic",
            Misc => "misc"
        };
        f.write_str(str)
    }
}

macro_rules! define_opcode {
    ($($op:ident, $category:expr, $name:ident, $args:expr, $imms:expr, $globals:expr, $fns:expr, $supported_modes:expr, $req_map:expr, $docs:expr)*) => {
        /// Instructions as defined [here].
        ///
        /// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
        #[derive(Debug)]
        pub enum OpcodeEventKind {
            $(
                $op {
                    args: Option<Vec<DataType>>,
                    // XXX: Possible issue: ALL counts must be know-able, or NONE
                    // vec![(type, count)], where count = -1, means unknown number
                    imms: Vec<(DataType, i32)>
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
            pub fn category(&self) -> OpcodeCategory {
                match self {
                    $(
                        Self::$op {..} => $category,
                    )*
                }
            }
                        /// Only specify the number of args since the arg type
                        /// isn't necessarily consistent based on just which opcode
                        /// we're at.
                        /// (Sometimes a specific opcode's arg0 is i32, sometimes it's not)
            fn get_args(&self) -> &Option<Vec<DataType>> {
                match self {
                    $(
                        Self::$op {args, ..}
                    )|* => &args,
                }
            }

            fn get_imms(&self) -> &Vec<(DataType, i32)> {
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
                    args: $args,
                    imms: $imms
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
        impl FromStrWithLoc for OpcodeEvent {
            fn from_str(name: &str, ty_info: Vec<(Expr, DataType)>, loc: Option<Location>) -> Self {
                match name {
                    $(stringify!($name) => Self::$name(ty_info, loc),)*
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
                Self::gen_args(&mut globals, kind.get_args());
                Self::gen_immediates(&mut globals, kind.get_imms());
                globals.insert(
                    "category".to_string(),
                     ProvidedGlobal::new(
                         "category".to_string(),
                         "The category of this opcode.".to_string(),
                         DataType::Str,
                         Some(crate::parser::types::Value::Str { val: kind.category().to_string() }),
                         true,
                     )
                );

                globals
            }

            fn gen_args(globals: &mut HashMap<String, ProvidedGlobal>, args: &Option<Vec<DataType>>) {
                if let Some(args) = args {
                    for (i, arg_ty) in args.iter().enumerate() {
                        let name = format!("arg{}", i);
                        globals.insert(
                            name.clone(),
                            ProvidedGlobal::new(
                                name.to_string(),
                                format!("The argument to the opcode at index {}.", i),
                                arg_ty.clone(),
                                None,
                                false,
                            ),
                        );
                    }
                } else {
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
                }
            }

            fn gen_immediates(globals: &mut HashMap<String, ProvidedGlobal>, imms: &[(DataType, i32)]) {
                for (idx, (ty, count)) in imms.iter().enumerate() {
                    if *count < 0 {
                        continue; // skip this immediate, count is unknown
                    }
                    let name = format!("imm{}", idx);
                    globals.insert(
                        name.clone(),
                        ProvidedGlobal::new(
                            name.to_string(),
                            format!("The immediate to the opcode at index {}.", idx),
                            ty.clone(),
                            None,
                            true
                        ),
                    );
                }
            }

            pub fn branching_modes() -> HashMap<String, WhammModeKind> {
                let mut defaults = WhammModeKind::default_modes();
                defaults.insert("at_target".to_string(), WhammModeKind::SemanticAfter);
                defaults
            }
            pub fn block_type_modes() -> HashMap<String, WhammModeKind> {
                HashMap::from([
                    (WhammModeKind::Before.name(), WhammModeKind::Before),
                    ("after".to_string(), WhammModeKind::SemanticAfter),
                    (WhammModeKind::Alt.name(), WhammModeKind::BlockAlt),
                    (WhammModeKind::Entry.name(), WhammModeKind::Entry),
                    (WhammModeKind::Exit.name(), WhammModeKind::Exit),
                ])
            }

            // ======================
            // ---- Constructors ----
            // ======================

            $(
            fn $name(ty_info: Vec<(Expr, DataType)>, loc: Option<Location>) -> Self {
                let kind = OpcodeEventKind::$name();
                let mut globals = Self::init_globals(&kind);
                // todo -- add configured globals
                globals.extend($globals);
                Self {
                    kind,
                    info: EventInfo {
                        supported_modes: $supported_modes,
                        docs: $docs.to_string(),
                        fns: $fns,
                        globals,
                        requires_map_lib: $req_map,
                        ty_info,
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

    fn ty_info(&self) -> &Vec<(Expr, DataType)> {
        &self.info.ty_info
    }

    fn supported_modes(&self) -> &HashMap<String, WhammModeKind> {
        &self.info.supported_modes
    }

    fn requires_map_lib(&self) -> bool {
        self.info.requires_map_lib
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
        probe_rule: &ProbeRule,
        print_globals: bool,
        print_functions: bool,
        tabs: &mut usize,
        buffer: &mut Buffer,
    ) {
        if !self.info.probe_map.is_empty() {
            // we've matched some modes
            probe_rule.print_bold_mode(buffer);
        }

        for (.., probes) in self.info.probe_map.iter() {
            if let Some(probe) = probes.iter().next() {
                // check to see if we have an alias for this probe kind
                let modes = self.supported_modes();
                let mut alias = None;
                for (kind_alias, kind) in modes {
                    if discriminant(kind) == discriminant(&probe.mode()) {
                        alias = Some(kind_alias);
                    }
                }

                // only print out the docs for some probe type one time!
                probe.print_mode_docs(alias, print_globals, print_functions, tabs, buffer);
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
}
