mod common;


use log::error;
use std::fs;
use std::process::{Command, Stdio};
use std::path::Path;
use walrus::Module;
use whamm::common::error::ErrorGen;
use whamm::generator::init_generator::InitGenerator;
use whamm::generator::emitters::{Emitter, WasmRewritingEmitter};
use whamm::generator::instr_generator::InstrGenerator;

const APP_WASM_PATH: &str = "tests/apps/users.wasm";

const OUT_BASE_DIR: &str = "target";
const OUT_WASM_NAME: &str = "out.wasm";

fn get_wasm_module() -> Module {
    // Read app Wasm into Walrus module
    let _config =  walrus::ModuleConfig::new();
    Module::from_file(APP_WASM_PATH).unwrap()
}

/// This test just confirms that a wasm module can be instrumented with the preconfigured
/// whammys without errors occurring.
#[test]
fn instrument_with_fault_injection() {
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_fault_injection(&mut err);
    assert!(processed_scripts.len() > 0);
    err.fatal_report("Integration Test");

    for (whammy_path, script_text, mut whamm, symbol_table, behavior, simple_ast) in processed_scripts {
        let app_wasm = get_wasm_module();
        let mut err = ErrorGen::new(whammy_path.clone(), script_text, 0);
        let mut emitter = WasmRewritingEmitter::new(
            app_wasm,
            symbol_table
        );
        // Phase 0 of instrumentation (emit globals and provided fns)
        let mut init = InitGenerator {
            emitter: Box::new(&mut emitter),
            context_name: "".to_string(),
            err: &mut err
        };
        assert!(init.run(&mut whamm));
        err.fatal_report("Integration Test");

        // Phase 1 of instrumentation (actually emits the instrumentation code)
        // This structure is necessary since we need to have the fns/globals injected (a single time)
        // and ready to use in every body/predicate.
        let mut instr = InstrGenerator {
            tree: &behavior,
            emitter: Box::new(&mut emitter),
            ast: simple_ast,
            context_name: "".to_string(),
            curr_provider_name: "".to_string(),
            curr_package_name: "".to_string(),
            curr_event_name: "".to_string(),
            curr_probe_name: "".to_string(),
            curr_probe: None,
            err: &mut err
        };
        // TODO add assertions here once I have error logic in place to check that it worked!
        instr.run(&behavior);
        err.fatal_report("Integration Test");

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
        match emitter.dump_to_file(out_wasm_path.clone()) {
            Err(e) => err.add_error(e),
            _ => {}
        }
        err.fatal_report("Integration Test");

        let mut wasm2wat = Command::new("wasm2wat");
        wasm2wat.stdout(Stdio::null())
            .arg(out_wasm_path);

        // wasm2wat verification check
        match wasm2wat.status() {
            Ok(code) => {
                if !code.success() {
                    assert!(false, "`wasm2wat` verification check failed!");
                }
                assert!(true);
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
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_wizard_monitors(&mut err);
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}

#[test]
fn instrument_with_replay() {
    let mut err = ErrorGen::new("".to_string(), "".to_string(), 0);
    let processed_scripts = common::setup_replay(&mut err);
    // TODO -- change this when you've supported this monitor type
    assert_eq!(processed_scripts.len(), 0);
}