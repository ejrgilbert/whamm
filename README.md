<picture>
  <img width="175" alt="The logo for whamm!. Shows a spice jar with the WebAssembly logo, but with the 'h' and 'mm' letters written in between the 'wa' to spell 'whamm'."  src="/docs/logos/whamm!_logo.png">
</picture>

# whamm! #
![build](https://github.com/ejrgilbert/whamm/actions/workflows/rust.yml/badge.svg)
[![book](https://img.shields.io/badge/book-WIP-4d76ae.svg)](https://ejrgilbert.github.io/whamm/intro.html)

## Debugging Wasm? Put some `whamm!` on it! ##

`whamm!` is a tool for "Wasm Application Monitoring and Manipulation"<sup>[1](#silent_h)</sup>, a DSL inspired by the D language.

## Getting Started ##
Take a look at the official [`whamm!` book](https://ejrgilbert.github.io/whamm/intro.html) for how to get started with this language.

### Build ###

To run basic build:
```shell
cargo build
```

### Test ###

In order to run the tests, a WebAssembly interpreter must be configured.
The supported interpreters are:
1. the Wizard engine interpreter. https://github.com/titzer/wizard-engine/tree/master
2. the Wasm reference interpreter. https://github.com/WebAssembly/spec/tree/main/interpreter
   - Note that the Wizard interpreter does not run on Macs (yet...), so the Wasm reference interpreter will need to be configured in this context.

**How to build the [Wizard GH project]() to acquire these binaries:**
1. [Install OCaml](https://opam.ocaml.org/doc/Install.html)
2. Download [`progress`](https://github.com/titzer/progress) and ensure the `progress` binary is on your `PATH`
3. Download and build [`wizeng`](https://github.com/titzer/wizard-engine/blob/master/doc/Building.md)
   - After running `make -j`, the binary `spectest.x86-linux` should be located at `wizard-engine/bin/spectest.x86-linux`
4. Build the Wasm reference interpreter through the Wizard repo, after running the below commands, the binary `wasm` should be located at `wizard-engine/test/wasm-spec/repos/spec/interpreter/wasm`
   ```bash
   # Configure OCaml
   opam init -y
   eval $(opam env)
   opam install -y dune
   opam install -y menhir
   
   # Build the wasm ref interpreter
   ./wizard-engine/test/wasm-spec/update.sh
   pushd wizard-engine/test/wasm-spec/repos/spec/interpreter
   make
   popd
   ```

The interpreter binaries must be runnable using the following commands (this can be done by placing symbolic links to the respective binaries):
1. For Wizard: `./output/tests/interpreters/spectest.x86-linux`
2. For Wasm-Ref: `./output/tests/interpreters/wasm`

To run tests:
```shell
cargo test
cargo test parser # Only run the tests for the `parser` module
cargo test -- --nocapture # With stdout tracing
```

### Run ###

To run project (there are example Scripts in `tests/scripts` folder):
```shell
cargo run -- instr --app <path_to_app_wasm> --script <path_to_script> <path_for_compiled_output>
```

To specify log level:
```shell
RUST_LOG={ error | warn | info | debug | trace | off } cargo run -- --app <path_to_app_wasm> --script <path_to_script> <path_for_compiled_output>
```

To visually debug the decision tree used during Wasm bytecode emission:
```shell
cargo run -- vis-script --script <path_to_script>
```

## Available Packages ##

NOTE: There was discussion for moving the probe `mode` to the front of the specification (e.g. `mode:provider:package:event`);
however, after thinking through this, I don't think it makes sense until I have a firmer grasp on the types of modes we will
have in this language. If there are more than before/after/alt (that are event-specific), then it would be confusing from a
language-intuition perspective. This is primarily because reading through the spec implies a movement from higher-to-lower
levels of granularity, everything being provided by what proceeds it. If we were to move `mode` to the front, but then have
event-specific options, this property would no longer hold.

Currently available: 
- `wasm:bytecode`

To be added:
- `thread` operation events
- `gc` operation events
- `function` enter/exit/unwind events
- `memory` access (read/write) events
- `table` access (read/write) events
- `component` operation events
- `BEGIN`/`END` events
- `traps`
- `exception` throw/rethrow/catch events

Example:
`wasi:http:send_req:alt`
`wasm:bytecode:call:alt`
`wasm:fn:enter:before`

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

<a name="silent_h">1</a>: The 'h' is silent.
