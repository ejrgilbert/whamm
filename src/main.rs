extern crate core;

use crate::parser::whamm_parser::*;
use crate::behavior::builder_visitor::*;
use crate::verifier::verifier::*;
// use crate::generator::emitters::{WasmRewritingEmitter};
// use crate::generator::code_generator::{CodeGenerator};

pub mod parser;
pub mod behavior;
pub mod verifier;
pub mod generator;

use clap::Parser;
use log::{info, error, trace};
use std::process::exit;

fn setup_logger() {
    env_logger::init();
}

/// `whamm` instruments a Wasm application with the Probes defined in the specified Whammy.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to the application's Wasm module we want to instrument.
    #[clap(short, long, value_parser)]
    app: String,
    /// The path to the Whammy containing the instrumentation Probe definitions.
    #[clap(short, long, value_parser)]
    whammy: String,
    /// The path that the instrumented version of the Wasm app should be output to.
    #[clap(short, long, value_parser, default_value = "./output/output.wasm")]
    output_path: String,

    /// Whether to emit Virgil code as the instrumentation code
    #[clap(short, long, action, default_value = "false")]
    virgil: bool,

    /// Whether to run the verifier on the specified whammy
    #[clap(long, short, action, default_value = "true")]
    run_verifier: bool
}

fn main() {
    // TODO add subcommands for virgil/wasm with different options per subcommand
    //      https://github.com/clap-rs/clap/blob/4e07b438584bb8a19e37599d4c5b11797bec5579/examples/git.rs
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
    let args = Args::parse();
    let app_wasm_path = args.app;
    let whammy_path = args.whammy;
    let whammy = std::fs::read_to_string(&whammy_path);
    let output_wasm_path = args.output_path;

    let emit_virgil = args.virgil;
    let run_verifier = args.run_verifier;

    match whammy {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            let whamm = match parse_script(unparsed_str) {
                Ok(ast) => {
                    info!("successfully parsed");
                    ast
                },
                Err(error) => {
                    error!("Parse failed: {}", error);
                    exit(1);
                }
            };

            // Build the behavior tree from the AST
            let mut behavior = build_behavior_tree(&whamm);
            behavior.reset();
            trace!("{:#?}", behavior);
            // exit(0);

            // Build the symbol table from the AST
            let mut symbol_table = verify(&whamm, run_verifier);
            trace!("{:#?}", symbol_table);
            symbol_table.reset();

            // // Read app Wasm into Walrus module
            // let _config =  walrus::ModuleConfig::new();
            // let app_wasm = walrus::Module::from_file(&app_wasm_path).unwrap();
            //
            // Configure the emitter based on target instrumentation code format
            // let emitter = if emit_virgil {
            //     unimplemented!();
            // } else {
            //     WasmRewritingEmitter::new(
            //     app_wasm,
            //     symbol_table
            // )};
            //
            // let mut generator = CodeGenerator::new(Box::new(emitter));
            //
            // generator.generate(&mut whamm);
            // generator.dump_to_file(output_wasm_path);
        },
        Err(error) => {
            error!("Cannot read specified file {}: {}", whammy_path, error);
            exit(1);
        }
    }

    Ok(())
}