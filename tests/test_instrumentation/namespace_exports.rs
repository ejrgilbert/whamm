use crate::util::CORE_WASM_PATH;
use std::path::Path;
use whamm::api::instrument::{instrument_with_config, Config, Instrumented};
use wirm::Module;

const SCRIPT: &str = "var count: i32; wasm:opcode:nop:before { count++; }";
const APP: &str = r#"
    (module
        (func (export "_start") nop)
    )"#;

fn instrument(app_wat: &str, testing: bool) -> Instrumented {
    let app_bytes = wat::parse_str(app_wat).expect("assemble app wat");
    let core_lib =
        whamm::api::load_core_lib_from_path(Path::new(CORE_WASM_PATH)).expect("core lib");
    let defs = whamm::api::load_defs_from_path(Path::new("./"));
    let user_libs = whamm::api::parse_user_libs(vec![]).expect("user libs");

    instrument_with_config(
        app_bytes,
        SCRIPT.as_bytes().to_vec(),
        user_libs,
        Config {
            testing,
            ..Default::default()
        },
        Some(core_lib),
        Some(defs),
    )
    .expect("instrumentation failed")
}

fn export_names(instrumented: &Instrumented) -> Vec<String> {
    let module = Module::parse(&instrumented.module, true, true).expect("parse output");
    module.exports.iter().map(|e| e.name.clone()).collect()
}

#[test]
fn no_getter_pollution_by_default() {
    let names = export_names(&instrument(APP, false));
    assert!(
        !names.iter().any(|n| n.contains("get_count")),
        "default build should emit no `get_count` accessor, got {names:?}"
    );
}

#[test]
fn getter_namespaced_in_testing_mode() {
    let names = export_names(&instrument(APP, true));
    assert!(
        names.iter().any(|n| n == "__whamm_get_count"),
        "testing build should emit the namespaced accessor, got {names:?}"
    );
    assert!(
        !names.iter().any(|n| n == "get_count"),
        "testing build must not emit a bare `get_count`, got {names:?}"
    );
}
