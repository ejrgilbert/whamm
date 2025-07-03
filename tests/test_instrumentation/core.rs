use crate::test_instrumentation::helper::{
    build_whamm_core_lib, run_basic_instrumentation, run_core_suite, run_script, run_whamm_bin,
    setup_fault_injection, setup_numerics_monitors, setup_replay, setup_tests,
    setup_wizard_monitors, wat2wasm_on_dir, DEFAULT_CORE_LIB_PATH, DEFAULT_DEFS_PATH,
};
use crate::util::setup_logger;
use log::error;
use std::fs;
use std::process::Command;
use whamm::api::utils::wasm2wat_on_file;

const APP_WASM_PATH: &str = "tests/apps/core_suite/handwritten/basic.wasm";

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// scripts without errors occurring.
#[test]
fn instrument_dfinity_with_fault_injection() {
    setup_logger();
    let processed_scripts = setup_fault_injection("dfinity");
    assert!(!processed_scripts.is_empty());
    wat2wasm_on_dir("tests/apps/core_suite/handwritten");

    let wasm_path = "tests/apps/dfinity/users.wasm";

    for (script_path, ..) in processed_scripts {
        run_script(
            &script_path,
            wasm_path,
            fs::read(wasm_path).unwrap(),
            vec![],
            None,
            false,
            true,
        );
    }
}

#[test]
fn instrument_handwritten_wasm_call() {
    setup_logger();
    let original_wat_path = "tests/apps/core_suite/handwritten/add.wat";
    let original_wasm_path = "tests/apps/core_suite/handwritten/add.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/tests/integration-handwritten_add.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
}

#[test]
fn instrument_no_matches() {
    setup_logger();
    let original_wat_path = "tests/apps/core_suite/handwritten/no_matched_events.wat";
    let original_wasm_path = "tests/apps/core_suite/handwritten/no_matched_events.wasm";
    let monitor_path = "tests/scripts/instr.mm";
    let instrumented_wasm_path = "output/tests/integration-no_matched_events.wasm";

    run_basic_instrumentation(
        original_wat_path,
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
    );
}

#[test]
fn instrument_control_flow() {
    setup_logger();
    // Add the target
    let res = Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg("wasm32-wasip1")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'instrument_control_flow' add target failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }

    // Build the control_flow Rust project
    let res = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasip1")
        .current_dir("wasm_playground/control_flow")
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'instrument_control_flow' build project failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
    assert!(res.status.success());

    let monitor_path = "tests/scripts/instr.mm";
    let original_wasm_path = "wasm_playground/control_flow/target/wasm32-wasip1/debug/cf.wasm";
    let instrumented_wasm_path = "output/tests/integration-control_flow.wasm";

    run_whamm_bin(
        original_wasm_path,
        monitor_path,
        instrumented_wasm_path,
        DEFAULT_DEFS_PATH,
        DEFAULT_CORE_LIB_PATH,
    );
    wasm2wat_on_file(instrumented_wasm_path);
}

#[test]
fn instrument_spin_with_fault_injection() {
    setup_logger();
    let processed_scripts = setup_fault_injection("spin");
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_wizard_monitors() {
    setup_logger();
    let processed_scripts = setup_wizard_monitors();
    assert!(!processed_scripts.is_empty());

    build_whamm_core_lib();
    wat2wasm_on_dir("tests/apps/core_suite/handwritten");
    for (script_path, ..) in processed_scripts {
        run_script(
            &script_path,
            APP_WASM_PATH,
            fs::read(APP_WASM_PATH).unwrap(),
            vec![],
            None,
            false,
            true,
        );
    }
}

#[test]
fn instrument_with_replay() {
    setup_logger();
    let processed_scripts = setup_replay();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_numerics_scripts() {
    setup_logger();
    let processed_scripts = setup_numerics_monitors();
    assert!(!processed_scripts.is_empty());

    run_core_suite("numerics", processed_scripts, true, true, true)
}

#[test]
fn instrument_with_branch_monitor_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/branch-monitor");
    assert!(!processed_scripts.is_empty());

    // TODO -- fix wizard side (THEN merge with below test)
    //   - pull `fname`, `targets`, `num_targets`, `default_target`
    run_core_suite("branch-monitor", processed_scripts, true, true, true)
}
#[test]
fn instrument_with_branch_monitor_rewriting_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/branch-monitor_rewriting");
    assert!(!processed_scripts.is_empty());

    run_core_suite(
        "branch-monitor_rewriting",
        processed_scripts,
        true,
        false,
        true,
    )
}
#[test]
fn instrument_with_local_n_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/localN");
    assert!(!processed_scripts.is_empty());

    run_core_suite("localN", processed_scripts, true, true, true)
}

#[test]
fn instrument_with_calls_monitor_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/calls-monitor");
    assert!(!processed_scripts.is_empty());

    // TODO -- fix wizard side (THEN merge with below test)
    //   - pull `fname`
    run_core_suite("calls-monitor", processed_scripts, true, true, true)
}
#[test]
fn instrument_with_calls_monitor_rewriting_scripts() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/calls-monitor_rewriting");
    assert!(!processed_scripts.is_empty());

    run_core_suite(
        "calls-monitor_rewriting",
        processed_scripts,
        true,
        false,
        true,
    )
}

#[test]
fn components() {
    setup_logger();
    let processed_scripts = setup_tests("core_suite/components");
    assert!(!processed_scripts.is_empty());

    run_core_suite("core-components", processed_scripts, true, false, false)
}
