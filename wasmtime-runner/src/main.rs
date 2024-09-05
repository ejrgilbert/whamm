// use wasmtime::*;
// use wasmtime_wasi::WasiCtxBuilder;
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let engine = Engine::default();
//     // let module = Module::from_file(&engine, "../wasm_playground/maps/target/wasm32-wasi/release/maps.wasm")?;
//     let my_state = MyState::new();
//     let mut linker = wasmtime::Linker::<MyState>::new(&engine);
//
//     // Set up WASI with custom configuration
//     let wasi = WasiCtxBuilder::new()
//         .inherit_stdio() // Inherit standard input/output
//         .build();
//     wasmtime_wasi::preview1::wasi_snapshot_preview1::add_to_linker(&mut linker, |my_state| &mut my_state.wasi)?;
//
//     let mut store = Store::new(&engine, wasi);
//     let instance = linker.instantiate(&mut store, my_state)?;
//
//     // Call the exported functions
//     let print_info = instance.get_typed_func::<(i32, i32), ()>(&mut store, "print_info")?;
//     print_info.call(&mut store, (1, 42))?;
//
//     Ok(())
// }

use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

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
    let module = Module::from_file(&engine, "../output/output.wasm")?;
    // let module = Module::from_file(&engine, "../multi-mem.wat")?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}