use crate::parser::rules::{
    event_factory, mode_factory, Event, EventInfo, FromStr, Mode, NameOptions, Package,
    PackageInfo, Probe, WhammMode, WhammProbe,
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

    fn docs(&self) -> &String {
        &self.info.docs
    }

    fn loc(&self) -> &Option<Location> {
        &self.info.loc
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

pub enum OpcodeEventKind {
    Block,
    Loop,
    Call,
    CallIndirect,
    LocalGet,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    Const,
    Binop,
    Unop,
    Select,
    Unreachable,
    Br,
    BrIf,
    IfElse,
    BrTable,
    Drop,
    Return,
    MemorySize,
    MemoryGrow,
    MemoryInit,
    DataDrop,
    MemoryCopy,
    MemoryFill,
    Load,
    Store,
    AtomicRmw,
    Cmpxchg,
    AtomicNotify,
    AtomicWait,
    AtomicFence,
    TableGet,
    TableSet,
    TableGrow,
    TableSize,
    TableFill,
    RefNull,
    RefIsNull,
    RefFunc,
    V128Bitselect,
    I8x16Swizzle,
    I8x16Shuffle,
    LoadSimd,
    TableInit,
    ElemDrop,
    TableCopy,
}
impl OpcodeEventKind {
    pub fn name(&self) -> String {
        match self {
            OpcodeEventKind::Block => "block".to_string(),
            OpcodeEventKind::Loop => "loop".to_string(),
            OpcodeEventKind::Call => "call".to_string(),
            OpcodeEventKind::CallIndirect => "call_indirect".to_string(),
            OpcodeEventKind::LocalGet => "local_get".to_string(),
            OpcodeEventKind::LocalSet => "local_set".to_string(),
            OpcodeEventKind::LocalTee => "local_tee".to_string(),
            OpcodeEventKind::GlobalGet => "global_get".to_string(),
            OpcodeEventKind::GlobalSet => "global_set".to_string(),
            OpcodeEventKind::Const => "const".to_string(),
            OpcodeEventKind::Binop => "binop".to_string(),
            OpcodeEventKind::Unop => "unop".to_string(),
            OpcodeEventKind::Select => "select".to_string(),
            OpcodeEventKind::Unreachable => "unreachable".to_string(),
            OpcodeEventKind::Br => "br".to_string(),
            OpcodeEventKind::BrIf => "br_if".to_string(),
            OpcodeEventKind::IfElse => "if_else".to_string(),
            OpcodeEventKind::BrTable => "br_table".to_string(),
            OpcodeEventKind::Drop => "drop".to_string(),
            OpcodeEventKind::Return => "return".to_string(),
            OpcodeEventKind::MemorySize => "memory_size".to_string(),
            OpcodeEventKind::MemoryGrow => "memory_grow".to_string(),
            OpcodeEventKind::MemoryInit => "memory_init".to_string(),
            OpcodeEventKind::DataDrop => "data_drop".to_string(),
            OpcodeEventKind::MemoryCopy => "memory_copy".to_string(),
            OpcodeEventKind::MemoryFill => "memory_fill".to_string(),
            OpcodeEventKind::Load => "load".to_string(),
            OpcodeEventKind::Store => "store".to_string(),
            OpcodeEventKind::AtomicRmw => "atomic_rmw".to_string(),
            OpcodeEventKind::Cmpxchg => "cmpxchg".to_string(),
            OpcodeEventKind::AtomicNotify => "atomic_notify".to_string(),
            OpcodeEventKind::AtomicWait => "atomic_wait".to_string(),
            OpcodeEventKind::AtomicFence => "atomic_fence".to_string(),
            OpcodeEventKind::TableGet => "table_get".to_string(),
            OpcodeEventKind::TableSet => "table_set".to_string(),
            OpcodeEventKind::TableGrow => "table_grow".to_string(),
            OpcodeEventKind::TableSize => "table_size".to_string(),
            OpcodeEventKind::TableFill => "table_fill".to_string(),
            OpcodeEventKind::RefNull => "ref_null".to_string(),
            OpcodeEventKind::RefIsNull => "ref_is_null".to_string(),
            OpcodeEventKind::RefFunc => "ref_func".to_string(),
            OpcodeEventKind::V128Bitselect => "v128_bitselect".to_string(),
            OpcodeEventKind::I8x16Swizzle => "i8x16_swizzle".to_string(),
            OpcodeEventKind::I8x16Shuffle => "i8x16_shuffle".to_string(),
            OpcodeEventKind::LoadSimd => "load_simd".to_string(),
            OpcodeEventKind::TableInit => "table_init".to_string(),
            OpcodeEventKind::ElemDrop => "elem_drop".to_string(),
            OpcodeEventKind::TableCopy => "table_copy".to_string(),
        }
    }

    /// Only specify the number of args since the arg type
    /// isn't necessarily consistent based on just which opcode
    /// we're at.
    /// (Sometimes a specific opcode's arg0 is i32, sometimes it's not)
    fn get_num_args(&self) -> u32 {
        match self {
            OpcodeEventKind::Call => 0, // dynamically determined
            OpcodeEventKind::BrIf => 1,
            _ => 0,
        }
    }

    fn get_immediates(&self) -> Vec<DataType> {
        match self {
            OpcodeEventKind::Call => vec![DataType::I32],
            OpcodeEventKind::BrIf => vec![DataType::I32],
            _ => vec![],
        }
    }
}

pub struct OpcodeEvent {
    info: EventInfo,
    kind: OpcodeEventKind,
}
impl NameOptions for OpcodeEvent {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "block".to_string(),
            "loop".to_string(),
            "call".to_string(),
            "call_indirect".to_string(),
            "local_get".to_string(),
            "local_set".to_string(),
            "local_tee".to_string(),
            "global_get".to_string(),
            "global_set".to_string(),
            "const".to_string(),
            "binop".to_string(),
            "unop".to_string(),
            "select".to_string(),
            "unreachable".to_string(),
            "br".to_string(),
            "br_if".to_string(),
            "if_else".to_string(),
            "br_table".to_string(),
            "drop".to_string(),
            "return".to_string(),
            "memory_size".to_string(),
            "memory_grow".to_string(),
            "memory_init".to_string(),
            "data_drop".to_string(),
            "memory_copy".to_string(),
            "memory_fill".to_string(),
            "load".to_string(),
            "store".to_string(),
            "atomic_rmw".to_string(),
            "cmpxchg".to_string(),
            "atomic_notify".to_string(),
            "atomic_wait".to_string(),
            "atomic_fence".to_string(),
            "table_get".to_string(),
            "table_set".to_string(),
            "table_grow".to_string(),
            "table_size".to_string(),
            "table_fill".to_string(),
            "ref_null".to_string(),
            "ref_is_null".to_string(),
            "ref_func".to_string(),
            "v128_bitselect".to_string(),
            "i8x16_swizzle".to_string(),
            "i8x16_shuffle".to_string(),
            "load_simd".to_string(),
            "table_init".to_string(),
            "elem_drop".to_string(),
            "table_copy".to_string(),
        ]
    }
}
impl FromStr for OpcodeEvent {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "block" => Self::block(loc),
            "loop" => Self::_loop(loc),
            "call" => Self::call(loc),
            "call_indirect" => Self::call_indirect(loc),
            "local_get" => Self::local_get(loc),
            "local_set" => Self::local_set(loc),
            "local_tee" => Self::local_tee(loc),
            "global_get" => Self::global_get(loc),
            "global_set" => Self::global_set(loc),
            "const" => Self::_const(loc),
            "binop" => Self::binop(loc),
            "unop" => Self::unop(loc),
            "select" => Self::select(loc),
            "unreachable" => Self::unreachable(loc),
            "br" => Self::br(loc),
            "br_if" => Self::br_if(loc),
            "if_else" => Self::if_else(loc),
            "br_table" => Self::br_table(loc),
            "drop" => Self::drop(loc),
            "return" => Self::_return(loc),
            "memory_size" => Self::memory_size(loc),
            "memory_grow" => Self::memory_grow(loc),
            "memory_init" => Self::memory_init(loc),
            "data_drop" => Self::data_drop(loc),
            "memory_copy" => Self::memory_copy(loc),
            "memory_fill" => Self::memory_fill(loc),
            "load" => Self::load(loc),
            "store" => Self::store(loc),
            "atomic_rmw" => Self::atomic_rmw(loc),
            "cmpxchg" => Self::cmpxchg(loc),
            "atomic_notify" => Self::atomic_notify(loc),
            "atomic_wait" => Self::atomic_wait(loc),
            "atomic_fence" => Self::atomic_fence(loc),
            "table_get" => Self::table_get(loc),
            "table_set" => Self::table_set(loc),
            "table_grow" => Self::table_grow(loc),
            "table_size" => Self::table_size(loc),
            "table_fill" => Self::table_fill(loc),
            "ref_null" => Self::ref_null(loc),
            "ref_is_null" => Self::ref_is_null(loc),
            "ref_func" => Self::ref_func(loc),
            "v128_bitselect" => Self::v128_bitselect(loc),
            "i8x16_swizzle" => Self::i8x16_swizzle(loc),
            "i8x16_shuffle" => Self::i8x16_shuffle(loc),
            "load_simd" => Self::load_simd(loc),
            "table_init" => Self::table_init(loc),
            "elem_drop" => Self::elem_drop(loc),
            "table_copy" => Self::table_copy(loc),
            _ => panic!("unsupported OpcodeEvent: {name}"),
        }
    }
}
impl OpcodeEvent {
    // =========================
    // ---- Globals Helpers ----
    // =========================

    fn init_globals(kind: OpcodeEventKind) -> HashMap<String, ProvidedGlobal> {
        let mut globals = HashMap::new();
        Self::gen_args(&mut globals, kind.get_num_args());
        Self::gen_immediates(&mut globals, kind.get_immediates());

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

    fn gen_immediates(globals: &mut HashMap<String, ProvidedGlobal>, immediates: Vec<DataType>) {
        for (idx, ty) in immediates.iter().enumerate() {
            let name = format!("imm{}", idx);
            globals.insert(
                name.clone(),
                ProvidedGlobal::new(
                    name.to_string(),
                    format!("The immediate to the opcode at index {}.", idx),
                    ty.to_owned(),
                    true,
                ),
            );
        }
    }

    // ======================
    // ---- Constructors ----
    // ======================

    fn block(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Block,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Block),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn _loop(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Loop,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Loop),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn call(loc: Option<Location>) -> Self {
        // TODO add the following functionality:
        // - `result` global
        // -
        let fns = vec![ProvidedFunction::new(
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
        )];

        let mut globals = Self::init_globals(OpcodeEventKind::Call);

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
            "target_imp_name".to_string(),
            ProvidedGlobal::new(
                "target_imp_name".to_string(),
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

        Self {
            kind: OpcodeEventKind::Call,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                fns,
                globals,
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn call_indirect(loc: Option<Location>) -> Self {
        // TODO
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("table_idx".to_string(), (
        //     ProvidedFunctionality {
        //         name: "table_idx".to_string(),
        //         docs: "Index into the table specifying a function to indirectly call.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "table_idx".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("func_type_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "func_type_id".to_string(),
        //         docs: "The ID of the type that holds the signature for the called function.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "func_type_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::CallIndirect,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::CallIndirect),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn local_get(loc: Option<Location>) -> Self {
        // TODO
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("local_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "local_id".to_string(),
        //         docs: "The ID of the local variable referenced in this instruction.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "local_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::LocalGet,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_get".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::LocalGet),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn local_set(loc: Option<Location>) -> Self {
        // TODO
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("local_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "local_id".to_string(),
        //         docs: "The ID of the local variable referenced in this instruction.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "local_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::LocalSet,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_set".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::LocalSet),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn local_tee(loc: Option<Location>) -> Self {
        // TODO
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("local_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "local_id".to_string(),
        //         docs: "The ID of the local variable referenced in this instruction.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "local_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::LocalTee,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::LocalTee),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn global_get(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("global_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "global_id".to_string(),
        //         docs: "The ID of the global variable referenced in this instruction.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "global_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::GlobalGet,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_get".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::GlobalGet),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn global_set(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("global_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "global_id".to_string(),
        //         docs: "The ID of the global variable referenced in this instruction.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "global_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::GlobalSet,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_set".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::GlobalSet),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn _const(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("wasm_type".to_string(), (
        //     ProvidedFunctionality {
        //         name: "wasm_type".to_string(),
        //         docs: "The type of this constant value. \
        //         The possible values of this global are the names of the enum variants located at: \
        //         https://docs.rs/walrus/latest/walrus/ir/enum.Value.html".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::Str,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "wasm_type".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // TODO -- Should there be a way to check the actual value of this constant?
        Self {
            kind: OpcodeEventKind::Const,
            info: EventInfo {
                docs:
                    "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const"
                        .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Const),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn binop(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("binop_type".to_string(), (
        //     ProvidedFunctionality {
        //         name: "binop_type".to_string(),
        //         docs: "The type of this binary operation.\
        //         The possible values of this global are the names of the enum variants located at: \
        //         https://docs.rs/walrus/latest/walrus/ir/enum.BinaryOp.html".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::Str,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "binop_type".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // TODO -- No way to check lhs/rhs using walrus since due to lack of
        //     typing info at this point. Maybe wasmparser will support this.
        Self {
            kind: OpcodeEventKind::Binop,
            info: EventInfo {
                docs: "Consume two operands and produce one result of the respective type. \
                    The types of binary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Binop),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn unop(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("unop_type".to_string(), (
        //     ProvidedFunctionality {
        //         name: "unop_type".to_string(),
        //         docs: "The type of this binary operation. \
        //         The possible values of this global are the names of the enum variants located at: \
        //         https://docs.rs/walrus/latest/walrus/ir/enum.UnaryOp.html".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::Str,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "unop_type".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // TODO -- No way to check operand using walrus since due to lack of
        //     typing info at this point. Maybe wasmparser will support this.
        Self {
            kind: OpcodeEventKind::Unop,
            info: EventInfo {
                docs: "Consume one operand and produce one result of the respective type. \
                    The types of unary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Unop),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn select(loc: Option<Location>) -> Self {
        // TODO -- No way to check lhs/rhs using walrus since due to lack of
        //     typing info at this point. Maybe wasmparser will support this.
        Self {
            kind: OpcodeEventKind::Select,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Select),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn unreachable(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Unreachable,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Unreachable),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn br(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Br,
            info: EventInfo {
                docs:
                    "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
                        .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Br),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn br_if(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::BrIf,
            info: EventInfo {
                docs:
                    "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
                        .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::BrIf),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn if_else(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::IfElse,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::IfElse),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn br_table(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::BrTable,
            info: EventInfo {
                docs:
                    "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br"
                        .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::BrTable),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn drop(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Drop,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Drop".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Drop),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn _return(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Return,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Return),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn memory_size(loc: Option<Location>) -> Self {
        // I'm worried about what instrumenting things like this looks like...
        // are these technically parameters? Should I save these off?
        // Comment out for now and figure out later!
        // globals.insert("mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_id".to_string(),
        //         docs: "The ID of the target memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::MemorySize,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Size"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::MemorySize),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn memory_grow(loc: Option<Location>) -> Self {
        // I'm worried about what instrumenting things like this looks like...
        // are these technically parameters? Should I save these off?
        // Comment out for now and figure out later!

        // ;; grow memory by 1 page
        // ;; grow returns in 1 for success and -1 for failure
        // ;; will fail if you change to more more than 1 page
        //     (memory.grow (i32.const 1))
        // globals.insert("mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_id".to_string(),
        //         docs: "The ID of the target memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::MemoryGrow,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Grow"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::MemoryGrow),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn memory_init(loc: Option<Location>) -> Self {
        // I'm worried about what instrumenting things like this looks like...
        // are these technically parameters? Should I save these off?
        // Comment out for now and figure out later!
        //https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md#memoryinit-instruction
        // globals.insert("mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_id".to_string(),
        //         docs: "The ID of the target memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("data_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "data_id".to_string(),
        //         docs: "The ID of the data to copy in.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "data_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::MemoryInit,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::MemoryInit),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn data_drop(loc: Option<Location>) -> Self {
        // Unsure what intuitively makes sense to expose here
        // Comment out for now and figure out later!
        // globals.insert("data_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "data_id".to_string(),
        //         docs: "The ID of the data to drop.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "data_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::DataDrop,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::DataDrop),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn memory_copy(loc: Option<Location>) -> Self {
        // I'm worried about what instrumenting things like this looks like...
        // are these technically parameters? Should I save these off?
        // Comment out for now and figure out later!

        // ;; Copy data in specific memory  [100, 125] to [50, 75]
        // i32.const 50 ;; Destination address to copy to
        // i32.const 100 ;; Source address to copy from
        // i32.const 25 ;; Number of bytes to copy
        // memory.copy (memory 2)  ;; Copy memory within memory with index 2
        // globals.insert("src_mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "src_mem_id".to_string(),
        //         docs: "The ID of the source memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "src_mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("dst_mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "dst_mem_id".to_string(),
        //         docs: "The ID of the destination memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "dst_mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::MemoryCopy,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::MemoryCopy),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn memory_fill(loc: Option<Location>) -> Self {
        // TODO
        // ;; Fill region at offset/range in default memory with 255
        // i32.const 200 ;; The pointer to the region to update
        // i32.const 255 ;; The value to set each byte to (must be < 256)
        // i32.const 100 ;; The number of bytes to update
        // memory.fill ;; Fill default memory
        Self {
            kind: OpcodeEventKind::MemoryFill,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Fill"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::MemoryFill),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn load(loc: Option<Location>) -> Self {
        // TODO
        // I'm worried about what instrumenting things like this looks like...
        // are these technically parameters? Should I save these off?
        // Comment out for now and figure out later!
        // globals.insert("mem_id".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_id".to_string(),
        //         docs: "The ID of the target memory.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_id".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("wasm_type".to_string(), (
        //     ProvidedFunctionality {
        //         name: "wasm_type".to_string(),
        //         docs: "The type of this load operation.\
        //         The possible values of this global are the names of the enum variants located at: \
        //         https://docs.rs/walrus/latest/walrus/ir/enum.LoadKind.html".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::Str,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "wasm_type".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("mem_align".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_align".to_string(),
        //         docs: "The expected alignment (expressed as the exponent of a power of 2).".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_align".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        // globals.insert("mem_offset".to_string(), (
        //     ProvidedFunctionality {
        //         name: "mem_offset".to_string(),
        //         docs: "The memory address offset.".to_string()
        //     },
        //     Global {
        //         is_comp_provided: true,
        //         ty: DataType::U32,
        //         var_name: Expr::VarId {
        //             is_comp_provided: true,
        //             name: "mem_offset".to_string(),
        //             loc: None
        //         },
        //         value: None
        //     }
        // ));
        Self {
            kind: OpcodeEventKind::Load,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Load),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn store(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Store,
            info: EventInfo {
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store"
                    .to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Store),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn atomic_rmw(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::AtomicRmw,
            info: EventInfo {
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#read-modify-write".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::AtomicRmw),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn cmpxchg(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::Cmpxchg,
            info: EventInfo {
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#compare-exchange".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::Cmpxchg),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn atomic_notify(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::AtomicNotify,
            info: EventInfo {
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::AtomicNotify),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn atomic_wait(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::AtomicWait,
            info: EventInfo {
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::AtomicWait),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn atomic_fence(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::AtomicFence,
            info: EventInfo {
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#fence-operator".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::AtomicFence),
                loc,
                probe_map: HashMap::new()
            }
        }
    }
    fn table_get(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableGet,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableGet),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_set(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableSet,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableSet),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_grow(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableGrow,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableGrow),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_size(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableSize,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableSize),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_fill(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableFill,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableFill),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn ref_null(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::RefNull,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::RefNull),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn ref_is_null(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::RefIsNull,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::RefIsNull),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn ref_func(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::RefFunc,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::RefFunc),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn v128_bitselect(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::V128Bitselect,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::V128Bitselect),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn i8x16_swizzle(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::I8x16Swizzle,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::I8x16Swizzle),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn i8x16_shuffle(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::I8x16Shuffle,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::I8x16Shuffle),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn load_simd(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::LoadSimd,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::LoadSimd),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_init(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableInit,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableInit),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn elem_drop(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::ElemDrop,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::ElemDrop),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
    fn table_copy(loc: Option<Location>) -> Self {
        Self {
            kind: OpcodeEventKind::TableCopy,
            info: EventInfo {
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: Self::init_globals(OpcodeEventKind::TableCopy),
                loc,
                probe_map: HashMap::new(),
            },
        }
    }
}
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
