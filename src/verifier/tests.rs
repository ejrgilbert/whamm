use crate::parser::tests;
use crate::verifier::verifier;
use std::collections::HashMap;

use crate::common::error::ErrorGen;
use log::{debug, error, info};
use wirm::Module;
// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const TOGGLE_PATH: &str = "./tests/libs/module/toggle/toggle.wasm";
const VALID_SCRIPTS: &[&str] = &[
    "wasm:opcode:call:alt { new_target_fn_name = redirect_to_fault_injector; }",
    r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    alt_call_by_name("instr_redirect_to_fault_injector");
}
    "#,
    r#"
        var a: bool;
        var b: i32;
        fn nested_fn(a: i32) -> i32 {
            return a;
        }
        fn dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }
    "#,
    r#"
        var i: i32;
        wasm:opcode:call:before /
            target_fn_name == "add"
        /{
            i = 1;
        }
    "#,
    r#"
        var a: bool;
        var b: i32;
        fn nested_fn(a: i32) -> i32 {
            return a;
        }
        fn dummy_fn() {
            b = nested_fn();
        }
        wasm::call:alt {
            dummy_fn();
        }
    "#,
    r#"
        var a: bool = strcmp((1, 2), "bookings");
        wasm::call:alt {
            a = strcmp((1, 2), "bookings");
        }
    "#,
    r#"
        fn my_fn(a: i32) -> i32 {
            return a;
        }
        var a: i32 = 5;
        wasm::call:alt {
            var b: i32 = my_fn(a);
        }
    "#,
    r#"
        var count: map<i32, i32>;
        fn my_fn() -> i32 {
            count[0] = 1;
            return count[0];
        }
        wasm::call:alt {
            count[1] = count[3];
            var a: i32 = my_fn();
        }
    "#,
    r#"
        report var a: i32;
        wasm::br:before {
            a = 1;
            report var b: bool;
        }
    "#,
    // numerics
    "wasm:opcode:call:alt { var num: i32 = 0; }",
    "wasm:opcode:call:alt { var num: i64 = 0; }",
    r#"
        var count: i32;
        wasm::i64.const:before / imm0 == 9223372036854775807 / {
            count++;
        }
    "#,
];

const TYPE_ERROR_SCRIPTS: &[&str] = &[
    // binary operations
    "wasm:opcode:call:alt {
        var i: i32 = 1 << (1, 2, 3);
    }",
    "wasm:opcode:call:alt {
        var i: i32 = 1 >> \"blah\";
    }",
    "wasm:opcode:call:alt {
        var i: i32 = 1 ^ (1, 2, 3);
    }",
    "wasm:opcode:call:alt {
        var i: i32 = 1 & (1, 2, 3);
    }",
    "wasm:opcode:call:alt {
        var i: i32 = 1 | (1, 2, 3);
    }",
    "wasm:opcode:call:alt {
        var v: f32 = 1e1;
        var i: f32 = v << 1;
    }",
    "wasm:opcode:call:alt {
        var v: f32 = 1e1;
        var i: f32 = v >> 1;
    }",
    "wasm:opcode:call:alt {
        var v: f32 = 1e1;
        var i: f32 = v & 1;
    }",
    "wasm:opcode:call:alt {
        var v: f32 = 1e1;
        var i: f32 = v | 1;
    }",
    "wasm:opcode:call:alt {
        var v: f32 = ~ 1e1;
    }",
    "wasm:opcode:call:alt {
        var v: f64 = 1e1;
        var i: f64 = v << 1;
    }",
    "wasm:opcode:call:alt {
        var v: f64 = 1e1;
        var i: f64 = v >> 1;
    }",
    "wasm:opcode:call:alt {
        var v: f64 = 1e1;
        var i: f64 = v & 1;
    }",
    "wasm:opcode:call:alt {
        var v: f64 = 1e1;
        var i: f64 = v | 1;
    }",
    "wasm:opcode:call:alt {
        var v: f64 = ~ 1e1;
    }",
    "wasm:opcode:call:alt / (1 + 3) / { var i: i32; }",
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
    // Compiler bound global
    r#"
wasm::call:alt {
    target_fn_type = 1;
}
    "#,
    // global declaration
    r#"
var x: i32;
wasm::call:alt {
    x = "str";
}
    "#,
    // tuple
    r#"
var x: (i32, i32);
wasm::call:alt {
    x = (1, 2, 3);
}
    "#,
    // local declaration
    r#"
wasm::call:alt {
    var x: i32;
    x = "str";
}
    "#,
    // Ternary (TODO: We do not emit code for ternary yet)
    r#"
var i: i32;
wasm::call:alt {
    i = 1 ? 2 : 3;
}
    "#,
    r#"
var i: bool;
var a: i32;
wasm:opcode:br:before {
    a = i ? 1 : true;
}
    "#,
    // calls (comp bound function)
    r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
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
var u: i32;
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
        var a: bool;
        var b: i32;
        fn strcmp(){
            a = false;
        }
        fn nested_fn(a: i32) -> i32 {
            return a;
        }
        fn dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }
    "#,
    r#"
        var a: bool;
        var b: i32;
        fn nested_fn(a: i32) -> i32 {
            return a;
        }
        fn nested_fn(a: i32) -> i32 {
            return a;
        }
        fn dummy_fn() {
            b = nested_fn(5);
            a = strcmp((b, 8), "bookings");
        }
        wasm::call:alt {
            dummy_fn();
        }
    "#,
    r#"
        var a: i32;
        fn nested_fn() -> bool {
            return "hi";
            return 1;
        }
        fn dummy_fn() {
            a = nested_fn();
        }
        wasm::call:alt {
            dummy_fn();
        }
    "#,
    r#"
    fn my_fn(a: i32) -> i32 {
        if(a > 5){
            return 1;
        }
        else{
            return true;
        }
        a = 5;
    }
    wasm::call:alt{
        var a: bool = true;
        var b: i32 = 5;
        if(a){
            b = 6;
        }
        else{
            b = 7;
        }
        if(b){
        }
        if(b == 5){
        }
    }
    "#,
    r#"
        fn strcmp () {}
        wasm::call:alt {
            strcmp();
        }
    "#,
    r#"
        var a: bool = true;
        if(a) {
            var b: i32 = 5;
        }
        wasm::call:alt {
        }
    "#,
    r#"
        fn my_func() -> bool {
            return true;
        }
        var a: bool = my_func();
        wasm::call:alt {
        }
    "#,
    r#"
        fn my_fn(a: i32) -> i32 {
            return a;
        }
        wasm::call:alt {
            var a: i32 = 5;
            var a: i32;
            var b: i32 = my_fn(a);
        }
    "#,
    r#"
        fn my_fn(a: i32) -> i32 {
            return a;
        }
        wasm::call:alt {
            var a: i32 = 5;
            var a: i32;
            var b: i32 = my_fn(a);
        }
    "#,
    r#"
        fn my_fn(a: i32) -> i32 {
            var a: bool;
            return a;
        }
        var my_fn: i32;
        wasm::call:alt {
            var b: i32 = my_fn(a);
            var my_fn: i32;
            var strcmp: i32;
        }
    "#,
    r#"
        var count: map<i32, i32>;
        fn my_fn() -> i32 {
            count[0] = false;
            return count[0];
        }
        wasm::call:alt {
            count[1] = count[3];
            var a: i32 = my_fn();
            count[2] = a == count[1];
        }
    "#,
    r#"
    var count: map<map<i32, i32>, map<i32, i32>>;

        wasm::call:alt {

        }
    "#,
    r#"
        wasm::call:alt {
            var a: (i32, map<i32, i32>);
        }
    "#,
    r#"
        wasm::call:alt {
            var a: (i32, map<i32, i32>);
            var b: map<i32, i32>;
            if((1, b) == a){
            }
        }
    "#,
    r#"
        report var a: i32;
        fn my_fn() {
            report var c: i32;
        }
        wasm::br:before {
            a = 1;
            report var b: bool;
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
        let table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
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
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    let mut ast = tests::get_ast(script, &mut err);
    let table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    debug!("{:#?}", table);

    // 7 scopes: whamm, strcmp, drop_args, script0, wasm, alt_call_by_name, alt_call_by_id, opcode, call, alt, probe itself
    let num_scopes = 11;
    // records: num_scopes PLUS (at_func_end, str_addr, func_id, func_name, value, probe_id, fid, fname, opidx, pc, opname, bytecode, localN, target_imp_name, target_fn_name, target_fn_type, target_imp_module, imm0, arg[0:9]+, category_name, category_id)
    let num_recs = num_scopes + 20;
    // asserts on very high level table structure
    assert_eq!(num_scopes, table.scopes.len());

    println!("{:#?}", table.records);

    debug!("==================\n{:#?}", table.records);
    assert_eq!(num_recs, table.records.len());
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    let mut ast = tests::get_ast(script, err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, err);
    verifier::type_check(&mut ast, &mut table, err).0
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
        var a: bool;
        wasm::call:alt {
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
}
// #[test]
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
        fn my_fn(a: i32) -> i32 {
            var a: bool;
            return a;
        }
        var my_fn: i32;
        var a: i32;
        var wasm: i32;
        wasm::call:alt {
            var b: i32 = my_fn(a);
            var my_fn: i32;
            var strcmp: i32;
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(err.has_errors);
}
#[test]
pub fn test_recursive_calls() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        fn make5(a: i32) -> i32 {
            if(a<5){
                return make5(a+1);
            }
            return a;
        }
        wasm::call:alt {
            var a: u32 = 0;
            var b: i32 = make5(a as i32);
        }
    "#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
}
#[test]
pub fn testing_map() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
    wasm:opcode:call:after {
        var my_map: map<(i32, i32, i32), i32>;
        var b: (i32, i32, i32) = (1, 2, 3);
        my_map[b] = 2;
        var c: i32 = my_map[b];
    }
    "#;

    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
}
#[test]
pub fn test_report_decl() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        var a: i32;
        wasm::br:before {
            a = 1;
            report var b: bool;
        }"#;
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
}

#[test]
pub fn test_dynamic_call_in_global_scope() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);

    // cannot have dynamic user library calls in global scope
    let script = r#"
use toggle;

report var g: i32 = toggle.get_nonzero_nested(1, @static toggle.get_value());

wasm:opcode:*:before / toggle.should_inject(fid as i32, @static toggle.get_value()) as bool / {
    report var val: i32;
    val = toggle.should_inject(fid as i32, @static toggle.get_value());
}
    "#;
    let mut ast = tests::get_ast(script, &mut err);

    let toggle_bytes = std::fs::read(TOGGLE_PATH).unwrap_or_else(|_| {
        panic!(
            "Could not read the core wasm module expected to be at location: {}",
            TOGGLE_PATH
        )
    });
    let toggle_lib: HashMap<String, (Option<String>, &[u8])> =
        HashMap::from([("toggle".to_string(), (None, toggle_bytes.as_slice()))]);

    let mut table = verifier::build_symbol_table(&mut ast, &toggle_lib, false, &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(err.has_errors);
}
//TODO: uncomment after BEGIN is working

//WE DONT HAVE BEGIN WORKING YET
// #[test]
// pub fn test_whamm_module() {
//     setup_logger();
//     let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
//
//     let script = r#"
//         BEGIN {
//             var a: i32;
//         }
//     "#;
//     info!("Typechecking: {}", script);
//     let res = is_valid_script(script, &mut err);
//
//     err.report();
//     assert!(!err.has_errors);
//     assert!(res);
// }
