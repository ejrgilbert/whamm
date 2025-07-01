use crate::common::error::ErrorGen;
use crate::parser::types::Whamm;
use crate::parser::whamm_parser::parse_script;
use crate::parser::yml_processor::pull_all_yml_files;
use log::{error, info};

pub mod numerics;
pub mod whamm_scripts;
const DEFS_PATH: &str = "./";

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn get_ast(script: &str, err: &mut ErrorGen) -> Whamm {
    info!("Getting the AST");
    match parse_script(&pull_all_yml_files(&DEFS_PATH), &script.to_string(), err) {
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
