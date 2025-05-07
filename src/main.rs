extern crate core;

use cli::{Cmd, WhammCli};
use std::fs;

use crate::common::error::ErrorGen;
use crate::parser::whamm_parser::*;

mod cli;
pub mod common;
pub mod emitter;
pub mod generator;
pub mod lang_features;
pub mod parser;
pub mod verifier;
mod wast;

use crate::common::instr::Config;
use clap::Parser;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::exit;

const ENABLE_WIZARD_ALT: bool = false;
const CORE_WASM_PATH: &str = "./whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";
const MAX_ERRORS: i32 = 15;

fn setup_logger() {
    env_logger::init();
}

fn main() {
    read_yml();
    // if let Err(e) = try_main() {
    //     eprintln!("error: {}", e);
    //     for c in e.iter_chain().skip(1) {
    //         eprintln!("  caused by {}", c);
    //     }
    //     eprintln!("{}", e.backtrace());
    //     exit(1)
    // }
}

fn try_main() -> Result<(), failure::Error> {
    setup_logger();

    // Get information from user command line args
    let cli = WhammCli::parse();

    match cli.command {
        Cmd::Info {
            rule,
            globals,
            functions,
        } => {
            run_info(rule, globals, functions);
        }
        Cmd::Wast { wast_path } => {
            run_wast(wast_path);
        }
        Cmd::Instr(args) => {
            let app_path = if let Some(app_path) = args.app {
                app_path
            } else if !args.wizard {
                panic!("When performing bytecode rewriting (not the wizard target), a path to the target application is required!\nSee `whamm instr --help`")
            } else {
                "".to_string()
            };
            let core_lib_path = if let Some(core_lib) = args.core_lib {
                core_lib
            } else {
                CORE_WASM_PATH.to_string()
            };
            common::instr::run_with_path(
                &core_lib_path,
                app_path,
                args.script,
                args.user_libs,
                args.output_path,
                MAX_ERRORS,
                Config::new(
                    args.wizard,
                    ENABLE_WIZARD_ALT,
                    args.metrics,
                    args.no_bundle,
                    args.no_body,
                    args.no_pred,
                    args.no_report,
                    args.testing,
                    args.link_strategy,
                ),
            );
        }
    }

    Ok(())
}

fn run_info(rule: String, print_globals: bool, print_functions: bool) {
    // Parse the script and generate the information
    let mut err = ErrorGen::new("".to_string(), rule.clone(), MAX_ERRORS);
    print_info(rule, print_globals, print_functions, &mut err);

    err.fatal_report("PrintInfo");
}

fn run_wast(wast_path: String) {
    wast::test_harness::setup_and_run_tests(&vec![PathBuf::from(wast_path)])
        .expect("WAST Test failed!");
    println!("The wast test passed!");
}

fn read_yml() {
    let mut yml_files = vec![];

    // push events first (sets up the anchors)
    for path in glob("providers/packages/events/*.yaml").expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        yml_files.push(unparsed_file);
    }

    // push packages next (sets up the anchors)
    for path in glob("providers/packages/*.yaml").expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        yml_files.push(unparsed_file);
    }

    // finally the providers
    for path in glob("providers/*.yaml").expect("failed to read glob pattern") {
        let file_name = path.as_ref().unwrap();
        let unparsed_file = fs::read_to_string(file_name)
            .unwrap_or_else(|_| panic!("Unable to read file at {:?}", &path));
        yml_files.push(unparsed_file);
    }

    let mut all_yml = "".to_string();
    for yml in yml_files.iter() {
        all_yml += yml;
    }

    // let f = std::fs::File::open("providers/wasm.yaml").expect("Could not open file.");
    let def: MonitorModuleDefinition =
        serde_yml::from_str(&all_yml).expect("Could not read values.");
    println!("{:?}", def);
}

#[derive(Debug, Serialize, Deserialize)]
struct Cfg {
    update_frequency_sec: u32,
    num_threads: u32,
    data_sources: Vec<String>,
}

// TODO -- start working on reading from yaml into this type of structure
#[derive(Debug, Serialize, Deserialize)]
struct MonitorModuleDefinition {
    providers: Vec<Provider>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Provider {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<(String, String)>,
    packages: Vec<Package>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<(String, String)>,
    events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    bound_vars: Vec<BoundVar>,
    bound_fns: Vec<BoundFunc>,
    supported_modes: Vec<String>,
    req_map: bool,
    docs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundVar {
    name: String,
    docs: String,
    #[serde(rename = "type")]
    ty: String,
    derived_from: String
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundFunc {

}
