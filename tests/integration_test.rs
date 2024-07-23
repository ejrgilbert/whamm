mod common;

use log::error;
use orca::ir::module::Module as WasmModule;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use wabt::{wasm2wat, Wat2Wasm};
use whamm::common::error::ErrorGen;
use whamm::emitter::rewriting::module_emitter::{MemoryTracker, ModuleEmitter};
use whamm::emitter::rewriting::visiting_emitter::VisitingEmitter;
use whamm::generator::init_generator::InitGenerator;
use whamm::generator::instr_generator::InstrGenerator;
use whamm::generator::simple_ast::build_simple_ast;

const APP_WASM_PATH: &str = "tests/apps/dfinity/users.wasm";

const OUT_BASE_DIR: &str = "target";
const OUT_WASM_NAME: &str = "out.wasm";

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// scripts without errors occurring.
#[test]
fn instrument_dfinity_with_fault_injection() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_fault_injection("dfinity", &mut err);
    assert!(!processed_scripts.is_empty());
    err.fatal_report("Integration Test");

    for (script_path, script_text, mut whamm, mut symbol_table) in processed_scripts {
        // Build the behavior tree from the AST
        let simple_ast = build_simple_ast(&whamm, &mut err);

        let buff = fs::read(APP_WASM_PATH).unwrap();
        let mut app_wasm =
            WasmModule::parse_only_module(&buff, false).expect("Failed to parse Wasm module");
        let mut err = ErrorGen::new(script_path.clone(), script_text, 0);

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
        err.fatal_report("Integration Test");

        // Phase 1 of instrumentation (actually emits the instrumentation code)
        // This structure is necessary since we need to have the fns/globals injected (a single time)
        // and ready to use in every body/predicate.
        let mut instr = InstrGenerator::new(
            VisitingEmitter::new(&mut app_wasm, &mut symbol_table, &mem_tracker),
            simple_ast,
            &mut err,
        );
        // TODO add assertions here once I have error logic in place to check that it worked!
        instr.run();
        err.fatal_report("Integration Test");

        if !Path::new(OUT_BASE_DIR).exists() {
            if let Err(err) = fs::create_dir(OUT_BASE_DIR) {
                error!("{}", err.to_string());
                panic!("Could not create base output path.");
            }
        }

        let out_wasm_path = format!("{OUT_BASE_DIR}/{OUT_WASM_NAME}");
        if let Err(e) = app_wasm.emit_wasm(&out_wasm_path) {
            err.add_error(ErrorGen::get_unexpected_error(
                true,
                Some(format!(
                    "Failed to dump instrumented wasm to {} from error: {}",
                    &out_wasm_path, e
                )),
                None,
            ))
        }
        err.fatal_report("Integration Test");

        let mut wasm2wat = Command::new("wasm2wat");
        wasm2wat.stdout(Stdio::null()).arg(out_wasm_path);

        // wasm2wat verification check
        match wasm2wat.status() {
            Ok(code) => {
                if !code.success() {
                    panic!("`wasm2wat` verification check failed!");
                }
            }
            Err(err) => {
                error!("{}", err.to_string());
                panic!("`wasm2wat` verification check failed!");
            }
        };
    }
}

#[test]
fn instrument_handwritten_wasm_call() {
    common::setup_logger();
    // executable is located at target/debug/whamm
    let executable = "target/debug/whamm";

    // if you want to change the wat file
    // (calling wat2wasm from a child process doesn't work
    //  since somehow the executable can't write to the file system directly)
    let file_data = fs::read("tests/apps/handwritten/add.wat").unwrap();
    let wasm_data = Wat2Wasm::new()
        .write_debug_names(true)
        .convert(file_data)
        .unwrap();
    fs::write("tests/apps/handwritten/add.wasm", wasm_data).unwrap();

    let res = Command::new(executable)
        .arg("instr")
        .arg("--script")
        .arg("tests/scripts/instr.mm")
        .arg("--app")
        .arg("tests/apps/handwritten/add.wasm")
        .arg("--output-path")
        .arg("output/integration-handwritten_add.wasm")
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());

    let file_data = fs::read("output/integration-handwritten_add.wasm").unwrap();
    let wat_data = wasm2wat(file_data).unwrap();
    println!("{}", wat_data);
}

#[test]
fn instrument_no_matches() {
    common::setup_logger();
    // executable is located at target/debug/whamm
    let executable = "target/debug/whamm";

    // if you want to change the wat file
    // (calling wat2wasm from a child process doesn't work
    //  since somehow the executable can't write to the file system directly)
    let file_data = fs::read("tests/apps/handwritten/no_matched_events.wat").unwrap();
    let wasm_data = Wat2Wasm::new()
        .write_debug_names(true)
        .convert(file_data)
        .unwrap();
    fs::write("tests/apps/handwritten/no_matched_events.wasm", wasm_data).unwrap();

    let res = Command::new(executable)
        .arg("instr")
        .arg("--script")
        .arg("tests/scripts/instr.mm")
        .arg("--app")
        .arg("tests/apps/handwritten/no_matched_events.wasm")
        .arg("--output-path")
        .arg("output/integration-no_matched_events.wasm")
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());

    let file_data = fs::read("output/integration-no_matched_events.wasm").unwrap();
    let wat_data = wasm2wat(file_data).unwrap();
    println!("{}", wat_data);
}

#[test]
fn instrument_control_flow() {
    common::setup_logger();
    let executable = "target/debug/whamm";

    // run cargo run on control flow
    let a = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    assert!(a.status.success());

    let res = Command::new(executable)
        .arg("instr")
        .arg("--script")
        .arg("tests/scripts/instr.mm")
        .arg("--app")
        .arg("wasm_playground/control_flow/target/wasm32-unknown-unknown/debug/cf.wasm")
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());

    let file_data = fs::read("output/output.wasm").unwrap();
    let wat_data = wasm2wat(file_data).unwrap();
    fs::write("output/output.wat", wat_data).unwrap();
}

#[test]
fn instrument_spin_with_fault_injection() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_fault_injection("spin", &mut err);
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_wizard_monitors() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_wizard_monitors(&mut err);
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_replay() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_replay(&mut err);
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}
