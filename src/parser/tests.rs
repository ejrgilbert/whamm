use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::*;

use glob::{glob, glob_with};

use crate::common::error::ErrorGen;
use crate::parser::print_visitor::AsStrVisitor;
use crate::parser::rules::core::WhammModeKind;
use log::{debug, error, info, warn};

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const VALID_SCRIPTS: &[&str] = &[
    // Ternary
    r#"
wasm:opcode:br:before {
    index = i ? 1 : 0;
}
    "#,
    // Variations of PROBE_RULE
    "BEGIN { }",
    "END { }",
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
    r#"wasm:opcode:br:before / "i" <= 1 / { }"#, // TODO make invalid in type checking
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
map<i32, i32> count;
BEGIN { }
    "#,
    r#"
map<i32, i32> count;
count = 0;
BEGIN { }
    "#,
    //function stuff
    r#"
    fn_name(i32 param) -> i32{}
    BEGIN { }
    "#,
    r#"
    fn_name(i32 param0, i32 param1) -> i32{}
    BEGIN { }
    "#,
    r#"
    fn_name() -> i32{
        i = 0;
    }
    BEGIN { }
        "#,
    r#"
    fn_name() -> i32{
        i = 0;
        i++;
    }
    BEGIN { }
        "#,
    r#"
    wasm:opcode:br:before {
        i32 i;
        return i;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        return;
    }
    "#,
    r#"
    add_vars(i32 a, i32 b) -> i32{
        a++;
        b--;
        return a + b;
    }
    wasm:opcode:br:before {
        i32 a;
        i32 b;
        i32 c;
        c = add_vars(a, b);
    }
    "#,
    r#"
    do_nothing(i32 a, i32 b){

    }
    BEGIN { }
    "#,
    r#"
    nested_fn() -> i32 {
        return 5;
    }
    outter_fn() -> i32 {
        return nested_fn() + 1;
    }
    BEGIN {}
    "#,
    // Statements (either assignment or function call)
    r#"
    wasm:opcode:br:before {
        i32 return123;
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
        i32 a;
        report i32 c;
        wasm::br:before {
            a = 1;
            report bool b;
        }
    "#,
    // TODO -- uncomment when we've supported special_decl_init
    // r#"
    //     i32 a;
    //     report i32 c;
    //     wasm::br:before {
    //         a = 1;
    //         report bool b = true;
    //     }
    // "#,
    // unshared variables
    r#"
        i32 a;
        unshared i32 c;
        wasm::br:before {
            a = 1;
            unshared bool b;
        }
    "#,
    // TODO -- uncomment when we've supported special_decl_init
    // r#"
    //     i32 a;
    //     unshared i32 c;
    //     wasm::br:before {
    //         a = 1;
    //         unshared bool b = true;
    //     }
    // "#,
    // special variables
    r#"
        unshared report i32 c;
        report unshared i32 c;
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
            bool a = true;
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
        map<i32, i32> count;
        my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        BEGIN {
            count[1] = my_fn();
        }
    "#,
    r#"
        map<i32, i32> count;
        BEGIN {
            count[1] = 1+1;
        }
    "#,
    // valid "variants" of reserved keywords
    "wasm:opcode:call:alt { i32 arg; }",
    "wasm:opcode:call:alt { arg = 1; }",
    "wasm:opcode:call:alt { arg0 = 1; }",
    //using tuples
    r#"
        (i32, i32) sample = (1, 2);
        dummy_fn() {
            a = strcmp(sample, "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        i32 i;
        i = 5;
        i32 j = 5;
        BEGIN{
            strcmp((arg0, arg1), "bookings");
        }
    "#,
    // numerics
    "wasm:opcode:call:alt { u32 num = 0; }",
    "wasm:opcode:call:alt { i32 num = 0; }",
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
    "wasm:opcode:_loop:before {}",
    "wasm:opcode:_loop:after {}",
    "wasm:opcode:_loop:alt {}",
    "wasm:opcode:_loop:entry {}",
    "wasm:opcode:_loop:exit {}",
    "wasm:opcode:_if:before {}",
    "wasm:opcode:_if:after {}",
    "wasm:opcode:_if:alt {}",
    "wasm:opcode:_if:entry {}",
    "wasm:opcode:_if:exit {}",
    "wasm:opcode:_else:before {}",
    "wasm:opcode:_else:after {}",
    "wasm:opcode:_else:alt {}",
    "wasm:opcode:_else:entry {}",
    "wasm:opcode:_else:exit {}",
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

const FATAL_SCRIPTS: &[&str] = &[
    // invalid probe rule
    r#"
core::br:before / i == 1 / { i = 0; }  // SHOULD FAIL HERE

    "#,
    // Numerics (not supported yet: https://github.com/ejrgilbert/whamm/issues/141)
    "wasm:opcode:call:alt { i64 num = 0; }",
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
    "wasm:opcode:_loop:at_target {}",
    "wasm:opcode:_if:at_target {}",
    "wasm:opcode:_else:at_target {}",
];

const INVALID_SCRIPTS: &[&str] = &[
    // globals
    r#"
map<i32, i32> count;
    "#,
    // Variations of PROBE_RULE
    "wasm:opcode:call:alt: { }",
    "wasm:opcode:call:alt",
    "wasm:opcode:call:dne",
    // Empty predicate
    "wasm:opcode:call:alt  // { }",
    "wasm:opcode:call:alt / 5i < r77 / { }",
    //            "wasm:opcode:call:alt / i < 1 < 2 / { }", // TODO -- make invalid on semantic pass
    //            "wasm:opcode:call:alt / (1 + 3) / { i }", // TODO -- make invalid on type check
    r#"wasm:opcode:call:alt  / i == """" / { }"#,
    // bad statement
    "wasm:opcode:call:alt / i == 1 / { i; }",
    r#"
    wasm:opcode:br:before {
        i32 return;
    }
    "#,
    r#"
    wasm:opcode:br:before {
        i32 if;
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
    fn_name() -> i32{
    wasm:opcode:br:before {
    }
        "#,
    r#"
    fn_name(, arg0) -> i32{}
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
            bool a = true;
            elif(a){};
        }
    // reserved keywords
    "wasm:opcode:call:alt { i32 arg0; }",
    r#"
map<i32, i32> arg0;
    "#,
    r#"
        map<i32> count;
        my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        BEGIN {
            count[1] = my_fn();
        }
    "#,
    r#"
        map<i32, i32> count;
        my_fn() -> i32{
            count[0] = 0;
            return count[0];
        }
        BEGIN {
            count[] = my_fn();
        }
    "#,
    // use report multiple times
    r#"
        i32 a;
        report unshared report i32 c;
        wasm::br:before {
            a = 1;
            report bool b;
        }
    "#,
    r#"
        i32 a;
        report i32 c;
        wasm::br:before {
            a = 1;
            report unshared report bool b;
        }
    "#,
    // use unshared multiple times
    r#"
        i32 a;
        unshared report unshared i32 c;
        wasm::br:before {
            a = 1;
            unshared bool b;
        }
    "#,
    r#"
        i32 a;
        unshared i32 c;
        wasm::br:before {
            a = 1;
            unshared report unshared bool b;
        }
    "#,
];
const SPECIAL: &[&str] = &["BEGIN { }", "END { }", "wasm:::alt { }"];

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

pub fn get_ast(script: &str, err: &mut ErrorGen) -> Whamm {
    info!("Getting the AST");
    match parse_script(&script.to_string(), err) {
        Some(ast) => {
            // print_ast(&ast);
            ast
        }
        None => {
            error!("Could not get ast from script: {}", script);
            if err.has_errors {
                err.report();
            }
            assert!(!err.has_errors);
            panic!();
        }
    }
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    parse_script(&script.to_string(), err).is_some() && !err.has_errors
}

pub fn run_test_on_valid_list(scripts: Vec<String>, err: &mut ErrorGen) {
    for script in scripts {
        info!("Parsing: {}", script);

        let res = is_valid_script(&script, err);
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
        &mut err,
    );
}

#[test]
pub fn test_parse_fatal_scripts() {
    setup_logger();
    for script in FATAL_SCRIPTS {
        println!("Parsing: {}", script);
        let result = std::panic::catch_unwind(|| {
            let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
            is_valid_script(script, &mut err)
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
}

#[test]
pub fn test_parse_invalid_scripts() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    for script in INVALID_SCRIPTS {
        info!("Parsing: {}", script);
        let res = is_valid_script(script, &mut err);
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
my_func() -> i32 {
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
    assert_eq!(0, ast.globals.len());
    assert_eq!(1, ast.fns.len()); // strcmp

    let script = ast.scripts.first().unwrap();
    assert_eq!(1, script.fns.len()); // my_func

    let my_func = script.fns.first().unwrap();
    assert_eq!(6, my_func.body.stmts.len());
    // provider
    assert_eq!(1, script.providers.len());
    let provider = script.providers.get("wasm").unwrap();
    assert_eq!("wasm", provider.name());
    assert_eq!(3, provider.get_provided_globals().len());
    assert_eq!(0, provider.get_provided_fns().len());

    assert_eq!(1, provider.len_packages());
    let package = provider.packages().next().unwrap();
    assert_eq!("opcode", package.name());
    assert_eq!(0, package.get_provided_globals().len());
    assert_eq!(1, package.get_provided_fns().len());

    assert_eq!(1, package.len_events());
    let event = package.events().next().unwrap();
    assert_eq!("call", event.name());
    assert_eq!(5, event.get_provided_globals().len());
    assert_eq!(2, event.get_provided_fns().len());

    assert_eq!(1, event.probes().len());
    assert_eq!(1, event.probes().get("alt").unwrap().len());

    let probe = event.probes().get("alt").unwrap().first().unwrap();
    assert_eq!(0, probe.get_mode_provided_globals().len());
    assert_eq!(0, probe.get_mode_provided_fns().len());
    assert_eq!(WhammModeKind::Alt, probe.mode());

    // probe predicate
    assert!(probe.predicate().is_some());

    // probe body
    assert!(&probe.body().is_some());
    assert_eq!(5, probe.body().as_ref().unwrap().stmts.len());
}

#[test]
pub fn test_ast_special_cases() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(SPECIAL.iter().map(|s| s.to_string()).collect(), &mut err);
}

#[allow(unused)]
pub(crate) fn print_ast(ast: &Whamm) {
    let mut visitor = AsStrVisitor { indent: 0 };
    debug!("{}", visitor.visit_whamm(ast));
}

#[test]
pub fn testing_strcmp() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        BEGIN{
            strcmp((arg0, arg1), "bookings");
        }
    
    "#;

    assert!(is_valid_script(script, &mut err));
}

#[test]
fn test_global_stmts() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        wasm::call:alt{
            (i32, i32) a = (arg0, arg1);
            strcmp((arg0, arg1), "bookings");
        }
    "#;
    assert!(is_valid_script(script, &mut err));
}

#[test]
pub fn testing_block() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        dummy_fn() {
            a = strcmp((arg0, arg1), "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        BEGIN{
            if (0 == 1) {
                strcmp((arg0, arg1), "bookings");
            } else {
                dummy_fn();
            }
        }
    "#;

    let res = is_valid_script(script, &mut err);
    err.report();
    assert!(res);
}
#[test]
pub fn testing_global_def() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        (i32, i32) sample = (1, 2);
        dummy_fn() {
            a = strcmp(sample, "bookings");
            strcmp((arg0, arg1), "bookings");
        }
        i32 i;
        i = 5; 
        i32 j = 5;
        BEGIN{
            strcmp((i, j), "bookings");
        }
    "#;

    assert!(is_valid_script(script, &mut err));
}
#[test]
pub fn testing_map() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        map<i32, map<i32, i32>> count;
        my_fn() -> i32 {
            map<i32, i32> a;
            count[0] = a;
            return a[0];
        }
        wasm::call:alt {
            i32 a = my_fn();
        }
    "#;

    assert!(is_valid_script(script, &mut err));
}
#[test]
pub fn testing_tuple_map() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        map<(i32, i32, i32), i32> count;
        wasm::br|br_if:before {
          // count stores an array of counters
          count[(fid, pc, index)]++;
        }
    "#;

    assert!(is_valid_script(script, &mut err));
}
#[test]
pub fn test_report_decl() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let script = r#"
        i32 a;
        report i32 c;
        wasm::br:before {
            a = 1;
            report bool b;
        }
    "#;
    assert!(is_valid_script(script, &mut err));
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
    run_test_on_valid_list(scripts, &mut err);
}

#[test]
pub fn wizard_monitors() {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.is_empty() {
        warn!("No test scripts found for `wizard_monitors` test.");
    }
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(scripts, &mut err);
}

#[test]
pub fn replay() {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.is_empty() {
        warn!("No test scripts found for `replay` test.");
    }
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(scripts, &mut err);
}
