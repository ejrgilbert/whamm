use crate::util::{CORE_WASM_PATH, setup_logger};
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
    let script_path = "tests/scripts/core_suite/branch-monitor_rewriting/branch-br__br_if__br_table.mm";
    let side_effects = instrument_as_dry_run(
        CORE_WASM_PATH,
        "./",
        wasm_path.to_string(),
        script_path.to_string(),
        vec![],
    )
    .expect("Failed to run dry-run");


    let mut sorted_side_effects: Vec<_> = side_effects.iter().collect();
    sorted_side_effects.sort_by_key(|&(key, _)| key);
    for (ty, injections) in sorted_side_effects.iter() {
        println!("====={}=====", "=".repeat(ty.to_string().len()));
        println!("==== {} ====", ty.to_string().to_uppercase());
        println!("====={}=====", "=".repeat(ty.to_string().len()));

        for inj in injections.iter() {
            println!("{:#?}", inj);
        }
        println!();
    }
}