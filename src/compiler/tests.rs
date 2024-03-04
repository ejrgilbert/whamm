use crate::compiler::dtrace_compiler::*;
use crate::parser::tests;

use log::error;
use std::path::PathBuf;

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const VALID_SCRIPTS: &'static [&'static str] = &[
    r#"
wasm::call:alt / strpaircmp((arg2, arg3), "record") / {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#
];

// =============
// = The Tests =
// =============

#[test]
pub fn test_emit_wasm() {
    // TODO:
    //   1. build wasm symbols
    //   2. support tuple
    //   3. add symbols for providers/modules/etc.
    for script in VALID_SCRIPTS {
        match tests::get_ast(script) {
            Some(ast) => {
                let wasm_path = PathBuf::from("tests/apps/users.wasm");
                let output_path = "./output.wasm".to_string();
                assert!(emit_wasm(&ast, &wasm_path, &output_path));
            },
            None => {
                error!("Could not get ast from script: {script}");
                assert!(false);
            }
        };
    }
}