extern crate core;

use cli::{Cmd, WhammCli};

use crate::common::error::ErrorGen;
use crate::parser::whamm_parser::*;

mod cli;
pub mod common;
pub mod emitter;
pub mod generator;
pub mod linker;
pub mod parser;
pub mod verifier;
mod wast;

use crate::common::instr::Config;
use clap::Parser;
use std::path::PathBuf;
use std::process::exit;

const CORE_WASM_PATH: &str = "./wasm_playground/maps/target/wasm32-wasip1/debug/maps.wasm";
const MAX_ERRORS: i32 = 15;

fn setup_logger() {
    env_logger::init();
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        for c in e.iter_chain().skip(1) {
            eprintln!("  caused by {}", c);
        }
        eprintln!("{}", e.backtrace());
        exit(1)
    }
}

fn try_main() -> Result<(), failure::Error> {
    setup_logger();

    // Get information from user command line args
    let cli = WhammCli::parse();

    match cli.command {
        Cmd::Info {
            spec,
            globals,
            functions,
        } => {
            run_info(spec, globals, functions);
        }
        Cmd::Wast { wast_path } => {
            run_wast(wast_path);
        }
        Cmd::Instr(args) => {
            common::instr::run_with_path(
                CORE_WASM_PATH,
                args.app,
                args.script,
                args.output_path,
                MAX_ERRORS,
                Config::new(
                    args.virgil,
                    args.testing,
                    args.link_strategy
                )
            );
        }
    }

    Ok(())
}

fn run_info(spec: String, print_globals: bool, print_functions: bool) {
    // Parse the script and generate the information
    let mut err = ErrorGen::new("".to_string(), spec.clone(), MAX_ERRORS);
    print_info(spec, print_globals, print_functions, &mut err);

    err.fatal_report("PrintInfo");
}

fn run_wast(wast_path: String) {
    wast::test_harness::setup_and_run_tests(&vec![PathBuf::from(wast_path)])
        .expect("WAST Test failed!");
    println!("The wast test passed!");
}
