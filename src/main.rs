extern crate core;
mod cli;

use cli::{Cmd, WhammCli};
use std::env;

use cli::LibraryLinkStrategyArg;
use clap::Parser;
use std::path::PathBuf;
use std::process::exit;
use whamm::api::instrument::{instrument_with_config, Config, LibraryLinkStrategy};
use whamm::api::utils::{print_info, run_wast_tests_at, write_to_file};

const ENABLE_WIZARD_ALT: bool = false;
const CORE_WASM_PATH: &str = "whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";

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

    // Set up the whamm home directory
    let whamm_home = format!(
        "{}/",
        env::var("WHAMM_HOME")
            .unwrap_or_else(|_| "./".to_string())
            .trim_end_matches("/")
    );

    // Get information from userinsstr command line args
    let cli = WhammCli::parse();

    match cli.command {
        Cmd::Info {
            rule,
            vars,
            functions,
            defs_path,
        } => {
            let defs = if let Some(path) = defs_path {
                path
            } else {
                whamm_home
            };
            print_info(rule, &defs, vars, functions);
        }
        Cmd::Wast { wast_path } => {
            run_wast_tests_at(&vec![PathBuf::from(wast_path)]);
        }
        Cmd::Instr(args) => {
            let app_path = if let Some(app_path) = args.app {
                app_path
            } else if !args.wizard {
                panic!("When performing bytecode rewriting (not the wizard target), a path to the target application is required!\nSee `whamm instr --help`")
            } else {
                "".to_string()
            };
            let core_lib_path = if let Some(core_lib) = args.core_lib {
                core_lib
            } else {
                format!("{}/{}", whamm_home, CORE_WASM_PATH)
            };
            let defs = if let Some(path) = args.defs_path {
                path
            } else {
                whamm_home
            };
            let result = instrument_with_config(
                &core_lib_path,
                &defs,
                app_path,
                args.script,
                args.user_libs,
                Config::new(
                    args.wizard,
                    ENABLE_WIZARD_ALT,
                    args.metrics,
                    args.no_bundle,
                    args.no_body,
                    args.no_pred,
                    args.no_report,
                    args.testing,
                    args.link_strategy.map(|s| s.into()),
                ),
            );
            write_to_file(result, args.output_path);
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
