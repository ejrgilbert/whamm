use crate::parser::tests;
use crate::verifier::verifier;

use log::{error};
use crate::common::error::ErrorGen;

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
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    for script in VALID_SCRIPTS {
        match tests::get_ast(script, &mut err) {
            Some(ast) => {
                let table = verifier::build_symbol_table(&ast, &mut err);
                println!("{:#?}", table);
            },
            None => {
                error!("Could not get ast from script: {}", script);
                err.report();
                assert!(false);
            }
        };
    }
}
#[test]
pub fn test_build_table_with_asserts() {
    setup_logger();
    let script = r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    strpaircmp((arg0, arg1), "bookings") &&
    strpaircmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    match tests::get_ast(script, &mut err) {
        Some(ast) => {
            let table = verifier::build_symbol_table(&ast, &mut err);
            println!("{:#?}", table);

            // 7 scopes: whamm, strcmp, script, wasm, bytecode, call, alt
            let num_scopes = 7;
            // records: num_scopes PLUS (target_fn_type, target_imp_module, target_imp_name, new_target_fn_name,
            //          str_addr, value)
            let num_recs = num_scopes + 6;

            // asserts on very high level table structure
            assert_eq!(num_scopes, table.scopes.len());
            assert_eq!(num_recs, table.records.len());
        },
        None => {
            error!("Could not get ast from script: {}", script);
            assert!(false);
        }
    };
}

#[test]
pub fn test_type_checker() {
    setup_logger();
    let script = r#"
    wasm::call:alt /
    1 == "str" &&
    target_fn_type == "import"
/ {

}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);


    match tests::get_ast(script, &mut err) {
        Some(ast) => {
            verifier::verify(&ast);
        },
        None => {
            error!("Could not get ast from script: {}", script);
            assert!(false);
        }
    };
}