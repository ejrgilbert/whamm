# The `wasmtime-runner` #

This runner enables us to run a module that prints to the console by running on `wasmtime`'s `wasi-preview1`.
This lets us run things that depend on this functionality (like the report variable API).

To run an instrumented module on this runtime:
- The module should be at `output/output.wasm`
- In the base of this directory, execute: `cargo run`
