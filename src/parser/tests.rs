use crate::parser::dtrace_parser::*;
use crate::parser::types::{Dtrace};

use glob::{glob, glob_with};

use log::{info, error, warn};

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const VALID_SCRIPTS: &'static [&'static str] = &[
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
    "wasm:bytecode:br:before / \"i\" <= 1 / { }",
    "wasm:bytecode:br:before / i54 < r77 / { }",
    "wasm:bytecode:br:before / i54 < r77 / { }",
    "wasm:bytecode:br:before / i != 7 / { }",
    "wasm:bytecode:br:before / (i == \"1\") && (b == \"2\") / { }",
    "wasm:bytecode:br:before / i == \"1\" && b == \"2\" / { }",
    "wasm:bytecode:br:before / i == (1 + 3) / { count = 0; }",

    // Function calls
    r#"
wasm::call:alt / strpaircmp((arg2, arg3), "record") / {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#,
    r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_fn_module == "ic0" &&
    target_fn_name == "call_new" &&
    strpaircmp((arg0, arg1), "bookings") &&
    strpaircmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#,

    // Statements
    r#"
    wasm:bytecode:br:before {
        i = 0;
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
];

const INVALID_SCRIPTS: &'static [&'static str] = &[
    // Variations of PROBE_SPEC
    "wasm:bytecode:call:alt: { }",
    "wasm:bytecode:call:alt",
    "wasm:bytecode:call:alt: { }",
    "wasm:bytecode:call:dne",

    // Empty predicate
    "wasm:bytecode:call:alt  // { }",
    "wasm:bytecode:call:alt / 5i < r77 / { }",
    //            "wasm:bytecode:call:alt / i < 1 < 2 / { }", // TODO -- make invalid on semantic pass
    //            "wasm:bytecode:call:alt / (1 + 3) / { i }", // TODO -- make invalid on type check
    "wasm:bytecode:call:alt  / i == \"\"\"\" / { }",

    // bad statement
    "wasm:bytecode:call:alt / i == 1 / { i; }",
];

const SPECIAL: &'static [&'static str] = &[
    "BEGIN { }",
    "END { }",
    "wasm:::alt { }",
    "wasm:::alt { }"
];

// ====================
// = Helper Functions =
// ====================

const TEST_RSC_DIR: &str = "tests/dscripts/";
const PATTERN: &str = "*.d";
const TODO: &str = "*.TODO";

pub fn get_test_scripts(subdir: &str) -> Vec<String> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&*(TEST_RSC_DIR.to_owned() + subdir + "/" + &*PATTERN.to_owned()))
        .expect("Failed to read glob pattern") {
        let unparsed_file = std::fs::read_to_string(path.as_ref().unwrap()).expect(&*format!("Unable to read file at {:?}", &path));
        scripts.push(unparsed_file);
    }

    for path in glob_with(&*(TEST_RSC_DIR.to_owned() + subdir + "/" + &*TODO.to_owned()), options).expect("Failed to read glob pattern") {
        warn!("File marked with TODO: {}", path.as_ref().unwrap().display());
    }

    scripts
}

pub fn get_ast(script: &str) -> Option<Dtrace> {
    info!("Getting the AST");
    match parse_script(script.to_string()) {
        Ok(ast) => {
            Some(ast)
        },
        Err(e) => {
            error!("Parse failed {e}");
            None
        }
    }
}

fn is_valid_script(script: &str) -> bool {
    match get_ast(script) {
        Some(_ast) => {
            true
        },
        None => {
            false
        }
    }
}

pub fn run_test_on_valid_list(scripts: Vec<String>) {
    for script in scripts {
        info!("Parsing: {script}");
        assert!(
            is_valid_script(&script),
            "script = '{}' is not recognized as valid, but it should be",
            &script
        );
    }
}

// =============
// = The Tests =
// =============

// #[test]
// pub fn test_parse_valid_scripts() {
//     setup_logger();
//     run_test_on_valid_list(VALID_SCRIPTS.iter().map(|s| s.to_string()).collect());
// }
//
// #[test]
// pub fn test_parse_invalid_scripts() {
//     setup_logger();
//     for script in INVALID_SCRIPTS {
//         info!("Parsing: {script}");
//         assert!(
//             !is_valid_script(script),
//             "string = '{}' is recognized as valid, but it should not",
//             script
//         );
//     }
// }
//
// #[test]
// pub fn test_ast_special_cases() {
//     setup_logger();
//     run_test_on_valid_list(SPECIAL.iter().map(|s| s.to_string()).collect());
// }

#[test]
pub fn test_dtrace_print() {
    setup_logger();
    let script =     r#"
wasm::call:alt /
    target_fn_type == "import" &&
    target_fn_module == "ic0" &&
    target_fn_name == "call_new" &&
    strpaircmp((arg0, arg1), "bookings") &&
    strpaircmp((arg2, arg3), "record")
/ {
    new_target_fn_name = "redirect_to_fault_injector";
}
    "#;

    match get_ast(script) {
        Some(ast) => {
            // TODO -- add assertions on expected numbers in AST
            // TODO -- debug why provider has no submodules, maybe "*" not matching correctly?
            println!("{}", ast.as_str());
        },
        None => {
            error!("Could not get ast from script: {script}");
            assert!(false);
        }
    };
}

// #[test]
// pub fn test_implicit_probe_defs_dumper() {
//     setup_logger();
//     let script = "wasm:::alt / (i == \"1\") && (b == \"2\") / { i = 0; }";
//
//     match get_ast(script) {
//         Some(ast) => {
//             dump_ast(ast);
//         },
//         None => {
//             error!("Could not get ast from script: {script}");
//             assert!(false);
//         }
//     };
// }

// ===================
// = Full File Tests =
// ===================

// #[test]
// pub fn fault_injection() {
//     setup_logger();
//     let scripts = get_test_scripts("fault_injection");
//     if scripts.len() == 0 {
//         warn!("No test scripts found for `fault_injection` test.");
//     }
//     run_test_on_valid_list(scripts);
// }
//
// #[test]
// pub fn wizard_monitors() {
//     setup_logger();
//     let scripts = get_test_scripts("wizard_monitors");
//     if scripts.len() == 0 {
//         warn!("No test scripts found for `wizard_monitors` test.");
//     }
//     run_test_on_valid_list(scripts);
// }
//
// #[test]
// pub fn replay() {
//     setup_logger();
//     let scripts = get_test_scripts("replay");
//     if scripts.len() == 0 {
//         warn!("No test scripts found for `replay` test.");
//     }
//     run_test_on_valid_list(scripts);
// }
