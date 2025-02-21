#![allow(clippy::too_many_arguments)]
use crate::cli::LibraryLinkStrategyArg;
use crate::common::error::ErrorGen;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::InjectStrategy;
use crate::generator::metadata_collector::MetadataCollector;
use crate::generator::rewriting::init_generator::InitGenerator;
use crate::generator::rewriting::instr_generator::InstrGenerator;
use crate::generator::rewriting::simple_ast::{build_simple_ast, SimpleAST};
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::io::IOPackage;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::maps::MapLibPackage;
use crate::lang_features::libraries::core::LibPackage;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info};
use orca_wasm::ir::id::{FunctionID, GlobalID};
use orca_wasm::ir::types::{DataType as OrcaType, InitExpr, Value as OrcaValue};
use orca_wasm::{Instructions, Module};
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

    let wasm_result = run(
        core_wasm_path,
        &mut target_wasm,
        &whamm_script,
        &script_path,
        max_errors,
        config,
    );

    try_path(&output_wasm_path);
    if let Err(e) = std::fs::write(&output_wasm_path, wasm_result) {
        unreachable!(
            "Failed to dump instrumented wasm to {} from error: {}",
            &output_wasm_path, e
        )
    }
}

pub fn run(
    core_wasm_path: &str,
    target_wasm: &mut Module,
    whamm_script: &String,
    script_path: &str,
    max_errors: i32,
    config: Config,
) -> Vec<u8> {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Process the script
    let mut whamm = get_script_ast(whamm_script, &mut err);
    let (mut symbol_table, has_reports) = get_symbol_table(&mut whamm, &mut err);
    err.check_too_many();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();
    let mut mem_allocator = get_memory_allocator(target_wasm, true);

    // Merge in the core library IF NEEDED
    let mut map_package = MapLibPackage::default();
    let mut io_package = IOPackage::new(*mem_allocator.mem_tracker_global);
    let mut core_packages: Vec<&mut dyn LibPackage> = vec![&mut map_package, &mut io_package];
    let mut injected_funcs = crate::lang_features::libraries::actions::link_core_lib(
        &config.library_strategy,
        &whamm,
        target_wasm,
        core_wasm_path,
        &mut mem_allocator,
        &mut core_packages,
        &mut err,
    );
    let mut map_lib_adapter = map_package.adapter;
    let mut io_adapter = io_package.adapter;
    let mut report_vars = ReportVars::new();
    let mut unshared_var_handler = UnsharedVarHandler::default();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Collect the metadata for the AST and transform to different representation
    // specifically used for targeting Wizard during compilation.
    let mut metadata_collector = MetadataCollector::new(&mut symbol_table, &mut err, &config);
    metadata_collector.visit_whamm(&whamm);
    if config.wizard {
        run_instr_wizard(
            metadata_collector,
            target_wasm,
            &mut mem_allocator,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_vars,
            &mut unshared_var_handler,
        );
    } else {
        let simple_ast = build_simple_ast(&whamm, &mut err);
        run_instr_rewrite(
            &mut whamm,
            simple_ast,
            target_wasm,
            &mut symbol_table,
            has_reports,
            &mut mem_allocator,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_vars,
            &mut unshared_var_handler,
            &mut injected_funcs,
            &mut err,
        );
    }

    // Bump the memory pages to account for used memory
    mem_allocator.memory_grow(target_wasm);
    // Update the memory tracker global to point to the start of free memory
    mem_allocator.update_memory_global_ptr(target_wasm);

    // for debugging
    report_vars.print_metadata();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    target_wasm.encode()
}

fn run_instr_wizard(
    metadata_collector: MetadataCollector,
    target_wasm: &mut Module,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_vars: &mut ReportVars,
    unshared_var_handler: &mut UnsharedVarHandler,
) {
    let table = metadata_collector.table;
    let err = metadata_collector.err;
    let config = metadata_collector.config;
    let wiz_ast = metadata_collector.ast;
    let used_funcs = metadata_collector.used_provided_fns;
    let used_report_dts = metadata_collector.used_report_var_dts;
    let used_strings = metadata_collector.strings_to_emit;

    let mut injected_funcs = vec![];
    let mut wizard_unshared_var_handler =
        crate::lang_features::alloc_vars::wizard::UnsharedVarHandler::default();
    let mut gen = crate::generator::wizard::WizardGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Wizard,
            target_wasm,
            table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
            unshared_var_handler,
        ),
        io_adapter,
        context_name: "".to_string(),
        err,
        injected_funcs: &mut injected_funcs,
        config,
        curr_script_id: u8::MAX,
        unshared_var_handler: &mut wizard_unshared_var_handler,
    };
    gen.run(wiz_ast, used_funcs, used_report_dts, used_strings);
}

fn run_instr_rewrite(
    whamm: &mut Whamm,
    simple_ast: SimpleAST,
    target_wasm: &mut Module,
    symbol_table: &mut SymbolTable,
    has_reports: bool,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_vars: &mut ReportVars,
    unshared_var_handler: &mut UnsharedVarHandler,
    injected_funcs: &mut Vec<FunctionID>,
    err: &mut ErrorGen,
) {
    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            symbol_table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
            unshared_var_handler,
        ),
        context_name: "".to_string(),
        err,
        injected_funcs,
    };
    init.run(whamm);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            injected_funcs,
            symbol_table,
            mem_allocator,
            map_lib_adapter,
            io_adapter,
            report_vars,
            unshared_var_handler,
        ),
        simple_ast,
        err,
        has_reports,
    );
    instr.run();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    for (ty, list) in unshared_var_handler.available_gids.iter() {
        //should be 0, but good for cleanup
        for gid in list.iter() {
            err.add_compiler_warn(format!("Unused {ty} GID: {}", gid));
            target_wasm.delete_global(GlobalID(*gid));
        }
    }
}

fn get_memory_allocator(target_wasm: &mut Module, create_new_mem: bool) -> MemoryAllocator {
    // Create the memory tracker + the map and metadata tracker
    let mem_id = if create_new_mem {
        *target_wasm.add_local_memory(MemoryType {
            memory64: false,
            shared: false,
            initial: 1,
            maximum: None,
            page_size_log2: None,
        })
    } else {
        // memory ID is just zero
        0
    };

    // todo -- only add if needed!
    let mem_tracker_global = target_wasm.add_global(
        InitExpr::new(vec![Instructions::Value(OrcaValue::I32(0))]),
        OrcaType::I32,
        true,
        false,
    );

    MemoryAllocator {
        mem_id,
        curr_mem_offset: 0,
        emitted_strings: HashMap::new(),
        mem_tracker_global,
        used_mem_checker_fid: None,
    }
}

fn get_symbol_table(ast: &mut Whamm, err: &mut ErrorGen) -> (SymbolTable, bool) {
    let mut st = build_symbol_table(ast, err);
    err.check_too_many();
    let has_reports = verify_ast(ast, &mut st, err);
    (st, has_reports)
}

fn verify_ast(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) -> bool {
    let (passed, has_reports) = type_check(ast, st, err);
    if !passed {
        error!("AST failed verification!");
    }
    err.check_too_many();

    has_reports
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
