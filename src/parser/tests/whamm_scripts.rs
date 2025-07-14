use crate::parser::whamm_parser::*;

use crate::common::error::ErrorGen;

use crate::parser::provider_handler::ModeKind;
use crate::parser::tests::{get_ast, setup_logger};
use crate::parser::yml_processor::pull_all_yml_files;
use glob::{glob, glob_with};
use log::{error, info, warn};

const DEFS_PATH: &str = "./";
const VALID_SCRIPTS: &[&str] = &[
    r#"
wasm:func:entry {}
    "#,
    // with libraries
    r#"
use lib;

lib.call();
wasm:opcode:call(arg0: i32):before {
    lib.other_call(1, 2);
}
lib.blah();
wasm:opcode:call(arg0: i32):before {
    lib.other_call();
}
    "#,
    // type bounding
    r#"
wasm:opcode:call(arg0: i32):before {}
wasm:opcode:local.set(arg3: i64):before {}
wasm:opcode:call(local5: f32):before {}
wasm:opcode:call(local5: f32, arg0: u8):before {}
    "#,
    // casts
    r#"
var i: u8;

wasm:begin {
    var j: u32 = i as u32;
}
    "#,
    r#"
var i: u8;

wasm:begin {
    var b: bool = true;
    var j: u32 = b ? i as u32 : 1 as u32;
}
    "#,
    // all numeric types
    r#"
var i: u8;
var i: i8;
var i: u16;
var i: i16;
var i: u32;
var i: i32;
var i: u64;
var i: i64;
var i: f32;
var i: f64;
var i: bool;
var i: str;
var i: (i32, i32);
var i: map<i32, i32>;

wasm:begin { }
    "#,
    // Ternary
    r#"
wasm:opcode:br:before {
    index = i ? 1 : 0;
}
    "#,
    // Variations of PROBE_RULE
    "wasm:begin { }",
    "wasm:end { }",
    "wasm:opcode:call:alt { }",
    "wasm:opcode:call:before { }",
    "wasm:opcode:call:after { }",
    // Regexes
    "wasm:opc*:call:before { }",
    "wasm::call:after { }",
    ":::alt { }",
    "wasm::: { }",
    ":opcode:: { }",
    "::call: { }",
    ":::before { }",
    ":opcode:call:alt { }",
    "wasm::call:alt { }",
    "wasm:opcode::alt { }",
    // Predicates
    "wasm:opcode:br:before / i / { }",
    r#"wasm:opcode:br:before / "i" <= 1 / { }"#,
    "wasm:opcode:br:before / i54 < r77 / { }",
    "wasm:opcode:br:before / i54 < r77 / { }",
    "wasm:opcode:br:before / i != 7 / { }",
    r#"wasm:opcode:br:before / (i == "1") && (b == "2") / { }"#,
    r#"wasm:opcode:br:before / i == "1" && b == "2" / { }"#,
    "wasm:opcode:br:before / i == (1 + 3) / { count = 0; }",
    "wasm:opcode:br:before / !(a && b) / { count = 0; }",
    "wasm:opcode:br:before / !a / { count = 0; }",
    // Function calls
    r#"
wasm::call:alt / strcmp((arg2, arg3), "record") / {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#,
    "wasm::call:alt { fn_name(); }",
    "wasm::call:alt { fn_name(a); }",
    "wasm::call:alt { fn_name(a + a); }",
    r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#,
    r#"wasm:::alt / (i == "1") && (b == "2") / { i = 0; }"#,
    // globals
    r#"
var count: map<i32, i32>;
wasm:begin { }
    "#,
    r#"
var count: map<i32, i32>;
count = 0;
wasm:begin { }
    "#,
    //function stuff
    r#"
    fn fn_name(param: i32) -> i32{}
    wasm:begin { }
    "#,
    r#"
    fn fn_name(param0: i32, param1: i32) -> i32{}
    wasm:begin { }
    "#,
    r#"
    fn fn_name() -> i32{
        i = 0;
    }
    wasm:begin { }
        "#,
    r#"
    fn fn_name() -> i32{
        i = 0;
        i++;
    }
    wasm:begin { }
        "#,
    r#"
    wasm:opcode:br:before {
        var i: i32;
        return i;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        return;
    }
    "#,
    r#"
    fn add_vars(a: i32, b: i32) -> i32{
        a++;
        b--;
        return a + b;
    }
    wasm:opcode:br:before {
        var a: i32;
        var b: i32;
        var c: i32;
        c = add_vars(a, b);
    }
    "#,
    r#"
    fn do_nothing(a: i32, b: i32){

    }
    wasm:begin { }
    "#,
    r#"
    fn nested_fn() -> i32 {
        return 5;
    }
    fn outter_fn() -> i32 {
        return nested_fn() + 1;
    }
    wasm:begin {}
    "#,
    // Statements (either assignment or function call)
    r#"
    wasm:opcode:br:before {
        var return123: i32;
    }
    "#,
    r#"
wasm:opcode:br:before {
    i = -10;
}
    "#,
    r#"
    wasm:opcode:br:before {
        call_new();
    }
    "#,
    r#"
    wasm:opcode:br:before {
        i = 0;
        i ++;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        i = 0;
        i++;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        i = 0;
        i--;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        i = 0;
        i --;
    }
    "#,
    // report variables
    r#"
        var a: i32;
        report var c: i32;
        wasm::br:before {
            a = 1;
            report var b: bool;
        }
    "#,
    // TODO -- uncomment when we've supported special_decl_init
    // r#"
    //     var a: i32;
    //     report var c: i32;
    //     wasm::br:before {
    //         a = 1;
    //         report var b: bool = true;
    //     }
    // "#,
    // unshared variables
    r#"
        var a: i32;
        unshared var c: i32;
        wasm::br:before {
            a = 1;
            unshared var b: bool;
        }
    "#,
    // TODO -- uncomment when we've supported special_decl_init
    // r#"
    //     var a: i32;
    //     unshared var c: i32;
    //     wasm::br:before {
    //         a = 1;
    //         unshared var b: bool = true;
    //     }
    // "#,
    // special variables
    r#"
        unshared report var c: i32;
        report unshared var c: i32;
        wasm::br:before {}
    "#,
    // Comments
    r#"
/* comment */
wasm:opcode:br:before { }
    "#,
    "wasm:opcode:br:before { } // this is a comment",
    r#"
/* comment */
wasm:opcode:br:before { } // this is a comment
    "#,
    r#"
wasm:opcode:br:before {
    i = 0; // this is a comment
}
    "#,
    r#"
wasm:opcode:br:before {
    //has an empty comment
    i = 0; //
}
    "#,
    r#"
wasm:opcode:br:before {
    //has an empty block comment
    i = 0; /**/
}
    "#,
    // If/else stmts
    r#"
        wasm::call:alt{
            var a: bool = true;
            if(a){
                i = 0;
            } else {
                i = 1;
            }
            if(a){
                i = 0;
            } elif(b) {
                i = 1;
            }
        }

    "#,
    //maps
    r#"
        var count: map<i32, i32>;
        fn my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        wasm:begin {
            count[1] = my_fn();
        }
    "#,
    r#"
        var count: map<i32, i32>;
        wasm:begin {
            count[1] = 1+1;
        }
    "#,
    // valid "variants" of reserved keywords
    "wasm:opcode:call:alt { var arg: i32; }",
    "wasm:opcode:call:alt { arg = 1; }",
    "wasm:opcode:call:alt { arg0 = 1; }",
    //using tuples
    r#"
        var sample: (i32, i32) = (1, 2);
        fn dummy_fn() {
            a = strcmp(sample, "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        var i: i32;
        i = 5;
        var j: i32 = 5;
        wasm:begin {
            strcmp((arg0, arg1), "bookings");
        }
    "#,
    // numerics
    "wasm:opcode:call:alt { var num: u32 = 0; }",
    "wasm:opcode:call:alt { var num: i32 = 0; }",
    // trigger available modes per event
    "wasm:opcode:*:before {}",
    "wasm:opcode:br*:before {}",
    "wasm:opcode:unreachable:before {}",
    "wasm:opcode:unreachable:alt {}",
    "wasm:opcode:nop:before {}",
    "wasm:opcode:nop:after {}",
    "wasm:opcode:nop:alt {}",
    "wasm:opcode:block:before {}",
    "wasm:opcode:block:after {}",
    "wasm:opcode:block:alt {}",
    "wasm:opcode:block:entry {}",
    "wasm:opcode:block:exit {}",
    "wasm:opcode:loop:before {}",
    "wasm:opcode:loop:after {}",
    "wasm:opcode:loop:alt {}",
    "wasm:opcode:loop:entry {}",
    "wasm:opcode:loop:exit {}",
    "wasm:opcode:if:before {}",
    "wasm:opcode:if:after {}",
    "wasm:opcode:if:alt {}",
    "wasm:opcode:if:entry {}",
    "wasm:opcode:if:exit {}",
    "wasm:opcode:else:before {}",
    "wasm:opcode:else:after {}",
    "wasm:opcode:else:alt {}",
    "wasm:opcode:else:entry {}",
    "wasm:opcode:else:exit {}",
    "wasm:opcode:end:before {}",
    "wasm:opcode:end:after {}",
    "wasm:opcode:br:before {}",
    "wasm:opcode:br:after {}",
    "wasm:opcode:br:alt {}",
    "wasm:opcode:br:at_target {}",
    "wasm:opcode:call:before {}",
    "wasm:opcode:call:after {}",
    "wasm:opcode:call:alt {}",
];

const INVALID_SCRIPTS: &[&str] = &[
    // globals
    r#"
var count: map<i32, i32>;
    "#,
    // Variations of PROBE_RULE
    "wasm:opcode:call:alt: { }",
    "wasm:opcode:call:alt",
    "wasm:opcode:call:dne",
    // Empty predicate
    "wasm:opcode:call:alt  // { }",
    "wasm:opcode:call:alt / 5i < r77 / { }",
    r#"wasm:opcode:call:alt  / i == """" / { }"#,
    // bad statement
    "wasm:opcode:call:alt / i == 1 / { i; }",
    r#"
    wasm:opcode:br:before {
        var return: i32;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        var if: i32;
    }
    "#,
    // bad incrementor
    r#"
    wasm:opcode:br:before {
        i = 0;
        if(i++ == 0){
            i = 2;
        }
    }
        "#,
    // bad fn definitions
    r#"
    fn fn_name() -> i32{
    wasm:opcode:br:before {
    }
        "#,
    r#"
    fn fn_name(, arg0) -> i32{}
    wasm:opcode:br:before {
    }
        "#,
    // invalid if/else
    r#"
        wasm::call:alt{
            else {
                i = 0;
            }
        }
    "#,
    r#"
        wasm::call:alt{
            if(a){
                i = 0;
            } else {
                i = 1;
            }
            else {
                i = 0;
            }
        }
    "#,
    r#"
        wasm::call:alt{
            var a: bool = true;
            elif(a){};
        }
    // reserved keywords
    "wasm:opcode:call:alt { var arg0: i32; }",
    r#"
var arg0: map<i32, i32>;
    "#,
    r#"
        var count: map<i32>;
        fn my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        wasm:begin {
            count[1] = my_fn();
        }
    "#,
    r#"
        var count: map<i32, i32>;
        fn my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        wasm:begin {
            count[] = my_fn();
        }
    "#,
    // use report multiple times
    r#"
        var a: i32;
        report unshared report var c: i32;
        wasm::br:before {
            a = 1;
            report var b: bool;
        }
    "#,
    r#"
        var a: i32;
        report var c: i32;
        wasm::br:before {
            a = 1;
            report unshared report var b: bool;
        }
    "#,
    // use unshared multiple times
    r#"
        var a: i32;
        unshared report unshared var c: i32;
        wasm::br:before {
            a = 1;
            unshared b: bool;
        }
    "#,
    r#"
        var a: i32;
        unshared var ci32;
        wasm::br:before {
            a = 1;
            unshared report unshared var b: bool;
        }
    "#,
    // invalid probe rule
    r#"
core::br:before / i == 1 / { i = 0; }  // SHOULD FAIL HERE

    "#,
    // trigger unavailable modes per event
    "wasm:opcode:unreachable:after {}",
    "wasm:opcode:unreachable:at_target {}",
    "wasm:opcode:unreachable:entry {}",
    "wasm:opcode:unreachable:exit {}",
    "wasm:opcode:nop:at_target {}",
    "wasm:opcode:nop:semantic_after {}",
    "wasm:opcode:nop:entry {}",
    "wasm:opcode:nop:exit {}",
    "wasm:opcode:end:alt {}",
    "wasm:opcode:end:at_target {}",
    "wasm:opcode:end:entry {}",
    "wasm:opcode:end:exit {}",
    "wasm:opcode:br:entry {}",
    "wasm:opcode:br:exit {}",
    "wasm:opcode:call:at_target {}",
    "wasm:opcode:call:entry {}",
    "wasm:opcode:call:exit {}",
    "wasm:opcode:block:at_target {}",
    "wasm:opcode:loop:at_target {}",
    "wasm:opcode:if:at_target {}",
    "wasm:opcode:else:at_target {}",
];
const SPECIAL: &[&str] = &["wasm:begin { }", "wasm:end { }", "wasm:::alt { }"];

// ====================
// = Helper Functions =
// ====================

const TEST_RSC_DIR: &str = "tests/scripts/";
const PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

pub fn get_test_scripts(sub_dir: &str) -> Vec<String> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*PATTERN.to_owned()))
        .expect("Failed to read glob pattern")
    {
        let unparsed_file = std::fs::read_to_string(path.as_ref().unwrap())
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        scripts.push(unparsed_file);
    }

    for path in glob_with(
        &(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*TODO.to_owned()),
        options,
    )
    .expect("Failed to read glob pattern")
    {
        warn!(
            "File marked with TODO: {}",
            path.as_ref().unwrap().display()
        );
    }

    scripts
}

fn is_valid_script(script: &str, def_yamls: &Vec<String>, err: &mut ErrorGen) -> bool {
    parse_script(def_yamls, &script.to_string(), err).is_some() && !err.has_errors
}

pub fn run_test_on_valid_list(scripts: Vec<String>, def_yamls: &Vec<String>, err: &mut ErrorGen) {
    for script in scripts {
        println!("Parsing: {}", script);

        let res = is_valid_script(&script, def_yamls, err);
        if !res || err.has_errors {
            error!(
                "script = '{}' is not recognized as valid, but it should be",
                script
            )
        }
        if err.has_errors {
            err.report();
        }
        assert!(!&err.has_errors);
        assert!(res);
    }
}

// =============
// = The Tests =
// =============

#[test]
pub fn test_parse_valid_scripts() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(
        VALID_SCRIPTS.iter().map(|s| s.to_string()).collect(),
        &pull_all_yml_files(&DEFS_PATH),
        &mut err,
    );
}

#[test]
pub fn test_parse_invalid_scripts() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    for script in INVALID_SCRIPTS {
        info!("Parsing: {}", script);
        let res = is_valid_script(script, &pull_all_yml_files(&DEFS_PATH), &mut err);
        if res || !err.has_errors {
            error!(
                "string = '{}' is recognized as valid, but it should not",
                script
            )
        }
        assert!(err.has_errors);
        assert!(!&res);
    }
}
#[test]
pub fn test_whamm_with_asserts() {
    setup_logger();
    let script = r#"
fn my_func() -> i32 {
    return 5;
    return 5;
    return 5;
    return 5;
    return 5;
    return 5;
}
wasm::call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
    new_target_fn_name = "redirect_to_fault_injector";
    new_target_fn_name = "redirect_to_fault_injector";
    new_target_fn_name = "redirect_to_fault_injector";
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#;
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let ast = get_ast(script, &mut err);

    // script
    assert_eq!(1, ast.scripts.len()); // a single script
    assert_eq!(0, ast.bound_vars.len());
    assert_eq!(1, ast.fns.len()); // strcmp

    let script = ast.scripts.first().unwrap();
    assert_eq!(1, script.fns.len()); // my_func

    let my_func = script.fns.first().unwrap();
    assert_eq!(6, my_func.body.stmts.len());
    // provider
    assert_eq!(1, script.providers.len());
    let provider = script.providers.get("wasm").unwrap();
    assert_eq!("wasm", provider.def.name);
    assert_eq!(6, provider.def.bound_vars.len());
    assert_eq!(0, provider.def.bound_fns.len());

    assert_eq!(1, provider.packages.len());
    let (_, package) = provider.packages.iter().next().unwrap();
    assert_eq!("opcode", package.def.name);
    assert_eq!(0, package.def.bound_vars.len());
    assert_eq!(1, package.def.bound_fns.len());

    assert_eq!(1, package.events.len());
    let (_, event) = package.events.iter().next().unwrap();
    assert_eq!("call", event.def.name);
    assert_eq!(8, event.def.bound_vars.len());
    assert_eq!(2, event.def.bound_fns.len());

    assert_eq!(1, event.probes.len());
    assert_eq!(1, event.probes.get(&ModeKind::Alt).unwrap().len());

    let probe = event.probes.get(&ModeKind::Alt).unwrap().first().unwrap();
    assert_eq!(0, probe.def.bound_vars.len());
    assert_eq!(0, probe.def.bound_fns.len());
    assert_eq!(ModeKind::Alt, probe.kind);

    // probe predicate
    assert!(probe.predicate.is_some());

    // probe body
    assert!(&probe.body.is_some());
    assert_eq!(5, probe.body.as_ref().unwrap().stmts.len());
}

#[test]
pub fn test_ast_special_cases() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(
        SPECIAL.iter().map(|s| s.to_string()).collect(),
        &pull_all_yml_files(&DEFS_PATH),
        &mut err,
    );
}

#[test]
pub fn testing_strcmp() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        fn dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        wasm:begin{
            strcmp((arg0, arg1), "bookings");
        }

    "#;

    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}

#[test]
fn test_global_stmts() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        fn dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        wasm::call:alt{
            var a: (i32, i32) = (arg0, arg1);
            strcmp((arg0, arg1), "bookings");
        }
    "#;
    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}

#[test]
pub fn testing_block() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        fn dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        wasm:begin{
            if (0 == 1) {
                strcmp((arg0, arg1), "bookings");
            } else {
                dummy_fn();
            }
        }
    "#;

    let res = is_valid_script(script, &pull_all_yml_files(&DEFS_PATH), &mut err);
    err.report();
    assert!(res);
}
#[test]
pub fn testing_global_def() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        var sample: (i32, i32) = (1, 2);
        fn dummy_fn() {
            a = strcmp(sample, "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        var i: i32;
        i = 5;
        var j: i32 = 5;
        wasm:begin{
            strcmp((i, j), "bookings");
        }
    "#;

    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}
#[test]
pub fn testing_map() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        var count: map<i32, map<i32, i32>>;
        fn my_fn() -> i32 {
            var a: map<i32, i32>;
            count[0] = a;
            return a[0];
        }
        wasm::call:alt {
            var a: i32 = my_fn();
        }
    "#;

    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}
#[test]
pub fn testing_tuple_map() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        var count: map<(i32, i32, i32), i32>;
        wasm::br|br_if:before {
          // count stores an array of counters
          count[(fid, pc, index)]++;
        }
    "#;

    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}
#[test]
pub fn test_report_decl() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        var a: i32;
        report var c: i32;
        wasm::br:before {
            a = 1;
            report var b: bool;
        }
    "#;
    assert!(is_valid_script(
        script,
        &pull_all_yml_files(&DEFS_PATH),
        &mut err
    ));
}
// ===================
// = Full File Tests =
// ===================
#[test]
pub fn fault_injection() {
    setup_logger();
    let scripts = get_test_scripts("fault_injection");
    if scripts.is_empty() {
        warn!("No test scripts found for `fault_injection` test.");
    }
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(scripts, &pull_all_yml_files(&DEFS_PATH), &mut err);
}

#[test]
pub fn wizard_monitors() {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.is_empty() {
        warn!("No test scripts found for `wizard_monitors` test.");
    }
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(scripts, &pull_all_yml_files(&DEFS_PATH), &mut err);
}

#[test]
pub fn replay() {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.is_empty() {
        warn!("No test scripts found for `replay` test.");
    }
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(scripts, &pull_all_yml_files(&DEFS_PATH), &mut err);
}
