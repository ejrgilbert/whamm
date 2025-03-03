use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Arg, Event, FromStr, LocInfo, Package, ProbeRule,
};
use crate::for_each_opcode;
use crate::generator::ast::{Probe, ReqArgs, WhammParam};
use crate::parser::rules::core::WhammModeKind;
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{BinOp, DataType, Definition, Expr, RulePart, Value};
use log::warn;
use orca_wasm::ir::id::{FunctionID, GlobalID, TypeID};
use orca_wasm::ir::module::module_functions::{FuncKind, ImportedFunction, LocalFunction};
use orca_wasm::ir::module::module_globals::{GlobalKind, ImportedGlobal, LocalGlobal};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::DataType as OrcaType;
use std::collections::{HashMap, HashSet};
use wasmparser::{GlobalType, MemArg, Operator};

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
    fn get_loc_info(
        &self,
        app_wasm: &Module,
        fid: &FunctionID,
        instr: &Operator,
    ) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();
        match self.kind {
            WasmPackageKind::Opcode => {
                // nothing to add
            }
        }

        // Get location info from the rest of the configured rules
        self.events.iter().for_each(|event| {
            if let Some(mut other_loc_info) = event.get_loc_info(app_wasm, fid, instr) {
                loc_info.append(&mut other_loc_info);
            }
        });

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }
    fn add_events(&mut self, ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<Probe>>>) {
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
    probes: HashMap<WhammModeKind, Vec<Probe>>,
}
macro_rules! define_opcode_event {
($($op:ident, $category:expr, $name:ident, $num_args:expr, $imms:expr, $globals:expr, $fns:expr, $supported_modes:expr, $req_map:expr, $docs:expr)*) => {
impl FromStr for OpcodeEvent {
    fn from_str(name: &str) -> Self {
        match name {
            $(stringify!($name) => Self::$name(),)*
             _ => panic!("unsupported OpcodeEvent: {name}"),
        }
    }
}
impl OpcodeEvent {
    // ======================
    // ---- Constructors ----
    // ======================
    fn new(kind: OpcodeEventKind) -> Self {
        Self {
            kind,
            probes: HashMap::new(),
        }
    }

    $(
    fn $name() -> Self {
        Self::new(OpcodeEventKind::$name())
    }
    )*
}
};}
for_each_opcode!(define_opcode_event);

impl OpcodeEvent {
    // =================
    // ---- Helpers ----
    // =================

    fn probe_rule(&self) -> ProbeRule {
        ProbeRule {
            provider: Some(RulePart::new("wasm".to_string(), None)),
            package: Some(RulePart::new("opcode".to_string(), None)),
            event: Some(RulePart::new(self.kind.name(), None)),
            mode: None,
        }
    }
    pub fn get_ty_info_for_instr(
        app_wasm: &Module,
        curr_fid: &FunctionID,
        instr: &Operator,
    ) -> (Vec<Arg>, Option<u32>) {
        // TODO -- how to make this less manual?
        let (ty_list, ty_id): (Vec<Option<OrcaType>>, Option<u32>) = match instr {
            Operator::Call {
                function_index: fid,
            } => {
                match app_wasm.functions.get_kind(FunctionID(*fid)) {
                    FuncKind::Import(ImportedFunction { ty_id, .. })
                    | FuncKind::Local(LocalFunction { ty_id, .. }) => {
                        if let Some(ty) = app_wasm.types.get(*ty_id) {
                            let mut res = vec![];
                            for t in ty.params().iter().rev() {
                                res.push(Some(*t));
                            }
                            (res, Some(**ty_id))
                        } else {
                            // no type info found!!
                            warn!("No type information found for import with FID {fid}");
                            (vec![], None)
                        }
                    }
                }
            }
            Operator::If { .. } | Operator::BrIf { .. } | Operator::BrTable { .. } => {
                (vec![Some(OrcaType::I32)], None)
            }
            Operator::Block {
                blockty: wasmparser::BlockType::FuncType(ty_id),
            }
            | Operator::Loop {
                blockty: wasmparser::BlockType::FuncType(ty_id),
            } => {
                if let Some(ty) = app_wasm.types.get(TypeID(*ty_id)) {
                    let mut res = vec![];
                    for t in ty.params().iter() {
                        res.push(Some(*t));
                    }
                    (res, Some(*ty_id))
                } else {
                    // no type info found!!
                    warn!("No type information found for opcode");
                    (vec![], None)
                }
            }
            Operator::CallIndirect { type_index, .. } => {
                if let Some(ty) = app_wasm.types.get(TypeID(*type_index)) {
                    let mut res = vec![];
                    for t in ty.params().iter().rev() {
                        res.push(Some(*t));
                    }
                    (res, Some(*type_index))
                } else {
                    // no type info found!!
                    warn!("No type information found for CallIndirect");
                    (vec![], None)
                }
            }
            Operator::Drop => {
                // TODO -- how to express an unknown type?
                //     Lookup in the symbol table! We've placed type bounds in there during verification
                //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
                //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
                // e.g. [unknown]
                (vec![None], None)
            }
            Operator::Select => {
                // TODO -- how to express an unknown type?
                //     Lookup in the symbol table! We've placed type bounds in there during verification
                //     HOWEVER, we will need to keep a virtual stack to check if this match site is in fact
                //     a match based on the type bounds. (if they don't match up, not a match, don't emit)
                // e.g. [unknown, unknown, i32]
                (vec![None, None, Some(OrcaType::I32)], None)
            }
            Operator::LocalSet { local_index } | Operator::LocalTee { local_index } => {
                if let FuncKind::Local(LocalFunction { body, .. }) =
                    app_wasm.functions.get_kind(*curr_fid)
                {
                    if let Some((_, ty)) = body.locals.get(*local_index as usize) {
                        (vec![Some(*ty)], None)
                    } else {
                        (vec![], None) // ignore
                    }
                } else {
                    (vec![], None) // ignore
                }
            }
            Operator::GlobalSet { global_index } => {
                let ty = match app_wasm.globals.get_kind(GlobalID(*global_index)) {
                    GlobalKind::Import(ImportedGlobal {
                        ty: GlobalType { content_type, .. },
                        ..
                    })
                    | GlobalKind::Local(LocalGlobal {
                        ty: GlobalType { content_type, .. },
                        ..
                    }) => OrcaType::from(*content_type),
                };
                (vec![Some(ty)], None)
            }
            Operator::I32Load { .. }
            | Operator::I64Load { .. }
            | Operator::F32Load { .. }
            | Operator::F64Load { .. }
            | Operator::I32Load8S { .. }
            | Operator::I32Load8U { .. }
            | Operator::I32Load16S { .. }
            | Operator::I32Load16U { .. }
            | Operator::I64Load8S { .. }
            | Operator::I64Load8U { .. }
            | Operator::I64Load16S { .. }
            | Operator::I64Load16U { .. }
            | Operator::I64Load32S { .. }
            | Operator::I64Load32U { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::I32Store { .. }
            | Operator::I32Store8 { .. }
            | Operator::I32Store16 { .. } => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),
            Operator::I64Store { .. }
            | Operator::I64Store8 { .. }
            | Operator::I64Store16 { .. }
            | Operator::I64Store32 { .. } => (vec![Some(OrcaType::I32), Some(OrcaType::I64)], None),
            Operator::F32Store { .. } => (vec![Some(OrcaType::I32), Some(OrcaType::F32)], None),
            Operator::F64Store { .. } => (vec![Some(OrcaType::I32), Some(OrcaType::F64)], None),
            Operator::MemoryGrow { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::I32Eqz => (vec![Some(OrcaType::I32)], None),
            Operator::I32Ne
            | Operator::I32Eq
            | Operator::I32LtS
            | Operator::I32LtU
            | Operator::I32GtS
            | Operator::I32GtU
            | Operator::I32LeS
            | Operator::I32LeU
            | Operator::I32GeS
            | Operator::I32GeU => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),

            Operator::I32Clz | Operator::I32Ctz | Operator::I32Popcnt => {
                (vec![Some(OrcaType::I32)], None)
            }

            Operator::I32Add
            | Operator::I32Sub
            | Operator::I32Mul
            | Operator::I32DivS
            | Operator::I32DivU
            | Operator::I32RemS
            | Operator::I32RemU
            | Operator::I32And
            | Operator::I32Or
            | Operator::I32Xor
            | Operator::I32Shl
            | Operator::I32ShrS
            | Operator::I32ShrU
            | Operator::I32Rotl
            | Operator::I32Rotr => (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None),

            Operator::I64Eqz => (vec![Some(OrcaType::I64)], None),
            Operator::I64Eq
            | Operator::I64Ne
            | Operator::I64LtS
            | Operator::I64LtU
            | Operator::I64GtS
            | Operator::I64GtU
            | Operator::I64LeS
            | Operator::I64LeU
            | Operator::I64GeS
            | Operator::I64GeU => (vec![Some(OrcaType::I64), Some(OrcaType::I64)], None),

            Operator::I64Clz | Operator::I64Ctz | Operator::I64Popcnt => {
                (vec![Some(OrcaType::I64)], None)
            }
            Operator::I64Add
            | Operator::I64Sub
            | Operator::I64Mul
            | Operator::I64DivS
            | Operator::I64DivU
            | Operator::I64RemS
            | Operator::I64RemU
            | Operator::I64And
            | Operator::I64Or
            | Operator::I64Xor
            | Operator::I64Shl
            | Operator::I64ShrS
            | Operator::I64ShrU
            | Operator::I64Rotl
            | Operator::I64Rotr => (vec![Some(OrcaType::I64), Some(OrcaType::I64)], None),

            Operator::F32Eq
            | Operator::F32Ne
            | Operator::F32Lt
            | Operator::F32Gt
            | Operator::F32Le
            | Operator::F32Ge => (vec![Some(OrcaType::F32), Some(OrcaType::F32)], None),

            Operator::F32Abs
            | Operator::F32Neg
            | Operator::F32Ceil
            | Operator::F32Floor
            | Operator::F32Trunc
            | Operator::F32Nearest
            | Operator::F32Sqrt => (vec![Some(OrcaType::F32)], None),
            Operator::F32Add
            | Operator::F32Sub
            | Operator::F32Mul
            | Operator::F32Div
            | Operator::F32Min
            | Operator::F32Max
            | Operator::F32Copysign => (vec![Some(OrcaType::F32), Some(OrcaType::F32)], None),

            Operator::F64Eq
            | Operator::F64Ne
            | Operator::F64Lt
            | Operator::F64Gt
            | Operator::F64Le
            | Operator::F64Ge => (vec![Some(OrcaType::F64), Some(OrcaType::F64)], None),

            Operator::F64Abs
            | Operator::F64Neg
            | Operator::F64Ceil
            | Operator::F64Floor
            | Operator::F64Trunc
            | Operator::F64Nearest
            | Operator::F64Sqrt => (vec![Some(OrcaType::F32)], None),
            Operator::F64Add
            | Operator::F64Sub
            | Operator::F64Mul
            | Operator::F64Div
            | Operator::F64Min
            | Operator::F64Max
            | Operator::F64Copysign => (vec![Some(OrcaType::F64), Some(OrcaType::F64)], None),

            Operator::I32WrapI64
            | Operator::F32ConvertI64S
            | Operator::F32ConvertI64U
            | Operator::F64ConvertI64S
            | Operator::F64ConvertI64U
            | Operator::F64ReinterpretI64
            | Operator::I64Extend8S
            | Operator::I64Extend16S
            | Operator::I64Extend32S => (vec![Some(OrcaType::I64)], None),
            Operator::I32TruncF32S | Operator::I32TruncF32U => (vec![Some(OrcaType::F32)], None),
            Operator::I32TruncF64S
            | Operator::I32TruncF64U
            | Operator::I64TruncF64S
            | Operator::I64TruncF64U
            | Operator::F32DemoteF64
            | Operator::I64ReinterpretF64
            | Operator::I32TruncSatF64S
            | Operator::I32TruncSatF64U
            | Operator::I64TruncSatF64S
            | Operator::I64TruncSatF64U => (vec![Some(OrcaType::F64)], None),
            Operator::I64ExtendI32S
            | Operator::I64ExtendI32U
            | Operator::F32ConvertI32S
            | Operator::F32ConvertI32U
            | Operator::F64ConvertI32S
            | Operator::F64ConvertI32U
            | Operator::F32ReinterpretI32
            | Operator::I32Extend8S
            | Operator::I32Extend16S => (vec![Some(OrcaType::I32)], None),
            Operator::I64TruncF32S
            | Operator::I64TruncF32U
            | Operator::F64PromoteF32
            | Operator::I32ReinterpretF32
            | Operator::I32TruncSatF32S
            | Operator::I32TruncSatF32U
            | Operator::I64TruncSatF32S
            | Operator::I64TruncSatF32U => (vec![Some(OrcaType::F32)], None),

            Operator::MemoryCopy { .. }
            | Operator::MemoryFill { .. }
            | Operator::TableInit { .. }
            | Operator::TableCopy { .. } => (
                vec![
                    Some(OrcaType::I32),
                    Some(OrcaType::I32),
                    Some(OrcaType::I32),
                ],
                None,
            ),

            Operator::TableGet { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::MemoryAtomicNotify { .. } => {
                (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
            }
            Operator::MemoryAtomicWait32 { .. } => (
                vec![
                    Some(OrcaType::I32),
                    Some(OrcaType::I32),
                    Some(OrcaType::I64),
                ],
                None,
            ),
            Operator::MemoryAtomicWait64 { .. } => (
                vec![
                    Some(OrcaType::I32),
                    Some(OrcaType::I64),
                    Some(OrcaType::I64),
                ],
                None,
            ),

            Operator::I32AtomicLoad { .. }
            | Operator::I64AtomicLoad { .. }
            | Operator::I32AtomicLoad8U { .. }
            | Operator::I32AtomicLoad16U { .. }
            | Operator::I64AtomicLoad8U { .. }
            | Operator::I64AtomicLoad16U { .. }
            | Operator::I64AtomicLoad32U { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::I32AtomicStore { .. }
            | Operator::I32AtomicStore8 { .. }
            | Operator::I32AtomicStore16 { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::I64AtomicStore { .. }
            | Operator::I64AtomicStore8 { .. }
            | Operator::I64AtomicStore16 { .. }
            | Operator::I64AtomicStore32 { .. } => (vec![Some(OrcaType::I32)], None),

            Operator::I32AtomicRmwAdd { .. }
            | Operator::I32AtomicRmw8AddU { .. }
            | Operator::I32AtomicRmw16AddU { .. }
            | Operator::I32AtomicRmwSub { .. }
            | Operator::I32AtomicRmw8SubU { .. }
            | Operator::I32AtomicRmw16SubU { .. }
            | Operator::I32AtomicRmwAnd { .. }
            | Operator::I32AtomicRmw8AndU { .. }
            | Operator::I32AtomicRmw16AndU { .. }
            | Operator::I32AtomicRmwOr { .. }
            | Operator::I32AtomicRmw8OrU { .. }
            | Operator::I32AtomicRmw16OrU { .. }
            | Operator::I32AtomicRmwXor { .. }
            | Operator::I32AtomicRmw8XorU { .. }
            | Operator::I32AtomicRmw16XorU { .. }
            | Operator::I32AtomicRmwXchg { .. }
            | Operator::I32AtomicRmw8XchgU { .. }
            | Operator::I32AtomicRmw16XchgU { .. }
            | Operator::I32AtomicRmwCmpxchg { .. }
            | Operator::I32AtomicRmw8CmpxchgU { .. }
            | Operator::I32AtomicRmw16CmpxchgU { .. } => {
                (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
            }

            Operator::I64AtomicRmwAdd { .. }
            | Operator::I64AtomicRmw8AddU { .. }
            | Operator::I64AtomicRmw16AddU { .. }
            | Operator::I64AtomicRmw32AddU { .. }
            | Operator::I64AtomicRmwSub { .. }
            | Operator::I64AtomicRmw8SubU { .. }
            | Operator::I64AtomicRmw16SubU { .. }
            | Operator::I64AtomicRmw32SubU { .. }
            | Operator::I64AtomicRmwAnd { .. }
            | Operator::I64AtomicRmw8AndU { .. }
            | Operator::I64AtomicRmw16AndU { .. }
            | Operator::I64AtomicRmw32AndU { .. }
            | Operator::I64AtomicRmwOr { .. }
            | Operator::I64AtomicRmw8OrU { .. }
            | Operator::I64AtomicRmw16OrU { .. }
            | Operator::I64AtomicRmw32OrU { .. }
            | Operator::I64AtomicRmwXor { .. }
            | Operator::I64AtomicRmw8XorU { .. }
            | Operator::I64AtomicRmw16XorU { .. }
            | Operator::I64AtomicRmw32XorU { .. }
            | Operator::I64AtomicRmwXchg { .. }
            | Operator::I64AtomicRmw8XchgU { .. }
            | Operator::I64AtomicRmw16XchgU { .. }
            | Operator::I64AtomicRmw32XchgU { .. }
            | Operator::I64AtomicRmwCmpxchg { .. }
            | Operator::I64AtomicRmw8CmpxchgU { .. }
            | Operator::I64AtomicRmw16CmpxchgU { .. }
            | Operator::I64AtomicRmw32CmpxchgU { .. } => {
                (vec![Some(OrcaType::I32), Some(OrcaType::I32)], None)
            }

            Operator::Unreachable
            | Operator::Nop
            | Operator::Else
            | Operator::End
            | Operator::Br { .. }
            | Operator::Return
            | Operator::LocalGet { .. }
            | Operator::GlobalGet { .. }
            | Operator::MemorySize { .. }
            | Operator::I32Const { .. }
            | Operator::I64Const { .. }
            | Operator::F32Const { .. }
            | Operator::F64Const { .. }
            | Operator::StructNewDefault { .. }
            | Operator::MemoryInit { .. }
            | Operator::DataDrop { .. }
            | Operator::ElemDrop { .. }
            | Operator::RefNull { .. }
            | Operator::RefFunc { .. }
            | Operator::TableSize { .. }
            | Operator::AtomicFence => (vec![], None),

            _ => (vec![], None), // ignore other opcodes
        };

        let mut args = vec![];
        for (idx, ty) in ty_list.iter().enumerate() {
            args.push(Arg::new(format!("arg{}", idx), ty.to_owned()));
        }

        (args, ty_id)
    }
}
impl Event for OpcodeEvent {
    fn get_loc_info(
        &self,
        app_wasm: &Module,
        curr_fid: &FunctionID,
        instr: &Operator,
    ) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();

        // create a combination of WhammParams for all probes here
        let mut all_params = HashSet::new();
        for (_, probes) in self.probes.iter() {
            for Probe { metadata, .. } in probes.iter() {
                for param in metadata.body_args.params.iter() {
                    all_params.insert(param);
                }

                for param in metadata.pred_args.params.iter() {
                    all_params.insert(param);
                }
            }
        }
        let mut req_args = ReqArgs::None;

        match self.kind {
            OpcodeEventKind::Unreachable { .. } => {
                if let Operator::Unreachable = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Nop { .. } => {
                if let Operator::Nop = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Block { .. } => {
                if let Operator::Block { blockty: _ } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Loop { .. } => {
                if let Operator::Loop { blockty: _ } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::If { .. } => {
                if let Operator::If { blockty: _ } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Else { .. } => {
                if let Operator::Else = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TryTable { .. } => {
                if let Operator::TryTable { try_table: _ } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Throw { .. } => {
                if let Operator::Throw { tag_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*tag_index)), &mut loc_info);
                        }
                    }

                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ThrowRef { .. } => {
                if let Operator::ThrowRef = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::End { .. } => {
                if let Operator::End = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Br { .. } => {
                if let Operator::Br { relative_depth } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrIf { .. } => {
                if let Operator::BrIf { relative_depth } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrTable { .. } => {
                if let Operator::BrTable { targets } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::NumTargets => {
                                loc_info.static_data.insert(
                                    WhammParam::NumTargets.to_string(),
                                    Some(Value::gen_u32(targets.len())),
                                );
                            }
                            WhammParam::DefaultTarget => {
                                loc_info.static_data.insert(
                                    WhammParam::DefaultTarget.to_string(),
                                    Some(Value::gen_u32(targets.default())),
                                );
                            }
                            WhammParam::Targets => {
                                let mut target_map = HashMap::new();
                                for (i, target) in targets.targets().enumerate() {
                                    if let Ok(target) = target {
                                        target_map.insert(i as u32, target);
                                    }
                                }
                                loc_info.add_dynamic_value(
                                    WhammParam::Targets.to_string(),
                                    Value::U32U32Map {
                                        val: Box::new(target_map),
                                    },
                                );
                            }
                            WhammParam::Imm { n, ty } => {
                                if *n > targets.len() {
                                    // this location doesn't match since the immN is out of bound
                                    // of the immN's available
                                    return None;
                                }
                                assert!(matches!(ty, DataType::U32));

                                if *n == targets.len() {
                                    // requesting the default value!
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(targets.default())),
                                        &mut loc_info,
                                    );
                                }

                                for (i, target) in targets.targets().enumerate() {
                                    if let Ok(target) = target {
                                        if *n == i as u32 {
                                            define_imm_n(
                                                i as u32,
                                                Some(Value::gen_u32(target)),
                                                &mut loc_info,
                                            );
                                            break;
                                        }
                                    }
                                }
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Return { .. } => {
                if let Operator::Return = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Call { .. } => {
                if let Operator::Call {
                    function_index: fid,
                } = instr
                {
                    let func_info = match app_wasm.functions.get_kind(FunctionID(*fid)) {
                        FuncKind::Import(ImportedFunction { import_id, .. }) => {
                            let import = app_wasm.imports.get(*import_id);
                            FuncInfo {
                                func_kind: "import".to_string(),
                                module: import.module.to_string(),
                                name: import.name.to_string(),
                            }
                        }
                        FuncKind::Local(LocalFunction { func_id, .. }) => FuncInfo {
                            func_kind: "local".to_string(),
                            module: match &app_wasm.module_name {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                            name: match &app_wasm.functions.get_name(*func_id) {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                        },
                    };

                    for param in all_params {
                        match param {
                            WhammParam::TargetFnName => {
                                loc_info.static_data.insert(
                                    "target_fn_name".to_string(),
                                    Some(Value::Str {
                                        val: func_info.name.to_string(),
                                    }),
                                );
                            }
                            WhammParam::TargetFnType => {
                                loc_info.static_data.insert(
                                    "target_fn_type".to_string(),
                                    Some(Value::Str {
                                        val: func_info.func_kind.to_string(),
                                    }),
                                );
                            }
                            WhammParam::TargetImpModule => {
                                loc_info.static_data.insert(
                                    "target_imp_module".to_string(),
                                    Some(Value::Str {
                                        val: func_info.module.to_string(),
                                    }),
                                );
                            }
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(0, Some(Value::gen_u32(*fid)), &mut loc_info);
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::CallIndirect { .. } => {
                if let Operator::CallIndirect {
                    type_index,
                    table_index,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*type_index)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*table_index)), &mut loc_info);
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ReturnCall { .. } => {
                if let Operator::ReturnCall {
                    function_index: fid,
                } = instr
                {
                    let func_info = match app_wasm.functions.get_kind(FunctionID(*fid)) {
                        FuncKind::Import(ImportedFunction { import_id, .. }) => {
                            let import = app_wasm.imports.get(*import_id);
                            FuncInfo {
                                func_kind: "import".to_string(),
                                module: import.module.to_string(),
                                name: import.name.to_string(),
                            }
                        }
                        FuncKind::Local(LocalFunction { func_id, .. }) => FuncInfo {
                            func_kind: "local".to_string(),
                            module: match &app_wasm.module_name {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                            name: match &app_wasm.functions.get_name(*func_id) {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                        },
                    };

                    for param in all_params {
                        match param {
                            WhammParam::TargetFnName => {
                                loc_info.static_data.insert(
                                    "target_fn_name".to_string(),
                                    Some(Value::Str {
                                        val: func_info.name.to_string(),
                                    }),
                                );
                            }
                            WhammParam::TargetFnType => {
                                loc_info.static_data.insert(
                                    "target_fn_type".to_string(),
                                    Some(Value::Str {
                                        val: func_info.func_kind.to_string(),
                                    }),
                                );
                            }
                            WhammParam::TargetImpModule => {
                                loc_info.static_data.insert(
                                    "target_imp_module".to_string(),
                                    Some(Value::Str {
                                        val: func_info.module.to_string(),
                                    }),
                                );
                            }
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(0, Some(Value::gen_u32(*fid)), &mut loc_info);
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ReturnCallIndirect { .. } => {
                if let Operator::ReturnCallIndirect {
                    type_index,
                    table_index,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*type_index)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*table_index)), &mut loc_info);
                            } else {
                                panic!(
                                    "WhammParam not available for ReturnCallIndirect opcode: {}",
                                    param
                                );
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Drop { .. } => {
                if let Operator::Drop = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Select { .. } => {
                if let Operator::Select = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TypedSelect { .. } => {
                if let Operator::TypedSelect { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalGet { .. } => {
                if let Operator::LocalGet { local_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*local_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalSet { .. } => {
                if let Operator::LocalSet { local_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*local_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalTee { .. } => {
                if let Operator::LocalTee { local_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*local_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalGet { .. } => {
                if let Operator::GlobalGet { global_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*global_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalSet { .. } => {
                if let Operator::GlobalSet { global_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*global_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load { .. } => {
                if let Operator::I32Load {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load { .. } => {
                if let Operator::I64Load {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Load { .. } => {
                if let Operator::F32Load {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load8S { .. } => {
                if let Operator::I32Load8S {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load8U { .. } => {
                if let Operator::I32Load8U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load16S { .. } => {
                if let Operator::I32Load16S {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load16U { .. } => {
                if let Operator::I32Load16U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load8S { .. } => {
                if let Operator::I64Load8S {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load8U { .. } => {
                if let Operator::I64Load8U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load16S { .. } => {
                if let Operator::I64Load16S {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load16U { .. } => {
                if let Operator::I64Load16U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load32S { .. } => {
                if let Operator::I64Load32S {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load32U { .. } => {
                if let Operator::I64Load32U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store { .. } => {
                if let Operator::I32Store {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store { .. } => {
                if let Operator::I64Store {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Store { .. } => {
                if let Operator::F32Store {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Store { .. } => {
                if let Operator::F64Store {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store8 { .. } => {
                if let Operator::I32Store8 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store16 { .. } => {
                if let Operator::I32Store16 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store8 { .. } => {
                if let Operator::I64Store8 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store16 { .. } => {
                if let Operator::I64Store16 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store32 { .. } => {
                if let Operator::I64Store32 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    req_args =
                        create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }

            OpcodeEventKind::MemorySize { .. } => {
                if let Operator::MemorySize { mem } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*mem)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryGrow { .. } => {
                if let Operator::MemoryGrow { mem } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*mem)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Const { .. } => {
                if let Operator::I32Const { value } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::I32));

                            define_imm_n(0, Some(Value::gen_i32(*value)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Const { .. } => {
                if let Operator::I64Const { value } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::I64));

                            define_imm_n(0, Some(Value::gen_i64(*value)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Const { .. } => {
                if let Operator::F32Const { value } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::F32));

                            define_imm_n(0, Some(Value::gen_f32(f32::from(*value))), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Const { .. } => {
                if let Operator::F64Const { value } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::F64));

                            define_imm_n(0, Some(Value::gen_f64(f64::from(*value))), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefNull { .. } => {
                if let Operator::RefNull { .. } = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefIsNull { .. } => {
                if let Operator::RefNull { .. } = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefFunc { .. } => {
                if let Operator::RefFunc { function_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*function_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefEq { .. } => {
                if let Operator::RefEq = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Eqz { .. } => {
                if let Operator::I32Eqz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Eq { .. } => {
                if let Operator::I32Eq = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Ne { .. } => {
                if let Operator::I32Ne = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32LtS { .. } => {
                if let Operator::I32LtS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32LtU { .. } => {
                if let Operator::I32LtU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32GtS { .. } => {
                if let Operator::I32GtS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32GtU { .. } => {
                if let Operator::I32GtU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32LeS { .. } => {
                if let Operator::I32LeS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32LeU { .. } => {
                if let Operator::I32LeU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32GeS { .. } => {
                if let Operator::I32GeS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32GeU { .. } => {
                if let Operator::I32GeU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Eqz { .. } => {
                if let Operator::I64Eqz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Eq { .. } => {
                if let Operator::I64Eq = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Ne { .. } => {
                if let Operator::I64Ne = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64LtS { .. } => {
                if let Operator::I64LtS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64LtU { .. } => {
                if let Operator::I64LtU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64GtS { .. } => {
                if let Operator::I64GtS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64GtU { .. } => {
                if let Operator::I64GtU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64LeS { .. } => {
                if let Operator::I64LeS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64LeU { .. } => {
                if let Operator::I64LeU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64GeS { .. } => {
                if let Operator::I64GeS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64GeU { .. } => {
                if let Operator::I64GeU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Eq { .. } => {
                if let Operator::F32Eq = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Ne { .. } => {
                if let Operator::F32Ne = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Lt { .. } => {
                if let Operator::F32Lt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Gt { .. } => {
                if let Operator::F32Gt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Le { .. } => {
                if let Operator::F32Le = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Ge { .. } => {
                if let Operator::F32Ge = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Eq { .. } => {
                if let Operator::F64Eq = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Ne { .. } => {
                if let Operator::F64Ne = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Lt { .. } => {
                if let Operator::F64Lt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Gt { .. } => {
                if let Operator::F64Gt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Le { .. } => {
                if let Operator::F64Le = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Ge { .. } => {
                if let Operator::F64Ge = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Clz { .. } => {
                if let Operator::I32Clz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Ctz { .. } => {
                if let Operator::I32Ctz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Popcnt { .. } => {
                if let Operator::I32Popcnt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Add { .. } => {
                if let Operator::I32Add = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Sub { .. } => {
                if let Operator::I32Sub = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Mul { .. } => {
                if let Operator::I32Mul = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32DivS { .. } => {
                if let Operator::I32DivS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32DivU { .. } => {
                if let Operator::I32DivU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32RemS { .. } => {
                if let Operator::I32RemS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32RemU { .. } => {
                if let Operator::I32RemU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32And { .. } => {
                if let Operator::I32And = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Or { .. } => {
                if let Operator::I32Or = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Xor { .. } => {
                if let Operator::I32Xor = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Shl { .. } => {
                if let Operator::I32Shl = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32ShrS { .. } => {
                if let Operator::I32ShrS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32ShrU { .. } => {
                if let Operator::I32ShrU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Rotl { .. } => {
                if let Operator::I32Rotl = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Rotr { .. } => {
                if let Operator::I32Rotr = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Clz { .. } => {
                if let Operator::I64Clz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Ctz { .. } => {
                if let Operator::I64Ctz = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Popcnt { .. } => {
                if let Operator::I64Popcnt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Add { .. } => {
                if let Operator::I64Add = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Sub { .. } => {
                if let Operator::I64Sub = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Mul { .. } => {
                if let Operator::I64Mul = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64DivS { .. } => {
                if let Operator::I64DivS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64DivU { .. } => {
                if let Operator::I64DivU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64RemS { .. } => {
                if let Operator::I64RemS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64RemU { .. } => {
                if let Operator::I64RemU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64And { .. } => {
                if let Operator::I64And = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Or { .. } => {
                if let Operator::I64Or = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Xor { .. } => {
                if let Operator::I64Xor = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Shl { .. } => {
                if let Operator::I64Shl = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64ShrS { .. } => {
                if let Operator::I64ShrS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64ShrU { .. } => {
                if let Operator::I64ShrU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Rotl { .. } => {
                if let Operator::I64Rotl = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Rotr { .. } => {
                if let Operator::I64Rotr = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Abs { .. } => {
                if let Operator::F32Abs = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Neg { .. } => {
                if let Operator::F32Neg = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Ceil { .. } => {
                if let Operator::F32Ceil = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Floor { .. } => {
                if let Operator::F32Floor = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Trunc { .. } => {
                if let Operator::F32Trunc = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Nearest { .. } => {
                if let Operator::F32Nearest = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Sqrt { .. } => {
                if let Operator::F32Sqrt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Add { .. } => {
                if let Operator::F32Add = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Sub { .. } => {
                if let Operator::F32Sub = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Mul { .. } => {
                if let Operator::F32Mul = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Div { .. } => {
                if let Operator::F32Div = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Min { .. } => {
                if let Operator::F32Min = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Max { .. } => {
                if let Operator::F32Max = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Copysign { .. } => {
                if let Operator::F32Copysign = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Abs { .. } => {
                if let Operator::F64Abs = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Neg { .. } => {
                if let Operator::F64Neg = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Ceil { .. } => {
                if let Operator::F64Ceil = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Floor { .. } => {
                if let Operator::F64Floor = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Trunc { .. } => {
                if let Operator::F64Trunc = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Nearest { .. } => {
                if let Operator::F64Nearest = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Sqrt { .. } => {
                if let Operator::F64Sqrt = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Add { .. } => {
                if let Operator::F64Add = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Sub { .. } => {
                if let Operator::F64Sub = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Mul { .. } => {
                if let Operator::F64Mul = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Div { .. } => {
                if let Operator::F64Div = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Min { .. } => {
                if let Operator::F64Min = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Max { .. } => {
                if let Operator::F64Max = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Copysign { .. } => {
                if let Operator::F64Copysign = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32WrapI64 { .. } => {
                if let Operator::I32WrapI64 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncF32S { .. } => {
                if let Operator::I32TruncF32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncF32U { .. } => {
                if let Operator::I32TruncF32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncF64S { .. } => {
                if let Operator::I32TruncF64S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncF64U { .. } => {
                if let Operator::I32TruncF64U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64ExtendI32S { .. } => {
                if let Operator::I64ExtendI32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64ExtendI32U { .. } => {
                if let Operator::I64ExtendI32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncF32S { .. } => {
                if let Operator::I64TruncF32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncF32U { .. } => {
                if let Operator::I64TruncF32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32ConvertI32S { .. } => {
                if let Operator::F32ConvertI32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32ConvertI32U { .. } => {
                if let Operator::F32ConvertI32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32ConvertI64S { .. } => {
                if let Operator::F32ConvertI64S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32ConvertI64U { .. } => {
                if let Operator::F32ConvertI64U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32DemoteF64 { .. } => {
                if let Operator::F32DemoteF64 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64ConvertI32S { .. } => {
                if let Operator::F64ConvertI32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64ConvertI32U { .. } => {
                if let Operator::F64ConvertI32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64ConvertI64S { .. } => {
                if let Operator::F64ConvertI64S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64ConvertI64U { .. } => {
                if let Operator::F64ConvertI64U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64PromoteF32 { .. } => {
                if let Operator::F64PromoteF32 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32ReinterpretF32 { .. } => {
                if let Operator::I32ReinterpretF32 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64ReinterpretF64 { .. } => {
                if let Operator::I64ReinterpretF64 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32ReinterpretI32 { .. } => {
                if let Operator::F32ReinterpretI32 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64ReinterpretI64 { .. } => {
                if let Operator::F64ReinterpretI64 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Extend8S { .. } => {
                if let Operator::I32Extend8S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Extend16S { .. } => {
                if let Operator::I32Extend16S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Extend8S { .. } => {
                if let Operator::I64Extend8S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Extend16S { .. } => {
                if let Operator::I64Extend16S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Extend32S { .. } => {
                if let Operator::I64Extend32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructNew { .. } => {
                if let Operator::StructNew { struct_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*struct_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructNewDefault { .. } => {
                if let Operator::StructNewDefault { struct_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*struct_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGet { .. } => {
                if let Operator::StructGet {
                    struct_type_index,
                    field_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*struct_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*field_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            WhammParam::FieldIdx => {
                                loc_info.static_data.insert(
                                    "field_idx".to_string(),
                                    Some(Value::gen_u32(*field_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGetS { .. } => {
                if let Operator::StructGetS {
                    struct_type_index,
                    field_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*struct_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*field_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            WhammParam::FieldIdx => {
                                loc_info.static_data.insert(
                                    "field_idx".to_string(),
                                    Some(Value::gen_u32(*field_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGetU { .. } => {
                if let Operator::StructGetU {
                    struct_type_index,
                    field_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*struct_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*field_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            WhammParam::FieldIdx => {
                                loc_info.static_data.insert(
                                    "field_idx".to_string(),
                                    Some(Value::gen_u32(*field_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructSet { .. } => {
                if let Operator::StructSet {
                    struct_type_index,
                    field_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*struct_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*field_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*struct_type_index)),
                                );
                            }
                            WhammParam::FieldIdx => {
                                loc_info.static_data.insert(
                                    "field_idx".to_string(),
                                    Some(Value::gen_u32(*field_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayNew { .. } => {
                if let Operator::ArrayNew { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayNewDefault { .. } => {
                if let Operator::ArrayNewDefault { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));

                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayNewFixed { .. } => {
                if let Operator::ArrayNewFixed {
                    array_type_index,
                    array_size,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_size)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayNewData { .. } => {
                if let Operator::ArrayNewData {
                    array_type_index,
                    array_data_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_data_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayNewElem { .. } => {
                if let Operator::ArrayNewElem {
                    array_type_index,
                    array_elem_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_elem_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayGet { .. } => {
                if let Operator::ArrayGet { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));
                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayGetS { .. } => {
                if let Operator::ArrayGetS { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));
                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayGetU { .. } => {
                if let Operator::ArrayGetU { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));
                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArraySet { .. } => {
                if let Operator::ArraySet { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));
                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayLen { .. } => {
                if let Operator::ArrayLen = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayFill { .. } => {
                if let Operator::ArrayFill { array_type_index } = instr {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert_eq!(*n, 0);
                                assert!(matches!(ty, DataType::U32));
                                define_imm_n(
                                    0,
                                    Some(Value::gen_u32(*array_type_index)),
                                    &mut loc_info,
                                );
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayCopy { .. } => {
                if let Operator::ArrayCopy {
                    array_type_index_dst,
                    array_type_index_src,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(
                                    *n,
                                    Some(Value::gen_u32(*array_type_index_dst)),
                                    &mut loc_info,
                                );
                            } else if *n == 1 {
                                define_imm_n(
                                    *n,
                                    Some(Value::gen_u32(*array_type_index_src)),
                                    &mut loc_info,
                                );
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayInitData { .. } => {
                if let Operator::ArrayInitData {
                    array_type_index,
                    array_data_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_data_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ArrayInitElem { .. } => {
                if let Operator::ArrayInitElem {
                    array_type_index,
                    array_elem_index,
                } = instr
                {
                    for param in all_params {
                        match param {
                            WhammParam::Imm { n, ty } => {
                                assert!(matches!(ty, DataType::U32));
                                if *n == 0 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_type_index)),
                                        &mut loc_info,
                                    );
                                } else if *n == 1 {
                                    define_imm_n(
                                        *n,
                                        Some(Value::gen_u32(*array_elem_index)),
                                        &mut loc_info,
                                    );
                                } else {
                                    panic!("WhammParam not available for opcode: {}", param);
                                }
                            }
                            WhammParam::Tid => {
                                loc_info.static_data.insert(
                                    "tid".to_string(),
                                    Some(Value::gen_u32(*array_type_index)),
                                );
                            }
                            // other => if matches!(param.def(), Definition::CompilerStatic) {
                            //     panic!("WhammParam not supported for opcode: {}", other);
                            // }
                            _ => {}
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefTest { .. } => {
                if let Operator::RefTestNonNull { hty: _ } | Operator::RefTestNullable { hty: _ } =
                    instr
                {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefCast { .. } => {
                if let Operator::RefCastNonNull { hty: _ } | Operator::RefCastNullable { hty: _ } =
                    instr
                {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrOnCast { .. } => {
                if let Operator::BrOnCast {
                    relative_depth,
                    from_ref_type: _,
                    to_ref_type: _,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrOnCastFail { .. } => {
                if let Operator::BrOnCastFail {
                    relative_depth,
                    from_ref_type: _,
                    to_ref_type: _,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::AnyConvertExtern { .. } => {
                if let Operator::AnyConvertExtern = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ExternConvertAny { .. } => {
                if let Operator::ExternConvertAny = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefI31 { .. } => {
                if let Operator::RefI31 = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I31GetS { .. } => {
                if let Operator::I31GetS = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I31GetU { .. } => {
                if let Operator::I31GetU = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncSatF32S { .. } => {
                if let Operator::I32TruncSatF32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncSatF32U { .. } => {
                if let Operator::I32TruncSatF32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncSatF64S { .. } => {
                if let Operator::I32TruncSatF64S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32TruncSatF64U { .. } => {
                if let Operator::I32TruncSatF64U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncSatF32S { .. } => {
                if let Operator::I64TruncSatF32S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncSatF32U { .. } => {
                if let Operator::I64TruncSatF32U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncSatF64S { .. } => {
                if let Operator::I64TruncSatF64S = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64TruncSatF64U { .. } => {
                if let Operator::I64TruncSatF64U = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryInit { .. } => {
                if let Operator::MemoryInit { data_index, mem } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*data_index)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*mem)), &mut loc_info);
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryCopy { .. } => {
                if let Operator::MemoryCopy { dst_mem, src_mem } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*dst_mem)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*src_mem)), &mut loc_info);
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryFill { .. } => {
                if let Operator::MemoryFill { mem } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*mem)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::DataDrop { .. } => {
                if let Operator::DataDrop { data_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*data_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ElemDrop { .. } => {
                if let Operator::ElemDrop { elem_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*elem_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableCopy { .. } => {
                if let Operator::TableCopy {
                    dst_table,
                    src_table,
                } = instr
                {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*dst_table)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*src_table)), &mut loc_info);
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableInit { .. } => {
                if let Operator::TableInit { elem_index, table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert!(matches!(ty, DataType::U32));
                            if *n == 0 {
                                define_imm_n(*n, Some(Value::gen_u32(*elem_index)), &mut loc_info);
                            } else if *n == 1 {
                                define_imm_n(*n, Some(Value::gen_u32(*table)), &mut loc_info);
                            } else {
                                panic!("WhammParam not available for opcode: {}", param);
                            }
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableFill { .. } => {
                if let Operator::TableFill { table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*table)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableGet { .. } => {
                if let Operator::TableGet { table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*table)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableSet { .. } => {
                if let Operator::TableSet { table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*table)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableGrow { .. } => {
                if let Operator::TableGrow { table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*table)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TableSize { .. } => {
                if let Operator::TableSize { table } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));
                            define_imm_n(0, Some(Value::gen_u32(*table)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryAtomicNotify { .. } => {
                if let Operator::MemoryAtomicNotify {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryAtomicWait32 { .. } => {
                if let Operator::MemoryAtomicWait32 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryAtomicWait64 { .. } => {
                if let Operator::MemoryAtomicWait64 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::AtomicFence { .. } => {
                if let Operator::AtomicFence = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicLoad { .. } => {
                if let Operator::I32AtomicLoad {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicLoad { .. } => {
                if let Operator::I64AtomicLoad {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicLoad8U { .. } => {
                if let Operator::I32AtomicLoad8U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicLoad16U { .. } => {
                if let Operator::I32AtomicLoad16U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicLoad8U { .. } => {
                if let Operator::I64AtomicLoad8U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicLoad16U { .. } => {
                if let Operator::I64AtomicLoad16U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicLoad32U { .. } => {
                if let Operator::I64AtomicLoad32U {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicStore { .. } => {
                if let Operator::I32AtomicStore {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicStore8 { .. } => {
                if let Operator::I32AtomicStore8 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicStore16 { .. } => {
                if let Operator::I32AtomicStore16 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicStore { .. } => {
                if let Operator::I64AtomicStore {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicStore8 { .. } => {
                if let Operator::I64AtomicStore8 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicStore16 { .. } => {
                if let Operator::I64AtomicStore16 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicStore32 { .. } => {
                if let Operator::I64AtomicStore32 {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwAdd { .. } => {
                if let Operator::I32AtomicRmwAdd {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8AddU { .. } => {
                if let Operator::I32AtomicRmw8AddU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16AddU { .. } => {
                if let Operator::I32AtomicRmw16AddU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwAdd { .. } => {
                if let Operator::I64AtomicRmwAdd {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8AddU { .. } => {
                if let Operator::I64AtomicRmw8AddU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16AddU { .. } => {
                if let Operator::I64AtomicRmw16AddU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32AddU { .. } => {
                if let Operator::I64AtomicRmw32AddU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwSub { .. } => {
                if let Operator::I32AtomicRmwSub {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8SubU { .. } => {
                if let Operator::I32AtomicRmw8SubU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16SubU { .. } => {
                if let Operator::I32AtomicRmw16SubU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwSub { .. } => {
                if let Operator::I64AtomicRmwSub {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8SubU { .. } => {
                if let Operator::I64AtomicRmw8SubU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16SubU { .. } => {
                if let Operator::I64AtomicRmw16SubU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32SubU { .. } => {
                if let Operator::I64AtomicRmw32SubU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwAnd { .. } => {
                if let Operator::I32AtomicRmwAnd {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8AndU { .. } => {
                if let Operator::I32AtomicRmw8AndU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16AndU { .. } => {
                if let Operator::I32AtomicRmw16AndU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwAnd { .. } => {
                if let Operator::I64AtomicRmwAnd {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8AndU { .. } => {
                if let Operator::I64AtomicRmw8AndU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16AndU { .. } => {
                if let Operator::I64AtomicRmw16AndU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32AndU { .. } => {
                if let Operator::I64AtomicRmw32AndU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwOr { .. } => {
                if let Operator::I32AtomicRmwOr {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8OrU { .. } => {
                if let Operator::I32AtomicRmw8OrU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16OrU { .. } => {
                if let Operator::I32AtomicRmw16OrU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwOr { .. } => {
                if let Operator::I64AtomicRmwOr {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8OrU { .. } => {
                if let Operator::I64AtomicRmw8OrU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16OrU { .. } => {
                if let Operator::I64AtomicRmw16OrU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32OrU { .. } => {
                if let Operator::I64AtomicRmw32OrU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwXor { .. } => {
                if let Operator::I32AtomicRmwXor {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8XorU { .. } => {
                if let Operator::I32AtomicRmw8XorU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16XorU { .. } => {
                if let Operator::I32AtomicRmw16XorU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwXor { .. } => {
                if let Operator::I64AtomicRmwXor {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8XorU { .. } => {
                if let Operator::I64AtomicRmw8XorU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16XorU { .. } => {
                if let Operator::I64AtomicRmw16XorU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32XorU { .. } => {
                if let Operator::I64AtomicRmw32XorU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwXchg { .. } => {
                if let Operator::I32AtomicRmwXchg {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8XchgU { .. } => {
                if let Operator::I32AtomicRmw8XchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16XchgU { .. } => {
                if let Operator::I32AtomicRmw16XchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwXchg { .. } => {
                if let Operator::I64AtomicRmwXchg {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8XchgU { .. } => {
                if let Operator::I64AtomicRmw8XchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16XchgU { .. } => {
                if let Operator::I64AtomicRmw16XchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32XchgU { .. } => {
                if let Operator::I64AtomicRmw32XchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmwCmpxchg { .. } => {
                if let Operator::I32AtomicRmwCmpxchg {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw8CmpxchgU { .. } => {
                if let Operator::I32AtomicRmw8CmpxchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32AtomicRmw16CmpxchgU { .. } => {
                if let Operator::I32AtomicRmw16CmpxchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmwCmpxchg { .. } => {
                if let Operator::I64AtomicRmwCmpxchg {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw8CmpxchgU { .. } => {
                if let Operator::I64AtomicRmw8CmpxchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw16CmpxchgU { .. } => {
                if let Operator::I64AtomicRmw16CmpxchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64AtomicRmw32CmpxchgU { .. } => {
                if let Operator::I64AtomicRmw32CmpxchgU {
                    memarg:
                        MemArg {
                            align,
                            offset,
                            memory,
                            ..
                        },
                } = instr
                {
                    create_memarg_globals(&all_params, &mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::CallRef { .. } => {
                if let Operator::CallRef { type_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*type_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ReturnCallRef { .. } => {
                if let Operator::ReturnCallRef { type_index } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*type_index)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::RefAsNonNull { .. } => {
                if let Operator::RefAsNonNull = instr {
                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrOnNull { .. } => {
                if let Operator::BrOnNull { relative_depth } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(*n, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrOnNonNull { .. } => {
                if let Operator::BrOnNonNull { relative_depth } = instr {
                    for param in all_params {
                        if let WhammParam::Imm { n, ty } = param {
                            assert_eq!(*n, 0);
                            assert!(matches!(ty, DataType::U32));

                            define_imm_n(0, Some(Value::gen_u32(*relative_depth)), &mut loc_info);
                        }
                    }

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
        }

        let (all_args, ..) = OpcodeEvent::get_ty_info_for_instr(app_wasm, curr_fid, instr);

        // figure out which args are requested based on matched probes
        let mut probes_to_remove = vec![];
        for (i, (_, probe)) in loc_info.probes.iter_mut().enumerate() {
            if probe.metadata.body_args.req_args.matches(all_args.len()) {
                req_args.combine(&probe.metadata.body_args.req_args);
            } else {
                // remove probe!
                probes_to_remove.push(i);
                continue;
            }
            if probe.metadata.pred_args.req_args.matches(all_args.len()) {
                req_args.combine(&probe.metadata.pred_args.req_args);
            } else {
                // remove probe!
                probes_to_remove.push(i);
                continue;
            }
        }
        for i in probes_to_remove.iter() {
            loc_info.probes.remove(*i);
        }

        if req_args.is_some() {
            loc_info.args = req_args.of(all_args);
        }

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }

    fn add_probes(&mut self, probes: &HashMap<WhammModeKind, Vec<Probe>>) {
        self.probes = probe_factory(probes);
    }
}

fn create_memarg_globals(
    all_params: &HashSet<&WhammParam>,
    loc_info: &mut LocInfo,
    align: u8,
    offset: u64,
    memory: u32,
) -> ReqArgs {
    let mut req_args = ReqArgs::None;
    for param in all_params {
        match param {
            WhammParam::Align => {
                loc_info
                    .static_data
                    .insert("align".to_string(), Some(Value::gen_u32(align as u32)));
            }
            WhammParam::Offset => {
                loc_info
                    .static_data
                    .insert("offset".to_string(), Some(Value::gen_u64(offset)));
            }
            WhammParam::Memory => {
                loc_info
                    .static_data
                    .insert("memory".to_string(), Some(Value::gen_u32(memory)));
            }
            WhammParam::EffectiveAddr => {
                req_args = ReqArgs::FirstN { n: 0 };
                // only have to add to offset if it's nonzero!
                if offset != 0 {
                    loc_info.add_dynamic_assign(
                        "effective_addr".to_string(),
                        DataType::U32,
                        Expr::BinOp {
                            lhs: Box::new(Expr::VarId {
                                definition: Definition::CompilerDynamic,
                                name: "arg0".to_string(),
                                loc: None,
                            }),
                            op: BinOp::Add,
                            rhs: Box::new(Expr::Primitive {
                                val: Value::gen_u32(offset as u32),
                                loc: None,
                            }),
                            done_on: DataType::U32,
                            loc: None,
                        },
                    );
                } else {
                    loc_info
                        .static_data
                        .insert("effective_addr".to_string(), Some(Value::gen_u64(offset)));
                }
            }
            _ => {}
        }
    }
    req_args
}

fn define_imm_n(n: u32, val: Option<Value>, loc_info: &mut LocInfo) {
    loc_info.static_data.insert(format!("imm{n}"), val);
}
