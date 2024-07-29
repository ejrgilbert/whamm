use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/your_module.wasm")?;
    let mut linker = Linker::new(&engine);

    // Set up WASI with custom configuration
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio() // Inherit standard input/output
        .build();
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;

    // Call the exported functions
    let print_info = instance.get_typed_func::<(i32, i32), ()>(&mut store, "print_info")?;
    print_info.call(&mut store, (1, 42))?;

    Ok(())
}