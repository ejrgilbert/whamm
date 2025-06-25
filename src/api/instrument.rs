#![allow(clippy::too_many_arguments)]

use crate::common::error::WhammError;
use crate::common::instr;
use crate::emitter::tag_handler::{get_reasons_from_tag, Reason};
use log::error;
use orca_wasm::ir::module::module_types::Types;
use orca_wasm::ir::module::side_effects::{
    InjectType as OrcaInjectType, Injection as OrcaInjection,
};
use orca_wasm::ir::types::{DataType as OrcaType, FuncInstrMode, InstrumentationMode};
use orca_wasm::Module;
use std::collections::HashMap;
use std::process::exit;
use wasmparser::{ExternalKind, TypeRef};

pub const MAX_ERRORS: i32 = 15;

/// Using the passed Whamm script and configuration, instrument the target Wasm module via bytecode rewriting.
///
/// * `core_wasm_path`: The path to the core library wasm module.
/// * `defs_path`: The path to the provider definitions.
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules.
/// * `config`: The configuration to use when performing the instrumentation.
pub fn instrument_with_config(
    core_wasm_path: &str,
    defs_path: &str,
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    config: Config,
) -> Vec<u8> {
    instr::run_with_path(
        core_wasm_path,
        defs_path,
        app_wasm_path,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        config,
    )
}

/// Using the passed Whamm script, instrument the target Wasm module via bytecode rewriting.
///
/// * `core_wasm_path`: The path to the core library wasm module.
/// * `defs_path`: The path to the provider definitions.
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules.
pub fn instrument_with_rewriting(
    core_wasm_path: &str,
    defs_path: &str,
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
) -> Vec<u8> {
    instrument_with_config(
        core_wasm_path,
        defs_path,
        app_wasm_path,
        script_path,
        user_lib_paths,
        Config::default_rewriting(),
    )
}

/// Using the passed Whamm script, instrument the target Wasm module via bytecode rewriting.
///
/// * `core_wasm_path`: The path to the core library wasm module.
/// * `defs_path`: The path to the provider definitions.
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules.
pub fn instrument_module_with_rewriting(
    core_wasm_path: &str,
    defs_path: &str,
    target_wasm: &mut Module,
    script_path: String,
    user_lib_paths: Vec<String>,
) -> Vec<u8> {
    instr::run_on_module_and_encode(
        core_wasm_path,
        defs_path,
        target_wasm,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        Config::default_rewriting(),
    )
}

/// Using the passed Whamm script, generate a monitor module that encodes instructions for
/// dynamically applying instrumentation to an arbitrary Wasm module at runtime.
///
/// * `core_wasm_path`: The path to the core library wasm module.
/// * `defs_path`: The path to the provider definitions.
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules.
pub fn generate_monitor_module(
    core_wasm_path: &str,
    defs_path: &str,
    script_path: String,
    user_lib_paths: Vec<String>,
) -> Vec<u8> {
    instrument_with_config(
        core_wasm_path,
        defs_path,
        "".to_string(),
        script_path,
        user_lib_paths,
        Config::default_monitor_module(),
    )
}

/// Using the passed Whamm script, perform a dry run of instrumentation and return metadata
/// encoding the side effects that would occur for some program (`app_wasm_path`).
///
/// * `core_wasm_path`: The path to the core library wasm module.
/// * `defs_path`: The path to the provider definitions.
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules.
pub fn instrument_as_dry_run(
    core_wasm_path: &str,
    defs_path: &str,
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
) -> Result<HashMap<OrcaInjectType, Vec<Injection>>, Vec<WhammError>> {
    let buff = std::fs::read(app_wasm_path).unwrap();
    let mut target_wasm = Module::parse(&buff, false).unwrap();

    match instr::dry_run_on_bytes(
        core_wasm_path,
        defs_path,
        &mut target_wasm,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        Config::default_rewriting(),
    ) {
        Ok(mut side_effects) => {
            let mut injections = HashMap::new();
            for (ty, l) in side_effects.iter_mut() {
                let mut list = Vec::new();
                for inj in l.iter_mut() {
                    list.extend(Injection::from(inj));
                }
                injections.insert(*ty, list);
            }
            Ok(injections)
        }
        Err(errs) => Err(errs),
    }
}

/// The instrumentation configuration
pub struct Config {
    /// Whether to emit a monitor module that can be used to dynamically instrument a program
    pub as_monitor_module: bool,
    /// Whether we allow probes that cause 'alternate' behavior in wizard
    pub enable_wizard_alt: bool,

    /// Whether to print metrics collected as whamm performs various actions.
    pub metrics: bool,
    /// Whether to omit the bundling logic of instrumentation (for evaluation purposes).
    pub no_bundle: bool,
    /// Whether to omit the logic of probe bodies (for evaluation purposes).
    pub no_body: bool,
    /// Whether to omit the logic of predication (for evaluation purposes).
    pub no_pred: bool,
    /// Whether to omit the logic to flush a report (for evaluation purposes).
    pub no_report: bool,

    /// Whether to emit extra exported functions that are helpful during testing.
    pub testing: bool,

    /// The strategy to take when handling the injecting references to the `whamm!` library.
    pub library_strategy: LibraryLinkStrategy,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            as_monitor_module: false,
            enable_wizard_alt: false,
            metrics: false,
            no_bundle: false,
            no_body: false,
            no_pred: false,
            no_report: false,
            testing: false,
            library_strategy: LibraryLinkStrategy::Imported,
        }
    }
}
impl Config {
    pub fn default_rewriting() -> Self {
        Self::default()
    }
    pub fn default_monitor_module() -> Self {
        Self {
            as_monitor_module: true,
            ..Default::default()
        }
    }
    pub fn new(
        as_monitor_module: bool,
        enable_wizard_alt: bool,
        metrics: bool,
        no_bundle: bool,
        no_body: bool,
        no_pred: bool,
        no_report: bool,
        testing: bool,
        library_strategy: Option<LibraryLinkStrategy>,
    ) -> Self {
        if testing {
            error!("Generating helper methods for testing mode is not yet supported!");
            exit(1);
        }

        if no_bundle && (!no_body || !no_pred) {
            panic!("Cannot disable argument bundling without also disabling body and predicate emitting! Otherwise invalid Wasm would be generated.")
        }
        Self {
            as_monitor_module,
            enable_wizard_alt,
            metrics,
            no_bundle,
            no_body,
            no_pred,
            no_report,
            testing,
            library_strategy: library_strategy.unwrap_or_default(),
        }
    }
}

/// Options for handling instrumentation libraries.
#[derive(Clone, Copy, Debug)]
pub enum LibraryLinkStrategy {
    /// Merge the library with the `app.wasm` **target VM must support multi-memory**.
    /// Will create a new memory in the `app.wasm` to be targeted by the instrumentation.
    Merged,
    /// Link the library through Wasm imports into `app.wasm` (target VM must support dynamic linking).
    /// Naturally, the instrumentation memory will reside in its own module instantiation.
    Imported,
}
impl Default for LibraryLinkStrategy {
    fn default() -> Self {
        Self::Imported
    }
}

#[derive(Debug)]
pub enum Injection {
    // Module additions
    /// Represents an import that has been added to the module.
    Import {
        /// The module being imported from.
        module: String,
        /// The name of the imported item.
        name: String,
        /// The type of the import.
        type_ref: TypeRef,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },
    /// Represents an export that has been added to the module.
    Export {
        /// The name of the exported item.
        name: String,
        /// The kind of the exported item.
        kind: ExternalKind,
        /// The index of the exported item.
        index: u32,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },
    Type {
        ty: Types,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a memory that has been added to the module.
    Memory {
        /// The memory's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The initial number of pages for this memory.
        initial: u64,
        /// The maximum number of pages for this memory.
        maximum: Option<u64>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents an active data segment that has been added to the module.
    ActiveData {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The memory offset where this active data segment will be automatically
        /// initialized.
        /// Contains the WAT of the instructions.
        offset_expr: Vec<String>,
        /// The data of the data segment.
        data: Vec<u8>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a passive data segment that has been added to the module.
    PassiveData {
        /// The data of the data segment.
        data: Vec<u8>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a global that has been added to the module.
    Global {
        /// The global's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The global's type.
        ty: OrcaType,
        /// Whether the global is shared.
        shared: bool,
        /// Whether the global is mutable.
        mutable: bool,
        /// Contains the WAT of the instructions.
        init_expr: Vec<String>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a local function that has been added to the module.
    Func {
        /// The function's ID.
        id: u32,
        /// The function's name.
        fname: Option<String>,
        /// The function's signature (params, results).
        sig: (Vec<OrcaType>, Vec<OrcaType>),
        /// The function's local variables
        locals: Vec<OrcaType>,
        /// The body of the function (in WAT).
        body: Vec<String>,

        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a local variable that has been added to a module's local function.
    Local {
        /// The ID of the function this local is inserted into.
        target_fid: u32,
        ty: OrcaType,

        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        reasons: Vec<Reason>,
    },

    /// Represents a table that has been added to the module.
    Table { reasons: Vec<Reason> },
    /// Represents a table element that has been added to the module.
    Element { reasons: Vec<Reason> },

    // Probes
    /// Represents a probe that has been injected into the module at a specific location in a function.
    OpProbe {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        /// The opcode offset in the target that this probe is inserted at.
        target_opcode_idx: u32,
        /// The mode of the probe to use during insertion.
        mode: InstrumentationMode,
        /// The body of the probe (in WAT).
        body: Vec<String>,
        reasons: Vec<Reason>,
    },
    /// Represents a probe that has been injected into a module's function (as a specialized function mode).
    FuncProbe {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        /// The mode of the probe to use during insertion.
        mode: FuncInstrMode,
        /// The body of the probe (in WAT).
        body: Vec<String>,
        reasons: Vec<Reason>,
    },
}
impl Injection {
    fn from(injection: &mut OrcaInjection) -> Vec<Self> {
        match injection {
            OrcaInjection::Import {
                module,
                name,
                type_ref,
                tag,
            } => vec![Self::Import {
                module: module.to_owned(),
                name: name.to_owned(),
                type_ref: type_ref.to_owned(),
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::Export {
                name,
                kind,
                index,
                tag,
            } => vec![Self::Export {
                name: name.to_owned(),
                kind: kind.to_owned(),
                index: *index,
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::Type { ty, tag, .. } => vec![Self::Type {
                ty: ty.to_owned(),
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::Memory {
                id,
                initial,
                maximum,
                tag,
            } => vec![Self::Memory {
                id: *id,
                initial: *initial,
                maximum: maximum.to_owned(),
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::ActiveData {
                memory_index,
                offset_expr,
                data,
                tag,
            } => {
                let offset_expr_wat = format!("{:?}", offset_expr);
                vec![Self::ActiveData {
                    memory_index: *memory_index,
                    offset_expr: vec![offset_expr_wat],
                    data: data.to_owned(),
                    reasons: get_reasons_from_tag(tag.data_mut()),
                }]
            }
            OrcaInjection::PassiveData { data, tag } => {
                vec![Self::PassiveData {
                    data: data.to_owned(),
                    reasons: get_reasons_from_tag(tag.data_mut()),
                }]
            }
            OrcaInjection::Global {
                id,
                ty,
                shared,
                mutable,
                init_expr,
                tag,
            } => {
                let init_expr_wat = format!("{:?}", init_expr);
                vec![Self::Global {
                    id: *id,
                    ty: ty.to_owned(),
                    shared: *shared,
                    mutable: *mutable,
                    init_expr: vec![init_expr_wat],
                    reasons: get_reasons_from_tag(tag.data_mut()),
                }]
            }
            OrcaInjection::Func {
                id,
                fname,
                sig,
                locals,
                body,
                tag,
            } => {
                let mut body_ops = vec![];
                for instr in body.iter() {
                    body_ops.push(format!("{:?}", instr.op));
                }
                vec![Self::Func {
                    id: *id,
                    fname: fname.to_owned(),
                    sig: sig.to_owned(),
                    locals: locals.to_owned(),
                    body: body_ops,
                    reasons: get_reasons_from_tag(tag.data_mut()),
                }]
            }
            OrcaInjection::Local {
                target_fid,
                ty,
                tag,
            } => vec![Self::Local {
                target_fid: *target_fid,
                ty: ty.to_owned(),
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::Table { tag } => vec![Self::Table {
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::Element { tag } => vec![Self::Table {
                reasons: get_reasons_from_tag(tag.data_mut()),
            }],
            OrcaInjection::FuncProbe {
                target_fid,
                mode,
                body,
                tag,
            } => {
                let mut injections = vec![];
                let mut start_idx = 0;
                let reasons = get_reasons_from_tag(tag.data_mut());
                for reason in reasons.iter() {
                    if let Reason::UserProbe { op_idx_end, .. }
                    | Reason::WhammProbe { op_idx_end } = reason
                    {
                        let mut body_wat = vec![];
                        for op in body[start_idx..*op_idx_end as usize].iter() {
                            body_wat.push(format!("{:?}\n", op));
                        }
                        injections.push(Self::FuncProbe {
                            target_fid: *target_fid,
                            mode: mode.to_owned(),
                            body: body_wat,
                            reasons: vec![reason.clone()],
                        });

                        start_idx = *op_idx_end as usize;
                    } else {
                        panic!("Should be a probe reason, but got: {:?}", reason)
                    }
                }

                injections
            }
            OrcaInjection::FuncLocProbe {
                target_fid,
                target_opcode_idx,
                mode,
                body,
                tag,
            } => {
                let mut injections = vec![];
                let mut start_idx = 0;
                // println!("{:#?}", body);
                let reasons = get_reasons_from_tag(tag.data_mut());
                for reason in reasons.iter() {
                    if let Reason::UserProbe { op_idx_end, .. } = reason {
                        let mut body_wat = vec![];
                        for op in body[start_idx..*op_idx_end as usize].iter() {
                            body_wat.push(format!("{:?}\n", op));
                        }
                        injections.push(Self::OpProbe {
                            target_fid: *target_fid,
                            target_opcode_idx: *target_opcode_idx,
                            mode: mode.to_owned(),
                            body: body_wat,
                            reasons: vec![reason.clone()],
                        });

                        start_idx = *op_idx_end as usize;
                    } else {
                        panic!("Should be a user probe reason!")
                    }
                }

                injections
            }
        }
    }
}
