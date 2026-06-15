extern crate core;
mod cli;

use cli::{Cmd, WhammCli};

use anyhow::{bail, Result};
use clap::Parser;
use cli::LibraryLinkStrategyArg;
use std::process::exit;
use whamm::api::instrument::{instrument_with_config, Config, LibraryLinkStrategy};
use whamm::api::utils::{print_info, run_wast_tests_at, write_to_file};
use whamm::api::{load_core_lib_from_path, load_defs_from_path, parse_user_libs};

const ENABLE_WEI_ALT: bool = false;

fn setup_logger() {
    env_logger::init();
}

pub fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        for c in e.chain().skip(1) {
            eprintln!("  caused by {}", c);
        }
        eprintln!("{}", e.backtrace());
        exit(1)
    }
}

fn try_main() -> Result<()> {
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
            run_wast_tests_at(&vec![wast_path]);
        }
        Cmd::Instr(args) => {
            let wasm_app = if let Some(app_path) = args.app {
                let raw = std::fs::read(&app_path)?;
                // Accept either binary `.wasm` or text `.wat`.
                wat::parse_bytes(&raw)?.into_owned()
            } else if !args.wei {
                bail!("When performing bytecode rewriting (not the wei target), a path to the target application is required!\nSee `whamm instr --help`")
            } else {
                vec![]
            };
            let script = std::fs::read(&args.script)?;
            let user_libs = parse_user_libs(args.user_libs)?;
            let core_lib = args
                .core_lib
                .as_deref()
                .map(load_core_lib_from_path)
                .transpose()?;
            let defs = args.defs_path.as_deref().map(load_defs_from_path);
            match instrument_with_config(
                wasm_app,
                script,
                user_libs,
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
                core_lib,
                defs,
            ) {
                Ok(res) => write_to_file(res, &args.output_path),
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
