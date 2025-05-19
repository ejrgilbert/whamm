<picture>
  <img width="175" alt="The logo for whamm!. Shows a spice jar with the WebAssembly logo, but with the 'h' and 'mm' letters written in between the 'wa' to spell 'whamm'."  src="/docs/logos/whamm!_logo.png">
</picture>

# whamm! #
![build](https://github.com/ejrgilbert/whamm/actions/workflows/test.yml/badge.svg)
[![book](https://img.shields.io/badge/book-WIP-4d76ae.svg)](https://ejrgilbert.github.io/whamm/intro.html)

## Debugging Wasm? Put some `whamm!` on it! ##

`whamm!` is a tool for "Wasm Application Monitoring and Manipulation"[^silent-h], a DSL inspired by the D language.

[^silent-h]: The 'h' is silent.

## Getting Started ##
Take a look at the official [`whamm!` book](https://ejrgilbert.github.io/whamm/intro.html) for how to get started with this language.

### Build ###

```shell
cargo build
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

### Run ###

To run project (there are example Scripts in `tests/scripts` folder):
```shell
# Bytecode rewriting target
cargo run -- instr --app <path_to_app_wasm> --script <path_to_script> <path_for_compiled_output>
# Wizard engine target
cargo run -- instr --script <path_to_script> <path_for_compiled_output> --wizard
```

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run -- --app <path_to_app_wasm> --script <path_to_script> <path_for_compiled_output>
```

To use the utility that provides information about match rule globals/functions that can be leveraged by a probe's logic/predicate:
```shell
cargo run -- info --rule "<match_rule_glob>" # e.g. "wasm:opcode:br*"
```

To run a script that uses special var types, at the moment this is `map` and `report` variables, do the following:
```shell
cd <project to instrument>
# debug mode does not optimize the code when compiling (could be helpful when testing to avoid optimizing away event targets)
cargo build --target wasm32-wasip1
cargo run -- instr --app path/to/app.wasm --script path/to/whamm/script.mm -o path/to/output.wasm

cd wasmtime-runner
# change WASM_MODULE to: path/to/output.wasm
vi src/main.rs
# Should print the report data as the app is executing
cargo run
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
