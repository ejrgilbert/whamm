# Instrumentation DSL #

This DSL is inspired by the dtrace D language.

## Tutorials ##

To run basic build:
```shell
cargo build
```

To run tests:
```shell
cargo test
cargo test parser # Only run the tests for the `parser` module
cargo test -- --nocapture # With stdout tracing
```

To run project (there are example dscripts in `tests/dscripts` folder):
```shell
cargo run -- --app <path_to_app_wasm> --dscript <path_to_dscript> <path_for_compiled_output>
```

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run -- --app <path_to_app_wasm> --dscript <path_to_dscript> <path_for_compiled_output>
```
