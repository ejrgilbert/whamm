use crate::compiler::dtrace_compiler::*;
use crate::parser::tests;

use log::{error, debug};
use std::env;
use std::path::PathBuf;

// =================
// = Setup Logging =
// =================

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// ====================
// = Helper Functions =
// ====================

const VALID_SCRIPTS: &'static [&'static str] = &[
    "dfinity:module:function:alt { }",
];

lazy_static::lazy_static! {
    pub static ref USERS_WASM: Vec<u8> = {
        let wasm_path = PathBuf::from("tests/apps/users.wasm");

        let err = format!("
                Could not find Internet Identity Wasm module for current build.

                I will look for it at {:?}, and you can specify another path with the environment variable USERS_WASM (note that I run from {:?}).

                In order to build the Wasm module, please run the following command:
                    II_DUMMY_CAPTCHA=1 ./scripts/build
                ", &wasm_path, &std::env::current_dir().map(|x| x.display().to_string()).unwrap_or_else(|_| "an unknown directory".to_string()));
        get_wasm_path("USERS_WASM".to_string(), &wasm_path).expect(&err)
    };
}

/**
 * Helper that returns the content of `default_path` if found, or None if the file does not exist.
 * The `env_var` environment variable is also read for custom location; if the variable is set
 * _but_ the Wasm module is not present, we simply panic (i.e. we don't return None)
 */
pub fn get_wasm_path(env_var: String, default_path: &PathBuf) -> Option<Vec<u8>> {
    match env::var_os(env_var.clone()) {
        None => {
            if !default_path.exists() {
                return None;
            }
            Some(
                std::fs::read(default_path)
                    .unwrap_or_else(|_| panic!("could not read Wasm module: {default_path:?}")),
            )
        }
        Some(path) => {
            let pathname: String = path
                .into_string()
                .unwrap_or_else(|_| panic!("Invalid string path for {env_var}"));
            let path = PathBuf::from(pathname.clone());
            if !path.exists() {
                panic!("Could not find {pathname}");
            }
            Some(
                std::fs::read(path.clone())
                    .unwrap_or_else(|_| panic!("could not read Wasm module: {path:?}")),
            )
        }
    }
}

// =============
// = The Tests =
// =============

#[test]
pub fn test_emit_wasm() {
    for script in VALID_SCRIPTS {
        match tests::get_ast(script) {
            Some(ast) => {
                assert!(emit_wasm(&ast, &USERS_WASM));
            },
            None => {
                error!("Could not get ast from script: {script}");
                assert!(false);
            }
        };
    }
}