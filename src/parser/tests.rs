use crate::parser::dtrace::*;
use glob::{glob, glob_with};

const VALID_SCRIPTS: &'static [&'static str] = &[
    // Variations of PROBE_SPEC
    r#"
    provider:module:function:name { }
    "#,
    "prov*:module:function { }",
    "provider:module { }",
    "provider { }",
    "::: { }",
    "provider::: { }",
    ":module:: { }",
    "::function: { }",
    ":::name { }",
    ":module:function:name { }",
    "provider::function:name { }",
    "provider:module::name { }",

    // Predicates
    "provider:module:function:name / i / { }",
    "provider:module:function:name / \"i\" <= 1 / { }",
    "provider:module:function:name / i54 < r77 / { }",
    "provider:module:function:name / i54 < r77 / { }",
    "provider:module:function:name / i != 7 / { }",
    "provider:module:function:name / (i == \"1\") && (b == \"2\") / { }",
    "provider:module:function:name / i == \"1\" && b == \"2\" / { }",
    "provider:module:function:name / i == (1 + 3) / { i; }",

    // Statements
    r#"
    provider:module:function:name {
        i;
    }
    "#,

    // Comments
    r#"
    /* comment */
    provider:module:function:name { }
    "#,
    "provider:module:function:name { } // this is a comment",
    r#"/* comment */
    provider:module:function:name { } // this is a comment
    "#,
    r#"
    provider:module:function:name {
        i; // this is a comment
    }
    "#,
];

const INVALID_SCRIPTS: &'static [&'static str] = &[
    // Variations of PROBE_SPEC
    "provider:module:function:name: { }",
    "provider:module:function:name",

    // Empty predicate
    "provider:module:function:name  // { }",
    "provider:module:function:name / 5i < r77 / { }",
    //            "provider:module:function:name / i < 1 < 2 / { }", // TODO -- make invalid on semantic pass
    //            "provider:module:function:name / (1 + 3) / { i }", // TODO -- make invalid on type check
    "provider:module:function:name  / i == \"\"\"\" / { }",

    // bad statement
    "provider:module:function:name / i == 1 / { 2i; }",
];

const SPECIAL: &'static [&'static str] = &[
    "BEGIN { }",
    "END { }",
    "dfinity:::alt { }"
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
        eprintln!("WARN: File marked with TODO: {}", path.as_ref().unwrap().display());
    }

    scripts
}

fn get_ast(script: &str) -> Option<Vec<AstNode>> {
    match parse_script(script.to_string()) {
        Ok(ast) => {
            Some(ast)
        },
        Err(e) => {
            eprintln!("Parse failed {e}");
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
        println!("Parsing: {script}");
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

#[test]
pub fn test_parse_valid_scripts() {
    run_test_on_valid_list(VALID_SCRIPTS.iter().map(|s| s.to_string()).collect());
}

#[test]
pub fn test_parse_invalid_scripts() {
    for script in INVALID_SCRIPTS {
        println!("Parsing: {script}");
        assert!(
            !is_valid_script(script),
            "string = '{}' is recognized as valid, but it should not",
            script
        );
    }
}

#[test]
pub fn test_ast_special_cases() {
    run_test_on_valid_list(SPECIAL.iter().map(|s| s.to_string()).collect());
}

#[test]
pub fn test_ast_dumper() {
    let script = "provider:module:function:name / (i == \"1\") && (b == \"2\") / { i; }";

    match get_ast(script) {
        Some(ast) => {
            dump_ast(ast);
        },
        None => {
            eprintln!("Could not get ast from script: {script}");
            assert!(false);
        }
    };
}

// ===================
// = Full File Tests =
// ===================

#[test]
pub fn fault_injection() {
    let scripts = get_test_scripts("fault_injection");
    if scripts.len() == 0 {
        eprintln!("WARN: No test scripts found for `fault_injection` test.");
    }
    run_test_on_valid_list(scripts);
}

#[test]
pub fn wizard_monitors() {
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.len() == 0 {
        eprintln!("WARN: No test scripts found for `wizard_monitors` test.");
    }
    run_test_on_valid_list(scripts);
}

#[test]
pub fn replay() {
    let scripts = get_test_scripts("replay");
    if scripts.len() == 0 {
        eprintln!("WARN: No test scripts found for `replay` test.");
    }
    run_test_on_valid_list(scripts);
}
