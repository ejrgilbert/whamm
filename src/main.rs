extern crate core;

use std::path::PathBuf;
use crate::parser::whamm_parser::*;
use crate::behavior::builder_visitor::*;
use crate::generator::emitters::{Emitter, WasmRewritingEmitter};
use crate::generator::init_generator::{InitGenerator};
use crate::generator::instr_generator::{InstrGenerator};
use crate::common::error::ErrorGen;

pub mod parser;
pub mod behavior;
pub mod verifier;
pub mod generator;
pub mod common;

use clap::{Args, Parser, Subcommand};
use graphviz_rust::exec_dot;
use log::{info, error};
use std::process::exit;
use graphviz_rust::cmd::{CommandArg, Format};
use project_root::get_project_root;
use walrus::Module;

use crate::behavior::tree::BehaviorTree;
use crate::behavior::visualize::visualization_to_file;
use crate::parser::types::Whamm;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, verify};

const MAX_ERRORS: i32 = 15;

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
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(whammy_path.clone(), "".to_string(), MAX_ERRORS);

    // Process the script
    let mut whamm = get_whammy_ast(&whammy_path, &mut err);
    let symbol_table = get_symbol_table(&whamm, run_verifier, &mut err);
    let (behavior_tree, simple_ast) = build_behavior(&whamm, &mut err);

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    let app_wasm = Module::from_file(&app_wasm_path).unwrap();

    // Configure the emitter based on target instrumentation code format
    let mut emitter = if emit_virgil {
        unimplemented!();
    } else {
        WasmRewritingEmitter::new(
            app_wasm,
            symbol_table
        )
    };

    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut init = InitGenerator {
        emitter: Box::new(&mut emitter),
        context_name: "".to_string(),
        err: &mut err
    };
    init.run(&mut whamm);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator {
        tree: &behavior_tree,
        emitter: Box::new(&mut emitter),
        ast: simple_ast,
        err: &mut err,
        context_name: "".to_string(),
        curr_provider_name: "".to_string(),
        curr_package_name: "".to_string(),
        curr_event_name: "".to_string(),
        curr_probe_name: "".to_string(),
        curr_probe: None,
    };
    instr.run(&behavior_tree);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    match emitter.dump_to_file(output_wasm_path) {
        Err(e) => err.add_error(e),
        _ => {}
    }
    // If there were any errors encountered, report and exit!
    err.check_has_errors();
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
}

fn run_vis_whammy(whammy_path: String, run_verifier: bool, output_path: String) {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(whammy_path.clone(), "".to_string(), MAX_ERRORS);

    let whamm = get_whammy_ast(&whammy_path, &mut err);
    verify_ast(&whamm, run_verifier, &mut err);
    let (behavior_tree, ..) = build_behavior(&whamm, &mut err);

    // if has any errors, should report and exit!
    err.check_has_errors();

    let path = match get_pb(&PathBuf::from(output_path.clone())) {
        Ok(pb) => {
            pb
        }
        Err(_) => {
            exit(1)
        }
    };

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

fn get_symbol_table(ast: &Whamm, run_verifier: bool, err: &mut ErrorGen) -> SymbolTable {
    let st = build_symbol_table(&ast, err);
    err.check_too_many();
    verify_ast(ast, run_verifier, err);
    st
}

fn verify_ast(ast: &Whamm, run_verifier: bool, err: &mut ErrorGen) {
    if run_verifier {
        if !verify(ast) {
            error!("AST failed verification!");
            exit(1);
        }
    }
    err.check_too_many();
}

fn get_whammy_ast(whammy_path: &String, err: &mut ErrorGen) -> Whamm {
    match std::fs::read_to_string(&whammy_path) {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            match parse_script(&unparsed_str, err) {
                Some(ast) => {
                    info!("successfully parsed");
                    err.check_too_many();
                    return ast;
                },
                None => {
                    err.report();
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

fn build_behavior(whamm: &Whamm, err: &mut ErrorGen) -> (BehaviorTree, SimpleAST) {
    // Build the behavior tree from the AST
    let (mut behavior, simple_ast) = build_behavior_tree(&whamm, err);
    err.check_too_many();
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