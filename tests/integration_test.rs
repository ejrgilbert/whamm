mod common;

use whamm::generator::emitters::{WasmRewritingEmitter};
use whamm::generator::code_generator::{CodeGenerator};

use log::error;
use std::process::Command;
use walrus::Module;

const APP_WASM_PATH: &str = "tests/apps/users.wasm";
const OUT_WASM_PATH: &str = "target/out.wasm";
const OUT_WAT_PATH: &str = "target/out.wat";

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

        generator.dump_to_file(OUT_WASM_PATH.to_string());

        let mut wasm2wat = Command::new("wasm2wat");
        wasm2wat.arg(OUT_WASM_PATH)
            .arg("-o")
            .arg(OUT_WAT_PATH);

        // wasm2wat verification check
        match wasm2wat.status() {
            Ok(code) => {
                if !code.success() {
                    assert!(false, "`wasm2wat` verification check failed!");
                }
                wasm2wat.output().expect("bad");
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