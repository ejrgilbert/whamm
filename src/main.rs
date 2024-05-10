extern crate core;

use std::io::Error;
use std::path::PathBuf;
use crate::parser::whamm_parser::*;
use crate::behavior::builder_visitor::*;
use crate::verifier::verifier::*;
// use crate::generator::emitters::{WasmRewritingEmitter};
// use crate::generator::code_generator::{CodeGenerator};

pub mod parser;
pub mod behavior;
pub mod verifier;
pub mod generator;

use clap::{Args, Parser, Subcommand};
use log::{info, error, trace};
use std::process::exit;
use opener::OpenError;
use project_root::get_project_root;
use crate::behavior::tree::BehaviorTree;
use crate::behavior::visualize::visualization_to_file;
use crate::parser::types::Whamm;

fn setup_logger() {
    env_logger::init();
}

/// `whamm` instruments a Wasm application with the Probes defined in the specified Whammy.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct WhammCli {
    // #[clap(flatten)]
    // global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    /// To instrument a Wasm application.
    Instr(InstrArgs),

    /// To visualize the generated behavior tree from the specified `whammy`
    VisTree {
        /// The path to the `whammy` file we want to visualize.
        #[clap(short, long, value_parser)]
        whammy: String,

        /// The path to output the visualization to.
        #[clap(short, long, value_parser, default_value = "output/vis.svg")]
        output_path: String,
    }
}

// #[derive(Debug, Args)]
// struct GlobalOpts {
//     // (not needed yet)
// }

#[derive(Debug, Args)]
struct InstrArgs {
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
    let cli = WhammCli::parse();

    match cli.command {
        Command::Instr(args) => {
            run_instr(args.app, args.whammy, args.output_path, args.virgil, args.run_verifier);
        }
        Command::VisTree {whammy, output_path} => {
            run_vis_tree(whammy, output_path);
        }
    }

    Ok(())
}

fn run_instr(app_wasm_path: String, whammy_path: String, output_wasm_path: String, emit_virgil: bool, run_verifier: bool) {
    let whamm = get_whammy_ast(&whammy_path);
    let behavior_tree = build_behavior(&whamm);

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
}

fn run_vis_tree(whammy_path: String, output_path: String) {
    let whamm = get_whammy_ast(&whammy_path);
    let behavior_tree = build_behavior(&whamm);

    let path = match get_pb(&PathBuf::from(output_path.clone())) {
        Ok(pb) => {
            pb
        }
        Err(_) => {
            exit(1)
        }
    };

    // visualization_to_file(&behavior_tree, path)
    match visualization_to_file(&behavior_tree, path) {
        Ok(_) => {
            match opener::open(output_path.clone()) {
                Err(err) => {
                    error!("Could not open visualization tree at: {}", output_path);
                    error!("{:?}", err)
                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    exit(0);
}

fn get_whammy_ast(whammy_path: &String) -> Whamm {
    match std::fs::read_to_string(&whammy_path) {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            match parse_script(unparsed_str) {
                Ok(ast) => {
                    info!("successfully parsed");
                    return ast;
                },
                Err(error) => {
                    error!("Parse failed: {}", error);
                    exit(1);
                }
            };
        },
        Err(error) => {
            error!("Cannot read specified file {}: {}", whammy_path, error);
            exit(1);
        }
    }
}

fn build_behavior(whamm: &Whamm) -> BehaviorTree {
    // Build the behavior tree from the AST
    let mut behavior = build_behavior_tree(&whamm);
    behavior.reset();

    behavior
}

pub(crate) fn get_pb(file_pb: &PathBuf) -> Result<PathBuf, String> {
    if file_pb.is_relative() {
        match get_project_root() {
            Ok(r) => {
                let mut full_path = r.clone();
                full_path.push(file_pb);
                Ok(full_path)
            }
            Err(e) => Err("the root folder does not exist.".to_string()),
        }
    } else {
        Ok(file_pb.clone())
    }
}