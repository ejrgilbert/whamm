mod common;

use crate::common::{run_basic_instrumentation, run_whamm_bin};
use orca_wasm::Module;
use std::fs;
use std::process::Command;
use whamm::common::error::ErrorGen;
use whamm::wast::test_harness::wasm2wat_on_file;

const APP_WASM_PATH: &str = "tests/apps/dfinity/users.wasm";

#[test]
fn run_wast_tests() {
    common::setup_logger();
    whamm::wast::test_harness::run_all().expect("WAST Tests failed!");
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
        let wasm = fs::read(APP_WASM_PATH).unwrap();
        let mut module_to_instrument = Module::parse(&wasm, false).unwrap();
        let _ = whamm::common::instr::run(
            &mut module_to_instrument,
            &script_text,
            &format!("{:?}", script_path.clone().as_path()),
            None,
            0,
            // false,
        );
        err.fatal_report("Integration Test");
    }
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
}

#[test]
fn instrument_control_flow() {
    common::setup_logger();
    // Build the control_flow Rust project
    let a = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    assert!(a.status.success());

    let monitor_path = "tests/scripts/instr.mm";
    let original_wasm_path =
        "wasm_playground/control_flow/target/wasm32-unknown-unknown/debug/cf.wasm";
    let instrumented_wasm_path = "output/integration-control_flow.wasm";

    run_whamm_bin(original_wasm_path, monitor_path, instrumented_wasm_path);
    wasm2wat_on_file(instrumented_wasm_path);
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
