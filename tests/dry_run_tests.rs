mod common;

use crate::common::CORE_WASM_PATH;
use whamm::api::instrument::instrument_as_dry_run;

// TODO add tests for:
//  - user global data
//  - user probes
//  - user probes that overlap
//  - added locals

#[test]
fn dry_run() {
    let wasm_path = "tests/apps/core_suite/handwritten/branches.wasm";
    let script_path = "tests/scripts/core_suite/branch-monitor/branch-on_hw-br__br_if.mm";
    let side_effects = instrument_as_dry_run(
        CORE_WASM_PATH,
        "./",
        wasm_path.to_string(),
        script_path.to_string(),
        vec![],
    )
    .expect("Failed to run dry-run");

    println!("{:#?}", side_effects);
}
