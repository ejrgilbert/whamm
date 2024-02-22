use crate::parser::dtrace::*;
pub mod parser;

use std::env;
use std::io;
use std::process::exit;

// TODO -- create CLI
fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Please provide path to a Dtrace script.");
        exit(1);
    }
    // Use first arg as filename to read
    let unparsed_file = std::fs::read_to_string(&args[1]);
    match unparsed_file {
        Ok(unparsed_str) => {
            match parse_script(&unparsed_str) {
                Ok(ast) => {
                    println!("successfully parsed");
                    for node in ast {
                        println!("{:?}", node);
                    }
                },
                Err(e) => {
                    eprintln!("Parse failed: {e}");
                }
            }
        },
        Err(e) => {
            eprintln!("Cannot read specified file {}: {e}", &args[1]);
            exit(1);
        }
    }

    Ok(())
}