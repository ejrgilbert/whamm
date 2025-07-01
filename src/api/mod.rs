pub mod instrument;
pub mod utils;

use crate::parser::yml_processor::pull_all_yml_files;

// Include the embedded resources (see build.rs for how this is built)
include!(concat!(env!("OUT_DIR"), "/bundled.rs"));

fn get_defs_and_lib(
    defs_path: Option<String>,
    core_lib_path: Option<String>,
) -> (Vec<String>, Vec<u8>) {
    (get_defs(defs_path), get_core_lib(core_lib_path))
}
pub(crate) fn get_defs(defs_path: Option<String>) -> Vec<String> {
    if let Some(defs_path) = defs_path {
        pull_all_yml_files(&defs_path)
    } else {
        DEF_YAMLS.iter().map(|s| s.to_string()).collect()
    }
}
fn get_core_lib(core_lib_path: Option<String>) -> Vec<u8> {
    if let Some(core_lib_path) = core_lib_path {
        // Read core library Wasm into Wirm module
        std::fs::read(&core_lib_path).unwrap_or_else(|_| {
            panic!(
                "Could not read the core wasm module expected to be at location: {}",
                core_lib_path
            )
        })
    } else {
        WHAMM_CORE_LIB_BYTES.to_vec()
    }
}
