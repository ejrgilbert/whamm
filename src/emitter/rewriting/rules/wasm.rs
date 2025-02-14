use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Arg, Event, FromStr, LocInfo, Package, ProbeRule,
};
use crate::for_each_opcode;
use crate::generator::rewriting::simple_ast::SimpleProbe;
use crate::parser::rules::core::WhammModeKind;
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{BinOp, DataType, Definition, Expr, RulePart, Value};
use log::warn;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::ir::module::module_functions::{FuncKind, ImportedFunction, LocalFunction};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::DataType as OrcaType;
use std::collections::HashMap;
use wasmparser::{MemArg, Operator};

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
    fn add_events(
        &mut self,
        ast_events: &HashMap<String, HashMap<WhammModeKind, Vec<SimpleProbe>>>,
    ) {
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
    probes: HashMap<WhammModeKind, Vec<SimpleProbe>>,
}
macro_rules! define_opcode_event {
($($op:ident, $name:ident, $num_args:expr, $imms:expr, $globals:expr, $fns:expr, $supported_modes:expr, $req_map:expr, $docs:expr)*) => {
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
    pub fn get_ty_info_for_instr(app_wasm: &Module, instr: &Operator) -> (Vec<Arg>, Option<u32>) {
        // TODO: there are 500 of them in wasmparser::Operator
        // compared to 48 of them in walrus::ir::Instr
        // How do we compress the Operators we need to concern
        let (ty_list, ty_id): (Vec<OrcaType>, Option<u32>) = match instr {
            Operator::Call {
                function_index: fid,
            } => {
                match app_wasm.functions.get_kind(FunctionID(*fid)) {
                    FuncKind::Import(ImportedFunction { ty_id, .. })
                    | FuncKind::Local(LocalFunction { ty_id, .. }) => {
                        if let Some(ty) = app_wasm.types.get(*ty_id) {
                            (ty.params().to_vec(), Some(**ty_id))
                        } else {
                            // no type info found!!
                            warn!("No type information found for import with FID {fid}");
                            (vec![], None)
                        }
                    }
                }
            }
            Operator::If { .. } | Operator::BrIf { .. } | Operator::BrTable { .. } => {
                (vec![OrcaType::I32], None)
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
}
impl Event for OpcodeEvent {
    fn get_loc_info(&self, app_wasm: &Module, instr: &Operator) -> Option<LocInfo> {
        let mut loc_info = LocInfo::new();

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
                if let Operator::Block { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Loop { .. } => {
                if let Operator::Loop { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::If { .. } => {
                if let Operator::If { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Else { .. } => {
                if let Operator::Else { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::TryTable { .. } => {
                if let Operator::TryTable { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Throw { .. } => {
                if let Operator::Throw { tag_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*tag_index)));
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::ThrowRef { .. } => {
                if let Operator::TryTable { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::End { .. } => {
                if let Operator::End { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Br { .. } => {
                if let Operator::Br { relative_depth } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*relative_depth)));
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrIf { .. } => {
                if let Operator::BrIf { relative_depth } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*relative_depth)));
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrTable { .. } => {
                if let Operator::BrTable { targets } = instr {
                    loc_info.static_data.insert(
                        "num_targets".to_string(),
                        Some(Value::gen_u32(targets.len())),
                    );
                    loc_info.static_data.insert(
                        "default_target".to_string(),
                        Some(Value::gen_u32(targets.default())),
                    );

                    let mut target_map = HashMap::new();

                    for (i, target) in targets.targets().enumerate() {
                        if let Ok(target) = target {
                            loc_info
                                .static_data
                                .insert(format!("imm{i}"), Some(Value::gen_u32(target)));
                            target_map.insert(i as u32, target);
                        }
                    }
                    loc_info.add_dynamic_value(
                        "targets".to_string(),
                        Value::U32U32Map {
                            val: Box::new(target_map),
                        },
                    );
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Return { .. } => {
                if let Operator::Return { .. } = instr {
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
                    // define static_data
                    loc_info.static_data.insert(
                        "target_fn_name".to_string(),
                        Some(Value::Str {
                            val: func_info.name.to_string(),
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_fn_type".to_string(),
                        Some(Value::Str {
                            val: func_info.func_kind.to_string(),
                        }),
                    );
                    loc_info.static_data.insert(
                        "target_imp_module".to_string(),
                        Some(Value::Str {
                            val: func_info.module.to_string(),
                        }),
                    );
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*fid)));

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
                if let Operator::TypedSelect {..} = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalGet { .. } => {
                if let Operator::LocalGet { local_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*local_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalSet { .. } => {
                if let Operator::LocalSet { local_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*local_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalTee { .. } => {
                if let Operator::LocalTee { local_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*local_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalGet { .. } => {
                if let Operator::GlobalGet { global_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*global_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalSet { .. } => {
                if let Operator::GlobalSet { global_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*global_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load { .. } => {
                if let Operator::I32Load { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load { .. } => {
                if let Operator::I64Load { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Load { .. } => {
                if let Operator::F32Load { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load8S { .. } => {
                if let Operator::I32Load8S { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load8U { .. } => {
                if let Operator::I32Load8U { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load16S { .. } => {
                if let Operator::I32Load16S { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Load16U { .. } => {
                if let Operator::I32Load16U { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load8S { .. } => {
                if let Operator::I64Load8S { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load8U { .. } => {
                if let Operator::I64Load8U { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load16S { .. } => {
                if let Operator::I64Load16S { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load16U { .. } => {
                if let Operator::I64Load16U { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load32S { .. } => {
                if let Operator::I64Load32S { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Load32U { .. } => {
                if let Operator::I64Load32U { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store { .. } => {
                if let Operator::I32Store { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store { .. } => {
                if let Operator::I64Store { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Store { .. } => {
                if let Operator::F32Store { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Store { .. } => {
                if let Operator::F64Store { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store8 { .. } => {
                if let Operator::I32Store8 { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Store16 { .. } => {
                if let Operator::I32Store16 { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store8 { .. } => {
                if let Operator::I64Store8 { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store16 { .. } => {
                if let Operator::I64Store16 { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Store32 { .. } => {
                if let Operator::I64Store32 { memarg: MemArg { align, offset, memory, .. } } = instr {
                    create_memarg_globals(&mut loc_info, *align, *offset, *memory);

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }


            OpcodeEventKind::MemorySize { .. } => {
                if let Operator::MemorySize { mem } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*mem)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::MemoryGrow { .. } => {
                if let Operator::MemoryGrow { mem } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*mem)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Const { .. } => {
                if let Operator::I32Const { value } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_i32(*value)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Const { .. } => {
                if let Operator::I64Const { value } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_i64(*value)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F32Const { .. } => {
                if let Operator::F32Const { value } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_f32(f32::from(value.clone()))));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::F64Const { .. } => {
                if let Operator::F64Const { value } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_f64(f64::from(value.clone()))));

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
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*function_index)));

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
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructNewDefault { .. } => {
                if let Operator::StructNewDefault { struct_type_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGet { .. } => {
                if let Operator::StructGet { struct_type_index, field_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("imm1".to_string(), Some(Value::gen_u32(*field_index)));
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("field_idx".to_string(), Some(Value::gen_u32(*field_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGetS { .. } => {
                if let Operator::StructGetS { struct_type_index, field_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("imm1".to_string(), Some(Value::gen_u32(*field_index)));
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("field_idx".to_string(), Some(Value::gen_u32(*field_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructGetU { .. } => {
                if let Operator::StructGetU { struct_type_index, field_index } = instr {
                    loc_info
                        .static_data
                        .insert("imm0".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("imm1".to_string(), Some(Value::gen_u32(*field_index)));
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("field_idx".to_string(), Some(Value::gen_u32(*field_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::StructSet { .. } => {
                if let Operator::StructSet { struct_type_index, field_index } = instr {
                    loc_info
                        .static_data
                        .insert("tid".to_string(), Some(Value::gen_u32(*struct_type_index)));
                    loc_info
                        .static_data
                        .insert("field_idx".to_string(), Some(Value::gen_u32(*field_index)));

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
        }

        if loc_info.has_match() {
            Some(loc_info)
        } else {
            None
        }
    }

    fn add_probes(&mut self, probes: &HashMap<WhammModeKind, Vec<SimpleProbe>>) {
        self.probes = probe_factory(probes);
    }
}

fn create_memarg_globals(loc_info: &mut LocInfo, align: u8, offset: u64, memory: u32) {
    loc_info
        .static_data
        .insert("align".to_string(), Some(Value::gen_u32(align as u32)));
    loc_info
        .static_data
        .insert("offset".to_string(), Some(Value::gen_u64(offset)));
    loc_info
        .static_data
        .insert("memory".to_string(), Some(Value::gen_u32(memory)));

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
        }
    );
}
