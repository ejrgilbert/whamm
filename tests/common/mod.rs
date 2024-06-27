use std::path::PathBuf;
use whamm::parser::types::Whamm;
use whamm::parser::whamm_parser::*;

use glob::{glob, glob_with};
use log::{error, info, warn};
use whamm::common::error::ErrorGen;
use whamm::verifier::types::SymbolTable;
use whamm::verifier::verifier::build_symbol_table;

// ====================
// = Helper Functions =
// ====================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

const TEST_RSC_DIR: &str = "tests/scripts/";
const PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

fn get_test_scripts(sub_dir: &str) -> Vec<(PathBuf, String)> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&(TEST_RSC_DIR.to_owned() + sub_dir + "/" + &*PATTERN.to_owned()))
        .expect("Failed to read glob pattern")
    {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = std::fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        scripts.push((file_name.clone(), unparsed_file));
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

fn get_ast(script: &str, err: &mut ErrorGen) -> Option<Whamm> {
    info!("Getting the AST");
    match parse_script(&script.to_string(), err) {
        Some(ast) => Some(ast),
        None => {
            error!("Parse failed");
            err.report();
            None
        }
    }
}

fn parse_all_scripts(
    scripts: Vec<(PathBuf, String)>,
    err: &mut ErrorGen,
) -> Vec<(PathBuf, String, Whamm)> {
    let mut mm_scripts = vec![];
    for (path, script) in scripts {
        info!("Parsing: {}", script);
        let ast_res = get_ast(&script, err);
        assert!(
            ast_res.is_some(),
            "script = '{}' is not recognized as valid, but it should be",
            &script
        );
        mm_scripts.push((path, script, ast_res.unwrap()));
    }
    mm_scripts
}

fn process_scripts(
    scripts: Vec<(PathBuf, String)>,
    err: &mut ErrorGen,
) -> Vec<(String, String, Whamm, SymbolTable)> {
    let asts = parse_all_scripts(scripts, err);

    // Build the symbol table from the AST
    let mut result = vec![];
    for (path, script_str, mut ast) in asts {
        let mut symbol_table = build_symbol_table(&mut ast, err);
        symbol_table.reset();

        result.push((
            path.into_os_string().into_string().unwrap(),
            script_str.clone(),
            ast,
            symbol_table,
        ));
    }

    result
}

pub fn setup_fault_injection(
    variation: &str,
    err: &mut ErrorGen,
) -> Vec<(String, String, Whamm, SymbolTable)> {
    setup_logger();
    let scripts = get_test_scripts(format!("fault_injection/{variation}").as_str());
    if scripts.is_empty() {
        warn!("No test scripts found for `fault_injection/{variation}` test.");
    }

    process_scripts(scripts, err)
}

pub fn setup_wizard_monitors(err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable)> {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.is_empty() {
        warn!("No test scripts found for `wizard_monitors` test.");
    }

    process_scripts(scripts, err)
}

pub fn setup_replay(err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable)> {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.is_empty() {
        warn!("No test scripts found for `replay` test.");
    }

    process_scripts(scripts, err)
}
