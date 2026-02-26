extern crate core;
mod cli;

use cli::{Cmd, WhammCli};

use crate::cli::{InstrArgs, WacArgs};
use clap::Parser;
use cli::LibraryLinkStrategyArg;
use std::path::PathBuf;
use whamm::api::instrument::{instrument_with_config, wac, Config, LibraryLinkStrategy};
use whamm::api::utils::{print_info, run_wast_tests_at, write_to_file};

use colored::Colorize;

const ENABLE_WEI_ALT: bool = false;

fn setup_logger() {
    env_logger::init();
}

pub fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        for c in e.iter_chain().skip(1) {
            eprintln!("  caused by {}", c);
        }
        panic!("{}", e.backtrace());
    }
}

fn try_main() -> Result<(), failure::Error> {
    setup_logger();

    // Get information from userinstr command line args
    let cli = WhammCli::parse();

    match cli.command {
        Cmd::Info {
            rule,
            vars,
            functions,
            defs_path,
        } => {
            if let Err(mut err) = print_info(rule, &defs_path, vars, functions) {
                err.report();
                panic!();
            }
        }
        Cmd::Wast { wast_path } => {
            run_wast_tests_at(&vec![PathBuf::from(wast_path)]);
        }
        Cmd::Instr(args) => {
            let app_path = if let Some(app_path) = &args.app {
                app_path.clone()
            } else if !args.wei {
                panic!("When performing bytecode rewriting (not the wei target), a path to the target application is required!\nSee `whamm instr --help`")
            } else {
                "".to_string()
            };
            match instrument_with_config(
                app_path,
                &args.script,
                &args.user_libs,
                Config::new(
                    args.wei,
                    ENABLE_WEI_ALT,
                    args.metrics,
                    args.no_bundle,
                    args.no_body,
                    args.no_pred,
                    args.no_report,
                    args.testing,
                    args.link_strategy.clone().map(|s| s.into()),
                ),
                &args.core_lib,
                &args.defs_path,
            ) {
                Ok((was_component, res)) => process_instr_result(was_component, res, &args),
                Err(mut e) => {
                    e.report();
                    panic!();
                }
            }
        }
        Cmd::Wac(args) => wac(&args.app, &args.output_path, &args.user_libs),
    }

    Ok(())
}

fn process_instr_result(was_component: bool, bytes: Vec<u8>, instr_args: &InstrArgs) {
    println!(
        "{}",
        "\n\nYour wasm binary has been instrumented successfully!".green()
    );
    write_to_file(bytes, &instr_args.output_path);

    if was_component {
        // print the `wac` command that should be run!
        println!("{}", "Run the following command to produce a single component containing all introduced library dependencies:".blue());
        println!(
            "{}",
            format!(
                "{}",
                WhammCli {
                    command: Cmd::Wac(WacArgs::from(instr_args))
                }
            )
            .blue()
        );
    }
}

impl From<LibraryLinkStrategyArg> for LibraryLinkStrategy {
    fn from(val: LibraryLinkStrategyArg) -> Self {
        match val {
            LibraryLinkStrategyArg::Merged => LibraryLinkStrategy::Merged,
            LibraryLinkStrategyArg::Imported => LibraryLinkStrategy::Imported,
        }
    }
}
