pub mod wast_harness;

use orca::ir::module::Module as WasmModule;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use whamm::parser::types::Whamm;
use whamm::parser::whamm_parser::*;

use glob::{glob, glob_with};
use log::{debug, error, info, warn};
use wabt::{wasm2wat, wat2wasm};
use whamm::common::error::ErrorGen;
use whamm::emitter::rewriting::module_emitter::{MemoryTracker, ModuleEmitter};
use whamm::emitter::rewriting::visiting_emitter::VisitingEmitter;
use whamm::generator::init_generator::InitGenerator;
use whamm::generator::instr_generator::InstrGenerator;
use whamm::generator::simple_ast::build_simple_ast;
use whamm::verifier::verifier::{build_symbol_table, type_check};

// ====================
// = Helper Functions =
// ====================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const TEST_RSC_DIR: &str = "tests/scripts/";
const MM_PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

fn get_test_scripts(sub_dir: &str) -> Vec<(PathBuf, String)> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*MM_PATTERN.to_owned()))
        .expect("Failed to read glob pattern")
    {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        scripts.push((file_name.clone(), unparsed_file));
    }

    for path in glob_with(
        &(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*TODO.to_owned()),
        options,
    )
    .expect("Failed to read glob pattern")
    {
        warn!(
            "File marked with TODO: {}",
            path.as_ref().unwrap().display()
        );
    }

    scripts
}

fn get_ast(script: &str, err: &mut ErrorGen) -> Option<Whamm> {
    info!("Getting the AST");
    match parse_script(&script.to_string(), err) {
        Some(ast) => Some(ast),
        None => {
            error!("Parse failed");
            err.report();
            None
        }
    }
}

pub fn run_whamm(
    wasm_module_bytes: &[u8],
    whamm_script: &String,
    script_path: &str,
) -> (Vec<u8>, String) {
    let mut err = ErrorGen::new(script_path.to_string(), whamm_script.clone(), 0);

    let ast_res = get_ast(whamm_script, &mut err);
    assert!(
        ast_res.is_some(),
        "script = '{}' is not recognized as valid, but it should be",
        &whamm_script
    );
    let mut whamm = ast_res.unwrap();
    err.fatal_report("IntegrationTest");

    // Verify phase
    let mut symbol_table = build_symbol_table(&mut whamm, &mut err);
    symbol_table.reset();
    type_check(&mut whamm, &mut symbol_table, &mut err);
    err.fatal_report("IntegrationTest");

    // Translate to the simple AST
    let simple_ast = build_simple_ast(&whamm, &mut err);

    let mut app_wasm =
        WasmModule::parse(wasm_module_bytes, false).expect("Failed to parse Wasm module");

    // Create the memory tracker
    if app_wasm.memories.len() > 1 {
        // TODO -- make this work with multi-memory
        panic!("only single memory is supported")
    };
    let mut mem_tracker = MemoryTracker {
        mem_id: 0,                  // Assuming the ID of the first memory is 0!
        curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
        emitted_strings: HashMap::new(),
    };

    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(&mut app_wasm, &mut symbol_table, &mut mem_tracker),
        context_name: "".to_string(),
        err: &mut err,
    };
    assert!(init.run(&mut whamm));
    err.fatal_report("IntegrationTest");

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(&mut app_wasm, &mut symbol_table, &mem_tracker),
        simple_ast,
        &mut err,
    );
    instr.run();
    err.fatal_report("IntegrationTest");

    let instrumented_module_wasm = app_wasm.encode();
    let instrumented_module_wat = match wasm2wat(&instrumented_module_wasm) {
        Err(e) => panic!("`wasm2wat` verification check failed with error: {}", e),
        Ok(wat) => wat,
    };

    (instrumented_module_wasm, instrumented_module_wat)
}

pub fn run_whamm_bin(original_wasm_path: &str, monitor_path: &str, instrumented_wasm_path: &str) {
    // executable is located at target/debug/whamm
    let executable = "target/debug/whamm";

    let res = Command::new(executable)
        .arg("instr")
        .arg("--script")
        .arg(monitor_path)
        .arg("--app")
        .arg(original_wasm_path)
        .arg("--output-path")
        .arg(instrumented_wasm_path)
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());
}

pub fn run_basic_instrumentation(
    original_wat_path: &str,
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
) {
    wat2wasm_on_file(original_wat_path, original_wasm_path);
    run_whamm_bin(original_wasm_path, monitor_path, instrumented_wasm_path);
    wasm2wat_on_file(instrumented_wasm_path);
}

pub fn wat2wasm_on_file(original_wat_path: &str, original_wasm_path: &str) {
    // if you want to change the wat file
    // (calling wat2wasm from a child process doesn't work
    //  since somehow the executable can't write to the file system directly)
    let file_data = fs::read(original_wat_path).unwrap();
    let wasm_data = match wat2wasm(file_data) {
        Err(e) => {
            panic!("wat2wasm failed with error: {}", e)
        }
        Ok(data) => data,
    };

    fs::write(original_wasm_path, wasm_data).unwrap();
}

pub fn wasm2wat_on_file(instrumented_wasm_path: &str) {
    let file_data = fs::read(instrumented_wasm_path).unwrap();
    let wat_data = match wasm2wat(file_data) {
        Err(e) => {
            panic!("wasm2wat failed with error: {}", e)
        }
        Ok(data) => data,
    };

    debug!("{}", wat_data);
}

/// create output path if it doesn't exist
fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

pub fn setup_fault_injection(variation: &str) -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts(format!("fault_injection/{variation}").as_str());
    if scripts.is_empty() {
        warn!("No test scripts found for `fault_injection/{variation}` test.");
    }

    scripts
}

pub fn setup_wizard_monitors() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.is_empty() {
        warn!("No test scripts found for `wizard_monitors` test.");
    }

    scripts
}

pub fn setup_replay() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.is_empty() {
        warn!("No test scripts found for `replay` test.");
    }

    scripts
}
