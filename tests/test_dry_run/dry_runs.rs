use crate::util::{print_side_effects, setup_logger, CORE_WASM_PATH};
use std::path::Path;
use whamm::api::instrument::{
    instrument_as_dry_run_rewriting, instrument_as_dry_run_wei, WhammError,
};
use whamm::api::{load_core_lib_from_path, load_defs_from_path};

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
        std::fs::read(wasm_path).unwrap(),
        std::fs::read(script_path).unwrap(),
        Default::default(),
        Some(load_core_lib_from_path(Path::new(CORE_WASM_PATH)).unwrap()),
        Some(load_defs_from_path(Path::new("./"))),
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
        std::fs::read(script_path).unwrap(),
        Default::default(),
        Some(load_core_lib_from_path(Path::new(CORE_WASM_PATH)).unwrap()),
        Some(load_defs_from_path(Path::new("./"))),
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
        std::fs::read(wasm_path).unwrap(),
        std::fs::read(script_path).unwrap(),
        Default::default(),
        Some(load_core_lib_from_path(Path::new(CORE_WASM_PATH)).unwrap()),
        Some(load_defs_from_path(Path::new("./"))),
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
