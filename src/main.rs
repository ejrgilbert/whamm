use crate::parser::dtrace_parser::*;
use crate::compiler::dtrace_compiler::*;
use crate::verifier::verifier::*;

pub mod parser;
pub mod verifier;
// TODO -- remove all compiler stuff
pub mod compiler;
pub mod generator;

use clap::Parser;
use log::{info, error};
use std::process::exit;
use std::path::PathBuf;

fn setup_logger() {
    env_logger::init();
}

/// `dtrace` instruments a Wasm application with the Probes defined in the specified Dscript.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path to the application's Wasm module we want to instrument.
    #[clap(short, long, value_parser)]
    wasm_app_path: String,
    /// The path to the Dscript containing the instrumentation Probe definitions.
    #[clap(short, long, value_parser)]
    dscript_path: String,
    /// The path that the instrumented version of the Wasm app should be output to.
    #[clap(short, long, value_parser, default_value = "./target/output.wasm")]
    output_path: String,

    #[clap(long, short, action)]
    run_verifier: bool
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
    let args = Args::parse();
    let wasm_app_path = PathBuf::from(args.wasm_app_path);
    let dscript_path = args.dscript_path;
    let dscript = std::fs::read_to_string(&dscript_path);
    let output_path = args.output_path;

    match dscript {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            let ast = match parse_script(unparsed_str) {
                Ok(ast) => {
                    info!("successfully parsed");
                    ast
                },
                Err(e) => {
                    error!("Parse failed: {e}");
                    exit(1);
                }
            };

            // Build the symbol table from the AST
            let (symbol_table, _core_probes, _wasm_probes) = verify(&ast);
            symbol_table.print();

            //

            emit_wasm(&ast, &wasm_app_path, &output_path);
        },
        Err(e) => {
            error!("Cannot read specified file {}: {e}", dscript_path);
            exit(1);
        }
    }

    Ok(())
}