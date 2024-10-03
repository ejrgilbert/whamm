# Whamm's Core Library #

## Maps Package ##

This provides the `maps` functionality for `whamm!` and depends on running on `wasi-preview1` to log `report` variables.
We enable running on `wasi` through the `wasmtime-runner` Rust project at the base of the `whamm!` project directory.

## To Run ##

To run:
- In the base of this project (`whamm/core_lib`), execute: `cargo build --release --target wasm32-wasip1`
- The built `wasm` binary will be located at `whamm/core_lib/target/wasm32-wasip1/release/core_lib.wasm`
- An example `whamm!` script lives at `./src/add_map.mm`

## Issues: ##

When building with `wasi-preview1`, the test `main` function gets optimized to where there are no `call` instrument-able events as targeted in the `add_map.mm` file.
> We will need to fix this by actually getting to where we can instrument by 'merging' modules, whether that be through:
>   - linking (link and import the library),
>   - merging with single memory (merge two modules with the library pointing to some memory offset), or
>   - merging with multiple memories (merge two modules with the library memory living in a second memory).
