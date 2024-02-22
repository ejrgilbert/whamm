use crate::parser::dtrace::*;

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
    "provider:module:function:name // { }",
    "provider:module:function:name / 5i < r77 / { }",
    //            "provider:module:function:name / i < 1 < 2 / { }", // TODO -- make invalid on semantic pass
    //            "provider:module:function:name / (1 + 3) / { i }", // TODO -- make invalid on type check

    // bad statement
    "provider:module:function:name / i == 1 / { 2i; }",
];

// ====================
// = Helper Functions =
// ====================

fn get_ast(script: &str) -> Option<Vec<AstNode>> {
    match parse_script(script) {
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

// =============
// = The Tests =
// =============

// #[test]
// pub fn test_parse_valid_scripts() {
//     for script in VALID_SCRIPTS {
//         println!("Parsing: {script}");
//         assert!(
//             is_valid_script(script),
//             "string = '{}' is not recognized as valid, but it should be",
//             script
//         );
//     }
// }

// #[test]
// pub fn test_parse_invalid_scripts() {
//     for script in INVALID_SCRIPTS {
//         println!("Parsing: {script}");
//         assert!(
//             !is_valid_script(script),
//             "string = '{}' is recognized as valid, but it should not",
//             script
//         );
//     }
// }

#[test]
pub fn test_ast_dumper() {
    // TODO -- next:
    //     1. Populate probe_spec
    //     2. Fix parsing with body below
    //     3. Add in the rest of the tests (see java project variant)
    //     4. Figure out how to fix the ID rule, need to know context to choose between VarId and ProbeId
    //     5. Remove need for EOI
    //     6. Verify ProbeId and VarId with Regex??
    let script = "provider:module:function:name / (i == \"1\") && (b == \"2\") / {  }";
    // let script = "provider:module:function:name / (i == \"1\") && (b == \"2\") / { i }"; // TODO -- fix

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
