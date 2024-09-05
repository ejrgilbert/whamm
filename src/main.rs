extern crate core;

use std::env;
use cli::{Cmd, WhammCli};

use crate::common::error::ErrorGen;
use crate::parser::whamm_parser::*;

mod cli;
pub mod common;
pub mod emitter;
pub mod generator;
pub mod parser;
pub mod verifier;
mod wast;

use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use whamm::common::instr::LibStrategy;
use crate::cli::{InstrArgs};
use crate::common::instr::Config;

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
            // check_args(&args);
            common::instr::run_with_path(
                args.app,
                args.script,
                args.output_path,
                MAX_ERRORS,
                // Config {
                //     virgil: args.virgil,
                //     testing: args.testing,
                //     library_strategy: args.lib,
                //     mem: args.mem,
                //     mem_offset: args.mem_offset
                // }
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

const DEFAULT_MEM_OFFSET: u32 = 1_052_576;
// fn check_args(args: &InstrArgs) -> Config {
//     let mut config = Config::new();
//     if matches!(&args.lib, Some(LibraryStrategy::Imported)) {
//         // should not have memory or offset set
//     }
//     if matches!(&args.lib, Some(LibraryStrategy::Merged)) {
//         match &args.mem {
//             Some(MemoryStrategy::Offset) => {
//                 if let Some(offset) = args.mem_offset {
//                     config.library_strategy = LibStrategy::MergedWithOffset(*offset);
//                 } else {
//                     config.library_strategy = LibStrategy::MergedWithOffset(DEFAULT_MEM_OFFSET);
//                 }
//             }
//             Some(MemoryStrategy::Multi) => {
//                 todo!()
//             },
//             None => {
//                 todo!()
//             }
//         }
//     }
//
//
//
//     // if lib is imported, should not have mem or offset configured
//     config
// }
