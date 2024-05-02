<picture>
  <img width="175" alt="The logo for whamm!. Shows a spice jar with the WebAssembly logo, but with the 'h' and 'mm' letters written in between the 'wa' to spell 'whamm'."  src="/docs/logos/whamm!_logo.png">
</picture>

# whamm! #
![build](https://github.com/ejrgilbert/whamm/actions/workflows/rust.yml/badge.svg)

## Debugging Wasm? Put some `whamm!` on it! ##

`whamm!` is a tool for "Wasm Application Monitoring and Manipulation"[^1], a DSL inspired by the D language.

[^1]: The 'h' is silent.

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

To run project (there are example MMScripts in `tests/mmscripts` folder):
```shell
cargo run -- --app <path_to_app_wasm> --mm <path_to_mmscript> <path_for_compiled_output>
```

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run -- --app <path_to_app_wasm> --mm <path_to_mmscript> <path_for_compiled_output>
```
