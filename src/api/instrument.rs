#![allow(clippy::too_many_arguments)]

use crate::api::get_defs_and_lib;
use crate::common::error::{CodeLocation, ErrorGen, WhammError as ErrorInternal};
use crate::common::instr;
use crate::emitter::tag_handler::{LineCol, Reason, get_reasons_from_tag};
use log::error;
use std::collections::HashMap;
use std::process::exit;
use wirm::Module;
use wirm::ir::module::module_types::Types;
use wirm::ir::module::side_effects::{InjectType as WirmInjectType, Injection as WirmInjection};
use wirm::ir::types::{DataType as WirmType, FuncInstrMode, InstrumentationMode};
use wirm::wasmparser::{ExternalKind, TypeRef};

pub const MAX_ERRORS: i32 = 15;

/// Using the passed Whamm script and configuration, instrument the target Wasm module via bytecode rewriting.
///
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `config`: The configuration to use when performing the instrumentation.
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn instrument_with_config(
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    config: Config,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    let (def_yamls, core_lib) = get_defs_and_lib(defs_path, core_lib_path);
    instr::run_with_path(
        &core_lib,
        &def_yamls,
        app_wasm_path,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        config,
    )
}

/// Using the passed Whamm script, instrument the target Wasm module via bytecode rewriting.
///
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn instrument_with_rewriting(
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    instrument_with_config(
        app_wasm_path,
        script_path,
        user_lib_paths,
        Config::default_rewriting(),
        core_lib_path,
        defs_path,
    )
}

/// Using the passed Whamm script, instrument the target Wasm module via bytecode rewriting.
///
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn instrument_module_with_rewriting(
    target_wasm: &mut Module,
    script_path: String,
    user_lib_paths: Vec<String>,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<Vec<u8>, Vec<WhammError>> {
    let (def_yamls, core_lib) = get_defs_and_lib(defs_path, core_lib_path);
    match instr::run_on_module_and_encode(
        &core_lib,
        &def_yamls,
        target_wasm,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        Config::default_rewriting(),
    ) {
        Ok(res) => Ok(res),
        Err(e) => Err(WhammError::from_errs(e.pull_errs())),
    }
}

/// Using the passed Whamm script, generate a monitor module that encodes instructions for
/// dynamically applying instrumentation to an arbitrary Wasm module at runtime.
///
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn generate_monitor_module(
    script_path: String,
    user_lib_paths: Vec<String>,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<Vec<u8>, Vec<WhammError>> {
    match instrument_with_config(
        "".to_string(),
        script_path,
        user_lib_paths,
        Config::default_monitor_module(),
        core_lib_path,
        defs_path,
    ) {
        Ok(res) => Ok(res),
        Err(e) => Err(WhammError::from_errs(e.pull_errs())),
    }
}

/// Using the passed Whamm script, perform a dry run of instrumentation and return metadata
/// encoding the side effects that would occur for some program (`app_wasm_path`).
///
/// * `app_wasm_path`: The path to the target application to instrument.
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn instrument_as_dry_run_rewriting(
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<HashMap<WirmInjectType, Vec<Injection>>, Vec<WhammError>> {
    let buff = std::fs::read(app_wasm_path).unwrap();
    let mut target_wasm = Module::parse(&buff, false, true).unwrap();

    let (def_yamls, core_lib) = get_defs_and_lib(defs_path, core_lib_path);
    let response = instr::dry_run_on_bytes(
        &core_lib,
        &def_yamls,
        &mut target_wasm,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        Config::default_rewriting(),
    );
    handle_dry_run_response(response)
}

/// Using the passed Whamm script, perform a dry run of non-intrusive instrumentation via the wei engine API
///
/// * `script_path`: The path to the whamm script .mm file.
/// * `user_lib_paths`: Optional list of paths to user-provided library wasm modules. These are comma-delimited, formatted <lib_name>=<lib_path, e.g.: --user_libs lib_name0=/path/to/lib0.wasm,lib_name1=/path/to/lib1.wasm
/// * `core_lib_path`: The path to the core library wasm module. Use `None` for library to use the default path.
/// * `defs_path`: The path to the provider definitions. Use `None` for library to use the default path.
pub fn instrument_as_dry_run_wei(
    script_path: String,
    user_lib_paths: Vec<String>,
    core_lib_path: Option<String>,
    defs_path: Option<String>,
) -> Result<HashMap<WirmInjectType, Vec<Injection>>, Vec<WhammError>> {
    let mut module = Module::default();
    let (def_yamls, core_lib) = get_defs_and_lib(defs_path, core_lib_path);

    let response = instr::dry_run_on_bytes(
        &core_lib,
        &def_yamls,
        &mut module,
        script_path,
        user_lib_paths,
        MAX_ERRORS,
        Config::default_monitor_module(),
    );
    handle_dry_run_response(response)
}

fn handle_dry_run_response(
    response: Result<HashMap<WirmInjectType, Vec<WirmInjection>>, Vec<ErrorInternal>>,
) -> Result<HashMap<WirmInjectType, Vec<Injection>>, Vec<WhammError>> {
    match response {
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
        Err(errs) => Err(WhammError::from_errs(errs)),
    }
}

/// The instrumentation configuration
#[derive(Default)]
pub struct Config {
    /// Whether to emit a monitor module that can be used to dynamically instrument a program
    pub as_monitor_module: bool,
    /// Whether we allow probes that cause 'alternate' behavior in wei
    pub enable_wei_alt: bool,

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
        enable_wei_alt: bool,
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
            panic!(
                "Cannot disable argument bundling without also disabling body and predicate emitting! Otherwise invalid Wasm would be generated."
            )
        }
        Self {
            as_monitor_module,
            enable_wei_alt,
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
#[derive(Clone, Copy, Debug, Default)]
pub enum LibraryLinkStrategy {
    /// Merge the library with the `app.wasm` **target VM must support multi-memory**.
    /// Will create a new memory in the `app.wasm` to be targeted by the instrumentation.
    Merged,
    /// Link the library through Wasm imports into `app.wasm` (target VM must support dynamic linking).
    /// Naturally, the instrumentation memory will reside in its own module instantiation.
    #[default]
    Imported,
}

#[derive(Debug)]
pub enum Injection {
    // Module additions
    /// Represents an import that has been added to the module.
    // TODO: possibly just return wat for this
    Import {
        /// The module being imported from.
        module: String,
        /// The name of the imported item.
        name: String,
        /// The type of the import.
        type_ref: TypeRef,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },
    /// Represents an export that has been added to the module.
    // TODO: possibly just return wat for this
    Export {
        /// The name of the exported item.
        name: String,
        /// The kind of the exported item.
        kind: ExternalKind,
        /// The index of the exported item.
        index: u32,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },
    /// Represents a type that has been added to the module.
    // TODO: possibly just return wat for this
    Type {
        ty: Types,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents a memory that has been added to the module.
    // TODO: possibly just return wat for this
    Memory {
        /// The memory's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The initial number of pages for this memory.
        initial: u64,
        /// The maximum number of pages for this memory.
        maximum: Option<u64>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents an active data segment that has been added to the module.
    // TODO: possibly just return wat for this
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
        cause: Cause,
    },

    /// Represents a passive data segment that has been added to the module.
    // TODO: possibly just return wat for this
    PassiveData {
        /// The data of the data segment.
        data: Vec<u8>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents a global that has been added to the module.
    // TODO: possibly just return wat for this
    Global {
        /// The global's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The global's type.
        ty: WirmType,
        /// Whether the global is shared.
        shared: bool,
        /// Whether the global is mutable.
        mutable: bool,
        /// Contains the WAT of the instructions.
        init_expr: Vec<String>,
        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents a local function that has been added to the module.
    Func {
        /// The function's ID.
        id: u32,
        /// The function's name.
        fname: Option<String>,
        /// The function's signature (params, results).
        sig: (Vec<WirmType>, Vec<WirmType>),
        /// The function's local variables
        locals: Vec<WirmType>,
        /// The body of the function (in WAT).
        body: Vec<String>,

        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents a local variable that has been added to a module's local function.
    Local {
        /// The ID of the function this local is inserted into.
        target_fid: u32,
        ty: WirmType,

        /// Explains why this was injected (if it can be isolated to a
        /// specific Whamm script location).
        cause: Cause,
    },

    /// Represents a table that has been added to the module.
    // TODO: possibly just return wat for this
    Table { cause: Cause },
    /// Represents a table element that has been added to the module.
    // TODO: possibly just return wat for this
    Element { cause: Cause },

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
        cause: Cause,
    },
    /// Represents a probe that has been injected into a module's function (as a specialized function mode).
    FuncProbe {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        /// The mode of the probe to use during insertion.
        mode: FuncInstrMode,
        /// The body of the probe (in WAT).
        body: Vec<String>,
        cause: Cause,
    },
}
impl Injection {
    fn from(injection: &mut WirmInjection) -> Vec<Self> {
        match injection {
            WirmInjection::Import {
                module,
                name,
                type_ref,
                tag,
            } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                assert_eq!(1, reasons.len());
                vec![Self::Import {
                    module: module.to_owned(),
                    name: name.to_owned(),
                    type_ref: type_ref.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Export {
                name,
                kind,
                index,
                tag,
            } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Export {
                    name: name.to_owned(),
                    kind: kind.to_owned(),
                    index: *index,
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Type { ty, tag, .. } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Type {
                    ty: ty.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Memory {
                id,
                initial,
                maximum,
                tag,
            } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Memory {
                    id: *id,
                    initial: *initial,
                    maximum: maximum.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::ActiveData {
                memory_index,
                offset_expr,
                data,
                tag,
            } => {
                let offset_expr_wat = format!("{:?}", offset_expr);
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::ActiveData {
                    memory_index: *memory_index,
                    offset_expr: vec![offset_expr_wat],
                    data: data.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::PassiveData { data, tag } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::PassiveData {
                    data: data.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Global {
                id,
                ty,
                shared,
                mutable,
                init_expr,
                tag,
            } => {
                let init_expr_wat = format!("{:?}", init_expr);
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Global {
                    id: *id,
                    ty: ty.to_owned(),
                    shared: *shared,
                    mutable: *mutable,
                    init_expr: vec![init_expr_wat],
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Func {
                id,
                fname,
                sig,
                locals,
                body,
                tag,
            } => {
                let mut body_ops = vec![];
                for op in body.iter() {
                    body_ops.push(format!("{:?}", op));
                }
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Func {
                    id: *id,
                    fname: fname.to_owned(),
                    sig: sig.to_owned(),
                    locals: locals.to_owned(),
                    body: body_ops,
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Local {
                target_fid,
                ty,
                tag,
            } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Local {
                    target_fid: *target_fid,
                    ty: ty.to_owned(),
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Table { tag } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Table {
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::Element { tag } => {
                let reasons = get_reasons_from_tag(tag.data_mut());
                vec![Self::Element {
                    cause: Cause::from(reasons.first().unwrap()),
                }]
            }
            WirmInjection::FuncProbe {
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
                            cause: Cause::from(reason),
                        });

                        start_idx = *op_idx_end as usize;
                    } else {
                        panic!("Should be a probe reason, but got: {:?}", reason)
                    }
                }

                injections
            }
            WirmInjection::FuncLocProbe {
                target_fid,
                target_opcode_idx,
                mode,
                body,
                tag,
            } => {
                let mut injections = vec![];
                let mut start_idx = 0;
                // println!("{:?}@{}:{} --> {:#?}", mode, target_fid, target_opcode_idx, body);
                if tag.is_empty() {
                    // This is an injection that was created by Wirm...to handle function entry/exit
                    return vec![];
                }
                let reasons = get_reasons_from_tag(tag.data_mut());
                for reason in reasons.iter() {
                    if let Reason::UserProbe { op_idx_end, .. }
                    | Reason::WhammProbe { op_idx_end, .. } = reason
                    {
                        let mut body_wat = vec![];
                        for op in body[start_idx..*op_idx_end as usize].iter() {
                            body_wat.push(format!("{:?}\n", op));
                        }
                        injections.push(Self::OpProbe {
                            target_fid: *target_fid,
                            target_opcode_idx: *target_opcode_idx,
                            mode: mode.to_owned(),
                            body: body_wat,
                            cause: Cause::from(reason),
                        });

                        start_idx = *op_idx_end as usize;
                    } else {
                        panic!(
                            "Should be a probe reason with an op index, but got: {:#?}",
                            reason
                        );
                    }
                }

                injections
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cause {
    // There's a reason in the Whamm script for this addition
    // it's due to a single character.
    UserPos { lc: LineCol },
    // There's a reason in the Whamm script for this addition
    // it's due to a span in the script.
    UserSpan { lc0: LineCol, lc1: LineCol },
    // There's a reason in the Whamm script for this addition
    // it's due to a probe.
    UserProbe { lc0: LineCol, lc1: LineCol },
    // The injection was for the Whamm language runtime
    Whamm,
}
impl From<&Reason> for Cause {
    fn from(value: &Reason) -> Self {
        match value {
            Reason::UserPos { lc } => Self::UserPos { lc: *lc },
            Reason::UserSpan { lc0, lc1 } => Self::UserSpan {
                lc0: *lc0,
                lc1: *lc1,
            },
            Reason::UserProbe { lc0, lc1, .. } => Self::UserProbe {
                lc0: *lc0,
                lc1: *lc1,
            },
            Reason::Whamm | Reason::WhammProbe { .. } => Self::Whamm,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WhammError {
    /// The location within the input string causing the error
    pub err_loc: Option<CodeLocation>,
    /// A location within the input string that can add context to the error
    pub info_loc: Option<CodeLocation>,
    pub msg: String,
}
impl From<&ErrorInternal> for WhammError {
    fn from(value: &ErrorInternal) -> Self {
        Self {
            err_loc: value.err_loc.clone(),
            info_loc: value.info_loc.clone(),
            msg: value.ty.message().to_string(),
        }
    }
}
impl WhammError {
    fn from_errs(values: Vec<ErrorInternal>) -> Vec<Self> {
        let mut errs = vec![];
        for e in values.iter() {
            errs.push(Self::from(e));
        }
        errs
    }
}
