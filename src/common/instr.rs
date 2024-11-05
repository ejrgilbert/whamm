#![allow(clippy::too_many_arguments)]
use crate::cli::LibraryLinkStrategyArg;
use crate::common::error::ErrorGen;
use crate::emitter::module_emitter::{MemoryTracker, ModuleEmitter};
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::generator::rewriting::init_generator::InitGenerator;
use crate::generator::rewriting::instr_generator::InstrGenerator;
use crate::generator::rewriting::simple_ast::{build_simple_ast, SimpleAST};
use crate::generator::wizard::metadata_collector::WizardProbeMetadataCollector;
use crate::libraries::core::io::io_adapter::IOAdapter;
use crate::libraries::core::io::IOPackage;
use crate::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::libraries::core::maps::MapLibPackage;
use crate::libraries::core::LibPackage;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info};
use orca_wasm::ir::id::GlobalID;
use orca_wasm::Module;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use wasmparser::MemoryType;

/// create output path if it doesn't exist
pub(crate) fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        std::fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

/// Copy to enable access for testing...
/// Options for handling instrumentation library.
#[derive(Clone, Debug)]
pub enum LibraryLinkStrategy {
    /// Merge the library with the `app.wasm` **target VM must support multi-memory**.
    /// Will create a new memory in the `app.wasm` to be targeted by the instrumentation.
    Merged,
    /// Link the library through Wasm imports into `app.wasm` (target VM must support dynamic linking).
    /// Naturally, the instrumentation memory will reside in its own module instantiation.
    Imported,
}
impl From<Option<LibraryLinkStrategyArg>> for LibraryLinkStrategy {
    fn from(value: Option<LibraryLinkStrategyArg>) -> Self {
        match value {
            Some(LibraryLinkStrategyArg::Imported) => LibraryLinkStrategy::Imported,
            Some(LibraryLinkStrategyArg::Merged) => LibraryLinkStrategy::Merged,
            None => {
                info!("Using default library linking strategy: 'imported'");
                LibraryLinkStrategy::Imported
            }
        }
    }
}

pub struct Config {
    /// Whether to emit `mon.wasm` for instrumenting with Wizard Engine
    pub wizard: bool,
    /// Whether we allow probes that cause 'alternate' behavior in wizard
    pub enable_wizard_alt: bool,

    /// Whether to emit extra exported functions that are helpful during testing.
    pub testing: bool,

    /// The strategy to take when handling the injecting references to the `whamm!` library.
    pub library_strategy: LibraryLinkStrategy,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            wizard: false,
            enable_wizard_alt: false,
            testing: false,
            library_strategy: LibraryLinkStrategy::Imported,
        }
    }
}
impl Config {
    pub fn new(
        wizard: bool,
        enable_wizard_alt: bool,
        testing: bool,
        link_strategy: Option<LibraryLinkStrategyArg>,
    ) -> Self {
        if testing {
            error!("Generating helper methods for testing mode is not yet supported!");
            exit(1);
        }
        let library_strategy = LibraryLinkStrategy::from(link_strategy);
        Self {
            wizard,
            enable_wizard_alt,
            testing,
            library_strategy,
        }
    }
}

pub fn run_with_path(
    core_wasm_path: &str,
    app_wasm_path: String,
    script_path: String,
    output_wasm_path: String,
    max_errors: i32,
    config: Config,
) {
    let buff = if !config.wizard {
        std::fs::read(app_wasm_path).unwrap()
    } else {
        vec![]
    };

    let mut target_wasm = if !config.wizard {
        // Read app Wasm into Orca module
        Module::parse(&buff, false).unwrap()
    } else {
        // Create a new wasm file to use as `mon.wasm`
        Module::default()
    };

    // read in the whamm script
    let whamm_script = match std::fs::read_to_string(script_path.clone()) {
        Ok(unparsed_str) => unparsed_str,
        Err(error) => {
            error!("Cannot read specified file {}: {}", script_path, error);
            exit(1);
        }
    };

    run(
        core_wasm_path,
        &mut target_wasm,
        &whamm_script,
        &script_path,
        Some(output_wasm_path),
        max_errors,
        config,
    );
}

pub fn run(
    core_wasm_path: &str,
    target_wasm: &mut Module,
    whamm_script: &String,
    script_path: &str,
    output_wasm_path: Option<String>,
    max_errors: i32,
    config: Config,
) -> Vec<u8> {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Process the script
    let mut whamm = get_script_ast(whamm_script, &mut err);
    let mut symbol_table = get_symbol_table(&mut whamm, &mut err);
    err.check_too_many();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Merge in the core library IF NEEDED
    let mut map_package = MapLibPackage::default();
    let mut io_package = IOPackage::default();
    let mut core_packages: Vec<&mut dyn LibPackage> = vec![&mut map_package, &mut io_package];
    crate::libraries::actions::link_core_lib(
        &config.library_strategy,
        &whamm,
        target_wasm,
        core_wasm_path,
        &mut core_packages,
        &mut err,
    );
    let mut map_lib_adapter = map_package.adapter;
    let mut io_adapter = io_package.adapter;
    let mut report_var_metadata = ReportVarMetadata::new();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    if config.wizard {
        run_instr_wizard(
            &mut whamm,
            // simple_ast,
            target_wasm,
            &mut symbol_table,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_var_metadata,
            &mut err,
            &config,
        );
    } else {
        let simple_ast = build_simple_ast(&whamm, &mut err);
        run_instr_rewrite(
            &mut whamm,
            simple_ast,
            target_wasm,
            &mut symbol_table,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_var_metadata,
            &mut err,
        );
    }
    // for debugging
    report_var_metadata.print_metadata();

    if let Some(output_wasm_path) = output_wasm_path {
        try_path(&output_wasm_path);
        if let Err(e) = target_wasm.emit_wasm(&output_wasm_path) {
            err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "Failed to dump instrumented wasm to {} from error: {}",
                    &output_wasm_path, e
                )),
                None,
            ))
        }
    }

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    target_wasm.encode()
}

fn run_instr_wizard(
    whamm: &mut Whamm,
    // simple_ast: SimpleAST,
    target_wasm: &mut Module,
    symbol_table: &mut SymbolTable,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err: &mut ErrorGen,
    config: &Config,
) {
    let mut mem_tracker = get_memory_tracker(target_wasm, true);

    // Collect the metadata for the AST and transform to different representation
    // specifically used for targeting Wizard during compilation.
    let mut metadata_collector = WizardProbeMetadataCollector::new(symbol_table, err, config);
    metadata_collector.visit_whamm(whamm);
    let wiz_ast = metadata_collector.wizard_ast;
    let used_funcs = metadata_collector.used_provided_fns;
    let used_strings = metadata_collector.strings_to_emit;

    let mut injected_funcs = vec![];
    let mut gen = crate::generator::wizard::WizardGenerator {
        emitter: ModuleEmitter::new(
            target_wasm,
            symbol_table,
            &mut mem_tracker,
            map_lib_adapter,
            report_var_metadata,
        ),
        io_adapter,
        context_name: "".to_string(),
        err,
        injected_funcs: &mut injected_funcs,
        config,
        curr_script_id: "".to_string(),
    };
    gen.run(wiz_ast, used_funcs, used_strings);
}

fn run_instr_rewrite(
    whamm: &mut Whamm,
    simple_ast: SimpleAST,
    target_wasm: &mut Module,
    symbol_table: &mut SymbolTable,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_var_metadata: &mut ReportVarMetadata,
    err: &mut ErrorGen,
) {
    let mut mem_tracker = get_memory_tracker(target_wasm, true);

    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut injected_funcs = vec![];
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(
            target_wasm,
            symbol_table,
            &mut mem_tracker,
            map_lib_adapter,
            report_var_metadata,
        ),
        context_name: "".to_string(),
        err,
        injected_funcs: &mut injected_funcs,
    };
    init.run(whamm);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(
            target_wasm,
            &injected_funcs,
            symbol_table,
            &mut mem_tracker,
            map_lib_adapter,
            io_adapter,
            report_var_metadata,
        ),
        simple_ast,
        err,
    );
    instr.run();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    for gid in report_var_metadata.available_i32_gids.iter() {
        //should be 0, but good for cleanup
        err.add_compiler_warn(format!("Unused i32 GID: {}", gid));
        target_wasm.delete_global(GlobalID(*gid));
    }
}

fn get_memory_tracker(target_wasm: &mut Module, create_new_mem: bool) -> MemoryTracker {
    // Create the memory tracker + the map and metadata tracker
    let mem_id = if create_new_mem {
        let id = target_wasm.memories.len() as u32;
        target_wasm.memories.push(MemoryType {
            memory64: false,
            shared: false,
            initial: 1,
            maximum: None,
            page_size_log2: None,
        });
        id
    } else {
        // memory ID is just zero
        0
    };

    MemoryTracker {
        mem_id,
        curr_mem_offset: 0,
        required_initial_mem_size: 0,
        emitted_strings: HashMap::new(),
    }
}

fn get_symbol_table(ast: &mut Whamm, err: &mut ErrorGen) -> SymbolTable {
    let mut st = build_symbol_table(ast, err);
    err.check_too_many();
    verify_ast(ast, &mut st, err);
    st
}

fn verify_ast(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) {
    if !type_check(ast, st, err) {
        error!("AST failed verification!");
    }
    err.check_too_many();
}

fn get_script_ast(script: &String, err: &mut ErrorGen) -> Whamm {
    // Parse the script and build the AST
    match parse_script(script, err) {
        Some(ast) => {
            info!("successfully parsed");
            err.check_too_many();
            ast
        }
        None => {
            err.report();
            exit(1);
        }
    }
}
