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

const TYPE_ERROR_SCRIPTS: &[&str] = &[
    // predicate
    // note that this will have cascading type check errors
    // might want to make type check errors fatal so that we can stop early
    r#"
wasm::call:alt /
    1 == "str" && // this should be a type error
    target_fn_type == "import"
/ {

}
    "#,
    r#"wasm:bytecode:br:before / "i" <= 1 / { }"#,
    r#"wasm::call:alt / (1 + 3) / {  }"#, // final type in predicate
    r#"wasm::call:alt / !1 / { }"#,       // unop
    // stmt
    // Compiler provided global
    r#"
wasm::call:alt {
    target_fn_type = 1;
}
    "#,
    // global declaration
    r#"
i32 x;
wasm::call:alt {
    x = "str";
}
    "#,
    // tuple
    r#"
(i32, i32) x;
wasm::call:alt {
    x = (1, 2, 3);
}
    "#,
    // local declaration
    r#"
wasm::call:alt {
    i32 x;
    x = "str";
}
    "#,
    // Ternary (TODO: We do not emit code for tenary yet)
    r#"
i32 i;
wasm::call:alt {
    i = 1 ? 2 : 3;
}
    "#,
    r#"
bool i;
i32 a;
wasm:bytecode:br:before {
    a = i ? 1 : true;
}
    "#,
    // calls (comp provided function)
    r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_imp_name == "call_new" &&
    strcmp((arg0, arg1), 1) &&
    strcmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "instr_redirect_to_fault_injector";
}
    "#,
    r#"
wasm::call:alt /
    // I can't typecheck this because the entire Tuple is assume to be good
    strcmp((arg2, "32q"), "bookings")
/ {
    new_target_fn_name = "instr_redirect_to_fault_injector";
}
    "#,
    // only allow arg0-9 to be unknown type
    r#"
i32 u;
wasm::call:alt {
    u = argdadf;
}
    "#,
    // long type check error
    r#"
wasm::call:alt /
    (1 == "str") &&
    true &&
    true &&
    true
/ {

}
    "#,
    // long type check error, but recognizes both sides
    r#"
wasm::call:alt /
    (1 == "str") &&
    true &&
    true &&
    true &&
    strcmp((arg0, "arg1"), "bookings")
/ {

}
    "#,
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

            // 7 scopes: whamm, strcmp, script0, wasm, bytecode, call, alt
            let num_scopes = 7;
            // records: num_scopes PLUS (str_addr, value, wasm_bytecode_loc, new_target_fn_name, target_imp_name, arg[0:9]+, target_fn_type, target_imp_module)
            let num_recs = num_scopes + 8;

            // asserts on very high level table structure
            assert_eq!(num_scopes, table.scopes.len());

            println!("==================\n{:#?}", table.records);
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
        Some(mut ast) => {
            let mut table = verifier::build_symbol_table(&mut ast, err);
            verifier::type_check(&ast, &mut table, err)
        }
        None => {
            error!("Should fail at type checking, not parsing: {}", script);
            assert!(false);
            false
        }
    }
}

// These tests are mostly making sure errors are reported at the right location
#[test]
pub fn test_type_errors() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    for script in TYPE_ERROR_SCRIPTS {
        info!("Typechecking: {}", script);
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
}
