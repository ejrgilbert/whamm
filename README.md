<picture>
  <img width="175" alt="The logo for whamm!. Shows a spice jar with the WebAssembly logo, but with the 'h' and 'mm' letters written in between the 'wa' to spell 'whamm'."  src="/docs/logos/whamm!_logo.png">
</picture>

# whamm! #
![build](https://github.com/ejrgilbert/whamm/actions/workflows/test.yml/badge.svg)
[![book](https://img.shields.io/badge/book-WIP-4d76ae.svg)](https://ejrgilbert.github.io/whamm/intro.html)

## Debugging Wasm? Put some `whamm!` on it! ##

`whamm!` is a tool for "Wasm Application Monitoring and Manipulation"[^silent-h], a DSL inspired by DTrace's D language.

[^silent-h]: The 'h' is silent.

## Getting Started ##
Take a look at the official [`whamm!` book](https://ejrgilbert.github.io/whamm/intro.html) for how to get started with this tool.

### Build ###

```shell
# Debug build
cargo build
# Release build
cargo build --release
```

### Run ###

#### Configure whamm ####
To configure whamm:
1. Build the binary: `cargo build`
2. Add the binary to your `PATH`, located at: `target/debug/whamm`
3. Configure `WHAMM_HOME` environment variable, should point to the base directory of the cloned repository.
4. Test the setup: you should be able to run `whamm --help` and `whamm info -fv --rule "wasm:opcode:i32.load:before"` from _anywhere_

#### Instrument with Bytecode Rewriting ####
To instrument an application with a whamm script (there are example Scripts in `tests/scripts` folder):
```shell
cargo run -- instr --app <path_to_app.wasm> --script <path_to_script.mm> -o path/to/output.wasm
```

To run an instrumented application, do the following:
```shell
# Build the whamm-core library
cd whamm_core
cargo build --target wasm32-wasip1 --release
ls -al ./target/wasm32-wasip1/release/whamm_core.wasm
cd ..

# To run via the Rust crate
cd wasmtime-runner
# Should print the report data when the app is finished executing
WASM_MODULE=path/to/output.wasm cargo run

# You can also just run wasmtime directly on the CLI...whatever you choose
wasmtime run --env TO_CONSOLE=true --preload whamm_core=path/to/whamm_core.wasm path/to/output.wasm
```

#### Instrument with an Engine Monitor Module ####
To instrument an application with a whamm script (there are example Scripts in `tests/scripts` folder):
```shell
cargo run -- instr --script <path_to_script.mm> --wizard -o path/to/output.wasm
```

To run an instrumented application, do the following:
```shell
# Build the whamm-core library
cd whamm_core
cargo build --target wasm32-wasip1 --release
ls -al ./target/wasm32-wasip1/release/whamm_core.wasm
cd ..

whamm instr --script path/to/whamm/script.mm --wizard -o path/to/output.wasm
# (See above for the path to the whamm_core library)
wizeng --env=TO_CONSOLE=true --expose=wizeng --monitors=path/to/output.wasm+path/to/whamm_core.wasm path/to/app.wasm
```

#### Utilities/helpful info ####

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run -- --app <path_to_app.wasm> --script <path_to_script.mm>
```

To use the utility that provides information about match rule bound vars/functions that can be leveraged by a probe's logic/predicate:
```shell
cargo run -- info --rule "<match_rule_glob>" # e.g. "wasm:opcode:br:*"
```

### Test ###

In order to run the tests, a WebAssembly interpreter must be configured.
The supported interpreters are:
1. the Wizard engine interpreter. https://github.com/titzer/wizard-engine/tree/master
2. the Wasm reference interpreter. https://github.com/WebAssembly/spec/tree/main/interpreter

The Wizard Engine execution script must also be configured, located at: https://github.com/titzer/wizard-engine/tree/master

**How to build the [Wizard GH project]() to acquire these binaries:**
1. [Install OCaml](https://opam.ocaml.org/doc/Install.html)
2. Download [`progress`](https://github.com/titzer/progress) and ensure the `progress` binary is on your `PATH`
3. Download and build [`wizeng`](https://github.com/titzer/wizard-engine/blob/master/doc/Building.md)
   - After running `make -j`, the binaries `spectest.*` should be located at `wizard-engine/bin/spectest.*`
4. Build the Wasm reference interpreter through the Wizard repo, after running the below commands, the binary `wasm` should be located at `wizard-engine/test/wasm-spec/repos/spec/interpreter/wasm`
   ```bash
   # Configure OCaml
   opam init
   eval $(opam env)
   opam install dune
   opam install menhir

   # Build the wasm ref interpreter
   ./wizard-engine/test/wasm-spec/update.sh
   pushd wizard-engine/test/wasm-spec/repos/spec/interpreter
   make
   popd
   ```

The interpreter binaries must be runnable using the following commands (this can be done by placing symbolic links to the respective binaries):
1. For Wizard: `./output/tests/engines/wizard-spectest`
2. For Wasm-Ref: `./output/tests/engines/wasm`

The Wizard binary must be runnable using the following command (this can be done by placing symbolic links to the respective binaries):
1. For `wizeng`: `./output/tests/engines/wizeng`

To run tests:
```shell
cargo test
cargo test parser # Only run the tests for the `parser` module
cargo test -- --nocapture # With stdout tracing
```

## Available Packages ##

NOTE: There was discussion for moving the probe `mode` to the front of the match rule (e.g. `mode:provider:package:event`);
however, after thinking through this, I don't think it makes sense until I have a firmer grasp on the types of modes we will
have in this language. If there are more than before/after/alt (that are event-specific), then it would be confusing from a
language-intuition perspective. This is primarily because reading through the spec implies a movement from higher-to-lower
levels of granularity, everything being provided by what proceeds it. If we were to move `mode` to the front, but then have
event-specific options, this property would no longer hold.

Currently available:
- `wasm:opcode`

To be added:
- `thread` operation events
- `gc` operation events
- `function` enter/exit/unwind events
- `memory` access (read/write) events
- `table` access (read/write) events
- `component` operation events
- `wasm:begin`/`wasm:end` events
- `traps`
- `exception` throw/rethrow/catch events

Example:
- `wasi:http:send_req:alt`
- `wasm:opcode:call:alt`
- `wasm:fn:enter:before`

# The book #

If you are wanting to deploy the book locally, use the following commands:
```bash
# Install the mdbook cargo package
cargo install mdbook

# Start the mdbook server and open the URL
cd docs
mdbook serve --open
```

This can be useful for offline learning OR for debugging documentation while doing updates (any local changes will automatically be updated in the served book pages).
