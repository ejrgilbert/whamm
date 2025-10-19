use std::fs::File;
use std::env;
use wasi_common::sync::{Dir, WasiCtxBuilder};
use wasmtime::*;

const WASM_MODULE: &str = "../output/output.wasm";
const CORE_LIB_NAME: &str = "whamm_core";
const CORE_LIB_MODULE: &str = "../tests/libs/whamm_core.wasm";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut libs: Vec<(String, String)> = vec![];
    let mut core_lib = false;
    for arg in args.iter() {
        let parts: Vec<&str> = arg.split('=').collect();
        if parts.len() == 2 {
            let lib_name = parts[0];
            let lib_path = parts[1];
            libs.push((lib_name.to_string(), lib_path.to_string()));

            if lib_name == CORE_LIB_NAME {
                core_lib = true;
            }
        }
    }
    if !core_lib {
        libs.push((CORE_LIB_NAME.to_string(), CORE_LIB_MODULE.to_string()));
    }

    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    // let config = engine.config();
    // println!("{:?}", config);
    // config.wasm_multi_memory(true);
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .inherit_env()?
        .preopened_dir(Dir::from_std_file(File::open("../")?), "./")?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    for (lib_name, lib_path) in libs.iter() {
        let lib_wasm = if let Ok(wasm) = Module::from_file(&engine, lib_path) {
            wasm
        } else {
            Module::from_file(&engine, format!("../{lib_path}"))?
        };
        linker.module(&mut store, lib_name, &lib_wasm)?;
    }
    let wasm_module = match env::var("WASM_MODULE") {
        Ok(val) => val,
        Err(_) => WASM_MODULE.to_string(),
    };
    let app_wasm = Module::from_file(&engine, wasm_module)?;
    linker.module(&mut store, "", &app_wasm)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}