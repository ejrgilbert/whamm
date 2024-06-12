use crate::parser::tests;
use crate::verifier::verifier;

use crate::common::error::ErrorGen;
use log::{error, info};

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const VALID_SCRIPTS: &[&str] =
    &["wasm:bytecode:call:alt { new_target_fn_name = redirect_to_fault_injector; }"];

// =============
// = The Tests =
// =============

#[test]
pub fn test_build_table() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    for script in VALID_SCRIPTS {
        match tests::get_ast(script, &mut err) {
            Some(mut ast) => {
                let table = verifier::build_symbol_table(&mut ast, &mut err);
                println!("{:#?}", table);
            }
            None => {
                error!("Could not get ast from script: {}", script);
                err.report();
                panic!();
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
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    match tests::get_ast(script, &mut err) {
        Some(mut ast) => {
            let table = verifier::build_symbol_table(&mut ast, &mut err);
            println!("{:#?}", table);

            // 7 scopes: whamm, strcmp, script, wasm, bytecode, call, alt
            let num_scopes = 7;
            // records: num_scopes PLUS (target_fn_type, target_imp_module, target_imp_name, new_target_fn_name,
            //          tos, wasm_bytecode_loc, str_addr, value)
            let num_recs = num_scopes + 8;

            // asserts on very high level table structure
            assert_eq!(num_scopes, table.scopes.len());
            assert_eq!(num_recs, table.records.len());
        }
        None => {
            error!("Could not get ast from script: {}", script);
            panic!();
        }
    };
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    match tests::get_ast(script, err) {
        Some(ast) => verifier::type_check(
            &mut ast.clone(),
            &mut verifier::build_symbol_table(&mut ast.clone(), err),
            err,
        ),
        None => {
            error!("Should fail at type checking, not parsing: {}", script);
            assert!(false);
            false
        }
    }
}

// These tests are mostly making sure errors are reported at the right location
#[test]
pub fn test_type_error_in_predicate() {
    setup_logger();
    let script = r#"
    wasm::call:alt /
    1 == "str" && // this should be a type error
    target_fn_type == "import"
/ {

}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    // TODO: is_valid_script doesn't have side effect on the passed in `err`
    // since the err is changed at struct type_check

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }

    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_in_predicate2() {
    setup_logger();
    let script = r#"
    wasm:bytecode:br:before / "i" <= 1 / { }
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_stmt() {
    setup_logger();
    let script = r#"
    wasm::call:alt {
    target_fn_type = 1;
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_pred_final() {
    setup_logger();
    let script = r#"
    wasm:bytecode:call:alt / (1 + 3) / {  }
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_global_decl() {
    setup_logger();
    let script = r#"
    i32 x;
    wasm::call:alt {
    x = "str";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_tuple() {
    setup_logger();
    let script = r#"
    (i32, i32) x;
    wasm::call:alt {
    x = (1, 2, 3);
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_local_decl() {
    setup_logger();
    let script = r#"
    wasm::call:alt {
    i32 x;
    x = "str";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_in_predicate_unop() {
    setup_logger();
    let script = r#"
    wasm::call:alt / !1 / {
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}

#[test]
pub fn test_type_error_in_ternary() {
    setup_logger();
    let script = r#"
    i32 i;
    wasm::call:alt {
    i = 1 ? 2 : 3;
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    info!("Parsing: {}", script);
    let res = is_valid_script(script, &mut err);

    if res || !err.has_errors {
        error!(
            "string = '{}' is recognized as valid, but it should not",
            script
        )
    }
    err.report();
    assert!(err.has_errors);
    assert!(!&res);
}