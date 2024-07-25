mod common;

use crate::common::run_whamm;
use log::error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use wabt::{wasm2wat, Wat2Wasm};
use whamm::common::error::ErrorGen;

const APP_WASM_PATH: &str = "tests/apps/dfinity/users.wasm";

const OUT_BASE_DIR: &str = "target";
const OUT_WASM_NAME: &str = "out.wasm";

#[test]
fn run_wast_tests() {
    common::wast_harness::main().expect("WAST Tests failed!");
}

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// scripts without errors occurring.
#[test]
fn instrument_dfinity_with_fault_injection() {
    common::setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_fault_injection("dfinity");
    assert!(!processed_scripts.is_empty());
    err.fatal_report("Integration Test");

    for (script_path, script_text) in processed_scripts {
        let instrumented_module = run_whamm(
            &fs::read(APP_WASM_PATH).unwrap(),
            &script_text,
            &format!("{:?}", script_path.clone().as_path()),
        );
        err.fatal_report("Integration Test");

        // wasm2wat verification check
        if let Err(e) = wasm2wat(&instrumented_module) {
            panic!("`wasm2wat` verification check failed with error: {e}");
        }
    }
}

fn run_wat2wasm(original_wat_path: &str, original_wasm_path: &str) {
    // if you want to change the wat file
    // (calling wat2wasm from a child process doesn't work
    //  since somehow the executable can't write to the file system directly)
    let file_data = fs::read(original_wat_path).unwrap();
    let wasm_data = Wat2Wasm::new()
        .write_debug_names(true)
        .convert(file_data)
        .unwrap();
    fs::write(original_wasm_path, wasm_data).unwrap();
}

fn run_wasm2wat(instrumented_wasm_path: &str) {
    let file_data = fs::read(instrumented_wasm_path).unwrap();
    let wat_data = wasm2wat(file_data).unwrap();
    println!("{}", wat_data);
}

fn run_whamm_bin(original_wasm_path: &str, monitor_path: &str, instrumented_wasm_path: &str) {
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

fn run_basic_instrumentation(
    original_wat_path: &str,
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
) {
    run_wat2wasm(original_wat_path, original_wasm_path);
    run_whamm_bin(original_wasm_path, monitor_path, instrumented_wasm_path);
}

#[test]
fn instrument_handwritten_wasm_call() {
    common::setup_logger();
    let original_wat_path = "tests/apps/handwritten/add.wat";
    let original_wasm_path = "tests/apps/handwritten/add.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/integration-handwritten_add.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
    run_wasm2wat(instrumented_wasm_path);
}

#[test]
fn instrument_no_matches() {
    common::setup_logger();
    let original_wat_path = "tests/apps/handwritten/no_matched_events.wat";
    let original_wasm_path = "tests/apps/handwritten/no_matched_events.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/integration-no_matched_events.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
    run_wasm2wat(instrumented_wasm_path);
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

fn test_with_wasmtime(
    original_wat_path: &str,
    original_wasm_path: &str,
    monitor_path: &str,
    instrumented_wasm_path: &str,
) {
    // executable is located at target/debug/whamm
    let wasmtime = "wasmtime";

    run_wat2wasm(original_wat_path, original_wasm_path);

    // running on its own SHOULD fail
    let res = Command::new(wasmtime)
        .arg(original_wasm_path)
        .output()
        .expect("failed to execute process");
    assert!(!res.status.success());

    run_whamm_bin(original_wasm_path, monitor_path, instrumented_wasm_path);

    // running should now be successful!
    let res = Command::new(wasmtime)
        .arg(instrumented_wasm_path)
        .output()
        .expect("failed to execute process");
    assert!(res.status.success());
}

#[test]
fn instrument_spin_with_fault_injection() {
    common::setup_logger();
    let processed_scripts = common::setup_fault_injection("spin");
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_wizard_monitors() {
    common::setup_logger();
    let processed_scripts = common::setup_wizard_monitors();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_replay() {
    common::setup_logger();
    let processed_scripts = common::setup_replay();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}
