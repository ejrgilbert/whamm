use crate::parser::types::{Whamm, WhammVisitor};
use crate::parser::whamm_parser::*;

use glob::{glob, glob_with};

use crate::common::error::ErrorGen;
use crate::parser::print_visitor::AsStrVisitor;
use log::{error, info, warn};

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const VALID_SCRIPTS: &[&str] = &[
    // Ternary
    r#"
wasm:bytecode:br:before {
    index = i ? 1 : 0;
}
    "#,
    // Variations of PROBE_SPEC
    "BEGIN { }",
    "END { }",
    "wasm:bytecode:call:alt { }",
    "wasm:bytecode:call:before { }",
    "wasm:bytecode:call:after { }",
    // Regexes
    "wasm:byt*:call:before { }",
    "wasm::call:after { }",
    ":::alt { }",
    "wasm::: { }",
    ":bytecode:: { }",
    "::call: { }",
    ":::before { }",
    ":bytecode:call:alt { }",
    "wasm::call:alt { }",
    "wasm:bytecode::alt { }",
    // Predicates
    "wasm:bytecode:br:before / i / { }",
    r#"wasm:bytecode:br:before / "i" <= 1 / { }"#, // TODO make invalid in type checking
    "wasm:bytecode:br:before / i54 < r77 / { }",
    "wasm:bytecode:br:before / i54 < r77 / { }",
    "wasm:bytecode:br:before / i != 7 / { }",
    r#"wasm:bytecode:br:before / (i == "1") && (b == "2") / { }"#,
    r#"wasm:bytecode:br:before / i == "1" && b == "2" / { }"#,
    "wasm:bytecode:br:before / i == (1 + 3) / { count = 0; }",
    "wasm:bytecode:br:before / !(a && b) / { count = 0; }",
    "wasm:bytecode:br:before / !a / { count = 0; }",
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
    target_imp_name == "call_new" &&
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
    wasm:bytecode:br:before {
        i32 i;
        return i;
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        return;
    }
    "#,
    r#"
    add_vars(i32 a, i32 b) -> i32{
        a++;
        b--;
        return a + b;
    }
    wasm:bytecode:br:before {
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
    wasm:bytecode:br:before {
        i32 return123;
    }
    "#,
    r#"
wasm:bytecode:br:before {
    i = 0;
}
    "#,
    r#"
    wasm:bytecode:br:before {
        call_new();
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        i = 0;
        i ++;
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        i = 0;
        i++;
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        i = 0;
        i--;
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        i = 0;
        i --;
    }
    "#,
    // Comments
    r#"
/* comment */
wasm:bytecode:br:before { }
    "#,
    "wasm:bytecode:br:before { } // this is a comment",
    r#"
/* comment */
wasm:bytecode:br:before { } // this is a comment
    "#,
    r#"
wasm:bytecode:br:before {
    i = 0; // this is a comment
}
    "#,
    r#"
wasm:bytecode:br:before {
    //has an empty comment
    i = 0; //
}
    "#,
    r#"
wasm:bytecode:br:before {
    //has an empty block comment
    i = 0; /**/
}
    "#,
];

const INVALID_SCRIPTS: &[&str] = &[
    // globals
    r#"
map<i32, i32> count;
    "#,
    // Variations of PROBE_SPEC
    "wasm:bytecode:call:alt: { }",
    "wasm:bytecode:call:alt",
    "wasm:bytecode:call:dne",
    // Empty predicate
    "wasm:bytecode:call:alt  // { }",
    "wasm:bytecode:call:alt / 5i < r77 / { }",
    //            "wasm:bytecode:call:alt / i < 1 < 2 / { }", // TODO -- make invalid on semantic pass
    //            "wasm:bytecode:call:alt / (1 + 3) / { i }", // TODO -- make invalid on type check
    r#"wasm:bytecode:call:alt  / i == """" / { }"#,
    // bad statement
    "wasm:bytecode:call:alt / i == 1 / { i; }",
    r#"
    wasm:bytecode:br:before {
        i32 return;
    }
    "#,
    r#"
    wasm:bytecode:br:before {
        i32 if;
    }
    "#,
    // bad incrementor
    r#"
    wasm:bytecode:br:before {
        i = 0;
        if(i++ == 0){
            i = 2;
        }
    }
        "#,
    // bad fn definitions
    r#"
    fn_name() -> i32{
    wasm:bytecode:br:before {
    }
        "#,
    r#"
    fn_name(, arg0) -> i32{}
    wasm:bytecode:br:before {
    }
        "#,
];

const SPECIAL: &[&str] = &["BEGIN { }", "END { }", "wasm:::alt { }", "wasm:::alt { }"];

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

pub fn get_ast(script: &str, err: &mut ErrorGen) -> Option<Whamm> {
    info!("Getting the AST");
    parse_script(&script.to_string(), err)
}

fn is_valid_script(script: &str, err: &mut ErrorGen) -> bool {
    get_ast(script, err).is_some()
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
my_func() -> i32{
    return 5;
}
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

    match get_ast(script, &mut err) {
        Some(ast) => {
            // script
            assert_eq!(1, ast.scripts.len()); // a single script
            let script = ast.scripts.first().unwrap();
            //functions length - strcmp and my_func
            assert_eq!(1, script.fns.len());
            // provider
            assert_eq!(1, script.providers.len());
            let provider = script.providers.get("wasm").unwrap();
            assert_eq!("wasm", provider.name);
            assert_eq!(0, provider.globals.len());
            assert_eq!(0, provider.fns.len());

            assert_eq!(1, provider.packages.len());
            let package = provider.packages.get("bytecode").unwrap();
            assert_eq!("bytecode", package.name);
            assert_eq!(2, package.globals.len());
            assert_eq!(0, package.fns.len());

            assert_eq!(1, package.events.len());
            let event = package.events.get("call").unwrap();
            assert_eq!("call", event.name);
            assert_eq!(4, event.globals.len());
            assert_eq!(0, event.fns.len());

            assert_eq!(1, event.probe_map.len());
            assert_eq!(1, event.probe_map.get("alt").unwrap().len());

            let probe = event.probe_map.get("alt").unwrap().first().unwrap();
            assert_eq!(0, probe.globals.len());
            assert_eq!(0, probe.fns.len());
            assert_eq!("alt", probe.mode);

            // probe predicate
            assert!(probe.predicate.is_some());

            // probe body
            assert!(&probe.body.is_some());
            assert_eq!(1, probe.body.as_ref().unwrap().len());

            print_ast(&ast);

            if err.has_errors {
                err.report()
            }
            assert!(!err.has_errors);
        }
        None => {
            error!("Could not get ast from script: {}", script);
            err.report();
            panic!();
        }
    };
}
#[test]
pub fn test_ast_special_cases() {
    setup_logger();
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    run_test_on_valid_list(SPECIAL.iter().map(|s| s.to_string()).collect(), &mut err);
}

fn print_ast(ast: &Whamm) {
    let mut visitor = AsStrVisitor { indent: 0 };
    println!("{}", visitor.visit_whamm(ast));
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

    match get_ast(script, &mut err) {
        Some(ast) => {
            print_ast(&ast);
        }
        None => {
            error!("Could not get ast from script: {}", script);
            if err.has_errors {
                err.report();
            }
            assert!(!err.has_errors);
        }
    };
}
pub fn testing_block() {
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

    match get_ast(script, &mut err) {
        Some(ast) => {
            print_ast(&ast);
        }
        None => {
            error!("Could not get ast from script: {}", script);
            if err.has_errors {
                err.report();
            }
            assert!(!err.has_errors);
        }
    };
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
