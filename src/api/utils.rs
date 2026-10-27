use crate::api::get_defs;
use crate::api::instrument::MAX_ERRORS;
use crate::common::error::ErrorGen;
use crate::common::instr;
use crate::parser;
use crate::wast::test_harness::setup_and_run_tests;
use log::debug;
use std::path::{Path, PathBuf};

/// Utility to print the info about a match rule to console.
///
/// * `rule`: The rule to print information for, follows the following pattern `provider:package:event:mode`
/// * `defs_path`: The path to follow to find the provider definitions
/// * `print_vars`: Whether to print the bound variables for the match rule
/// * `print_functions`: Whether to print the bound functions for the match rule
pub fn print_info(
    rule: String,
    defs_path: Option<PathBuf>,
    print_vars: bool,
    print_functions: bool,
) -> Result<(), Box<ErrorGen>> {
    // Parse the script and generate the information
    let mut err = ErrorGen::new("".to_string(), rule.clone(), MAX_ERRORS);
    let def_yamls = get_defs(defs_path.as_deref());
    parser::whamm_parser::print_info(rule, &def_yamls, print_vars, print_functions, &mut err)?;

    if err.has_errors {
        Err(Box::new(err))
    } else {
        Ok(())
    }
}

/// Write a Wasm module encoded as a vec of bytes to the specified location.
///
/// * `module`: The module to write to the file.
/// * `output_wasm_path`: Where to write the module to.
pub fn write_to_file(module: Vec<u8>, output_wasm_path: &Path) -> std::io::Result<()> {
    instr::write_to_file(module, output_wasm_path)
}

/// Run all wast files through the harness.
pub fn run_wast_harness() -> Result<(), std::io::Error> {
    crate::wast::test_harness::clean();

    // Find all the wast files to run as tests
    let wast_tests = crate::wast::test_harness::find_wast_tests();
    setup_and_run_tests(&wast_tests)?;

    Ok(())
}

/// Run all wast files through the harness found at a specific location.
pub fn run_wast_tests_at(wast_tests: &Vec<PathBuf>) {
    setup_and_run_tests(wast_tests).expect("WAST Test failed!");
    println!("The wast test passed!");
}

/// Validate the wasm module at the specified path.
pub fn validate_wasm_file(instrumented_wasm_path: &str) {
    use wirm::wasmparser::{Validator, WasmFeatures};
    debug!("Validating Wasm module at file: {instrumented_wasm_path}");
    let bytes = std::fs::read(instrumented_wasm_path)
        .unwrap_or_else(|e| panic!("failed to read {instrumented_wasm_path}: {e}"));

    if let Err(e) = Validator::new_with_features(WasmFeatures::all()).validate_all(&bytes) {
        panic!("Wasm validation failed on {instrumented_wasm_path}: {e}");
    }
}
