use walrus::Module;
use whamm::generator::emitters::{WasmRewritingEmitter};
use whamm::generator::code_generator::{CodeGenerator};

mod common;

const APP_WASM_PATH: &str = "tests/apps/users.wasm";

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