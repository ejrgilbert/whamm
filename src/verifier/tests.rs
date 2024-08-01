use crate::parser::tests;
use crate::verifier::verifier;

use crate::common::error::ErrorGen;
use log::{debug, error, info};

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const VALID_SCRIPTS: &[&str] = &[
    "wasm:opcode:call:alt { new_target_fn_name = redirect_to_fault_injector; }",
    r#"
        bool a;
        i32 b;
        nested_fn(i32 a) -> i32 {
            return a;
        }
        dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }   
    "#,
    r#"
        i32 i;
        wasm:opcode:call:before /
            target_imp_name == "add"
        /{
            i = 1;
        }
    "#,
    r#"
        bool a;
        i32 b;
        nested_fn(i32 a) -> i32 {
            return a;
        }
        dummy_fn() {
            b = nested_fn();
        }
        wasm::call:alt {
            dummy_fn();
        }   
    "#,
    r#"
        bool a = strcmp((1, 2), "bookings");
        wasm::call:alt {
            a = strcmp((1, 2), "bookings");
        }
    "#,
    r#"
        my_fn(i32 a) -> i32 {
            return a;
        }
        i32 a = 5;
        wasm::call:alt {
            i32 b = my_fn(a);
        }
    "#,
];

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
    r#"wasm:opcode:br:before / "i" <= 1 / { }"#,
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
    // Ternary (TODO: We do not emit code for ternary yet)
    r#"
i32 i;
wasm::call:alt {
    i = 1 ? 2 : 3;
}
    "#,
    r#"
bool i;
i32 a;
wasm:opcode:br:before {
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
    u = argasdf;
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
    r#"
        bool a;
        i32 b;
        strcmp(){
            a = false;
        }
        nested_fn(i32 a) -> i32 {
            return a;
        }
        dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }   
    "#,
    r#"
        bool a;
        i32 b;
        nested_fn(i32 a) -> i32 {
            return a;
        }
        nested_fn(i32 a) -> i32 {
            return a;
        }
        dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }   
    "#,
    r#"
        i32 a;
        nested_fn() -> bool {
            return "hi";
            return 1;
        }
        dummy_fn() {
            a = nested_fn();
        }
        wasm::call:alt {
            dummy_fn();
        }   
    "#,
    r#"
    my_fn(i32 a) -> i32 {
        if(a > 5){
            return 1;
        }
        else{
            return true;
        };
        a = 5;
    }
    wasm::call:alt{
        bool a = true;
        i32 b = 5;
        if(a){
            b = 6;
        }
        else{
            b = 7;
        };
        if(b){
        };
        if(b == 5){
        };
    }
    "#,
    r#"
        strcmp () {}
        wasm::call:alt {
            strcmp();
        }
    "#,
    r#"
        bool a = true;
        if(a) {
            i32 b = 5;
        };
        wasm::call:alt {
        }
    "#,
    r#"
        my_func() -> bool {
            return true;
        }
        bool a = my_func();
        wasm::call:alt {
        }
    "#,
    r#"
        my_fn(i32 a) -> i32 {
            return a;
        }
        wasm::call:alt {
            i32 a = 5;
            i32 a;
            i32 b = my_fn(a);
        }
    "#,
    r#"
        my_fn(i32 a) -> i32 {
            return a;
        }
        wasm::call:alt {
            i32 a = 5;
            i32 a;
            i32 b = my_fn(a);
        }
    "#,
    r#"
        my_fn(i32 a) -> i32 {
            bool a;
            return a;
        }
        i32 my_fn;
        wasm::call:alt {
            i32 b = my_fn(a);
            i32 my_fn;
            i32 strcmp;
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
        let mut ast = tests::get_ast(script, &mut err);
        let table = verifier::build_symbol_table(&mut ast, &mut err);
        debug!("{:#?}", table);
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

    let mut ast = tests::get_ast(script, &mut err);
    let table = verifier::build_symbol_table(&mut ast, &mut err);
    debug!("{:#?}", table);

    // 11 scopes: whamm, strcmp, script0, wasm, alt_call_by_name, alt_call_by_id, opcode, call, alt, puti, puts
    let num_scopes = 11;
    // records: num_scopes PLUS (str_addr, func_id, func_name, value, target_imp_name, target_fn_type, target_imp_module, imm0, arg[0:9]+, wasm_bytecode_loc)
    let _num_recs = num_scopes + 10;

    // asserts on very high level table structure
    assert_eq!(num_scopes, table.scopes.len());

    // debug!("==================\n{:#?}", table.records);
    // TODO: I'm not sure where the extra 2 came from
    // assert_eq!(num_recs, table.records.len());
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    let mut ast = tests::get_ast(script, err);
    let mut table = verifier::build_symbol_table(&mut ast, err);
    verifier::type_check(&mut ast, &mut table, err)
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
#[test]
pub fn test_template() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        bool a;
        wasm::call:alt {
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &mut err);
    let res = verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
    assert!(res);
}
#[test]
pub fn test_expect_fatal() {
    let result = std::panic::catch_unwind(|| {
        expect_fatal_error();
    });
    match result {
        Ok(_) => {
            panic!("Expected a fatal error, but got Ok");
        }
        Err(_) => {
            //this means the function properly exited with a fatal error
        }
    }
}
pub fn expect_fatal_error() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        my_fn(i32 a) -> i32 {
            bool a;
            return a;
        }
        i32 my_fn;
        i32 a;
        i32 wasm;
        wasm::call:alt {
            i32 b = my_fn(a);
            i32 my_fn;
            i32 strcmp;
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &mut err);
    let res = verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(err.has_errors);
    assert!(!res);
}
#[test]
pub fn test_recursive_calls() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        make5(i32 a) -> i32 {
            if(a<5){
                return make5(a+1);
            };
            return a;
        }
        wasm::call:alt {
            i32 a = 0;
            i32 b = make5(a);
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &mut err);
    let res = verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
    assert!(res);
}
//TODO: uncomment after BEGIN is working

//WE DONT HAVE BEGIN WORKING YET
// #[test]
// pub fn test_whamm_module() {
//     setup_logger();
//     let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

//     let script = r#"
//         BEGIN {
//             i32 a;
//         }
//     "#;
//     info!("Typechecking: {}", script);
//     let res = is_valid_script(script, &mut err);

//     err.report();
//     assert!(!err.has_errors);
//     assert!(res);
// }
