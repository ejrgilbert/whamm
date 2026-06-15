pub mod instrument;
pub mod utils;

use crate::api::instrument::UserLibs;
use crate::parser::yml_processor::pull_all_yml_files;
use anyhow::{anyhow, Result};
use std::path::Path;

#[cfg(debug_assertions)]
pub static WHAMM_CORE_LIB_BYTES: &[u8] = include_bytes!("../../embedded/debug/whamm_core.wasm");

#[cfg(not(debug_assertions))]
pub static WHAMM_CORE_LIB_BYTES: &[u8] = include_bytes!("../../embedded/release/whamm_core.wasm");

// Include the embedded resources (see build.rs for how this is built)
include!(concat!(env!("OUT_DIR"), "/bundled.rs"));

/// Load provider YAML definitions from a directory path.
pub fn load_defs_from_path(defs_path: &Path) -> Vec<String> {
    pull_all_yml_files(defs_path.to_str().expect("defs path is not valid UTF-8"))
}

/// Read the core library wasm bytes from a path on disk.
pub fn load_core_lib_from_path(core_lib_path: &Path) -> Result<Vec<u8>> {
    Ok(std::fs::read(core_lib_path)?)
}

/// Returns provider YAML contents from `defs_path` if given,
/// otherwise the embedded defaults.
/// Used by tooling (e.g. `print_info`) that still wants path-shaped
/// input.
pub(crate) fn get_defs(defs_path: Option<&Path>) -> Vec<String> {
    match defs_path {
        Some(path) => load_defs_from_path(path),
        None => DEF_YAMLS.iter().map(|s| s.to_string()).collect(),
    }
}

/// Parse user-lib specs (formatted
/// `lib_name(import_override)?=/path/to/lib.wasm`) into the
/// bytes-based [`UserLibs`] map. Reads each library's wasm bytes from
/// disk.
///
/// Used by both the `whamm` CLI and by integration tests that
/// exercise the same string format.
pub fn parse_user_libs(specs: Vec<String>) -> Result<UserLibs> {
    let mut libs = UserLibs::new();
    for spec in specs.iter() {
        let (name_chunk, lib_path) = spec.split_once('=').ok_or_else(|| {
            anyhow!("A user lib should be specified using the following format: <lib_name>=/path/to/lib.wasm (got: {spec})")
        })?;
        let (lib_name, import_override) = match name_chunk.split_once('(') {
            Some((name, rest)) => (
                name.to_string(),
                Some(
                    rest.strip_suffix(')')
                        .ok_or_else(|| anyhow!("user lib spec missing ')' in name part: {spec}"))?
                        .to_string(),
                ),
            ),
            None => (name_chunk.to_string(), None),
        };
        let bytes = std::fs::read(Path::new(lib_path))?;
        libs.insert(lib_name, (import_override, bytes));
    }
    Ok(libs)
}
