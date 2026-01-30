#![allow(clippy::too_many_arguments)]
use crate::api::instrument::Config;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::metrics::Metrics;
use crate::emitter::InjectStrategy;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::generator::metadata_collector::MetadataCollector;
use crate::generator::rewriting::init_generator::InitGenerator;
use crate::generator::rewriting::instr_generator::InstrGenerator;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::core::io::IOPackage;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::maps::MapLibPackage;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::{LibPackage, WHAMM_CORE_LIB_NAME};
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::FunctionID;
use wirm::ir::module::side_effects::{InjectType as WirmInjectType, Injection as WirmInjection};
use wirm::ir::types::{DataType as WirmType, InitExpr, Value as WirmValue};
use wirm::opcode::Instrumenter;
use wirm::wasmparser::MemoryType;
use wirm::{InitInstr, Module, Opcode};

const ENGINE_BUFFER_NAME: &str = "whamm_buffer";
const ENGINE_BUFFER_START_NAME: &str = "whamm_buffer:start";
const ENGINE_BUFFER_MAX_NAME: &str = "whamm_buffer:max";
pub const ENGINE_BUFFER_MAX_SIZE: i32 = 2i32.pow(10); // max set to 1KB = 2^10 = 1024 bytes

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
) -> Result<Vec<u8>, Box<ErrorGen>> {
    let buff = if !config.as_monitor_module {
        std::fs::read(app_wasm_path).unwrap()
    } else {
        vec![]
    };

    let mut target_wasm = if !config.as_monitor_module {
        // Read app Wasm into Wirm module
        Module::parse(&buff, false, true).unwrap()
    } else {
        // Create a new wasm file to use as `mon.wasm`
        Module::default()
    };

    run_on_module_and_encode(
        core_lib,
        def_yamls,
        &mut target_wasm,
        script_path,
        user_lib_paths,
        max_errors,
        config,
    )
}

pub fn dry_run_on_bytes<'a>(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &'a mut Module,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Result<HashMap<WirmInjectType, Vec<WirmInjection<'a>>>, Vec<WhammError>> {
    let mut metrics = Metrics::default();
    if let Err(err) = run_on_module(
        core_lib,
        def_yamls,
        target_wasm,
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

pub fn parse_user_lib_paths(paths: Vec<String>) -> Vec<(String, Option<String>, String, Vec<u8>)> {
    let mut res = vec![];
    for path in paths.iter() {
        let parts = path.split('=').collect::<Vec<&str>>();
        assert_eq!(
            2,
            parts.len(),
            "A user lib should be specified using the following format: <lib_name>=/path/to/lib.wasm"
        );

        let lib_name_chunk = parts.first().unwrap().to_string();
        let name_parts = lib_name_chunk.split('(').collect::<Vec<&str>>();
        let lib_name = name_parts.first().unwrap().to_string();
        let lib_name_import_override = if name_parts.len() > 1 {
            Some(
                name_parts
                    .get(1)
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .to_string(),
            )
        } else {
            None
        };

        let lib_path = parts.get(1).unwrap();
        let buff = std::fs::read(lib_path).unwrap();

        res.push((
            lib_name,
            lib_name_import_override,
            lib_path.to_string(),
            buff,
        ));
    }

    res
}

pub fn run_on_module_and_encode(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut Module,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    let mut metrics = Metrics::default();
    run_on_module(
        core_lib,
        def_yamls,
        target_wasm,
        script_path,
        user_lib_paths,
        max_errors,
        &mut metrics,
        config,
    )?;

    let wasm = target_wasm.encode();
    metrics.flush();
    Ok(wasm)
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
            let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);
            err.add_instr_error(&format!(
                "Cannot read specified file {}: {}",
                script_path, error
            ));
            return Err(Box::new(err));
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
    user_libs: Vec<(String, Option<String>, String, Vec<u8>)>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: Config,
) -> Result<(), Box<ErrorGen>> {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Parse user libraries to Wasm modules
    let mut user_lib_paths: HashMap<String, String> = HashMap::new();
    let mut user_lib_modules: HashMap<String, (Option<String>, Module)> = HashMap::default();
    for (lib_name, lib_name_import_override, path, lib_buff) in user_libs.iter() {
        user_lib_modules.insert(
            lib_name.clone(),
            (
                lib_name_import_override.clone(),
                Module::parse(lib_buff, false, false).unwrap(),
            ),
        );
        user_lib_paths.insert(lib_name.clone(), path.clone());
    }
    // add the core library just in case the script needs it
    user_lib_modules.insert(
        WHAMM_CORE_LIB_NAME.to_string(),
        (None, Module::parse(core_lib, true, false).unwrap()),
    );

    // Process the script
    let mut whamm = match get_script_ast(def_yamls, whamm_script, &mut err) {
        Ok(whamm) => whamm,
        Err(_) => return Err(Box::new(err)),
    };
    // If there were any errors encountered during parsing, report and exit!
    if err.has_errors {
        return Err(Box::new(err));
    }
    let (mut symbol_table, has_reports) =
        match get_symbol_table(&mut whamm, &user_lib_modules, &mut err) {
            Ok(r) => r,
            Err(_) => return Err(Box::new(err)),
        };

    // If there were any errors encountered, report and exit!
    if err.has_errors {
        return Err(Box::new(err));
    }
    let mut mem_allocator = get_memory_allocator(target_wasm, true, config.as_monitor_module);

    // Collect the metadata for the AST and transform to different representation
    // specifically used for targeting wei during compilation.
    let mut metadata_collector = MetadataCollector::new(&mut symbol_table, &mut err, &config);
    metadata_collector.visit_whamm(&whamm);

    // Merge in the core library IF NEEDED
    let mut map_package = MapLibPackage::new(if config.as_monitor_module {
        InjectStrategy::Wei
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
    // If there were any errors encountered, report and exit!
    if metadata_collector.err.has_errors {
        return Err(Box::new(err));
    }

    // make the used user library functions the correct form
    let mut used_fns_per_lib: HashMap<String, HashSet<String>> = HashMap::default();
    let mut static_libs: HashSet<String> = HashSet::default();
    for ((used_lib, used_fn), is_static) in metadata_collector.used_user_library_fns.funcs.iter() {
        used_fns_per_lib
            .entry(used_lib.clone())
            .and_modify(|set| {
                set.insert(used_fn.clone());
            })
            .or_insert(HashSet::from_iter([used_fn.clone()].iter().cloned()));
        if *is_static {
            static_libs.insert(used_lib.clone());
        }
    }
    let mut map_lib_adapter = map_package.adapter;
    let mut io_adapter = io_package.adapter;
    let mut report_vars = ReportVars::new();

    if metadata_collector.err.has_errors {
        return Err(Box::new(err));
    }

    if config.as_monitor_module {
        run_instr_wei(
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

        if run_instr_rewrite(
            metrics,
            &mut whamm,
            metadata_collector,
            used_fns_per_lib,
            static_libs,
            user_lib_paths,
            user_lib_modules,
            target_wasm,
            has_reports,
            &mut mem_allocator,
            &mut io_adapter,
            &mut map_lib_adapter,
            &mut report_vars,
            &mut unshared_var_handler,
            &mut injected_core_lib_funcs,
        )
        .is_err()
        {
            return Err(Box::new(err));
        }

        // Bump the memory pages to account for used memory
        unshared_var_handler.memory_grow(target_wasm);
    }

    // Bump the memory pages to account for used memory
    mem_allocator.memory_grow(target_wasm);
    // Update the memory tracker global to point to the start of free memory
    mem_allocator.update_memory_global_ptrs(target_wasm);

    // for debugging
    report_vars.print_metadata();

    // report any warnings
    if err.has_warnings {
        err.report_warnings()
    }

    if err.has_errors {
        Err(Box::new(err))
    } else {
        Ok(())
    }
}

fn run_instr_wei(
    _metrics: &mut Metrics,
    metadata_collector: MetadataCollector,
    used_fns_per_lib: HashMap<String, HashSet<String>>,
    user_lib_modules: HashMap<String, (Option<String>, Module)>,
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
    let has_probe_state_init = metadata_collector.has_probe_state_init;

    let mut injected_funcs = vec![];
    let mut wei_unshared_var_handler =
        crate::lang_features::alloc_vars::wei::UnsharedVarHandler::new(target_wasm);
    let mut registry = WasmRegistry::default();

    let mut r#gen = crate::generator::wei::WeiGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Wei,
            target_wasm,
            table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
            // shouldn't need this for `wei`!
            &mut registry,
        ),
        io_adapter,
        context_name: "".to_string(),
        err,
        injected_funcs: &mut injected_funcs,
        config,
        used_fns_per_lib,
        user_lib_modules,
        curr_script_id: u8::MAX,
        unshared_var_handler: &mut wei_unshared_var_handler,
    };
    r#gen.run(
        wiz_ast,
        used_funcs,
        used_report_dts,
        used_strings,
        has_probe_state_init,
    );
    call_instr_init_at_start(None, target_wasm, err);
}

fn run_instr_rewrite(
    metrics: &mut Metrics,
    whamm: &mut Whamm,
    metadata_collector: MetadataCollector,
    used_fns_per_lib: HashMap<String, HashSet<String>>,
    static_libs: HashSet<String>,
    user_lib_paths: HashMap<String, String>,
    user_lib_modules: HashMap<String, (Option<String>, Module)>,
    target_wasm: &mut Module,
    has_reports: bool,
    mem_allocator: &mut MemoryAllocator,
    io_adapter: &mut IOAdapter,
    map_lib_adapter: &mut MapLibAdapter,
    report_vars: &mut ReportVars,
    unshared_var_handler: &mut UnsharedVarHandler,
    injected_funcs: &mut Vec<FunctionID>,
) -> Result<(), ()> {
    let table = metadata_collector.table;
    let err = metadata_collector.err;
    let ast = metadata_collector.ast;
    let used_funcs = metadata_collector.used_bound_fns;
    let used_strings = metadata_collector.strings_to_emit;
    let has_probe_state_init = metadata_collector.has_probe_state_init;
    let config = metadata_collector.config;

    let mut registry = WasmRegistry::new(&static_libs, &user_lib_paths, err);

    // Phase 0 of instrumentation (emit bound variables and fns)
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            table,
            mem_allocator,
            map_lib_adapter,
            report_vars,
            &mut registry,
        ),
        context_name: "".to_string(),
        err,
        used_fns_per_lib,
        user_lib_modules,
        injected_funcs,
    };
    init.run(whamm, used_funcs, used_strings, has_probe_state_init);
    if err.has_errors {
        return Err(());
    }

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let simple_ast = SimpleAST::new(ast);
    let mut init_func = FunctionBuilder::new(&[], &[]);
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(
            InjectStrategy::Rewriting,
            target_wasm,
            &mut init_func,
            injected_funcs,
            table,
            mem_allocator,
            map_lib_adapter,
            io_adapter,
            report_vars,
            unshared_var_handler,
            &mut registry,
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
    configure_init_func(init_func, target_wasm, err);
    if config.metrics {
        metrics.end(&match_time);
    }

    if err.has_errors { Err(()) } else { Ok(()) }
}
pub fn configure_init_func<'a>(
    init_func: FunctionBuilder<'a>,
    module: &mut Module<'a>,
    err: &mut ErrorGen,
) {
    let state_init_id = if init_func.body.num_instructions > 0 {
        // Call the probe init state function in the instr_init body
        let state_init_id = init_func.finish_module_with_tag(module, get_tag_for(&None));
        module.set_fn_name(state_init_id, "init_probe_state".to_string());
        Some(state_init_id)
    } else {
        None
    };
    call_instr_init_at_start(state_init_id, module, err);
}

fn call_instr_init_at_start(
    state_init_id: Option<FunctionID>,
    module: &mut Module,
    err: &mut ErrorGen,
) {
    if let Some(instr_init_fid) = module.functions.get_local_fid_by_name("instr_init") {
        if let Some(state_init_id) = state_init_id {
            if let Some(mut instr_init) = module.functions.get_fn_modifier(instr_init_fid) {
                instr_init.call(state_init_id);
            } else {
                unreachable!("Should have found the function in the module.")
            }
        }

        // now call `instr_init` in the module's start function
        let (start_fid, _was_created) = ModuleEmitter::get_or_create_start_func(module);
        if let Some(mut start_func) = module.functions.get_fn_modifier(FunctionID(start_fid)) {
            start_func.func_entry();
            start_func.call(instr_init_fid);

            let op_idx = start_func.curr_instr_len() as u32;
            start_func.append_tag_at(
                get_probe_tag_data(&None, op_idx),
                // location is unused
                wirm::Location::Module {
                    func_idx: FunctionID(0),
                    instr_idx: 0,
                },
            );
            start_func.finish_instr();
        } else {
            err.add_internal_error("Should have found the function in the module.", &None);
        }
    } else if state_init_id.is_some() {
        err.add_internal_error(
            "If there's a state init function, there should be an instr_init function!",
            &None,
        );
    }
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

    let (alloc_var_mem_id, alloc_var_mem_tracker_global, engine_mem_id, engine_mem_start_id) =
        if as_monitor_module {
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
            let engine_mem_start_id = target_wasm.add_global_with_tag(
                InitExpr::new(vec![InitInstr::Value(WirmValue::I32(0))]),
                WirmType::I32,
                false,
                false,
                get_tag_for(&None),
            );
            let engine_mem_max_id = target_wasm.add_global_with_tag(
                InitExpr::new(vec![InitInstr::Value(WirmValue::I32(
                    ENGINE_BUFFER_MAX_SIZE,
                ))]),
                WirmType::I32,
                false,
                false,
                get_tag_for(&None),
            );
            target_wasm.exports.add_export_mem_with_tag(
                ENGINE_BUFFER_NAME.to_string(),
                // just use the same memory we store our static strings in (enables more efficient strcmp in predicates for the engine)
                mem_id,
                get_tag_for(&None),
            );
            target_wasm.exports.add_export_global_with_tag(
                ENGINE_BUFFER_START_NAME.to_string(),
                *engine_mem_start_id,
                get_tag_for(&None),
            );
            target_wasm.exports.add_export_global_with_tag(
                ENGINE_BUFFER_MAX_NAME.to_string(),
                *engine_mem_max_id,
                get_tag_for(&None),
            );

            (
                Some(alloc_id),
                Some(alloc_tracker_global),
                Some(mem_id),
                Some(engine_mem_start_id),
            )
        } else {
            (None, None, None, None)
        };

    MemoryAllocator::new(
        mem_id,
        mem_tracker_global,
        alloc_var_mem_id,
        alloc_var_mem_tracker_global,
        engine_mem_id,
        engine_mem_start_id,
    )
}

fn get_symbol_table(
    ast: &mut Whamm,
    user_libs: &HashMap<String, (Option<String>, Module)>,
    err: &mut ErrorGen,
) -> Result<(SymbolTable, bool), ()> {
    let mut st = build_symbol_table(ast, user_libs, err);
    if err.too_many {
        return Err(());
    }

    let has_reports = verify_ast(ast, &mut st, err)?;
    Ok((st, has_reports))
}

fn verify_ast(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) -> Result<bool, ()> {
    let (passed, has_reports) = type_check(ast, st, err);
    if !passed {
        error!("AST failed verification!");
    }
    if err.too_many | !passed {
        return Err(());
    }

    Ok(has_reports)
}

fn get_script_ast(
    def_yamls: &Vec<String>,
    script: &String,
    err: &mut ErrorGen,
) -> Result<Whamm, ()> {
    // Parse the script and build the AST
    match parse_script(def_yamls, script, err) {
        Some(ast) => {
            info!("successfully parsed");
            if err.too_many {
                return Err(());
            }
            Ok(ast)
        }
        None => Err(()),
    }
}
