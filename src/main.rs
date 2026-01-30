extern crate core;
mod cli;

use cli::{Cmd, WhammCli};

use clap::Parser;
use cli::LibraryLinkStrategyArg;
use std::path::PathBuf;
use std::process::exit;
use whamm::api::instrument::{Config, LibraryLinkStrategy, instrument_with_config};
use whamm::api::utils::{print_info, run_wast_tests_at, write_to_file};

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
        eprintln!("{}", e.backtrace());
        exit(1)
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
            if let Err(mut err) = print_info(rule, defs_path, vars, functions) {
                err.report();
                exit(1);
            }
        }
        Cmd::Wast { wast_path } => {
            run_wast_tests_at(&vec![PathBuf::from(wast_path)]);
        }
        Cmd::Instr(args) => {
            let app_path = if let Some(app_path) = args.app {
                app_path
            } else if !args.wei {
                panic!(
                    "When performing bytecode rewriting (not the wei target), a path to the target application is required!\nSee `whamm instr --help`"
                )
            } else {
                "".to_string()
            };
            match instrument_with_config(
                app_path,
                args.script,
                args.user_libs,
                Config::new(
                    args.wei,
                    ENABLE_WEI_ALT,
                    args.metrics,
                    args.no_bundle,
                    args.no_body,
                    args.no_pred,
                    args.no_report,
                    args.testing,
                    args.link_strategy.map(|s| s.into()),
                ),
                args.core_lib,
                args.defs_path,
            ) {
                Ok(res) => write_to_file(res, args.output_path),
                Err(mut e) => {
                    e.report();
                    exit(1)
                }
            }
        }
    }

    Ok(())
}
impl From<LibraryLinkStrategyArg> for LibraryLinkStrategy {
    fn from(val: LibraryLinkStrategyArg) -> Self {
        match val {
            LibraryLinkStrategyArg::Merged => LibraryLinkStrategy::Merged,
            LibraryLinkStrategyArg::Imported => LibraryLinkStrategy::Imported,
        }
    }
}
