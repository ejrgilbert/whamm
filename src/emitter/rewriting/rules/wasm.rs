use crate::emitter::rewriting::rules::{
    event_factory, probe_factory, Arg, Event, FromStr, LocInfo, Package,
};
use crate::parser::rules::wasm::{OpcodeEventKind, WasmPackageKind};
use crate::parser::types::{DataType, ProbeSpec, SpecPart, Value};
use log::warn;
use orca::ir::module::Module;
use orca::ir::types::{DataType as OrcaType, FuncKind};
use std::collections::HashMap;

use crate::for_each_opcode;
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
macro_rules! define_opcode_event {
($($op:ident, $name:ident, $num_args:expr, $imms:expr, $globals:expr, $fns:expr, $docs:expr)*) => {
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
            Operator::If { .. } => (vec![OrcaType::I32], None),
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
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Nop { .. } => {
                if let Operator::Nop = instr {
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Block { .. } => {
                if let Operator::Block { .. } = instr {
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Loop { .. } => {
                if let Operator::Loop { .. } = instr {
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::If { .. } => {
                if let Operator::If { .. } = instr {
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Call { .. } => {
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
                        let relative_id = *fid - app_wasm.num_imported_functions as u32;
                        FuncInfo {
                            func_kind: "local".to_string(),
                            module: match &app_wasm.module_name {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                            name: match &app_wasm.get_fname(relative_id) {
                                Some(name) => name.clone(),
                                None => "".to_string(),
                            },
                        }
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
                    loc_info.add_probes(self.probe_spec(), &self.probes);
                }
            }
            OpcodeEventKind::Br { .. } => {
                if let Operator::Br { relative_depth } = instr {
                    loc_info.static_data.insert(
                        "imm0".to_string(),
                        Some(Value::I32 {
                            ty: DataType::I32,
                            // TODO -- check to see if this is a bad idea?
                            val: *relative_depth as i32,
                        }),
                    );

                    // add the probes for this event
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
