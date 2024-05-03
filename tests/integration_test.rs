mod common;

use whamm::generator::emitters::{WasmRewritingEmitter};
use whamm::generator::code_generator::{CodeGenerator};

use log::{error, info};
use std::fs;
use std::process::Command;
use std::path::Path;
use walrus::Module;

const APP_WASM_PATH: &str = "tests/apps/users.wasm";

const OUT_BASE_DIR: &str = "target";
const OUT_WASM_NAME: &str = "out.wasm";
const OUT_WAT_NAME: &str = "out.wat";

fn get_wasm_module() -> Module {
    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    Module::from_file(APP_WASM_PATH).unwrap()
}

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// whammys without errors occurring.
#[test]
fn instrument_with_fault_injection() {
    let processed_scripts = common::setup_fault_injection();
    assert!(processed_scripts.len() > 0);

    for (mut whamm, symbol_table) in processed_scripts {
        let app_wasm = get_wasm_module();
        let emitter = WasmRewritingEmitter::new(
            app_wasm,
            symbol_table
        );

        let mut generator = CodeGenerator::new(Box::new(emitter));

        assert!(generator.generate(&mut whamm));

        if !Path::new(OUT_BASE_DIR).exists() {
            match fs::create_dir(OUT_BASE_DIR) {
                Err(err) => {
                    error!("{}", err.to_string());
                    assert!(false, "Could not create base output path.");
                },
                _ => {}
            }
        }

        let out_wasm_path = format!("{OUT_BASE_DIR}/{OUT_WASM_NAME}");
        let out_wat_path = format!("{OUT_BASE_DIR}/{OUT_WAT_NAME}");

        generator.dump_to_file(out_wasm_path.to_string());
        let mut ls = Command::new("ls");
        ls.arg("-al")
            .arg(OUT_BASE_DIR);
        ls.status().expect("process failed to execute");

        let mut ls = Command::new("ls");
        ls.arg("-al")
            .arg(out_wasm_path.clone());
        ls.status().expect("process failed to execute");

        let mut ls = Command::new("ls");
        ls.arg("-al")
            .arg(out_wat_path.clone());
        ls.status().expect("process failed to execute");
        //
        // info!("out_wat_path: {out_wat_path}");

        let mut wasm2wat = Command::new("wasm2wat");
        wasm2wat.arg(out_wasm_path)
            .arg("-o")
            .arg(out_wat_path);

        // wasm2wat verification check
        match wasm2wat.status() {
            Ok(code) => {
                if !code.success() {
                    assert!(false, "`wasm2wat` verification check failed!");
                }
            }
            Err(err) => {
                error!("{}", err.to_string());
                assert!(false, "`wasm2wat` verification check failed!");
            }
        };
    }
}

#[test]
fn instrument_with_wizard_monitors() {
    let processed_scripts = common::setup_wizard_monitors();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_replay() {
    let processed_scripts = common::setup_replay();
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}