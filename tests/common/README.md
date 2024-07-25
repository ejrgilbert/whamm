# Whamm `.wast` Test Harness #

## Setup ##

## Conventions ##

1. Only one `module` per `.wast` file.
2. Use comment to specify the `whamm!` script, syntax: `;; WHAMM --> <whamm_script>`
   - The scripts are run on the `module` in the `.wast` file.
   - If there are multiple `asserts` under a `whamm!` comment, they are all run against the instrumented variation of the `module` that results from that `whamm!` script.
3. All asserts should _**fail**_ if they were to run without instrumentation.
   - This is not straightforward to ensure since running a `wast` file exits on the first failure.
4. NOTE: For wizard, don't do manipulations that change arg* (that requires the frame accessor). Instead change global state for now?

Example:
```webassembly
;; Test `wasm:opcode:call` event

(module
    ;; Auxiliary definitions
    (func $other (param i32) (result i32) (local.get 1))
    (func $dummy (param i32) (result i32) (local.get 0))

    ;; Test case functions
    (func (export "instrument_me") (result i32)
        (call $dummy (i32.const 0))
    )
)

;; WHAMM --> wasm:opcode:call:before { arg0 = 1; }
(assert_return (invoke "instrument_me") (i32.const 1))
(assert_return (invoke "instrument_me") (i32.const 1)) ;; will be run with the above WHAMM instrumentation

;; WHAMM --> wasm:opcode:call:alt { alt_call_by_name("other"); }
(assert_return (invoke "instrument_me") (i32.const 1))
(assert_return (invoke "instrument_me") (i32.const 1)) ;; will be run with the above WHAMM instrumentation
```

## The Harness Logic ##

The harness generates `*.bin.wast` files to run on a list of engines, e.g. `wizeng` and the spec interpreter.

The spec interpreter is located at `~/git/wizard-engine/test/wasm-spec/repos/spec/interpreter/wasm` on `gargantua`.
The wizard interpreter is located at `~/git/wizard-engine/bin/spectest.x86-linux` on `gargantua`.

To run:
```bash
# Generate *.bin.wast using the interpreter (also runs the test case)
~/git/wizard-engine/test/wasm-spec/repos/spec/interpreter/wasm whamm-tmp.wast -o whamm-tmp.bin.wast
# Run *.bin.wast using Wizard's interpreter
~/git/wizard-engine/bin/spectest.x86-linux whamm-tmp.bin.wast
```

This means we can just write a harness that generates a `.bin.wast` file following the conventions mentioned above and then iterate over the `vec` of configured engines to actually test that the assertions pass.
It should be _required_ for the spec interpreter to be configured, the wizard interpreter can be a bonus (since it doesn't run on Mac at the moment).
