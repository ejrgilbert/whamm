use crate::parser::tests;
use crate::verifier::verifier;
use std::collections::HashMap;

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
    "
    report var s: str = read_str(0, 0, 10);
    wasm:opcode:drop:before {
        var a: i32;
    }",
    // use global state in report variable DeclInit
    "
    var ptr: i32 = 0;
    wasm:opcode:drop:before {
        // `ptr` and `l` are scope-compatible!!
        report var l: i32 = ptr;
    }",
    "
    var ptr: i32 = 0;
    wasm:opcode:drop:before {
        // `ptr` and `l` are scope-compatible!!
        unshared var l: i32 = ptr;
    }",
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
    // use dynamic state in report variable DeclInit
    "wasm:opcode:drop:before {
        report var s: str = read_str(0, 0, 10);
    }",
    "wasm:opcode:drop:before {
        report var s: str;
        s = read_str(0, 0, 10);

        // `s` and `l` are in a different scope!!
        report var l: bool = s.len();
    }",
    "wasm:opcode:drop:before {
        report var s: str;
        s = read_str(0, 0, 10);

        // `s` and `l` are in a different scope!!
        report var l: bool = s.starts_with(\"wxyz\");
    }",
    "
    wasm:opcode:drop:before {
        var s: str = read_str(0, 0, 10);

        // `s` and `l` are in a different scope!!
        report var l: bool = s.ends_with(\"wxyz\");
    }",
    // use probe-local state in report variable DeclInit
    "
    wasm:opcode:drop:before {
        var ptr: i32 = 0;

        // `ptr` and `l` are in a different scope!!
        report var l: i32 = ptr;
    }",
    "
    wasm:opcode:drop:before {
        var ptr: i32 = 0;

        // `ptr` and `l` are in a different scope!!
        unshared var l: i32 = ptr;
    }",
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
        let table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
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
    let table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
    debug!("{:#?}", table);

    // 15 scopes: whamm, strcmp, strcontains, drop_args, mem, memcpy, page_size, mem_size, active_data_start, active_data_len, write_str, read_str, script0, wasm, alt_call_by_name, alt_call_by_id, opcode, call, alt, probe itself
    let num_scopes = 20;
    // records: num_scopes PLUS (memid, memid, APP_MEMID, str_addr, needle, src_mem, src_ptr, dst_me, dst_ptr, len, at_func_end, str_addr, s, mem, addr, len, starts_with, resN, ends_with, contains, target_mem, ptr, s, src_mem, ptr, l, func_id, func_name, value, probe_id, fid, fname, opidx, pc, opname, bytecode, localN, target_imp_name, target_fn_name, target_fn_type, target_imp_module, imm0, arg[0:9]+, category_name, category_id)
    let num_recs = num_scopes + 46;
    // asserts on very high level table structure
    assert_eq!(num_scopes, table.scopes.len());

    println!("{:#?}", table.records);

    debug!("==================\n{:#?}", table.records);
    assert_eq!(num_recs, table.records.len());
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    let mut ast = tests::get_ast(script, err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), err);
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

// ===============================
// = Polymorphism (generic probes) =
// ===============================

fn typecheck_generic(script: &str) -> (bool, crate::parser::types::Whamm) {
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let mut ast = tests::get_ast(script, &mut err);
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
    let passed = verifier::type_check(&mut ast, &mut table, &mut err).0 && !err.has_errors;
    err.report();
    (passed, ast)
}

fn first_probe_type_params(
    ast: &crate::parser::types::Whamm,
) -> Vec<(String, crate::parser::generic_constraint::GenericConstraint)> {
    let script = ast.scripts.first().unwrap();
    let provider = script.providers.get("wasm").unwrap();
    let (_, package) = provider.packages.iter().next().unwrap();
    let (_, event) = package.events.iter().next().unwrap();
    let probe = event.probes.values().next().unwrap().first().unwrap();
    probe.type_params.clone()
}

#[test]
pub fn test_generic_numeric_valid() {
    setup_logger();
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before {
            report var acc: T;
            acc = arg0 + arg1;
        }",
    );
    assert!(ok, "generic numeric probe with `+` should type-check");
}

#[test]
pub fn test_generic_int_shift_valid() {
    setup_logger();
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: int>(arg0: T, arg1: T):before {
            report var acc: T;
            acc = arg0 << arg1;
        }",
    );
    assert!(ok, "generic int probe with `<<` should type-check");
}

#[test]
pub fn test_generic_shift_needs_int_is_error() {
    setup_logger();
    // Declared `numeric`, but `<<` requires `int` -> contract error.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before {
            acc = arg0 << arg1;
        }",
    );
    assert!(!ok, "`<<` on a `numeric`-bounded var should be rejected");
}

#[test]
pub fn test_generic_bound_inferred_from_usage() {
    setup_logger();
    // Bare `<U>` (Top) used with `+` should have its bound inferred to `numeric`.
    let (ok, ast) = typecheck_generic(
        "wasm:opcode:call<U>(arg0: U, arg1: U):before {
            report var acc: U;
            acc = arg0 + arg1;
        }",
    );
    assert!(ok, "unbounded var used numerically should type-check");
    let tvs = first_probe_type_params(&ast);
    assert_eq!(
        vec![(
            "U".to_string(),
            crate::parser::generic_constraint::GenericConstraint::Numeric
        )],
        tvs,
        "U's effective bound should be inferred to numeric"
    );
}

#[test]
pub fn test_generic_distinct_vars_error() {
    setup_logger();
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric, U: numeric>(arg0: T, arg1: U):before {
            report var acc: T;
            acc = arg0 + arg1;
        }",
    );
    assert!(!ok, "combining distinct type parameters should be rejected");
}

#[test]
pub fn test_generic_var_with_concrete_error() {
    setup_logger();
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
            report var acc: T;
            acc = arg0 + arg1;
        }",
    );
    assert!(
        !ok,
        "combining a type parameter with a concrete type should be rejected"
    );
}

#[test]
pub fn test_generic_cast_var_with_concrete_valid() {
    setup_logger();
    // An explicit cast bridges a concrete operand and a type parameter: `arg1 as T`.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
            report var acc: T;
            acc = arg0 + (arg1 as T);
        }",
    );
    assert!(ok, "`arg1 as T` should bridge a concrete operand and `T`");
}

#[test]
pub fn test_generic_cast_widen_valid() {
    setup_logger();
    // Casting a concrete operand into a type parameter bound from another operand.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: i32, arg1: T):before {
            report var acc: T;
            acc = (arg0 as T) + arg1;
        }",
    );
    assert!(ok, "`arg0 as T` should bridge a concrete operand and `T`");
}

#[test]
pub fn test_generic_cast_unknown_param_error() {
    setup_logger();
    // Casting to an undeclared type parameter is a type error.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
            report var acc: T;
            acc = arg0 + (arg1 as Z);
        }",
    );
    assert!(!ok, "casting to an undeclared type parameter should error");
}

#[test]
pub fn test_generic_float_valid() {
    setup_logger();
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: float>(arg0: T, arg1: T):before {
            report var acc: T;
            acc = arg0 + arg1;
        }",
    );
    assert!(ok, "generic float probe with `+` should type-check");
}

#[test]
pub fn test_generic_float_shift_is_error() {
    setup_logger();
    // `<<` requires int; a `float`-bounded var can never satisfy it (empty meet).
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: float>(arg0: T, arg1: T):before {
            acc = arg0 << arg1;
        }",
    );
    assert!(!ok, "`<<` on a `float`-bounded var should be rejected");
}

#[test]
pub fn test_generic_multiple_type_params() {
    setup_logger();
    // Two independent type parameters, each used only with same-var operands.
    let (ok, ast) = typecheck_generic(
        "wasm:opcode:call<T: numeric, U: int>(arg0: T, arg1: T, arg2: U, arg3: U):before {
            report var a: T;
            report var b: U;
            a = arg0 + arg1;
            b = arg2 << arg3;
        }",
    );
    assert!(
        ok,
        "probe with two independent type parameters should type-check"
    );
    let tvs = first_probe_type_params(&ast);
    assert_eq!(
        vec![
            (
                "T".to_string(),
                crate::parser::generic_constraint::GenericConstraint::Numeric
            ),
            (
                "U".to_string(),
                crate::parser::generic_constraint::GenericConstraint::Int
            ),
        ],
        tvs
    );
}

#[test]
pub fn test_generic_type_param_in_predicate() {
    setup_logger();
    // A type parameter used in a comparison in the predicate yields boolean; probe is valid.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before / arg0 > arg1 / {
            report var acc: T;
            acc = arg0 + arg1;
        }",
    );
    assert!(
        ok,
        "type parameter used in a predicate comparison should type-check"
    );
}

#[test]
pub fn test_generic_int_literal_in_comparison_ok() {
    setup_logger();
    // An integer literal compared against a type parameter is fine: it's resolved to the site's
    // concrete type during per-site monomorphization.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T):before / arg0 > 0 / {
            report var acc: T;
            acc = arg0;
        }",
    );
    assert!(
        ok,
        "integer literal in a comparison against a type parameter should type-check"
    );
}

#[test]
pub fn test_generic_float_literal_in_comparison_error() {
    setup_logger();
    // A float literal can't be monomorphized to an arbitrary numeric type per site.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: numeric>(arg0: T):before / arg0 > 1.5 / {
            report var acc: T;
            acc = arg0;
        }",
    );
    assert!(
        !ok,
        "float literal combined with a `numeric` type parameter should be rejected"
    );
}

#[test]
pub fn test_generic_float_literal_with_float_param_ok() {
    setup_logger();
    // A float literal is fine when the parameter can only be a float type: every
    // instantiation casts the literal to its concrete float type.
    let (ok, _) = typecheck_generic(
        "wasm:opcode:call<T: float>(arg0: T):before / arg0 > 1.5 / {
            report var acc: T;
            acc = arg0;
        }",
    );
    assert!(
        ok,
        "float literal against a `float`-bounded type parameter should type-check"
    );
}

#[test]
pub fn test_generic_bound_inferred_to_int() {
    setup_logger();
    // Bare `<U>` used with `<<` should infer the bound down to `int`.
    let (ok, ast) = typecheck_generic(
        "wasm:opcode:call<U>(arg0: U, arg1: U):before {
            report var acc: U;
            acc = arg0 << arg1;
        }",
    );
    assert!(ok, "unbounded var used with `<<` should type-check");
    let tvs = first_probe_type_params(&ast);
    assert_eq!(
        vec![(
            "U".to_string(),
            crate::parser::generic_constraint::GenericConstraint::Int
        )],
        tvs,
        "U's effective bound should be inferred to int"
    );
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
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
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
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
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
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
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
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
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
    let mut table = verifier::build_symbol_table(&mut ast, &HashMap::default(), &mut err);
    verifier::type_check(&mut ast, &mut table, &mut err);
    err.report();
    assert!(!err.has_errors);
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
