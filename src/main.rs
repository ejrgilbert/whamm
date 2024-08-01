extern crate core;

use cli::{Cmd, WhammCli};
use std::collections::HashMap;

use crate::common::error::ErrorGen;
use crate::emitter::rewriting::module_emitter::{MemoryTracker, ModuleEmitter};
use crate::generator::init_generator::InitGenerator;
use crate::generator::instr_generator::InstrGenerator;
use crate::parser::whamm_parser::*;

mod cli;
pub mod common;
pub mod emitter;
pub mod generator;
pub mod parser;
pub mod verifier;

use crate::emitter::rewriting::visiting_emitter::VisitingEmitter;
use crate::generator::simple_ast::build_simple_ast;
use crate::parser::types::Whamm;
use crate::verifier::types::SymbolTable;
use crate::verifier::verifier::{build_symbol_table, type_check};
use clap::Parser;
use log::{error, info};
use orca::ir::module::Module as WasmModule;
use std::path::PathBuf;
use std::process::exit;

const MAX_ERRORS: i32 = 15;

fn setup_logger() {
    env_logger::init();
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
        Cmd::Info {
            spec,
            globals,
            functions,
        } => {
            run_info(spec, globals, functions);
        }
        Cmd::Instr(args) => {
            if args.wizard {
                run_wizard_gen(args.script, args.output_path);
            } else if args.app.is_none() {
                return Err(failure::err_msg(
                    "App Wasm path is required for instrumentation",
                ));
            } else {
                run_rewrite_instr(args.app.unwrap(), args.script, args.output_path);
            }
        }
    }

    Ok(())
}

/// create output path if it doesn't exist
fn try_path(path: &String) {
    if !PathBuf::from(path).exists() {
        std::fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

fn run_info(spec: String, print_globals: bool, print_functions: bool) {
    // Parse the script and generate the information
    let mut err = ErrorGen::new("".to_string(), spec.clone(), MAX_ERRORS);
    print_info(spec, print_globals, print_functions, &mut err);

    err.fatal_report("PrintInfo");
}

use crate::parser::print_visitor::AsStrVisitor;
use crate::parser::types::WhammVisitor;
pub fn print_ast(ast: &Whamm) {
    let mut visitor = AsStrVisitor { indent: 0 };
    println!("{}", visitor.visit_whamm(ast));
}

fn run_wizard_gen(script_path: String, output_wasm_path: String) {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.clone(), "".to_string(), MAX_ERRORS);

    // Process the script
    let mut whamm = get_script_ast(&script_path, &mut err);
    let mut symbol_table = get_symbol_table(&mut whamm, &mut err);
    err.check_too_many();

    // TODO:: is the memory tracker necessary here?
    let mut mem_tracker = MemoryTracker {
        mem_id: 0,                  // Assuming the ID of the first memory is 0!
        curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
        required_initial_mem_size: 27, // Size memory must be to account for the added data
        emitted_strings: HashMap::new(),
    };
    // print_ast(&whamm);

    let mut module = orca::Module::new();

    // inserted memory for InitGenerator
    module.memories.push(wasmparser::MemoryType {
        memory64: false,
        shared: false,
        initial: 2,
        maximum: Some(2),
        page_size_log2: None,
    });

    // TODO: how reusable is the InitGenerator?
    let mut init = crate::emitter::wizard::init_generator::InitGenerator {
        emitter: crate::emitter::wizard::module_emitter::ModuleEmitter::new(
            &mut module,
            &mut symbol_table,
            &mut mem_tracker,
        ),
        context_name: "".to_string(),
        err: &mut err,
    };
    init.run(&mut whamm);

    try_path(&output_wasm_path);
    if let Err(e) = module.emit_wasm(&output_wasm_path) {
        err.add_error(ErrorGen::get_unexpected_error(
            true,
            Some(format!(
                "Failed to dump instrumented wasm to {} from error: {}",
                &output_wasm_path, e
            )),
            None,
        ))
    }
}

fn run_rewrite_instr(app_wasm_path: String, script_path: String, output_wasm_path: String) {
    // Set up error reporting mechanism
    let mut err = ErrorGen::new(script_path.clone(), "".to_string(), MAX_ERRORS);

    // Process the script
    let mut whamm = get_script_ast(&script_path, &mut err);
    let mut symbol_table = get_symbol_table(&mut whamm, &mut err);
    let simple_ast = build_simple_ast(&whamm, &mut err);
    err.check_too_many();

    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Read app Wasm into Orca module
    let buff = std::fs::read(app_wasm_path).unwrap();
    let mut app_wasm = WasmModule::parse(&buff, false).unwrap();

    // Create the memory tracker
    if app_wasm.memories.len() > 1 {
        // TODO -- make this work with multi-memory
        panic!("only single memory is supported")
    };
    let mut mem_tracker = MemoryTracker {
        mem_id: 0,                     // Assuming the ID of the first memory is 0!
        curr_mem_offset: 1_052_576, // Set default memory base address to DEFAULT + 4KB = 1048576 bytes + 4000 bytes = 1052576 bytes
        required_initial_mem_size: 27, // Size memory must be to account for the added data
        emitted_strings: HashMap::new(),
    };

    // Phase 0 of instrumentation (emit globals and provided fns)
    let mut init = InitGenerator {
        emitter: ModuleEmitter::new(&mut app_wasm, &mut symbol_table, &mut mem_tracker),
        context_name: "".to_string(),
        err: &mut err,
    };
    init.run(&mut whamm);
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    // Phase 1 of instrumentation (actually emits the instrumentation code)
    // This structure is necessary since we need to have the fns/globals injected (a single time)
    // and ready to use in every body/predicate.
    let mut instr = InstrGenerator::new(
        VisitingEmitter::new(&mut app_wasm, &mut symbol_table, &mem_tracker),
        simple_ast,
        &mut err,
    );
    instr.run();
    // If there were any errors encountered, report and exit!
    err.check_has_errors();

    try_path(&output_wasm_path);
    if let Err(e) = app_wasm.emit_wasm(&output_wasm_path) {
        err.add_error(ErrorGen::get_unexpected_error(
            true,
            Some(format!(
                "Failed to dump instrumented wasm to {} from error: {}",
                &output_wasm_path, e
            )),
            None,
        ))
    }
    // If there were any errors encountered, report and exit!
    err.check_has_errors();
}

fn get_symbol_table(ast: &mut Whamm, err: &mut ErrorGen) -> SymbolTable {
    let mut st = build_symbol_table(ast, err);
    err.check_too_many();
    verify_ast(ast, &mut st, err);
    st
}

fn verify_ast(ast: &mut Whamm, st: &mut SymbolTable, err: &mut ErrorGen) {
    if !type_check(ast, st, err) {
        error!("AST failed verification!");
    }
    err.check_too_many();
}

fn get_script_ast(script_path: &String, err: &mut ErrorGen) -> Whamm {
    match std::fs::read_to_string(script_path) {
        Ok(unparsed_str) => {
            // Parse the script and build the AST
            match parse_script(&unparsed_str, err) {
                Some(ast) => {
                    info!("successfully parsed");
                    err.check_too_many();
                    ast
                }
                None => {
                    err.report();
                    exit(1);
                }
            }
        }
        Err(error) => {
            error!("Cannot read specified file {}: {}", script_path, error);
            exit(1);
        }
    }
}
