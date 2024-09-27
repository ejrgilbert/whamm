use crate::common::error::ErrorGen;
use crate::emitter::map_lib_adapter::MapLibAdapter;
use crate::emitter::report_var_metadata::ReportVarMetadata;
use crate::emitter::rewriting::module_emitter::{MemoryTracker, ModuleEmitter};
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::generator::init_generator::InitGenerator;
use crate::generator::instr_generator::InstrGenerator;
use crate::generator::simple_ast::build_simple_ast;
use crate::parser::types::Whamm;
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info};
use orca_wasm::ir::id::GlobalID;
use orca_wasm::Module;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use crate::cli::LibraryLinkStrategyArg;

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
    Imported
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
    /// Whether to emit Virgil code as the instrumentation code
    pub virgil: bool,

    /// Whether to emit extra exported functions that are helpful during testing.
    pub testing: bool,

    /// The strategy to take when handling the injecting references to the `whamm!` library.
    pub library_strategy: LibraryLinkStrategy,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            virgil: false,
            testing: false,
            library_strategy: LibraryLinkStrategy::Imported,
        }
    }
}
impl Config {
    pub fn new(virgil: bool, testing: bool, link_strategy: Option<LibraryLinkStrategyArg>) -> Self {
        if virgil {
            error!("Targeting Virgil is not yet supported!");
            exit(1);
        }
        if testing {
            error!("Generating helper methods for testing mode is not yet supported!");
            exit(1);
        }
        let library_strategy = LibraryLinkStrategy::from(link_strategy);
        Self {
            virgil,
            testing,
            library_strategy
        }
    }
}

pub fn run_with_path(
    app_wasm_path: String,
    script_path: String,
    output_wasm_path: String,
    max_errors: i32,
    config: Config
) {
    // Read app Wasm into Orca module
    let buff = std::fs::read(app_wasm_path).unwrap();
    let mut app_wasm = Module::parse(&buff, false).unwrap();

    // read in the whamm script
    let whamm_script = match std::fs::read_to_string(script_path.clone()) {
        Ok(unparsed_str) => unparsed_str,
        Err(error) => {
            error!("Cannot read specified file {}: {}", script_path, error);
            exit(1);
        }
    };

    run(
        &mut app_wasm,
        &whamm_script,
        &script_path,
        Some(output_wasm_path),
        max_errors,
        config,
    );
}

pub fn run(
    app_wasm: &mut Module,
    whamm_script: &String,
    script_path: &str,
    output_wasm_path: Option<String>,
    max_errors: i32,
    config: Config
) -> Vec<u8> {
    // TODO -- use config!
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Process the script
    let mut whamm = get_script_ast(whamm_script, &mut err);
    let mut symbol_table = get_symbol_table(&mut whamm, &mut err);
    let simple_ast = build_simple_ast(&whamm, &mut err);
    err.check_too_many();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // TODO Configure the generator based on target (wizard vs bytecode rewriting)

    // Create the memory tracker + the map and metadata tracker
    if app_wasm.memories.len() > 1 {
        // TODO -- make this work with multi-memory
        panic!("only single memory is supported")
    };
    let mut mem_tracker = MemoryTracker {
        mem_id: 0,                     // Assuming the ID of the first memory is 0!
        curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
        required_initial_mem_size: 27, // Size memory must be to account for the added data
        emitted_strings: HashMap::new(),
    };
    let mut map_lib_adapter = MapLibAdapter::new();
    let mut report_var_metadata = ReportVarMetadata::new();

    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut injected_funcs = vec![];
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(
            app_wasm,
            &mut symbol_table,
            &mut mem_tracker,
            &mut map_lib_adapter,
            &mut report_var_metadata,
        ),
        context_name: "".to_string(),
        err: &mut err,
        injected_funcs: &mut injected_funcs,
    };
    init.run(&mut whamm);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(
            app_wasm,
            &injected_funcs,
            &mut symbol_table,
            &mut mem_tracker,
            &mut map_lib_adapter,
            &mut report_var_metadata,
        ),
        simple_ast,
        &mut err,
    );
    instr.run();
    // If there were any errors encountered, report and exit!
    err.check_has_errors();
    report_var_metadata.print_metadata();
    for gid in report_var_metadata.available_i32_gids.iter() {
        //should be 0, but good for cleanup
        err.add_compiler_warn(format!("Unused i32 GID: {}", gid));
        app_wasm.delete_global(GlobalID(*gid));
    }

    if let Some(output_wasm_path) = output_wasm_path {
        try_path(&output_wasm_path);
        if let Err(e) = app_wasm.emit_wasm(&output_wasm_path) {
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
    app_wasm.encode()
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
