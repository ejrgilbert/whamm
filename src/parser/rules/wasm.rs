use std::collections::HashMap;
use termcolor::Buffer;
use crate::common::error::WhammError;
use crate::parser::rules::{Event, event_factory, FromStr, Mode, mode_factory, NameOptions, Package, Probe, WhammMode, WhammProbe};
use crate::parser::types::{DataType, Expr, Global, Location, ProbeSpec, ProvidedFunction, ProvidedFunctionality, ProvidedGlobal};

/// The base information needed for `WasmPackage`s, pulled out into a single struct.
pub struct WasmPackageInfo {
    // Statically defined, always the same
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>
}

pub enum WasmPackage {
    Bytecode {
        metadata: WasmPackageInfo,
        /// The events of the probes that have been used in the Script.
        events: HashMap<String, Box<BytecodeEvent>>,
    }
}
impl NameOptions for WasmPackage {
    fn get_name_options() -> Vec<String> {
        // Violates DRY principle, but works for now.
        // Maybe make this better some other time.
        vec![
            "bytecode".to_string()
        ]
    }
}
impl FromStr for WasmPackage {
    fn from_str(name: String, loc: Option<Location>) -> Self {
        match name.as_str() {
            "bytecode" => Self::bytecode(loc),
            _ => panic!("unsupported WasmPackage: {name}")
        }
    }
}
impl WasmPackage {

    // ======================
    // ---- Constructors ----
    // ======================

    fn bytecode(loc: Option<Location>) -> Self {
        Self::Bytecode {
            metadata: WasmPackageInfo {
                docs: "This package within the wasm provider contains enables the \
                    instrumentation of WebAssembly bytecode instructions.".to_string(),
                fns: vec![],
                globals: HashMap::from([(
                    "wasm_bytecode_loc".to_string(),
                    ProvidedGlobal::new(
                        "wasm_bytecode_loc".to_string(),
                        "A unique identifier tied to the probe's location in the Wasm bytecode.".to_string(),
                        DataType::I32
                    )
                )]),
                loc
            },
            events: HashMap::new()
        }
    }
}
impl Package for WasmPackage {
    
    // ==========================
    // ---- Instance Methods ----
    // ==========================
    
    fn name(&self) -> String {
        match self {
            Self::Bytecode{..} => {
                "bytecode".to_string()
            }
        }
    }

    fn docs(&self) -> &String {
        match self {
            Self::Bytecode{metadata: WasmPackageInfo { docs, ..}, ..} => {
                docs
            }
        }
    }

    fn print_event_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Bytecode{events, ..} => {
                for (.., event) in events.iter() {
                    crate::parser::rules::print_event_docs(event.as_ref(), print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        match self {
            Self::Bytecode{events, ..} => {
                for (.., event) in events.iter() {
                    event.print_mode_docs(print_globals, print_functions, tabs, buffer);
                }
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        match self {
            Self::Bytecode{metadata: WasmPackageInfo { fns, ..}, ..} => {
                fns
            }
        }
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        match self {
            Self::Bytecode{metadata: WasmPackageInfo { globals, ..}, ..} => {
                globals
            }
        }
    }

    fn assign_matching_events(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<(bool, bool), Box<WhammError>> {
        let mut matched_events = false;
        let mut matched_modes = false;
        match self {
            Self::Bytecode {events, ..} => {
                let matched:Vec<(Box<BytecodeEvent>, bool)> = event_factory(probe_spec, loc)?;
                for (event, found_match_for_mode) in matched {
                    matched_events = true;
                    matched_modes |= found_match_for_mode;
                    events.insert(event.name().clone(), event);
                }
            }
        }
        Ok((matched_events, matched_modes))
    }
}
/// The base information needed for `BytecodeEvent`s, pulled out into a single struct.
pub struct BytecodeEventInfo {
    // Statically defined, always the same
    pub name: String,
    pub docs: String,
    pub fns: Vec<ProvidedFunction>,               // Comp-provided
    pub globals: HashMap<String, ProvidedGlobal>, // Comp-provided

    // Tied to the user script
    pub loc: Option<Location>,
    probe_map: HashMap<String, Vec<Box<dyn Probe>>>
}

pub enum BytecodeEvent {
    Block (
        BytecodeEventInfo
    ),
    Loop (
        BytecodeEventInfo
    ),
    Call (
        BytecodeEventInfo
    ),
    CallIndirect (
        BytecodeEventInfo
    ),
    LocalGet (
        BytecodeEventInfo
    ),
    LocalSet (
        BytecodeEventInfo
    ),
    LocalTee (
        BytecodeEventInfo
    ),
    GlobalGet (
        BytecodeEventInfo
    ),
    GlobalSet (
        BytecodeEventInfo
    ),
    Const (
        BytecodeEventInfo
    ),
    Binop (
        BytecodeEventInfo
    ),
    Unop (
        BytecodeEventInfo
    ),
    Select (
        BytecodeEventInfo
    ),
    Unreachable (
        BytecodeEventInfo
    ),
    Br (
        BytecodeEventInfo
    ),
    BrIf (
        BytecodeEventInfo
    ),
    IfElse (
        BytecodeEventInfo
    ),
    BrTable (
        BytecodeEventInfo
    ),
    Drop (
        BytecodeEventInfo
    ),
    Return (
        BytecodeEventInfo
    ),
    MemorySize (
        BytecodeEventInfo
    ),
    MemoryGrow (
        BytecodeEventInfo
    ),
    MemoryInit (
        BytecodeEventInfo
    ),
    DataDrop (
        BytecodeEventInfo
    ),
    MemoryCopy (
        BytecodeEventInfo
    ),
    MemoryFill (
        BytecodeEventInfo
    ),
    Load (
        BytecodeEventInfo
    ),
    Store (
        BytecodeEventInfo
    ),
    AtomicRmw (
        BytecodeEventInfo
    ),
    Cmpxchg (
        BytecodeEventInfo
    ),
    AtomicNotify (
        BytecodeEventInfo
    ),
    AtomicWait (
        BytecodeEventInfo
    ),
    AtomicFence (
        BytecodeEventInfo
    ),
    TableGet (
        BytecodeEventInfo
    ),
    TableSet (
        BytecodeEventInfo
    ),
    TableGrow (
        BytecodeEventInfo
    ),
    TableSize (
        BytecodeEventInfo
    ),
    TableFill (
        BytecodeEventInfo
    ),
    RefNull (
        BytecodeEventInfo
    ),
    RefIsNull (
        BytecodeEventInfo
    ),
    RefFunc (
        BytecodeEventInfo
    ),
    V128Bitselect (
        BytecodeEventInfo
    ),
    I8x16Swizzle (
        BytecodeEventInfo
    ),
    I8x16Shuffle (
        BytecodeEventInfo
    ),
    LoadSimd (
        BytecodeEventInfo
    ),
    TableInit (
        BytecodeEventInfo
    ),
    ElemDrop (
        BytecodeEventInfo
    ),
    TableCopy (
        BytecodeEventInfo
    ),
}
impl NameOptions for BytecodeEvent {
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
            "table_copy".to_string()
        ]
    }
}
impl FromStr for BytecodeEvent {
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
            _ => panic!("unsupported BytecodeEvent: {name}")
        }
    }
}
impl BytecodeEvent {
    // ================
    // ---- Helper ----
    // ================
    
    fn get_metadata(&self) -> &BytecodeEventInfo {
        match self {
            Self::Block(metadata) |
            Self::Loop(metadata) |
            Self::Call(metadata) |
            Self::CallIndirect(metadata) |
            Self::LocalGet(metadata) |
            Self::LocalSet(metadata) |
            Self::LocalTee(metadata) |
            Self::GlobalGet(metadata) |
            Self::GlobalSet(metadata) |
            Self::Const(metadata) |
            Self::Binop(metadata) |
            Self::Unop(metadata) |
            Self::Select(metadata) |
            Self::Unreachable(metadata) |
            Self::Br(metadata) |
            Self::BrIf(metadata) |
            Self::IfElse(metadata) |
            Self::BrTable(metadata) |
            Self::Drop(metadata) |
            Self::Return(metadata) |
            Self::MemorySize(metadata) |
            Self::MemoryGrow(metadata) |
            Self::MemoryInit(metadata) |
            Self::DataDrop(metadata) |
            Self::MemoryCopy(metadata) |
            Self::MemoryFill(metadata) |
            Self::Load(metadata) |
            Self::Store(metadata) |
            Self::AtomicRmw(metadata) |
            Self::Cmpxchg(metadata) |
            Self::AtomicNotify(metadata) |
            Self::AtomicWait(metadata) |
            Self::AtomicFence(metadata) |
            Self::TableGet(metadata) |
            Self::TableSet(metadata) |
            Self::TableGrow(metadata) |
            Self::TableSize(metadata) |
            Self::TableFill(metadata) |
            Self::RefNull(metadata) |
            Self::RefIsNull(metadata) |
            Self::RefFunc(metadata) |
            Self::V128Bitselect(metadata) |
            Self::I8x16Swizzle(metadata) |
            Self::I8x16Shuffle(metadata) |
            Self::LoadSimd(metadata) |
            Self::TableInit(metadata) |
            Self::ElemDrop(metadata) |
            Self::TableCopy(metadata) => {
                metadata
            }
        }
    }

    fn get_metadata_mut(&mut self) -> &mut BytecodeEventInfo {
        match self {
            Self::Block(metadata) |
            Self::Loop(metadata) |
            Self::Call(metadata) |
            Self::CallIndirect(metadata) |
            Self::LocalGet(metadata) |
            Self::LocalSet(metadata) |
            Self::LocalTee(metadata) |
            Self::GlobalGet(metadata) |
            Self::GlobalSet(metadata) |
            Self::Const(metadata) |
            Self::Binop(metadata) |
            Self::Unop(metadata) |
            Self::Select(metadata) |
            Self::Unreachable(metadata) |
            Self::Br(metadata) |
            Self::BrIf(metadata) |
            Self::IfElse(metadata) |
            Self::BrTable(metadata) |
            Self::Drop(metadata) |
            Self::Return(metadata) |
            Self::MemorySize(metadata) |
            Self::MemoryGrow(metadata) |
            Self::MemoryInit(metadata) |
            Self::DataDrop(metadata) |
            Self::MemoryCopy(metadata) |
            Self::MemoryFill(metadata) |
            Self::Load(metadata) |
            Self::Store(metadata) |
            Self::AtomicRmw(metadata) |
            Self::Cmpxchg(metadata) |
            Self::AtomicNotify(metadata) |
            Self::AtomicWait(metadata) |
            Self::AtomicFence(metadata) |
            Self::TableGet(metadata) |
            Self::TableSet(metadata) |
            Self::TableGrow(metadata) |
            Self::TableSize(metadata) |
            Self::TableFill(metadata) |
            Self::RefNull(metadata) |
            Self::RefIsNull(metadata) |
            Self::RefFunc(metadata) |
            Self::V128Bitselect(metadata) |
            Self::I8x16Swizzle(metadata) |
            Self::I8x16Shuffle(metadata) |
            Self::LoadSimd(metadata) |
            Self::TableInit(metadata) |
            Self::ElemDrop(metadata) |
            Self::TableCopy(metadata) => {
                metadata
            }
        }
    }

    // ======================
    // ---- Constructors ----
    // ======================
    
    fn block(loc: Option<Location>) -> Self {
        Self::Block(
            BytecodeEventInfo {
                name: "block".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/block".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn _loop(loc: Option<Location>) -> Self {
        Self::Loop(
            BytecodeEventInfo {
                name: "loop".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/loop".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn call(loc: Option<Location>) -> Self {
        Self::Call (
            BytecodeEventInfo {
                name: "call".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                fns: vec![],
                globals: HashMap::from([(
                    "arg[0:9]+".to_string(),
                    ProvidedGlobal::new(
                        "arg[0:9]+".to_string(),
                        "To reference the arguments passed to the target function, can use any name matching this regex. For example, the first arg can be referenced with `arg0`.".to_string(),
                        DataType::Null
                    )
                ), (
                    "target_fn_type".to_string(),
                    ProvidedGlobal::new(
                        "target_fn_type".to_string(),
                        "The type of function being called at this call site. This constant will \
                            evaluate to either `local` or `import`.".to_string(),
                        DataType::Str
                    )
                ), (
                    "target_imp_module".to_string(),
                    ProvidedGlobal::new(
                        "target_imp_module".to_string(),
                        "The name of the module that the imported function comes from. \
                            To improve performance, pair with `target_fn_type == \"import\"` \
                            for faster short-circuiting.".to_string(),
                        DataType::Str
                    )
                ), (
                    "target_imp_name".to_string(),
                    ProvidedGlobal::new(
                        "target_imp_name".to_string(),
                        "The name of the imported function. \
                        To improve performance, pair with `target_fn_type == \"import\"` \
                        for faster short-circuiting.".to_string(),
                        DataType::Str
                    )
                ), (
                    "new_target_fn_name".to_string(),
                    ProvidedGlobal::new(
                        "new_target_fn_name".to_string(),
                        "(DEPRECATED) The name of the target function to call instead of the original.".to_string(),
                        DataType::Str
                    )
                )]),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::CallIndirect(
            BytecodeEventInfo {
                name: "call_indirect".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/call".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::LocalGet(
            BytecodeEventInfo {
                name: "local_get".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_get".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::LocalSet(
            BytecodeEventInfo {
                name: "local_set".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_set".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::LocalTee(
            BytecodeEventInfo {
                name: "local_tee".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Local_tee".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::GlobalGet(
            BytecodeEventInfo {
                name: "global_get".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_get".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::GlobalSet(
            BytecodeEventInfo {
                name: "global_set".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Variables/Global_set".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::Const(
            BytecodeEventInfo {
                name: "const".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Const".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::Binop(
            BytecodeEventInfo {
                name: "binop".to_string(),
                docs: "Consume two operands and produce one result of the respective type. \
                    The types of binary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::Unop(
            BytecodeEventInfo {
                name: "unop".to_string(),
                docs: "Consume one operand and produce one result of the respective type. \
                    The types of unary operations available to instrument depend on the operands \
                    of the respective instruction. \
                    A list of such operations is available here: \
                    https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn select(loc: Option<Location>) -> Self {
        // TODO -- No way to check lhs/rhs using walrus since due to lack of
        //     typing info at this point. Maybe wasmparser will support this.
        Self::Select(
            BytecodeEventInfo {
                name: "select".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Select".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn unreachable(loc: Option<Location>) -> Self {
        Self::Unreachable(
            BytecodeEventInfo {
                name: "unreachable".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/unreachable".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn br(loc: Option<Location>) -> Self {
        Self::Br(
            BytecodeEventInfo {
                name: "br".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                fns: vec![],
                globals: HashMap::from([(
                    "label_id".to_string(),
                    ProvidedGlobal::new(
                        "label_id".to_string(),
                        "The ID of the block to unconditionally break out of.".to_string(),
                        DataType::U32
                    )
                )]),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn br_if(loc: Option<Location>) -> Self {
        Self::BrIf(
            BytecodeEventInfo {
                name: "br_if".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                fns: vec![],
                globals: HashMap::from([(
                    "label_id".to_string(),
                    ProvidedGlobal::new(
                        "label_id".to_string(),
                        "The ID of the block to unconditionally break out of.".to_string(),
                        DataType::U32
                    )
                ), (
                    "condition".to_string(),
                    ProvidedGlobal::new(
                        "condition".to_string(),
                        "Contains the value of the condition to break on if true (0 is false, nonzero is true).".to_string(),
                        DataType::I32
                    )
                )]),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn if_else(loc: Option<Location>) -> Self {
        Self::IfElse(
            BytecodeEventInfo {
                name: "if_else".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/if...else".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn br_table(loc: Option<Location>) -> Self {
        Self::BrTable(
            BytecodeEventInfo {
                name: "br_table".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/br".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn drop(loc: Option<Location>) -> Self {
        Self::Drop(
            BytecodeEventInfo {
                name: "drop".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/Drop".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn _return(loc: Option<Location>) -> Self {
        Self::Return(
            BytecodeEventInfo {
                name: "return".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Control_flow/return".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::MemorySize(
            BytecodeEventInfo {
                name: "memory_size".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Size".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::MemoryGrow(
            BytecodeEventInfo {
                name: "memory_grow".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Grow".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::MemoryInit(
            BytecodeEventInfo {
                name: "memory_init".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::DataDrop(
            BytecodeEventInfo {
                name: "data_drop".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-memory".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::MemoryCopy(
            BytecodeEventInfo {
                name: "memory_copy".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Copy".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn memory_fill(loc: Option<Location>) -> Self {
        // TODO
        // ;; Fill region at offset/range in default memory with 255
        // i32.const 200 ;; The pointer to the region to update
        // i32.const 255 ;; The value to set each byte to (must be < 256)
        // i32.const 100 ;; The number of bytes to update
        // memory.fill ;; Fill default memory
        Self::MemoryFill(
            BytecodeEventInfo {
                name: "memory_fill".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Fill".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
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
        Self::Load(
            BytecodeEventInfo {
                name: "load".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Load".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn store(loc: Option<Location>) -> Self {
        Self::Store(
            BytecodeEventInfo {
                name: "store".to_string(),
                docs: "https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Memory/Store".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn atomic_rmw(loc: Option<Location>) -> Self {
        Self::AtomicRmw(
            BytecodeEventInfo {
                name: "atomic_rmw".to_string(),
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#read-modify-write".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn cmpxchg(loc: Option<Location>) -> Self {
        Self::Cmpxchg(
            BytecodeEventInfo {
                name: "cmpxchg".to_string(),
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#compare-exchange".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn atomic_notify(loc: Option<Location>) -> Self {
        Self::AtomicNotify(
            BytecodeEventInfo {
                name: "atomic_notify".to_string(),
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn atomic_wait(loc: Option<Location>) -> Self {
        Self::AtomicWait(
            BytecodeEventInfo {
                name: "atomic_wait".to_string(),
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#wait-and-notify-operators".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn atomic_fence(loc: Option<Location>) -> Self {
        Self::AtomicFence(
            BytecodeEventInfo {
                name: "atomic_fence".to_string(),
                docs: "https://github.com/WebAssembly/threads/blob/main/proposals/threads/Overview.md#fence-operator".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_get(loc: Option<Location>) -> Self {
        Self::TableGet(
            BytecodeEventInfo {
                name: "table_get".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_set(loc: Option<Location>) -> Self {
        Self::TableSet(
            BytecodeEventInfo {
                name: "table_set".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_grow(loc: Option<Location>) -> Self {
        Self::TableGrow(
            BytecodeEventInfo {
                name: "table_grow".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_size(loc: Option<Location>) -> Self {
        Self::TableSize(
            BytecodeEventInfo {
                name: "table_size".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_fill(loc: Option<Location>) -> Self {
        Self::TableFill(
            BytecodeEventInfo {
                name: "table_fill".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn ref_null(loc: Option<Location>) -> Self {
        Self::RefNull(
            BytecodeEventInfo {
                name: "ref_null".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn ref_is_null(loc: Option<Location>) -> Self {
        Self::RefIsNull(
            BytecodeEventInfo {
                name: "ref_is_null".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn ref_func(loc: Option<Location>) -> Self {
        Self::RefFunc(
            BytecodeEventInfo {
                name: "ref_func".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-ref".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn v128_bitselect(loc: Option<Location>) -> Self {
        Self::V128Bitselect(
            BytecodeEventInfo {
                name: "v128_bitselect".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn i8x16_swizzle(loc: Option<Location>) -> Self {
        Self::I8x16Swizzle(
            BytecodeEventInfo {
                name: "i8x16_swizzle".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn i8x16_shuffle(loc: Option<Location>) -> Self {
        Self::I8x16Shuffle(
            BytecodeEventInfo {
                name: "i8x16_shuffle".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn load_simd(loc: Option<Location>) -> Self {
        Self::LoadSimd(
            BytecodeEventInfo {
                name: "load_simd".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-vec".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_init(loc: Option<Location>) -> Self {
        Self::TableInit(
            BytecodeEventInfo {
                name: "table_init".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn elem_drop(loc: Option<Location>) -> Self {
        Self::ElemDrop(
            BytecodeEventInfo {
                name: "elem_drop".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
    fn table_copy(loc: Option<Location>) -> Self {
        Self::TableCopy(
            BytecodeEventInfo {
                name: "table_copy".to_string(),
                docs: "https://www.w3.org/TR/wasm-core-2/#syntax-instr-table".to_string(),
                fns: vec![],
                globals: HashMap::new(),
                loc,
                probe_map: HashMap::new()
            }
        )
    }
}
impl Event for BytecodeEvent {
    fn name(&self) -> &String {
        let metadata = self.get_metadata();

        &metadata.name
    }

    fn docs(&self) -> &String {
        let metadata = self.get_metadata();

        &metadata.docs
    }

    fn probes(&self) -> &HashMap<String, Vec<Box<dyn Probe>>> {
        let metadata = self.get_metadata();

        &metadata.probe_map
    }

    fn probes_mut(&mut self) -> &mut HashMap<String, Vec<Box<dyn Probe>>> {
        let metadata = self.get_metadata_mut();

        &mut metadata.probe_map    
    }

    fn print_mode_docs(&self, print_globals: bool, print_functions: bool, tabs: &mut usize, buffer: &mut Buffer) {
        for (.., probes) in self.probes().iter() {
            for probe in probes.iter() {
                probe.print_mode_docs(print_globals, print_functions, tabs, buffer);
            }
        }
    }

    fn get_provided_fns(&self) -> &Vec<ProvidedFunction> {
        let metadata = self.get_metadata();

        &metadata.fns
    }

    fn get_provided_globals(&self) -> &HashMap<String, ProvidedGlobal> {
        let metadata = self.get_metadata();
        
        &metadata.globals
    }

    fn assign_matching_modes(&mut self, probe_spec: &ProbeSpec, loc: Option<Location>) -> Result<bool, Box<WhammError>> {
        let mut matched_modes = false;
        let probes = self.probes_mut();
        let modes: Vec<Box<WhammMode>> = mode_factory(probe_spec, loc.clone())?;
        for mode in modes {
            matched_modes = true;
            probes.insert(mode.name(), vec![Box::new(WhammProbe::new(*mode, loc.clone(), None, None))]);
        }
        Ok(matched_modes)
    }
}