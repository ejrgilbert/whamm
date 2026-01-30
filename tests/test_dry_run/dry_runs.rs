use crate::util::{CORE_WASM_PATH, print_side_effects, setup_logger};
use whamm::api::instrument::{
    WhammError, instrument_as_dry_run_rewriting, instrument_as_dry_run_wei,
};

// TODO add tests for:
//  - user global data
//  - user probes
//  - user probes that overlap
//  - user libraries
//  - added locals

#[test]
fn dry_run() {
    setup_logger();
    let wasm_path = "tests/apps/core_suite/rust/cf.wasm";
    let script_path =
        "tests/scripts/core_suite/branch-monitor_rewriting/branch-br__br_if__br_table.mm";
    let side_effects = instrument_as_dry_run_rewriting(
        wasm_path.to_string(),
        script_path.to_string(),
        vec![],
        Some(CORE_WASM_PATH.to_string()),
        Some("./".to_string()),
    )
    .expect("Failed to run dry-run for bytecode rewriting");

    print_side_effects(&side_effects);
}

#[test]
fn dry_run_wei() {
    setup_logger();
    let script_path =
        "tests/scripts/core_suite/branch-monitor_rewriting/branch-br__br_if__br_table.mm";
    let side_effects = instrument_as_dry_run_wei(
        script_path.to_string(),
        vec![],
        Some(CORE_WASM_PATH.to_string()),
        Some("./".to_string()),
    )
    .expect("Failed to run dry-run for wei");

    print_side_effects(&side_effects);
}

#[test]
fn dry_run_errs() {
    setup_logger();
    let wasm_path = "tests/apps/core_suite/rust/cf.wasm";
    let script_path = "tests/scripts/error/bad.mm";
    let errs = instrument_as_dry_run_rewriting(
        wasm_path.to_string(),
        script_path.to_string(),
        vec![],
        Some(CORE_WASM_PATH.to_string()),
        Some("./".to_string()),
    )
    .expect_err("Should have failed to execute dry-run");

    print_side_effect_errs(&errs);
}

fn print_side_effect_errs(errs: &[WhammError]) {
    println!("================");
    println!("==== ERRORS ====");
    println!("================");

    for err in errs.iter() {
        println!("{:#?}", err);
    }
}
