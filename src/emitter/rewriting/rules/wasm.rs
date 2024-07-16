use crate::behavior::builder_visitor::SimpleProbe;
use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Event, FromStr, LocInfo, Package,
};
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{DataType, ProbeSpec, SpecPart, Value};
use std::collections::HashMap;
use walrus::ir::Instr;
use walrus::{FunctionKind, ImportedFunction, LocalFunction, ValType};

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
    fn get_loc_info(&self, app_wasm: &walrus::Module, instr: &Instr) -> Option<LocInfo> {
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
    pub fn get_args_for_instr(app_wasm: &walrus::Module, instr: &Instr) -> Vec<ValType> {
        match instr {
            Instr::Block(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Loop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Call(called_func) => match &app_wasm.funcs.get(called_func.func).kind {
                FunctionKind::Import(ImportedFunction { ty: ty_id, .. }) => {
                    let ty = app_wasm.types.get(*ty_id);
                    Vec::from(ty.params())
                }
                FunctionKind::Local(LocalFunction { args, .. }) => {
                    let mut fn_args = vec![];
                    args.iter().for_each(|arg_id| {
                        let arg = app_wasm.locals.get(*arg_id);
                        fn_args.push(arg.ty());
                    });
                    fn_args
                }
                FunctionKind::Uninitialized(ty_id) => {
                    let ty = app_wasm.types.get(*ty_id);

                    Vec::from(ty.params())
                }
            },
            Instr::CallIndirect(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::LocalGet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::LocalSet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::LocalTee(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::GlobalGet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::GlobalSet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Const(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Binop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Unop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Select(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Unreachable(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Br(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::BrIf(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::IfElse(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::BrTable(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Drop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Return(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::MemorySize(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::MemoryGrow(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::MemoryInit(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::DataDrop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::MemoryCopy(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::MemoryFill(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Load(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Store(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::AtomicRmw(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::Cmpxchg(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::AtomicNotify(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::AtomicWait(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::AtomicFence(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableGet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableSet(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableGrow(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableSize(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableFill(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::RefNull(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::RefIsNull(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::RefFunc(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::V128Bitselect(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::I8x16Swizzle(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::I8x16Shuffle(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::LoadSimd(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableInit(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::ElemDrop(..) => {
                // TODO -- define args
                vec![]
            }
            Instr::TableCopy(..) => {
                // TODO -- define args
                vec![]
            }
        }
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
    fn get_loc_info(&self, app_wasm: &walrus::Module, instr: &Instr) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();

        match self.kind {
            OpcodeEventKind::Block => {
                if let Instr::Block(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Loop => {
                if let Instr::Loop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Call => {
                if let Instr::Call(called_func) = instr {
                    // collect information about this instruction
                    let func = app_wasm.funcs.get(called_func.func);
                    let func_info = match &func.kind {
                        FunctionKind::Import(ImportedFunction {
                            import: import_id, ..
                        }) => {
                            let import = app_wasm.imports.get(*import_id);
                            FuncInfo {
                                func_kind: "import".to_string(),
                                module: import.module.clone(),
                                name: import.name.clone(),
                            }
                        }
                        FunctionKind::Local(LocalFunction { .. }) => FuncInfo {
                            func_kind: "local".to_string(),
                            module: "".to_string(),
                            name: func.name.clone().unwrap_or("".to_string()),
                        },
                        FunctionKind::Uninitialized(..) => FuncInfo {
                            func_kind: "uninitialized".to_string(),
                            module: "".to_string(),
                            name: "".to_string(),
                        },
                    };

                    // define static_data
                    loc_info.static_data.insert(
                        "target_imp_name".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.name.to_string(),
                            addr: None,
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_fn_type".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.func_kind.to_string(),
                            addr: None,
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_imp_module".to_string(),
                        Some(Value::Str {
                            ty: DataType::Str,
                            val: func_info.module.to_string(),
                            addr: None,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::CallIndirect => {
                if let Instr::CallIndirect(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalGet => {
                if let Instr::LocalGet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalSet => {
                if let Instr::LocalSet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LocalTee => {
                if let Instr::LocalTee(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalGet => {
                if let Instr::GlobalGet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalSet => {
                if let Instr::GlobalSet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Const => {
                if let Instr::Const(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Binop => {
                if let Instr::Binop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Unop => {
                if let Instr::Unop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Select => {
                if let Instr::Select(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Unreachable => {
                if let Instr::Unreachable(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Br => {
                if let Instr::Br(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::BrIf => {
                if let Instr::BrIf(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::IfElse => {
                if let Instr::IfElse(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::BrTable => {
                if let Instr::BrTable(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Drop => {
                if let Instr::Drop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Return => {
                if let Instr::Return(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemorySize => {
                if let Instr::MemorySize(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryGrow => {
                if let Instr::MemoryGrow(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryInit => {
                if let Instr::MemoryInit(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::DataDrop => {
                if let Instr::DataDrop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryCopy => {
                if let Instr::MemoryCopy(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryFill => {
                if let Instr::MemoryFill(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Load => {
                if let Instr::Load(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Store => {
                if let Instr::Store(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::AtomicRmw => {
                if let Instr::AtomicRmw(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Cmpxchg => {
                if let Instr::Cmpxchg(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::AtomicNotify => {
                if let Instr::AtomicNotify(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::AtomicWait => {
                if let Instr::AtomicWait(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::AtomicFence => {
                if let Instr::AtomicFence(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableGet => {
                if let Instr::TableGet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableSet => {
                if let Instr::TableSet(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableGrow => {
                if let Instr::TableGrow(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableSize => {
                if let Instr::TableSize(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableFill => {
                if let Instr::TableFill(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefNull => {
                if let Instr::RefNull(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefIsNull => {
                if let Instr::RefIsNull(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::RefFunc => {
                if let Instr::RefFunc(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::V128Bitselect => {
                if let Instr::V128Bitselect(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::I8x16Swizzle => {
                if let Instr::I8x16Swizzle(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::I8x16Shuffle => {
                if let Instr::I8x16Shuffle(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::LoadSimd => {
                if let Instr::LoadSimd(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableInit => {
                if let Instr::TableInit(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::ElemDrop => {
                if let Instr::ElemDrop(..) = instr {
                    // TODO define static vars
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::TableCopy => {
                if let Instr::TableCopy(..) = instr {
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
