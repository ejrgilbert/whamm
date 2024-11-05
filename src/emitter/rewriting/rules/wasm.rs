use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Arg, Event, FromStr, LocInfo, Package, ProbeRule,
};
use crate::for_each_opcode;
use crate::generator::rewriting::simple_ast::SimpleProbe;
use crate::parser::rules::core::WhammModeKind;
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{DataType, RulePart, Value};
use log::warn;
use orca_wasm::ir::id::FunctionID;
use orca_wasm::ir::module::module_functions::{FuncKind, ImportedFunction, LocalFunction};
use orca_wasm::ir::module::Module;
use orca_wasm::ir::types::DataType as OrcaType;
use std::collections::HashMap;
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
            provider: Some(RulePart {
                name: "wasm".to_string(),
                loc: None,
            }),
            package: Some(RulePart {
                name: "opcode".to_string(),
                loc: None,
            }),
            event: Some(RulePart {
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
                match app_wasm.functions.get_kind(FunctionID(*fid)) {
                    FuncKind::Import(ImportedFunction { ty_id, .. })
                    | FuncKind::Local(LocalFunction { ty_id, .. }) => {
                        if let Some(ty) = app_wasm.types.get(*ty_id) {
                            (ty.params.to_vec(), Some(**ty_id))
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
            OpcodeEventKind::End { .. } => {
                if let Operator::End { .. } = instr {
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::Br { .. } => {
                if let Operator::Br { relative_depth } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *relative_depth,
                        }),
                    );
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrIf { .. } => {
                if let Operator::BrIf { relative_depth } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *relative_depth,
                        }),
                    );
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::BrTable { .. } => {
                if let Operator::BrTable { targets } = instr {
                    loc_info.static_data.insert(
                        "num_targets".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: targets.len(),
                        }),
                    );
                    loc_info.static_data.insert(
                        "default_target".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: targets.default(),
                        }),
                    );

                    let mut target_map = HashMap::new();

                    for (i, target) in targets.targets().enumerate() {
                        if let Ok(target) = target {
                            loc_info.static_data.insert(
                                format!("imm{i}"),
                                Some(Value::U32 {
                                    ty: DataType::U32,
                                    val: target,
                                }),
                            );
                            target_map.insert(i as u32, target);
                        }
                    }
                    loc_info.dynamic_data.insert(
                        "targets".to_string(),
                        Some(Value::U32U32Map {
                            ty: DataType::Map {
                                key_ty: Box::new(DataType::U32),
                                val_ty: Box::new(DataType::U32),
                            },
                            val: Box::new(target_map),
                        }),
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
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *fid,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalGet { .. } => {
                if let Operator::LocalGet { local_index } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *local_index,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalSet { .. } => {
                if let Operator::LocalSet { local_index } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *local_index,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::LocalTee { .. } => {
                if let Operator::LocalTee { local_index } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *local_index,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalGet { .. } => {
                if let Operator::GlobalGet { global_index } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *global_index,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::GlobalSet { .. } => {
                if let Operator::GlobalSet { global_index } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::U32 {
                            ty: DataType::U32,
                            val: *global_index,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I32Const { .. } => {
                if let Operator::I32Const { value } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::I32 {
                            ty: DataType::I32,
                            val: *value,
                        }),
                    );

                    // add the probes for this event
                    loc_info.add_probes(self.probe_rule(), &self.probes);
                }
            }
            OpcodeEventKind::I64Const { .. } => {
                if let Operator::I64Const { value } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::I64 {
                            ty: DataType::I64,
                            val: *value,
                        }),
                    );

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
