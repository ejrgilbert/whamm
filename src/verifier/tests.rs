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
    "wasm:bytecode:call:alt { new_target_fn_name = redirect_to_fault_injector; }",
];

// =============
// = The Tests =
// =============

#[test]
pub fn test_build_table() {
    setup_logger();

    for script in VALID_SCRIPTS {
        match tests::get_ast(script) {
            Some(ast) => {
                let table = verifier::verify(&ast);
                println!("{:#?}", table);
            },
            None => {
                error!("Could not get ast from script: {script}");
                assert!(false);
            }
        };
    }
}