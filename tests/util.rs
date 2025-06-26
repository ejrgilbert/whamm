pub const CORE_WASM_PATH: &str = "./whamm_core/target/wasm32-wasip1/release/whamm_core.wasm";

pub fn setup_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
