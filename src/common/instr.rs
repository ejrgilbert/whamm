#![allow(clippy::too_many_arguments)]
use crate::api::instrument::Config;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::metrics::Metrics;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::tag_handler::get_tag_for;
use crate::emitter::InjectStrategy;
use crate::generator::metadata_collector::MetadataCollector;
use crate::generator::rewriting::init_generator::InitGenerator;
use crate::generator::rewriting::instr_generator::InstrGenerator;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::io::IOPackage;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::maps::MapLibPackage;
use crate::lang_features::libraries::core::{LibPackage, WHAMM_CORE_LIB_NAME};
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::exit;
use wasmparser::MemoryType;
use wirm::ir::id::FunctionID;
use wirm::ir::types::{DataType as WirmType, InitExpr, Value as WirmValue};
use wirm::{Component, InitInstr, Module};

use wirm::ir::module::side_effects::{InjectType as WirmInjectType, Injection as WirmInjection};

/// create output path if it doesn't exist
pub(crate) fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        std::fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

pub fn run_with_path(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Vec<u8> {
    let bytes = if !config.as_monitor_module {
        if let Ok(bytes) = std::fs::read(&app_wasm_path) {
            bytes
        } else {
            error!("Could not read from file: {app_wasm_path}");
            exit(1)
        }
    } else {
        vec![]
    };
    run_on_bytes_and_encode(
        core_lib,
        def_yamls,
        &bytes,
        script_path,
        user_lib_paths,
        max_errors,
        config,
    )
}

pub fn dry_run_on_bytes<'a>(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm_bytes: &'a [u8],
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Result<HashMap<WirmInjectType, Vec<WirmInjection<'a>>>, Vec<WhammError>> {
    // TODO -- support components here!
    let mut metrics = Metrics::default();
    let mut target_wasm = Module::parse(target_wasm_bytes, false).unwrap();
    if let Err(err) = run_on_module(
        core_lib,
        def_yamls,
        &mut target_wasm,
        script_path,
        user_lib_paths,
        max_errors,
        &mut metrics,
        config,
    ) {
        Err(err.pull_errs())
    } else {
        Ok(target_wasm.pull_side_effects())
    }
}

pub fn parse_user_lib_paths(paths: Vec<String>) -> Vec<(String, String, Vec<u8>)> {
    let mut res = vec![];
    for path in paths.iter() {
        let parts = path.split('=').collect::<Vec<&str>>();
        assert_eq!(2, parts.len(), "A user lib should be specified using the following format: <lib_name>=/path/to/lib.wasm");

        let lib_name = parts.first().unwrap().to_string();
        let lib_path = parts.get(1).unwrap();
        let buff = std::fs::read(lib_path).unwrap();

        res.push((lib_name, lib_path.to_string(), buff));
    }

    res
}

// pub fn run_on_bytes_and_encode(
//     core_lib: &[u8],
//     def_yamls: &Vec<String>,
//     target_wasm_bytes: Vec<u8>,
//     script_path: String,
//     user_lib_paths: Vec<String>,
//     max_errors: i32,
//     config: Config
// ) -> Vec<u8> {
//     let (metrics, new_wasm) = run_on_bytes(core_lib, def_yamls, target_wasm_bytes, script_path, user_lib_paths, max_errors, config);
//
//     let wasm = new_wasm.enco
//     todo!()
// }

pub fn run_on_bytes_and_encode(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm_bytes: &[u8],
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Vec<u8> {
    let mut metrics = Metrics::default();
    let encoded_bytes = if config.as_monitor_module {
        // handle emitting a monitor module
        // Create a new wasm file to use as `mon.wasm`
        let mut module = Module::default();
        if let Err(mut err) = run_on_module(
            core_lib,
            def_yamls,
            &mut module,
            script_path,
            user_lib_paths,
            max_errors,
            &mut metrics,
            config,
        ) {
            err.check_has_errors();
        }
        module.encode()
    } else {
        // handle a wasm component OR module
        match bytes_to_wasm(target_wasm_bytes) {
            (Some(mut module), None) => {
                // handle instrumenting a module
                if let Err(mut err) = run_on_module(
                    core_lib,
                    def_yamls,
                    &mut module,
                    script_path,
                    user_lib_paths,
                    max_errors,
                    &mut metrics,
                    config,
                ) {
                    err.check_has_errors();
                }
                module.encode()
            }
            (None, Some(mut component)) => {
                // handle instrumenting a component
                if let Err(mut err) = run_on_component(
                    core_lib,
                    def_yamls,
                    &mut component,
                    script_path,
                    user_lib_paths,
                    max_errors,
                    &mut metrics,
                    config,
                ) {
                    err.check_has_errors();
                }
                component.encode()
            }
            (None, None) => {
                // error, couldn't parse
                error!("Could not parse wasm bytes into a Module or a Component format.");
                exit(1)
            }
            (Some(_), Some(_)) => {
                // error, shouldn't parse as both
                error!("WHAMM BUG, please report: Something went wrong while parsing the Wasm bytes, shouldn't parse as BOTH a module and a component.");
                exit(1)
            }
        }
    };
    metrics.flush();

    encoded_bytes
}

fn bytes_to_wasm(target_wasm_bytes: &[u8]) -> (Option<Module>, Option<Component>) {
    // First try to parse as a wasm module
    if let Ok(module) = Module::parse(target_wasm_bytes, false) {
        (Some(module), None)
    } else if let Ok(component) = Component::parse(target_wasm_bytes, true) {
        (None, Some(component))
    } else {
        (None, None)
    }
}

pub fn run_on_component(
    core_lib: &[u8],
    def_yamls: &[String],
    target_wasm: &mut Component,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: Config,
) -> Result<(), Box<ErrorGen>> {
    // TODO -- figure out if this is a component (no need to make this a CLI option)
    //  I probably want to have an outer function that iterates over the modules within a
    //  component and then passes each &mut Module to the `run_on_module` function!
    //  This will keep from copy/pasting a ton of code.
    todo!()
}

pub fn run_on_module(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut Module,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: Config,
) -> Result<(), Box<ErrorGen>> {
    let user_libs = parse_user_lib_paths(user_lib_paths);

    // read in the whamm script
    let whamm_script = match std::fs::read_to_string(script_path.clone()) {
        Ok(unparsed_str) => unparsed_str,
        Err(error) => {
            error!("Cannot read specified file {}: {}", script_path, error);
            exit(1);
        }
    };

    run(
        core_lib,
        def_yamls,
        target_wasm,
        &whamm_script,
        &script_path,
        user_libs,
        max_errors,
        metrics,
        config,
    )
}

pub fn write_to_file(module: Vec<u8>, output_wasm_path: String) {
    try_path(&output_wasm_path);
    if let Err(e) = std::fs::write(&output_wasm_path, module) {
        unreachable!(
            "Failed to dump instrumented wasm to {} from error: {}",
            &output_wasm_path, e
        )
    }
}

pub fn run(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut Module,
    whamm_script: &String,
    script_path: &str,
    user_libs: Vec<(String, String, Vec<u8>)>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: Config,
) -> Result<(), Box<ErrorGen>> {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Parse user libraries to Wasm modules
    let mut user_lib_modules: HashMap<String, Module> = HashMap::default();
    for (lib_name, _, lib_buff) in user_libs.iter() {
        user_lib_modules.insert(lib_name.clone(), Module::parse(lib_buff, false).unwrap());
    }
    // add the core library just in case the script needs it
    user_lib_modules.insert(
        WHAMM_CORE_LIB_NAME.to_string(),
        Module::parse(core_lib, true).unwrap(),
    );

    // Process the script
    let mut whamm = get_script_ast(def_yamls, whamm_script, &mut err);
    let (mut symbol_table, has_reports) = get_symbol_table(&mut whamm, &user_lib_modules, &mut err);

    // If there were any errors encountered, report and exit!
    if err.has_errors {
        return Err(Box::new(err));
    }
    let mut mem_allocator = get_memory_allocator(target_wasm, true, config.as_monitor_module);

    // Collect the metadata for the AST and transform to different representation
    // specifically used for targeting Wizard during compilation.
    let mut metadata_collector = MetadataCollector::new(&mut symbol_table, &mut err, &config);
    metadata_collector.visit_whamm(&whamm);

    // Merge in the core library IF NEEDED
    let mut map_package = MapLibPackage::new(if config.as_monitor_module {
        InjectStrategy::Wizard
    } else {
        InjectStrategy::Rewriting
    });
    let mut io_package = IOPackage::new(*mem_allocator.mem_tracker_global);
    let mut core_packages: Vec<&mut dyn LibPackage> = vec![&mut map_package, &mut io_package];
    let mut injected_core_lib_funcs = crate::lang_features::libraries::actions::link_core_lib(
        config.library_strategy,
        &metadata_collector.ast,
        target_wasm,
        core_lib,
        &mut mem_allocator,
        &mut core_packages,
        metadata_collector.err,
    );

    // make the used user library functions the correct form
    let mut used_fns_per_lib: HashMap<String, HashSet<String>> = HashMap::default();
    for (used_lib, used_fn) in metadata_collector.used_user_library_fns.iter() {
        used_fns_per_lib
            .entry(used_lib.clone())
            .and_modify(|set| {
                set.insert(used_fn.clone());
            })
            .or_insert(HashSet::from_iter([used_fn.clone()].iter().cloned()));
    }
    let mut map_lib_adapter = map_package.adapter;
    let mut io_adapter = io_package.adapter;
    let mut report_vars = ReportVars::new();

    // If there were any errors encountered, report and exit!
    if metadata_collector.err.has_errors {
        return Err(Box::new(err));
    }

    if config.as_monitor_module {
        run_instr_wizard(
            metrics,
            metadata_collector,
            used_fns_per_lib,
            user_lib_modules,
            target_wasm,
            &mut mem_allocator,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_vars,
        );
    } else {
        let mut unshared_var_handler =
            UnsharedVarHandler::new(*target_wasm.add_local_memory_with_tag(
                MemoryType {
                    memory64: false,
                    shared: false,
                    initial: 1,
                    maximum: None,
                    page_size_log2: None,
                },
                get_tag_for(&None),
            ));

        run_instr_rewrite(
            metrics,
            &mut whamm,
            metadata_collector,
            used_fns_per_lib,
            user_lib_modules,
            target_wasm,
            has_reports,
            &mut mem_allocator,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_vars,
            &mut unshared_var_handler,
            &mut injected_core_lib_funcs,
        );

        // Bump the memory pages to account for used memory
        unshared_var_handler.memory_grow(target_wasm);
    }

    // Bump the memory pages to account for used memory
    mem_allocator.memory_grow(target_wasm);
    // Update the memory tracker global to point to the start of free memory
    mem_allocator.update_memory_global_ptr(target_wasm);

    // for debugging
    report_vars.print_metadata();

    if err.has_errors {
        // If there were any errors encountered, report and exit!
        Err(Box::new(err))
    } else {
        Ok(())
    }
}

fn run_instr_wizard(
    _metrics: &mut Metrics,
    metadata_collector: MetadataCollector,
    used_fns_per_lib: HashMap<String, HashSet<String>>,
    user_lib_modules: HashMap<String, Module>,
    target_wasm: &mut Module,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_vars: &mut ReportVars,
) {
    let table = metadata_collector.table;
    let err = metadata_collector.err;
    let config = metadata_collector.config;
    let wiz_ast = metadata_collector.ast;
    let used_funcs = metadata_collector.used_bound_fns;
    let used_report_dts = metadata_collector.used_report_var_dts;
    let used_strings = metadata_collector.strings_to_emit;

    let mut injected_funcs = vec![];
    let mut wizard_unshared_var_handler =
        crate::lang_features::alloc_vars::wizard::UnsharedVarHandler::new(target_wasm);
    let mut gen = crate::generator::wizard::WizardGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Wizard,
            target_wasm,
            table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
        ),
        io_adapter,
        context_name: "".to_string(),
        err,
        injected_funcs: &mut injected_funcs,
        config,
        used_fns_per_lib,
        user_lib_modules,
        curr_script_id: u8::MAX,
        unshared_var_handler: &mut wizard_unshared_var_handler,
    };
    gen.run(wiz_ast, used_funcs, used_report_dts, used_strings);
}

fn run_instr_rewrite(
    metrics: &mut Metrics,
    whamm: &mut Whamm,
    metadata_collector: MetadataCollector,
    used_fns_per_lib: HashMap<String, HashSet<String>>,
    user_lib_modules: HashMap<String, Module>,
    target_wasm: &mut Module,
    has_reports: bool,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_vars: &mut ReportVars,
    unshared_var_handler: &mut UnsharedVarHandler,
    injected_funcs: &mut Vec<FunctionID>,
) {
    let table = metadata_collector.table;
    let err = metadata_collector.err;
    let ast = metadata_collector.ast;
    let used_funcs = metadata_collector.used_bound_fns;
    let used_strings = metadata_collector.strings_to_emit;
    let config = metadata_collector.config;

    // Phase 0 of instrumentation (emit bound variables and fns)
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
        ),
        context_name: "".to_string(),
        err,
        used_fns_per_lib,
        user_lib_modules,
        injected_funcs,
    };
    init.run(whamm, used_funcs, used_strings);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let simple_ast = SimpleAST::new(ast);
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            injected_funcs,
            table,
            mem_allocator,
            map_lib_adapter,
            io_adapter,
            report_vars,
            unshared_var_handler,
        ),
        simple_ast,
        err,
        config,
        has_reports,
    );

    let match_time = "match&inject".to_string();
    if config.metrics {
        metrics.start(&match_time);
    }
    instr.run();
    if config.metrics {
        metrics.end(&match_time);
    }

    // If there were any errors encountered, report and exit!
    err.check_has_errors();
}

fn get_memory_allocator(
    target_wasm: &mut Module,
    create_new_mem: bool,
    as_monitor_module: bool,
) -> MemoryAllocator {
    // Create the memory tracker + the map and metadata tracker
    let mem_id = if create_new_mem {
        *target_wasm.add_local_memory_with_tag(
            MemoryType {
                memory64: false,
                shared: false,
                initial: 1,
                maximum: None,
                page_size_log2: None,
            },
            get_tag_for(&None),
        )
    } else {
        // memory ID is just zero
        0
    };

    // todo -- only add if needed!
    let mem_tracker_global = target_wasm.add_global_with_tag(
        InitExpr::new(vec![InitInstr::Value(WirmValue::I32(0))]),
        WirmType::I32,
        true,
        false,
        get_tag_for(&None),
    );

    let (alloc_var_mem_id, alloc_var_mem_tracker_global, engine_mem_id) = if as_monitor_module {
        let alloc_id = *target_wasm.add_local_memory_with_tag(
            MemoryType {
                memory64: false,
                shared: false,
                initial: 1,
                maximum: None,
                page_size_log2: None,
            },
            get_tag_for(&None),
        );
        let alloc_tracker_global = target_wasm.add_global_with_tag(
            InitExpr::new(vec![InitInstr::Value(WirmValue::I32(0))]),
            WirmType::I32,
            true,
            false,
            get_tag_for(&None),
        );
        let engine_id = *target_wasm.add_local_memory_with_tag(
            MemoryType {
                memory64: false,
                shared: false,
                initial: 1,
                maximum: None,
                page_size_log2: None,
            },
            get_tag_for(&None),
        );
        target_wasm
            .exports
            .add_export_mem("engine:data".to_string(), engine_id, None);

        (Some(alloc_id), Some(alloc_tracker_global), Some(engine_id))
    } else {
        (None, None, None)
    };

    MemoryAllocator::new(
        mem_id,
        mem_tracker_global,
        alloc_var_mem_id,
        alloc_var_mem_tracker_global,
        engine_mem_id,
    )
}

fn get_symbol_table(
    ast: &mut Whamm,
    user_libs: &HashMap<String, Module>,
    err: &mut ErrorGen,
) -> (SymbolTable, bool) {
    let mut st = build_symbol_table(ast, user_libs, err);
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

fn get_script_ast(def_yamls: &Vec<String>, script: &String, err: &mut ErrorGen) -> Whamm {
    // Parse the script and build the AST
    match parse_script(def_yamls, script, err) {
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
