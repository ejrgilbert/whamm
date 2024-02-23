use crate::parser::dtrace_parser::*;
use crate::compiler::dtrace_compiler::*;

pub mod parser;
pub mod compiler;

use log::*;
use std::env;
use std::io;
use std::process::exit;

// TODO -- create CLI
fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        error!("Please provide path to a Dtrace script.");
        exit(1);
    }
    // Use first arg as filename to read
    let unparsed_file = std::fs::read_to_string(&args[1]);
    match unparsed_file {
        Ok(unparsed_str) => {
            match parse_script(unparsed_str) {
                Ok(ast) => {
                    info!("successfully parsed");
                    for node in ast {
                        debug!("{:?}", node);
                    }
                },
                Err(e) => {
                    error!("Parse failed: {e}");
                }
            }
        },
        Err(e) => {
            error!("Cannot read specified file {}: {e}", &args[1]);
            exit(1);
        }
    }

    Ok(())
}