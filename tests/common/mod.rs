use std::fs;
use std::path::PathBuf;
use std::process::Command;

use glob::{glob, glob_with};
use log::{error, warn};
use whamm::wast::test_harness::wasm2wat_on_file;

// ====================
// = Helper Functions =
// ====================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const TEST_RSC_DIR: &str = "tests/scripts/";
const WAT_PATTERN: &str = "*.wat";
const DOT_WAT: &str = ".wat";
const DOT_WASM: &str = ".wasm";
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
    if !res.status.success() {
        error!(
            "'run_whamm_bin' add target failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
    }
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

pub fn wat2wasm_on_dir(dir: &str) {
    let mut wat_files = vec![];
    for path in glob(&(dir.to_owned() + "/" + &*WAT_PATTERN.to_owned()))
        .expect("Failed to read glob pattern")
    {
        let file_name = path.as_ref().unwrap();
        wat_files.push(file_name.clone());
    }

    for file in wat_files.iter() {
        let filename = file.to_str().unwrap();
        if let Some(stripped_name) = filename.strip_suffix(DOT_WAT) {
            wat2wasm_on_file(filename, &(stripped_name.to_owned() + DOT_WASM))
        }
    }
}

pub fn wat2wasm_on_file(original_wat_path: &str, original_wasm_path: &str) {
    let res = Command::new("wasm-tools")
        .arg("parse")
        .arg(original_wat_path)
        .arg("-o")
        .arg(original_wasm_path)
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        error!(
            "'wasm-tools parse' failed:\n{}\n{}",
            String::from_utf8(res.stdout).unwrap(),
            String::from_utf8(res.stderr).unwrap()
        );
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

pub fn setup_numerics_monitors() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("core_suite/numerics");
    if scripts.is_empty() {
        warn!("No test scripts found for `numerics` test.");
    }

    scripts
}

pub fn setup_branch_monitors() -> Vec<(PathBuf, String)> {
    setup_logger();
    let scripts = get_test_scripts("core_suite/branch-monitor");
    if scripts.is_empty() {
        warn!("No test scripts found for `report_vars` test.");
    }

    scripts
}
