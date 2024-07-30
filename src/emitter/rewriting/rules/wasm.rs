use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Arg, Event, FromStr, LocInfo, Package,
};
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{DataType, ProbeSpec, SpecPart, Value};
use log::warn;
use orca::ir::module::Module;
use orca::ir::types::{DataType as OrcaType, FuncKind};
use std::collections::HashMap;

use crate::generator::simple_ast::SimpleProbe;
use wasmparser::Operator;

pub struct WasmPackage {
    kind: WasmPackageKind,
    pub events: Vec<Box<dyn Event>>,
}
impl FromStr for WasmPackage {
    fn from_str(name: &str) -> Self {
        match name {
            "opcode" => Self::opcode(),
            _ => panic!("unsupported WasmPackage: {name}"),
        }
    }
}
impl WasmPackage {
    fn opcode() -> Self {
        Self {
            kind: WasmPackageKind::Opcode,
            events: vec![],
        }
    }
}
impl Package for WasmPackage {
    fn get_loc_info(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        match self.kind {
            WasmPackageKind::Opcode => {
                // nothing to add
            }
        }

        // Get location info from the rest of the configured rules
        self.events.iter().for_each(|event| {
            if let Some(mut other_loc_info) = event.get_loc_info(app_wasm, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<String, Vec<SimpleProbe>>>) {
        let events = match self.kind {
            WasmPackageKind::Opcode => event_factory::<OpcodeEvent>(ast_events),
        };
        self.events = events;
    }
}

#[derive(Debug)]
struct FuncInfo {
    func_kind: String,
    module: String,
    name: String,
}

pub struct OpcodeEvent {
    kind: OpcodeEventKind,
    // Map from probe_mode_name -> Vec[probes_of_this_mode]
    // Retains ordering of instrumentation units (in order of scripts passed by user)
    probes: HashMap<String, Vec<SimpleProbe>>,
}
impl FromStr for OpcodeEvent {
    fn from_str(name: &str) -> Self {
        match name {
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
impl OpcodeEvent {
    // =================
    // ---- Helpers ----
    // =================

    fn probe_spec(&self) -> ProbeSpec {
        ProbeSpec {
            provider: Some(SpecPart {
                name: "wasm".to_string(),
                loc: None,
            }),
            package: Some(SpecPart {
                name: "opcode".to_string(),
                loc: None,
            }),
            event: Some(SpecPart {
                name: self.kind.name(),
                loc: None,
            }),
            mode: None,
        }
    }
    pub fn get_ty_info_for_instr(app_wasm: &Module, instr: &Operator) -> (Vec<Arg>, Option<u32>) {
        // TODO: there are 500 of them in wasmparser::Operator
        // compared to 48 of them in walrus::ir::Instr
        // How do we compress the Operators we need to concern
        let (ty_list, ty_id): (Vec<OrcaType>, Option<u32>) = match instr {
            Operator::Call {
                function_index: fid,
            } => {
                match app_wasm.get_fn_kind(*fid) {
                    Some(FuncKind::Import(ty_id)) | Some(FuncKind::Local(ty_id)) => {
                        if let Some(ty) = app_wasm.types.get(ty_id as usize) {
                            (ty.params.to_vec(), Some(ty_id))
                        } else {
                            // no type info found!!
                            warn!("No type information found for import with FID {fid}");
                            (vec![], None)
                        }
                    }
                    None => {
                        // no type info found!!
                        warn!("No type information found for import with FID {fid}");
                        (vec![], None)
                    }
                }
            }
            Operator::Block { .. } => {
                // TODO -- define type info
                (vec![], None)
            }

            Operator::Loop { .. } => {
                // TODO -- define type info
                (vec![], None)
            }

            Operator::CallIndirect { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::LocalGet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::LocalSet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::LocalTee { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::GlobalGet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::GlobalSet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::I32Const { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::I64Const { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::F32Const { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::F64Const { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::Select { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::Unreachable { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::Br { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::BrIf { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::BrTable { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::Drop { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::Return { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::MemorySize { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::MemoryGrow { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::MemoryInit { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::DataDrop { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::MemoryCopy { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::MemoryFill { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::AtomicFence { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableGet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableSet { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableGrow { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableSize { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableFill { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::RefNull { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::RefIsNull { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::RefFunc { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::V128Bitselect { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::I8x16Swizzle { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::I8x16Shuffle { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableInit { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::ElemDrop { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            Operator::TableCopy { .. } => {
                // TODO -- define type info
                (vec![], None)
            }
            _ => {
                // TODO -- define type info
                (vec![], None)
            }
        };

        let mut args = vec![];
        for (idx, ty) in ty_list.iter().enumerate() {
            args.push(Arg::new(format!("arg{}", idx), ty.to_owned()));
        }
        (args, ty_id)
    }

    // ======================
    // ---- Constructors ----
    // ======================
    fn new(kind: OpcodeEventKind) -> Self {
        Self {
            kind,
            probes: HashMap::new(),
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
impl Event for OpcodeEvent {
    fn get_loc_info(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();

        match self.kind {
            OpcodeEventKind::Block => {
                if let Operator::Block { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Loop => {
                if let Operator::Loop { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Call => {
                if let Operator::Call {
                    function_index: fid,
                } = instr
                {
                    // low FIDs are imports (if fid < module.imports.len(), fid is an import)
                    let func_info = if let Some(import) = app_wasm.imports.get(*fid as usize) {
                        // This is an imported function (FIDs too large will return None)

                        // UNCOMMENT FOR DEBUGGING PURPOSES
                        // if import.name == "call_new" {
                        //     println!("call_new!!");
                        // }
                        FuncInfo {
                            func_kind: "import".to_string(),
                            module: import.module.to_string(),
                            name: import.name.to_string(),
                        }
                    } else {
                        // This is a local function
                        FuncInfo {
                            func_kind: "local".to_string(),
                            module: "".to_string(),
                            // TODO -- fix this when orca supports pulling func names
                            name: "".to_string(),
                        }
                    };

                    // define static_data
                    loc_info.static_data.insert(
                        "target_imp_name".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.name.to_string(),
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_fn_type".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.func_kind.to_string(),
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_imp_module".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.module.to_string(),
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::CallIndirect => {
                if let Operator::CallIndirect { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalGet => {
                if let Operator::LocalGet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalSet => {
                if let Operator::LocalSet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalTee => {
                if let Operator::LocalTee { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalGet => {
                if let Operator::GlobalGet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalSet => {
                if let Operator::GlobalSet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Const => {
                if let Operator::I32Const { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
                if let Operator::I64Const { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
                if let Operator::F32Const { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
                if let Operator::F64Const { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Binop => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::Binop{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::Unop => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::Unop{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::Select => {
                if let Operator::Select { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Unreachable => {
                if let Operator::Unreachable { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Br => {
                if let Operator::Br { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::BrIf => {
                if let Operator::BrIf { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::IfElse => {
                // TODO: we might need to change OpCodeEventKind
                unimplemented!()
            }
            OpcodeEventKind::BrTable => {
                if let Operator::BrTable { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Drop => {
                if let Operator::Drop { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Return => {
                if let Operator::Return { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemorySize => {
                if let Operator::MemorySize { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryGrow => {
                if let Operator::MemoryGrow { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryInit => {
                if let Operator::MemoryInit { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::DataDrop => {
                if let Operator::DataDrop { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryCopy => {
                if let Operator::MemoryCopy { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryFill => {
                if let Operator::MemoryFill { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Load => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::Load{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::Store => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::Store{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::AtomicRmw => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::AtomicRmw{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::Cmpxchg => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::Cmpxchg{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::AtomicNotify => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::AtomicNotify{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::AtomicWait => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::AtomicWait{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::AtomicFence => {
                if let Operator::AtomicFence { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableGet => {
                if let Operator::TableGet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableSet => {
                if let Operator::TableSet { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableGrow => {
                if let Operator::TableGrow { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableSize => {
                if let Operator::TableSize { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableFill => {
                if let Operator::TableFill { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefNull => {
                if let Operator::RefNull { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefIsNull => {
                if let Operator::RefIsNull { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefFunc => {
                if let Operator::RefFunc { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::V128Bitselect => {
                if let Operator::V128Bitselect { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::I8x16Swizzle => {
                if let Operator::I8x16Swizzle { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::I8x16Shuffle => {
                if let Operator::I8x16Shuffle { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LoadSimd => {
                // TODO: finish this
                unimplemented!()
                // if let Operator::LoadSimd{ .. } = instr {
                //     // TODO define static vars
                //     loc_info.add_probes(self.probe_spec(), &self.probes);
                // }
            }
            OpcodeEventKind::TableInit => {
                if let Operator::TableInit { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::ElemDrop => {
                if let Operator::ElemDrop { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableCopy => {
                if let Operator::TableCopy { .. } = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
        }

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_probes(&mut self, probes: &HashMap<String, Vec<SimpleProbe>>) {
        self.probes = probe_factory(probes);
    }
}
