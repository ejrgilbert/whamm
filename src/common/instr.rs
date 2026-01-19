#![allow(clippy::too_many_arguments)]
use crate::api::get_core_lib;
use crate::api::instrument::Config;
use crate::common::error::{ErrorGen, WhammError};
use crate::common::metrics::Metrics;
use crate::emitter::memory_allocator::MemoryAllocator;
use crate::emitter::module_emitter::ModuleEmitter;
use crate::emitter::rewriting::visiting_emitter::{get_main_or_start_fid, VisitingEmitter};
use crate::emitter::tag_handler::{get_probe_tag_data, get_tag_for};
use crate::emitter::InjectStrategy;
use crate::generator::metadata_collector::MetadataCollector;
use crate::generator::rewriting::init_generator::InitGenerator;
use crate::generator::rewriting::instr_generator::InstrGenerator;
use crate::generator::rewriting::simple_ast::SimpleAST;
use crate::lang_features::alloc_vars::rewriting::UnsharedVarHandler;
use crate::lang_features::libraries::actions::configure_component_libraries;
use crate::lang_features::libraries::core::io::io_adapter::IOAdapter;
use crate::lang_features::libraries::core::io::IOPackage;
use crate::lang_features::libraries::core::maps::map_adapter::MapLibAdapter;
use crate::lang_features::libraries::core::maps::MapLibPackage;
use crate::lang_features::libraries::core::{LibPackage, WHAMM_CORE_LIB_NAME};
use crate::lang_features::libraries::registry::WasmRegistry;
use crate::lang_features::report_vars::ReportVars;
use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::parse_script;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use log::{error, info, warn};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use wirm::ir::function::FunctionBuilder;
use wirm::ir::id::FunctionID;
use wirm::ir::module::side_effects::{InjectType as WirmInjectType, Injection as WirmInjection};
use wirm::ir::types::{DataType as WirmType, InitExpr, Value as WirmValue};
use wirm::opcode::Instrumenter;
use wirm::wasmparser::MemoryType;
use wirm::{Component, InitInstr, Module, Opcode};
use wirm::ir::component::ComponentHandle;

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
    core_lib_path: Option<String>,
    def_yamls: &Vec<String>,
    app_wasm_path: String,
    script_path: String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    let bytes = if !config.as_monitor_module {
        if let Ok(bytes) = std::fs::read(&app_wasm_path) {
            bytes
        } else {
            panic!("Could not read from file: {app_wasm_path}");
        }
    } else {
        vec![]
    };

    run_on_bytes_and_encode(
        core_lib_path,
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
    let mut metrics = Metrics::default();
    dry_run_module_or_component(
        target_wasm_bytes,
        core_lib,
        def_yamls,
        &script_path,
        &user_lib_paths,
        max_errors,
        &mut metrics,
        &config,
    )
}

fn dry_run_module_or_component<'a>(
    target_wasm_bytes: &'a [u8],
    core_lib_bytes: &[u8],
    def_yamls: &Vec<String>,
    script_path: &String,
    user_lib_paths: &[String],
    max_errors: i32,
    metrics: &mut Metrics,
    config: &Config,
) -> Result<HashMap<WirmInjectType, Vec<WirmInjection<'a>>>, Vec<WhammError>> {
    // handle a wasm component OR module
    match bytes_to_wasm(target_wasm_bytes) {
        (Some(mut module), None) => {
            if let Err(err) = run_on_module(
                core_lib_bytes,
                def_yamls,
                &mut module,
                script_path,
                &user_lib_paths.to_vec(),
                false,
                max_errors,
                metrics,
                config,
            ) {
                Err(err.pull_errs())
            } else {
                Ok(module.pull_side_effects())
            }
        }
        (None, Some(_component)) => {
            todo!("We haven't supported pulling side effects from components yet...sorry :/")
        }
        (None, None) => {
            // error, couldn't parse
            panic!("Could not parse wasm bytes into a Module or a Component format.");
        }
        (Some(_), Some(_)) => {
            // error, shouldn't parse as both
            panic!("WHAMM BUG, please report: Something went wrong while parsing the Wasm bytes, shouldn't parse as BOTH a module and a component.");
        }
    }
}

pub fn parse_user_lib_paths(paths: &Vec<String>) -> Vec<(String, Option<String>, String, Vec<u8>)> {
    let mut res = vec![];
    for path in paths.iter() {
        let parts = path.split('=').collect::<Vec<&str>>();
        assert_eq!(2, parts.len(), "A user lib should be specified using the following format: <lib_name>=/path/to/lib.wasm");

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

pub fn run_on_bytes_and_encode(
    core_lib_path: Option<String>,
    def_yamls: &Vec<String>,
    target_wasm_bytes: &[u8],
    script_path: String,
    user_libs: Vec<String>,
    max_errors: i32,
    config: Config,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    let mut metrics = Metrics::default();

    let res = if config.as_monitor_module {
        // handle emitting a monitor module
        // Create a new wasm file to use as `mon.wasm`
        let mut module = Module::default();
        let core_lib_bytes = get_core_lib(core_lib_path, true);

        // add the core library just in case the script needs it
        run_on_module(
            &core_lib_bytes,
            def_yamls,
            &mut module,
            &script_path,
            &user_libs,
            false,
            max_errors,
            &mut metrics,
            &config,
        )?;
        Ok(module.encode())
    } else {
        let core_lib_bytes = get_core_lib(
            core_lib_path,
            Module::parse(target_wasm_bytes, true, true).is_ok(),
        );

        // add the core library just in case the script needs it
        run_and_encode_module_or_component(
            target_wasm_bytes,
            &core_lib_bytes,
            def_yamls,
            &script_path,
            user_libs,
            max_errors,
            &mut metrics,
            &config,
        )
    };
    metrics.flush();

    res
}

fn run_and_encode_module_or_component(
    target_wasm_bytes: &[u8],
    core_lib_bytes: &[u8],
    def_yamls: &Vec<String>,
    script_path: &String,
    user_lib_paths: Vec<String>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: &Config,
) -> Result<Vec<u8>, Box<ErrorGen>> {
    // handle a wasm component OR module
    let res = match bytes_to_wasm(target_wasm_bytes) {
        (Some(mut module), None) => {
            match run_on_module(
                &core_lib_bytes,
                def_yamls,
                &mut module,
                script_path,
                &user_lib_paths,
                false,
                max_errors,
                metrics,
                config,
            ) {
                Ok(ran) => {
                    if !ran {
                        panic!("Module was not instrument-able (MUST have a main).");
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
            module.encode()
        }
        (None, Some(mut component)) => {
            // make sure none of the user libraries are provided as modules
            check_is_component(WHAMM_CORE_LIB_NAME, &core_lib_bytes);

            let user_libs = parse_user_lib_paths(&user_lib_paths);
            let mut user_lib_bytes = HashMap::new();
            for (name, _, _, bytes) in user_libs.iter() {
                check_is_component(name, bytes);
                user_lib_bytes.insert(name.to_string(), bytes.as_slice());
            }

            fn check_is_component(name: &str, bytes: &[u8]) {
                if Module::parse(bytes, true, true).is_ok() {
                    panic!("When instrumenting a component, the libraries MUST be provided as components, this library is a module: {}", name);
                }
            }
            match run_on_component(
                &core_lib_bytes,
                def_yamls,
                &mut component,
                script_path,
                &user_lib_paths,
                max_errors,
                metrics,
                config,
            ) {
                Ok(ran_on) => {
                    if let Some(id) = ran_on {
                        // Now that the component has been instrumented, we need to add in the support libraries
                        // so that they are linked appropriately!
                        configure_component_libraries(
                            id as u32,
                            &mut component,
                            &core_lib_bytes,
                            &user_lib_bytes,
                        );
                    } else {
                        panic!("Could not find an instrument-able module in the target component (MUST have a main).");
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }

            component.encode()
        }
        (None, None) => {
            // error, couldn't parse
            panic!("Could not parse wasm bytes into a Module or a Component format.");
        }
        (Some(_), Some(_)) => {
            // error, shouldn't parse as both
            panic!("WHAMM BUG, please report: Something went wrong while parsing the Wasm bytes, shouldn't parse as BOTH a module and a component.");
        }
    };
    info!("Successfully instrumented your Wasm application!");

    Ok(res)
}

fn bytes_to_wasm(target_wasm_bytes: &[u8]) -> (Option<Module<'_>>, Option<ComponentHandle<'_>>) {
    // First try to parse as a wasm module
    if let Ok(module) = Module::parse(target_wasm_bytes, false, true) {
        (Some(module), None)
    } else if let Ok(component) = Component::parse(target_wasm_bytes, true, true) {
        (None, Some(component))
    } else {
        (None, None)
    }
}

fn run_on_component(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut ComponentHandle,
    script_path: &String,
    user_lib_paths: &Vec<String>,
    max_errors: i32,
    metrics: &mut Metrics,
    config: &Config,
) -> Result<Option<usize>, Box<ErrorGen>> {
    // instrument the component's modules first
    for i in 0..target_wasm.modules.len() {
        if target_wasm.mut_module_at(i, |module| -> Result<bool, Box<ErrorGen>> {
            run_on_module(
                core_lib,
                def_yamls,
                module,
                script_path,
                user_lib_paths,
                true,
                max_errors,
                metrics,
                config,
            )
        })? {
            return Ok(Some(i));
        }
    }

    // then visit the component's components
    for i in 0..target_wasm.components.len() {
        if let Ok(id) = target_wasm.mut_component_at(i, |component|-> Result<Option<usize>, Box<ErrorGen>> {
            run_on_component(
                core_lib,
                def_yamls,
                component,
                script_path,
                user_lib_paths,
                max_errors,
                metrics,
                config,
            )
        }) {
            return Ok(id);
        }
    }

    Ok(None)
}

pub fn run_on_module(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut Module,
    script_path: &String,
    user_lib_paths: &Vec<String>,
    libs_as_components: bool,
    max_errors: i32,
    metrics: &mut Metrics,
    config: &Config,
) -> Result<bool, Box<ErrorGen>> {
    let user_libs = parse_user_lib_paths(user_lib_paths);

    // check if the module has a main or start function
    if !has_main_or_start(target_wasm) {
        // neither exists, unsure how to support this...this would be a library instead of an application I guess?
        // Maybe the answer is to expose query functions that can give a status update of the `report` vars?
        warn!("Your target Wasm has no main or start function...we do not support report variables in this scenario.");

        if libs_as_components {
            // we're instrumenting a component, need to skip this module for now
            return Ok(false);
        }
    }

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
        script_path,
        user_libs,
        libs_as_components,
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

fn has_main_or_start(module: &Module) -> bool {
    get_main_or_start_fid(module).is_some()
}

pub fn run(
    core_lib: &[u8],
    def_yamls: &Vec<String>,
    target_wasm: &mut Module,
    whamm_script: &String,
    script_path: &str,
    user_libs: Vec<(String, Option<String>, String, Vec<u8>)>,
    libs_as_components: bool,
    max_errors: i32,
    metrics: &mut Metrics,
    config: &Config,
) -> Result<bool, Box<ErrorGen>> {
    // TODO: Don't assume that libraries are modules or components, handle further down
    //       if necessary, can pass bool 'is_component' as well

    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.to_string(), "".to_string(), max_errors);

    // Parse user libraries to Wasm modules
    let mut user_lib_paths: HashMap<String, String> = HashMap::new();
    let mut user_lib_modules: HashMap<String, (Option<String>, &[u8])> = HashMap::default();
    for (lib_name, lib_name_import_override, path, lib_buff) in user_libs.iter() {
        user_lib_modules.insert(
            lib_name.clone(),
            (lib_name_import_override.clone(), lib_buff.as_slice()),
        );
        user_lib_paths.insert(lib_name.clone(), path.clone());
    }
    // add the core library just in case the script needs it
    user_lib_modules.insert(WHAMM_CORE_LIB_NAME.to_string(), (None, core_lib));

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
        match get_symbol_table(&mut whamm, &user_lib_modules, libs_as_components, &mut err) {
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
        libs_as_components,
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
            &user_lib_modules,
            libs_as_components,
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
            &user_lib_modules,
            libs_as_components,
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
        Ok(true)
    }
}

fn run_instr_wei(
    _metrics: &mut Metrics,
    metadata_collector: MetadataCollector,
    used_fns_per_lib: HashMap<String, HashSet<String>>,
    user_lib_modules: &HashMap<String, (Option<String>, &[u8])>,
    libs_as_components: bool,
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

    let mut gen = crate::generator::wei::WeiGenerator {
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
        libs_as_components,
        curr_script_id: u8::MAX,
        unshared_var_handler: &mut wei_unshared_var_handler,
    };
    gen.run(
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
    user_lib_modules: &HashMap<String, (Option<String>, &[u8])>,
    libs_as_components: bool,
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
        libs_as_components,
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

    if err.has_errors {
        Err(())
    } else {
        Ok(())
    }
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
    user_libs: &HashMap<String, (Option<String>, &[u8])>,
    libs_as_components: bool,
    err: &mut ErrorGen,
) -> Result<(SymbolTable, bool), ()> {
    let mut st = build_symbol_table(ast, user_libs, libs_as_components, err);
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
