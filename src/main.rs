use crate::parser::dtrace_parser::*;
use crate::compiler::dtrace_compiler::*;

pub mod parser;
pub mod compiler;

use log::{info, error};
use std::env;
use std::io;
use std::process::exit;
use std::path::PathBuf;

fn setup_logger() {
    env_logger::init();
}

pub fn get_wasm(path: PathBuf) -> Vec<u8> {
    if !path.exists() {
        error!("could not read Wasm module: {path:?}");
        exit(1);
    }
    match std::fs::read(&path) {
        Ok(wasm) => wasm,
        Err(err) => {
            error!("Could not read Wasm module '{:?}': {}", path, err);
            exit(1);
        }
    }
}

// TODO -- create CLI
fn main() -> io::Result<()> {
    setup_logger();
    let args: Vec<_> = env::args().collect();
    if args.len() <= 2 {
        // TODO -- this CLI output is not accurate...
        error!("Please provide path to a Dtrace script.");
        exit(1);
    }

    // Use first arg as the app Wasm to instrument
    let app_wasm_path = PathBuf::from(&args[1]);
    // let app_wasm = get_wasm(PathBuf::from(&args[1]));
    // Use second arg the probes definitions
    let probes = std::fs::read_to_string(&args[2]);
    match probes {
        Ok(unparsed_str) => {
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

            emit_wasm(&ast, &app_wasm_path);
        },
        Err(e) => {
            error!("Cannot read specified file {}: {e}", &args[1]);
            exit(1);
        }
    }

    Ok(())
}