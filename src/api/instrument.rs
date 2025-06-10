#![allow(clippy::too_many_arguments)]

use crate::common::instr;
use log::error;
use orca_wasm::Module;
use std::process::exit;

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
    instr::run_on_module(
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
    _core_wasm_path: &str,
    _defs_path: &str,
    _app_wasm_path: String,
    _script_path: String,
    _user_lib_paths: Option<Vec<String>>,
) {
    todo!()
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
