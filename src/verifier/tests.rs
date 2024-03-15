use crate::parser::tests;
use crate::verifier::verifier;

use log::{error};

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
    "wasm::call:alt { new_target_fn_name = redirect_to_fault_injector; }",
];

// =============
// = The Tests =
// =============

#[test]
pub fn test_build_table() {
    setup_logger();
    // TODO:
    //   1. add strcmp function
    //   2. support: target_fn_type, target_fn_module/name, new_target_fn_name
    //   3. add symbols for providers/modules/etc.
    for script in VALID_SCRIPTS {
        match tests::get_ast(script) {
            Some(ast) => {
                let (table, _core_probes, _wasm_probes) = verifier::verify(&ast);
                table.print();
            },
            None => {
                error!("Could not get ast from script: {script}");
                assert!(false);
            }
        };
    }
}