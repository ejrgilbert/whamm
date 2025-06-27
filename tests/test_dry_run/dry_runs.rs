use crate::util::{print_side_effects, setup_logger, CORE_WASM_PATH};
use whamm::api::instrument::instrument_as_dry_run;

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
    let side_effects = instrument_as_dry_run(
        wasm_path.to_string(),
        script_path.to_string(),
        vec![],
        Some(CORE_WASM_PATH.to_string()),
        Some("./".to_string()),
    )
    .expect("Failed to run dry-run");

    print_side_effects(&side_effects);
}
