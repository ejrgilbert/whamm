extern crate core;

use crate::parser::dtrace_parser::*;
use crate::verifier::verifier::*;
use crate::generator::emitters::{WasmRewritingEmitter};
use crate::generator::code_generator::{CodeGenerator};

pub mod parser;
pub mod verifier;
pub mod generator;

use clap::Parser;
use log::{info, error};
use std::process::exit;

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

    /// Whether to emit Virgil
    #[clap(short, long, action)]
    emit_virgil: bool,

    #[clap(long, short, action)]
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
    let app_wasm_path = args.wasm_app_path;
    let dscript_path = args.dscript_path;
    let dscript = std::fs::read_to_string(&dscript_path);
    let output_wasm_path = args.output_path;

    match dscript {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            let mut dtrace = match parse_script(unparsed_str) {
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
            let symbol_table = verify(&dtrace);
            println!("{:?}", symbol_table);

            // Read app Wasm into Walrus module
            let _config =  walrus::ModuleConfig::new();
            let app_wasm = walrus::Module::from_file(&app_wasm_path).unwrap();

            let emitter = WasmRewritingEmitter::new(
                app_wasm_path.clone(),
                app_wasm,
                symbol_table
            );

            let mut generator = CodeGenerator::new(Box::new(emitter));

            generator.generate(&mut dtrace);
            generator.dump_to_file(output_wasm_path);
        },
        Err(e) => {
            error!("Cannot read specified file {}: {e}", dscript_path);
            exit(1);
        }
    }

    Ok(())
}