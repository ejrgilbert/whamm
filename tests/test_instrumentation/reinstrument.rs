use crate::util::CORE_WASM_PATH;
use std::path::Path;
use whamm::api::instrument::instrument_module_with_rewriting;
use wirm::Module;

const APP_WAT: &str = "(module (func $g) (func (export \"_start\") (call $g)))";
const SCRIPT: &str = "wasm:opcode:call:before { report var c: i32; c = c + 1; }";

/// Instrument a whamm-instrumented module (do it 2x), shouldn't get mad about it.
#[test]
fn reinstrument_whamm_output_does_not_panic() {
    let app_bytes = wat::parse_str(APP_WAT).expect("assemble app wat");
    let core_lib =
        whamm::api::load_core_lib_from_path(Path::new(CORE_WASM_PATH)).expect("core lib");
    let defs = whamm::api::load_defs_from_path(Path::new("./"));
    let user_libs = whamm::api::parse_user_libs(vec![]).expect("user libs");

    // Pass 1: instrument the raw app.
    let mut module = Module::parse(&app_bytes, true, true).expect("parse app");
    let pass1 = instrument_module_with_rewriting(
        &mut module,
        SCRIPT.as_bytes().to_vec(),
        user_libs.clone(),
        Some(core_lib.clone()),
        Some(defs.clone()),
    )
    .expect("pass 1 instrumentation failed");

    // Pass 2: instrument whamm's own output.
    let mut module2 = Module::parse(&pass1.module, true, true).expect("parse pass-1 output");
    let pass2 = instrument_module_with_rewriting(
        &mut module2,
        SCRIPT.as_bytes().to_vec(),
        user_libs,
        Some(core_lib),
        Some(defs),
    );
    assert!(
        pass2.is_ok(),
        "re-instrumenting a whamm-produced module should succeed and validate"
    );
}
