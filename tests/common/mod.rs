use std::path::PathBuf;
use whamm::parser::whamm_parser::*;
use whamm::parser::types::Whamm;
use whamm::verifier::verifier::build_symbol_table;

use glob::{glob, glob_with};
use log::{info, error, warn};
use whamm::behavior::builder_visitor::{build_behavior_tree, SimpleAST};
use whamm::behavior::tree::BehaviorTree;
use whamm::common::error::ErrorGen;
use whamm::verifier::types::SymbolTable;

// =================
// = Setup Logging =
// =================

fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const TEST_RSC_DIR: &str = "tests/whammys/";
const PATTERN: &str = "*.mm";
const TODO: &str = "*.TODO";

fn get_test_scripts(subdir: &str) -> Vec<(PathBuf, String)> {
    let mut scripts = vec![];
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for path in glob(&*(TEST_RSC_DIR.to_owned() + subdir + "/" + &*PATTERN.to_owned()))
        .expect("Failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = std::fs::read_to_string(file_name).expect(&*format!("Unable to read file at {:?}", &path));
        scripts.push((file_name.clone(), unparsed_file));
    }

    for path in glob_with(&*(TEST_RSC_DIR.to_owned() + subdir + "/" + &*TODO.to_owned()), options).expect("Failed to read glob pattern") {
        warn!("File marked with TODO: {}", path.as_ref().unwrap().display());
    }

    scripts
}

fn get_ast(script: &str, err: &mut ErrorGen) -> Option<Whamm> {
    info!("Getting the AST");
    match parse_script(&script.to_string(), err) {
        Some(ast) => {
            Some(ast)
        },
        None => {
            error!("Parse failed");
            err.report();
            None
        }
    }
}

fn parse_all_scripts(scripts: Vec<(PathBuf, String)>, err: &mut ErrorGen) -> Vec<(PathBuf, String, Whamm)> {
    let mut whammys = vec![];
    for (path, script) in scripts {
        info!("Parsing: {}", script);
        let ast_res = get_ast(&script, err);
        assert!(
            ast_res.is_some(),
            "script = '{}' is not recognized as valid, but it should be",
            &script
        );
        whammys.push((path, script, ast_res.unwrap()));
    }
    whammys
}

fn process_scripts(scripts: Vec<(PathBuf, String)>, err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    let asts = parse_all_scripts(scripts, err);

    // Build the symbol table from the AST
    let mut result = vec![];
    for (path, script_str, ast) in asts {
        let mut symbol_table = build_symbol_table(&ast, err);
        symbol_table.reset();

        // Build the behavior tree from the AST
        let (mut behavior, simple_ast) = build_behavior_tree(&ast, err);
        behavior.reset();

        result.push((path.into_os_string().into_string().unwrap(), script_str, ast, symbol_table, behavior, simple_ast));
    }

    result
}

pub fn setup_fault_injection(err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("fault_injection");
    if scripts.len() == 0 {
        warn!("No test scripts found for `fault_injection` test.");
    }

    process_scripts(scripts, err)
}

pub fn setup_wizard_monitors(err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.len() == 0 {
        warn!("No test scripts found for `wizard_monitors` test.");
    }

    process_scripts(scripts, err)
}

pub fn setup_replay(err: &mut ErrorGen) -> Vec<(String, String, Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.len() == 0 {
        warn!("No test scripts found for `replay` test.");
    }

    process_scripts(scripts, err)
}
