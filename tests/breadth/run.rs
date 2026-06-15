// End-to-end smoke tests for the breadth scripts.

use anyhow::Result;
use std::path::Path;

use crate::test_instrumentation::helper::{
    run_testcase_rewriting, run_testcase_wei, ExpectedOutput,
};

const BREADTH_MM: &str = "tests/breadth/breadth.mm";
const BREADTH_WEI_MM: &str = "tests/breadth/breadth_wei.mm";

const TARGET_APP: &str = "tests/breadth/breadth_target.wasm";

fn outdir(name: &str) -> String {
    let dir = format!("output/tests/breadth/{name}");
    std::fs::create_dir_all(&dir).expect("create breadth outdir");
    dir
}

#[test]
#[ignore]
fn breadth_rewriting_compiles_and_runs() -> Result<()> {
    let out = outdir("rewriting");
    let instr_app = format!("{out}/output.wasm");
    run_testcase_rewriting(
        Path::new(BREADTH_MM),
        TARGET_APP,
        vec![],
        ExpectedOutput::None,
        &out,
        &instr_app,
    )
}

// Currently expected to fail wherever the wei target rejects a bound item the rewriting
// target accepts. Run with:
//     cargo test --test main breadth::run::breadth_wei_compiles_and_runs -- --ignored
// to track parity progress.
#[test]
#[ignore]
fn breadth_wei_compiles_and_runs() -> Result<()> {
    let out = outdir("wei");
    let instr_app = format!("{out}/mon.wasm");
    run_testcase_wei(
        Path::new(BREADTH_MM),
        TARGET_APP,
        vec![],
        ExpectedOutput::None,
        &out,
        &instr_app,
    )
}

// Engine-only items (probe_id, …) — exercises the wei-only side of the schema.
#[test]
#[ignore]
fn breadth_wei_engine_only_compiles_and_runs() -> Result<()> {
    let out = outdir("wei_engine_only");
    let instr_app = format!("{out}/mon.wasm");
    run_testcase_wei(
        Path::new(BREADTH_WEI_MM),
        TARGET_APP,
        vec![],
        ExpectedOutput::None,
        &out,
        &instr_app,
    )
}
