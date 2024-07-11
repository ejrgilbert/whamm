use crate::emitter::rewriting::rules::{Event, event_factory, FromStr, LocInfo, Package, ProcessLoc};
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::rules::Probe;
use std::collections::HashMap;
use walrus::ir::Instr;

pub struct WasmPackage<'a> {
    kind: WasmPackageKind,
    pub events: Vec<Box<dyn Event<'a>>>,
}
impl FromStr for WasmPackage<'_> {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "opcode" => Self::opcode(),
            _ => panic!("unsupported WasmPackage: {name}"),
        }
    }
}
impl WasmPackage<'_> {
    fn opcode() -> Self {
        Self {
            kind: WasmPackageKind::Opcode,
            events: vec![]
        }
    }
}
impl<'a> Package<'a> for WasmPackage<'a> {
    fn get_events_mut(&mut self) -> &mut Vec<Box<dyn Event<'a>>> {
        &mut self.events
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<String, Vec<Box<&'a dyn Probe>>>>) {
        match self.kind {
            WasmPackageKind::Opcode => {
                event_factory::<OpcodeEvent>(self as &mut dyn Package, ast_events);
            }
        }
    }
}
impl ProcessLoc for WasmPackage<'_> {
    fn get_loc_info(
        &self,
        _app_wasm: &walrus::Module,
        _instr: &Instr,
        _instr_name: &str,
    ) -> LocInfo {
        match self.kind {
            WasmPackageKind::Opcode => {
                todo!()
            }
        }
    }
}

pub struct OpcodeEvent<'a> {
    kind: OpcodeEventKind,
    // Map from probe_mode_name -> Vec[probes_of_this_mode]
    // Retains ordering of instrumentation units (in order of scripts passed by user)
    probes: HashMap<String, Vec<Box<&'a dyn Probe>>>,
}
impl FromStr for OpcodeEvent<'_> {
    fn from_str(name: &String) -> Self {
        match name.as_str() {
            "block" => Self::block(),
            "loop" => Self::_loop(),
            "call" => Self::call(),
            "call_indirect" => Self::call_indirect(),
            "local_get" => Self::local_get(),
            "local_set" => Self::local_set(),
            "local_tee" => Self::local_tee(),
            "global_get" => Self::global_get(),
            "global_set" => Self::global_set(),
            "const" => Self::_const(),
            "binop" => Self::binop(),
            "unop" => Self::unop(),
            "select" => Self::select(),
            "unreachable" => Self::unreachable(),
            "br" => Self::br(),
            "br_if" => Self::br_if(),
            "if_else" => Self::if_else(),
            "br_table" => Self::br_table(),
            "drop" => Self::drop(),
            "return" => Self::_return(),
            "memory_size" => Self::memory_size(),
            "memory_grow" => Self::memory_grow(),
            "memory_init" => Self::memory_init(),
            "data_drop" => Self::data_drop(),
            "memory_copy" => Self::memory_copy(),
            "memory_fill" => Self::memory_fill(),
            "load" => Self::load(),
            "store" => Self::store(),
            "atomic_rmw" => Self::atomic_rmw(),
            "cmpxchg" => Self::cmpxchg(),
            "atomic_notify" => Self::atomic_notify(),
            "atomic_wait" => Self::atomic_wait(),
            "atomic_fence" => Self::atomic_fence(),
            "table_get" => Self::table_get(),
            "table_set" => Self::table_set(),
            "table_grow" => Self::table_grow(),
            "table_size" => Self::table_size(),
            "table_fill" => Self::table_fill(),
            "ref_null" => Self::ref_null(),
            "ref_is_null" => Self::ref_is_null(),
            "ref_func" => Self::ref_func(),
            "v128_bitselect" => Self::v128_bitselect(),
            "i8x16_swizzle" => Self::i8x16_swizzle(),
            "i8x16_shuffle" => Self::i8x16_shuffle(),
            "load_simd" => Self::load_simd(),
            "table_init" => Self::table_init(),
            "elem_drop" => Self::elem_drop(),
            "table_copy" => Self::table_copy(),
            _ => panic!("unsupported OpcodeEvent: {name}"),
        }
    }
}
impl OpcodeEvent<'_> {
    // ======================
    // ---- Constructors ----
    // ======================
    fn new(kind: OpcodeEventKind) -> Self {
        Self {
            kind,
            probes: HashMap::new()
        }
    }

    fn block() -> Self {
        Self::new(OpcodeEventKind::Block)
    }
    fn _loop() -> Self {
        Self::new(OpcodeEventKind::Loop)
    }
    fn call() -> Self {
        Self::new(OpcodeEventKind::Call)
    }
    fn call_indirect() -> Self {
        Self::new(OpcodeEventKind::CallIndirect)
    }
    fn local_get() -> Self {
        Self::new(OpcodeEventKind::LocalGet)
    }
    fn local_set() -> Self {
        Self::new(OpcodeEventKind::LocalSet)
    }
    fn local_tee() -> Self {
        Self::new(OpcodeEventKind::LocalTee)
    }
    fn global_get() -> Self {
        Self::new(OpcodeEventKind::GlobalGet)
    }
    fn global_set() -> Self {
        Self::new(OpcodeEventKind::GlobalSet)
    }
    fn _const() -> Self {
        Self::new(OpcodeEventKind::Const)
    }
    fn binop() -> Self {
        Self::new(OpcodeEventKind::Binop)
    }
    fn unop() -> Self {
        Self::new(OpcodeEventKind::Unop)
    }
    fn select() -> Self {
        Self::new(OpcodeEventKind::Select)
    }
    fn unreachable() -> Self {
        Self::new(OpcodeEventKind::Unreachable)
    }
    fn br() -> Self {
        Self::new(OpcodeEventKind::Br)
    }
    fn br_if() -> Self {
        Self::new(OpcodeEventKind::BrIf)
    }
    fn if_else() -> Self {
        Self::new(OpcodeEventKind::IfElse)
    }
    fn br_table() -> Self {
        Self::new(OpcodeEventKind::BrTable)
    }
    fn drop() -> Self {
        Self::new(OpcodeEventKind::Drop)
    }
    fn _return() -> Self {
        Self::new(OpcodeEventKind::Return)
    }
    fn memory_size() -> Self {
        Self::new(OpcodeEventKind::MemorySize)
    }
    fn memory_grow() -> Self {
        Self::new(OpcodeEventKind::MemoryGrow)
    }
    fn memory_init() -> Self {
        Self::new(OpcodeEventKind::MemoryInit)
    }
    fn data_drop() -> Self {
        Self::new(OpcodeEventKind::DataDrop)
    }
    fn memory_copy() -> Self {
        Self::new(OpcodeEventKind::MemoryCopy)
    }
    fn memory_fill() -> Self {
        Self::new(OpcodeEventKind::MemoryFill)
    }
    fn load() -> Self {
        Self::new(OpcodeEventKind::Load)
    }
    fn store() -> Self {
        Self::new(OpcodeEventKind::Store)
    }
    fn atomic_rmw() -> Self {
        Self::new(OpcodeEventKind::AtomicRmw)
    }
    fn cmpxchg() -> Self {
        Self::new(OpcodeEventKind::Cmpxchg)
    }
    fn atomic_notify() -> Self {
        Self::new(OpcodeEventKind::AtomicNotify)
    }
    fn atomic_wait() -> Self {
        Self::new(OpcodeEventKind::AtomicWait)
    }
    fn atomic_fence() -> Self {
        Self::new(OpcodeEventKind::AtomicFence)
    }
    fn table_get() -> Self {
        Self::new(OpcodeEventKind::TableGet)
    }
    fn table_set() -> Self {
        Self::new(OpcodeEventKind::TableSet)
    }
    fn table_grow() -> Self {
        Self::new(OpcodeEventKind::TableGrow)
    }
    fn table_size() -> Self {
        Self::new(OpcodeEventKind::TableSize)
    }
    fn table_fill() -> Self {
        Self::new(OpcodeEventKind::TableFill)
    }
    fn ref_null() -> Self {
        Self::new(OpcodeEventKind::RefNull)
    }
    fn ref_is_null() -> Self {
        Self::new(OpcodeEventKind::RefIsNull)
    }
    fn ref_func() -> Self {
        Self::new(OpcodeEventKind::RefFunc)
    }
    fn v128_bitselect() -> Self {
        Self::new(OpcodeEventKind::V128Bitselect)
    }
    fn i8x16_swizzle() -> Self {
        Self::new(OpcodeEventKind::I8x16Swizzle)
    }
    fn i8x16_shuffle() -> Self {
        Self::new(OpcodeEventKind::I8x16Shuffle)
    }
    fn load_simd() -> Self {
        Self::new(OpcodeEventKind::LoadSimd)
    }
    fn table_init() -> Self {
        Self::new(OpcodeEventKind::TableInit)
    }
    fn elem_drop() -> Self {
        Self::new(OpcodeEventKind::ElemDrop)
    }
    fn table_copy() -> Self {
        Self::new(OpcodeEventKind::TableCopy)
    }
}
impl<'a> Event<'a> for OpcodeEvent<'a> {
    fn add_probes(&mut self, probes: &HashMap<String, Vec<Box<&'a dyn Probe>>>) {
        self.probes = probes.to_owned()
    }
}
impl ProcessLoc for OpcodeEvent<'_> {
    fn get_loc_info(
        &self,
        _app_wasm: &walrus::Module,
        _instr: &Instr,
        _instr_name: &str,
    ) -> LocInfo {
        match self.kind {
            OpcodeEventKind::Block => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Loop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Call => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes

                // let func = app_wasm.funcs.get(func.func);
                // // get information about the function call
                // let (func_info, params) = crate::emitter::rewriting::get_func_info(app_wasm, func);
                //
                // crate::emitter::rewriting::InstrInfo {
                //     instr_name: instr_name.to_owned(),
                //     instr_args: params,
                //     called_func_info: Some(func_info),
                // }

                todo!()
            }
            OpcodeEventKind::CallIndirect => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::LocalGet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::LocalSet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::LocalTee => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::GlobalGet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::GlobalSet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Const => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Binop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Unop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Select => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Unreachable => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Br => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // - label_id
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::BrIf => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // - label_id
                // - condition
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::IfElse => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::BrTable => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Drop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Return => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::MemorySize => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::MemoryGrow => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::MemoryInit => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::DataDrop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::MemoryCopy => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::MemoryFill => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Load => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Store => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::AtomicRmw => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::Cmpxchg => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::AtomicNotify => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::AtomicWait => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::AtomicFence => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableGet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableSet => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableGrow => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableSize => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableFill => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::RefNull => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::RefIsNull => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::RefFunc => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::V128Bitselect => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::I8x16Swizzle => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::I8x16Shuffle => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::LoadSimd => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableInit => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::ElemDrop => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
            OpcodeEventKind::TableCopy => {
                // check if the instr is of this event type
                // define static/dynamic vars
                // pull matched probes
                todo!()
            }
        }
    }
}