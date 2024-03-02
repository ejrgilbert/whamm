use crate::parser::tests;
use crate::verifier::verifier;
use crate::verifier::types;

use log::{debug, error};
use crate::verifier::types::SymbolTable;
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
    "dfinity:ic0:call_new:alt { redirect_to_fault_injector; }",
];

// =============
// = The Tests =
// =============

// TODO -- debug broken test
#[test]
pub fn test_build_table() {
    for script in VALID_SCRIPTS {
        match tests::get_ast(script) {
            Some(ast) => {
                let table = verifier::build_symbol_table(ast);
                // debug!("{:?}", table);
                println!();
            },
            None => {
                error!("Could not get ast from script: {script}");
                assert!(false);
            }
        };
    }
}