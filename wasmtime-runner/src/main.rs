use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

const WASM_MODULE: &str = "../output/output.wasm";
const CORE_LIB_NAME: &str = "whamm_core";
const CORE_LIB_MODULE: &str = "../core_lib/target/wasm32-wasip1/release/core_lib.wasm";

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let core_lib_wasm = Module::from_file(&engine, CORE_LIB_MODULE)?;
    linker.module(&mut store, CORE_LIB_NAME, &core_lib_wasm)?;
    let app_wasm = Module::from_file(&engine, WASM_MODULE)?;
    linker.module(&mut store, "", &app_wasm)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}