extern crate core;

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use crate::parser::whamm_parser::*;
use crate::behavior::builder_visitor::*;
use crate::verifier::verifier::*;
// use crate::generator::emitters::{Emitter, WasmRewritingEmitter};
// use crate::generator::init_generator::{CodeGenerator};

pub mod parser;
pub mod behavior;
pub mod verifier;
pub mod generator;

use clap::{Args, Parser, Subcommand};
use graphviz_rust::exec_dot;
use log::{info, error};
use std::process::exit;
use graphviz_rust::cmd::{CommandArg, Format};
use project_root::get_project_root;
use walrus::Module;
// use whamm::generator::init_generator::InitGenerator;
// use whamm::generator::instr_generator::InstrGenerator;
use crate::behavior::tree::BehaviorTree;
use crate::behavior::visualize::visualization_to_file;
use crate::parser::types::{Probe, Whamm};

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

    /// To visualize the relationship between various structures in the module and its instructions
    VisWasm {
        /// The path to the Wasm module we want to visualize.
        #[clap(short, long, value_parser)]
        wasm: String,

        /// The path to output the visualization to.
        #[clap(short, long, value_parser, default_value = "output/wasm.dot")]
        output_path: String,
    },

    /// To visualize the generated behavior tree from the specified `whammy`
    VisWhammy {
        /// The path to the `whammy` file we want to visualize.
        #[clap(short, long, value_parser)]
        whammy: String,

        /// Whether to run the verifier on the specified whammy
        #[clap(long, short, action, default_value = "false")] // TODO -- change this default value to true when I have this implemented
        run_verifier: bool,

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
    #[clap(long, short, action, default_value = "false")] // TODO -- change this default value to true when I have this implemented
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
    let cli = WhammCli::parse();

    match cli.command {
        Command::Instr(args) => {
            run_instr(args.app, args.whammy, args.output_path, args.virgil, args.run_verifier);
        }
        Command::VisWasm {wasm, output_path} => {
            run_vis_wasm(wasm, output_path);
        }
        Command::VisWhammy {whammy, run_verifier, output_path} => {
            run_vis_whammy(whammy, run_verifier, output_path);
        }
    }

    Ok(())
}

fn run_instr(app_wasm_path: String, whammy_path: String, output_wasm_path: String, emit_virgil: bool, run_verifier: bool) {
    let mut whamm = get_whammy_ast(&whammy_path, run_verifier);
    let (behavior_tree, simple_ast) = build_behavior(&whamm);

    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    let app_wasm = Module::from_file(&app_wasm_path).unwrap();

    // // Configure the emitter based on target instrumentation code format
    // let mut emitter = if emit_virgil {
    //     unimplemented!();
    // } else {
    //     WasmRewritingEmitter::new(
    //         app_wasm,
    //         symbol_table
    //     )
    // };
    //
    // // Phase 0 of instrumentation (emit globals and provided fns)
    // let mut init = InitGenerator {
    //     emitter: Box::new(&mut emitter),
    //     context_name: "".to_string(),
    // };
    // init.run(&mut whamm);
    //
    // // Phase 1 of instrumentation (actually emits the instrumentation code)
    // // This structure is necessary since we need to have the fns/globals injected (a single time)
    // // and ready to use in every body/predicate.
    // let mut instr = InstrumentationGenerator {
    //     emitter: Box::new(&mut emitter),
    //     ast: simple_ast,
    //     context_name: "".to_string(),
    // };
    // instr.run(&behavior_tree);
    //
    // emitter.dump_to_file(output_wasm_path);
}

fn dump_to_file(app_wasm: &mut Module, output_wasm_path: String) -> bool {
    match app_wasm.emit_wasm_file(&output_wasm_path) {
        Ok(_ok) => {
            true
        },
        Err(err) => {
            error!("Failed to dump instrumented wasm to {} from error: {}", &output_wasm_path, err);
            false
        },
    }
}

fn run_vis_wasm(wasm_path: String, output_path: String) {
    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    let app_wasm = Module::from_file(&wasm_path).unwrap();

    match app_wasm.write_graphviz_dot(output_path.clone()) {
        Ok(_) => {
            match std::fs::read_to_string(&output_path.clone()) {
                Ok(dot_str) => {
                    let svg_path = format!("{}.svg", output_path.clone());

                    match exec_dot(
                        dot_str,
                        vec![Format::Svg.into(), CommandArg::Output(svg_path.clone())]
                    ) {
                        Err(e) => {
                            println!("{}", e.to_string());
                            exit(1);
                        }
                        _ => {}
                    }

                    match opener::open(svg_path.clone()) {
                        Err(err) => {
                            error!("Could not open visualization of wasm at: {}", svg_path);
                            error!("{:?}", err)
                        }
                        _ => {}
                    }
                },
                Err(error) => {
                    error!("Cannot read specified file {}: {}", output_path, error);
                    exit(1);
                }
            };
        }
        Err(_) => {}
    }
    exit(0);
}

fn run_vis_whammy(whammy_path: String, run_verifier: bool, output_path: String) {
    let whamm = get_whammy_ast(&whammy_path, run_verifier);
    let (behavior_tree, ..) = build_behavior(&whamm);

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

fn get_whammy_ast(whammy_path: &String, run_verifier: bool) -> Whamm {
    match std::fs::read_to_string(&whammy_path) {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            match parse_script(unparsed_str, run_verifier) {
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

fn build_behavior(whamm: &Whamm) -> (BehaviorTree, HashMap<String, HashMap<String, HashMap<String, HashMap<String, Vec<Probe>>>>>) {
    // Build the behavior tree from the AST
    let (mut behavior, simple_ast) = build_behavior_tree(&whamm);
    behavior.reset();

    (behavior, simple_ast)
}

pub(crate) fn get_pb(file_pb: &PathBuf) -> Result<PathBuf, String> {
    if file_pb.is_relative() {
        match get_project_root() {
            Ok(r) => {
                let mut full_path = r.clone();
                full_path.push(file_pb);
                Ok(full_path)
            }
            Err(e) => Err(format!("the root folder does not exist: {:?}", e)),
        }
    } else {
        Ok(file_pb.clone())
    }
}