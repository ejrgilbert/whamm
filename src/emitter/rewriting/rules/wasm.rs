use crate::emitter::rewriting::rules::{Event, LocInfo, Package, ProcessLoc};
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::rules::Probe;
use std::collections::HashMap;
use walrus::ir::Instr;

pub struct WasmPackage {
    kind: WasmPackageKind,
    pub events: Vec<Box<dyn Event>>,
}
impl Package for WasmPackage {}
impl ProcessLoc for WasmPackage {
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
    // outer Vec represents script[0:9] for composable instrumentation,
    // inner HashMap is mode_name-> an ordered Vec of probes from the respective script
    probes: Vec<HashMap<String, Vec<&'a dyn Probe>>>,
}
impl Event for OpcodeEvent<'_> {}
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