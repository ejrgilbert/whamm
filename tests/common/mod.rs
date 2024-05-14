use whamm::parser::whamm_parser::*;
use whamm::parser::types::Whamm;
use whamm::verifier::verifier::build_symbol_table;

use glob::{glob, glob_with};
use log::{info, error, warn};
use whamm::behavior::builder_visitor::{build_behavior_tree, SimpleAST};
use whamm::behavior::tree::BehaviorTree;
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

fn get_test_scripts(subdir: &str) -> Vec<String> {
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

fn get_ast(script: &str) -> Option<Whamm> {
    info!("Getting the AST");
    match parse_script(script.to_string()) {
        Ok(ast) => {
            Some(ast)
        },
        Err(e) => {
            error!("Parse failed {}", e);
            None
        }
    }
}

fn parse_all_scripts(scripts: Vec<String>) -> Vec<Whamm> {
    let mut whammys = vec![];
    for script in scripts {
        info!("Parsing: {}", script);
        let ast_res = get_ast(&script);
        assert!(
            ast_res.is_some(),
            "script = '{}' is not recognized as valid, but it should be",
            &script
        );
        whammys.push(ast_res.unwrap());
    }
    whammys
}

fn process_scripts(scripts: Vec<String>) -> Vec<(Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    let asts = parse_all_scripts(scripts);

    // Build the symbol table from the AST
    let mut result = vec![];
    for ast in asts {
        let mut symbol_table = build_symbol_table(&ast);
        symbol_table.reset();

        // Build the behavior tree from the AST
        let (mut behavior, simple_ast) = build_behavior_tree(&ast);
        behavior.reset();

        result.push((ast, symbol_table, behavior, simple_ast));
    }

    result
}

pub fn setup_fault_injection() -> Vec<(Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("fault_injection");
    if scripts.len() == 0 {
        warn!("No test scripts found for `fault_injection` test.");
    }

    process_scripts(scripts)
}

pub fn setup_wizard_monitors() -> Vec<(Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("wizard_monitors");
    if scripts.len() == 0 {
        warn!("No test scripts found for `wizard_monitors` test.");
    }

    process_scripts(scripts)
}

pub fn setup_replay() -> Vec<(Whamm, SymbolTable, BehaviorTree, SimpleAST)> {
    setup_logger();
    let scripts = get_test_scripts("replay");
    if scripts.len() == 0 {
        warn!("No test scripts found for `replay` test.");
    }

    process_scripts(scripts)
}
