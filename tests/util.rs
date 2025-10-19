use std::collections::HashMap;
use whamm::api::instrument::Injection;
use wirm::ir::module::side_effects::InjectType;

pub const CORE_WASM_PATH: &str = "tests/libs/whamm_core.wasm";

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn print_side_effects(side_effects: &HashMap<InjectType, Vec<Injection>>) {
    let mut sorted_side_effects: Vec<_> = side_effects.iter().collect();
    sorted_side_effects.sort_by_key(|&(key, _)| key);
    for (ty, injections) in sorted_side_effects.iter() {
        println!("====={}=====", "=".repeat(ty.to_string().len()));
        println!("==== {} ====", ty.to_string().to_uppercase());
        println!("====={}=====", "=".repeat(ty.to_string().len()));

        for inj in injections.iter() {
            println!("{:#?}", inj);
        }
        println!();
    }
}
