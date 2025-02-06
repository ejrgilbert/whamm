# Whamm Demo #

These demo instructions assume that the `whamm` command has been added to your `PATH`.

This can be done via:
```shell
cargo build
alias whamm=$(pwd)/target/debug/whamm
```

## Bytecode Rewriting Target ##

Example running the `branch-monitor` script:
```shell
# Setup
pushd whamm_core
cargo build --target wasm32-wasip1 --release
popd
WHAMM_CORE=whamm_core/target/wasm32-wasip1/release/whamm_core.wasm
cd demo/
mkdir output/

# Instrument the application
whamm instr --script branch-monitor.mm --app app.wasm --core-lib $WHAMM_CORE -o output/app-instr.wasm

# Run the monitor (must be on wizard since it depends on the engine `puts` host function)
```

## Wizard Target ##

Example running the `branch-monitor` script:
```shell
# Setup
pushd whamm_core
cargo build --target wasm32-wasip1 --release
popd
WHAMM_CORE=whamm_core/target/wasm32-wasip1/release/whamm_core.wasm
cd demo/
mkdir output/

# Compile the monitor to a Wasm module that targets the Wizard engine extension
whamm instr --script branch-monitor.mm --core-lib $WHAMM_CORE -o output/wiz-mon.wasm --wizard

# Run the monitor (must be on wizard)
wizard --env=TO_CONSOLE=true --monitors=output/wiz-mon.wasm+$WHAMM_CORE app.wasm
```