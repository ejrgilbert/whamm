# The `.wast` Test Harness #

A [`.wast` file](https://webassembly.js.org/docs/contrib-wat-vs-wast.html#:~:text=WAST%20is%20a%20superset%20of,easier%20to%20write%20by%20hand) is used for testing purposes and simplifies the writing of tests for developers.
We use `.wast` files to encode assertions that should pass when running an instrumented variation of a `wasm` module.

## Writing `.wast` Tests ##

The high level structure looks like this:
```
<module_in_wat>

;; WHAMM --> <some_oneline_whamm_script>
<whamm0_assertion0> ;; The first assertion for the first whamm script
<whamm0_assertion1> ;; The second assertion for the first whamm script

;; WHAMM --> <some_oneline_whamm_script>
<whamm1_assertion0> ;; The first assertion for the second whamm script
<whamm1_assertion1> ;; The second assertion for the second whamm script
<whamm2_assertion1> ;; The third assertion for the second whamm script
```

The module encoded in the script above would be used for all the following whamm/assertion groups.
To verify that _all assertions **fail** before instrumenting_, 5 new `.wast` files would be generated with the original module and run on the configured interpreters.
To verify that _all assertions **pass** after instrumenting_, 2 new `.wast` files would be generated, one per specified `whamm` script, including the assertions under the respective `whamm` script.

Below is an example `.wast` test:
```webassembly
;; Test `wasm:opcode:call` event

;; @instrument
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
(assert_return (invoke "instrument_me") (i32.const 1)) ;; will be run with the above WHAMM instrumentation

;; WHAMM --> wasm:opcode:call:alt { alt_call_by_name("other"); }
(assert_return (invoke "instrument_me") (i32.const 1)) ;; will be run with the above WHAMM instrumentation
```

Below is an example `.wast` test using imports:
```webassembly
(module
    (func (export "dummy") (param i32) (result i32)
        local.get 0
    )
)

(register "test")

;; @instrument
(module
    ;; Imports
    (type (;0;) (func (param i32) (result i32)))
    (import "test" "dummy" (func $dummy (type 0)))

    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    ;; Test case functions
    (func $foo
        (call $dummy (i32.const 0))
        global.set $var
    )

    (start $foo)
    (export "foo" (func $foo))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
 )
 
;; WHAMM --> i32 count; wasm:opcode:call:alt / arg0 == 0 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
```

There are several conventions to follow when writing `.wast` test cases for `whamm`.
1. Only one `module`-to-instrument per `.wast` file.
   - The test setup goes at the top (which can include multiple modules when considering testing imports).
   - The `module`-to-instrument is the final part of the setup and is marked by `;; @instrument` above the module.
2. Use comment to specify the `whamm!` script, syntax: `;; WHAMM --> <whamm_script>`
   - The scripts are run on the `module` in the `.wast` file.
   - If there are multiple `asserts` under a `whamm!` comment, they are all run against the instrumented variation of the `module` that results from that `whamm!` script.
3. All asserts should _**fail**_ if they were to run without instrumentation.

NOTE: For wizard, don't do manipulations that change arg* (that requires the frame accessor). Instead change global state for now?


## The Harness Code ##

The harness is located in `tests/common/wast_harness.rs` with the `main` function as the entrypoint.
We invoke this harness through calling the `main` entrypoint in the `run_wast_tests` test case located in `tests/integration_test.rs`.

One can read the harness code and see that it performs the following logic:
1. **Split out test components** of each `.wast` file found under `tests/wast_suite` as an individual `WastTestCase`
   - a `wasm` module
   - a `whamm` script
   - a list of assertions that _should be true_ post-instrumentation
2. **Ensure all assertions fail before instrumenting** with `whamm`.
   We do this to be able to claim that correctness of instrumentation was the sole purpose that some test passed.
   We ensure that this property holds by first creating new `.wast` files with one assertion per file and making sure that they fail when run on a supported interpreter.
   - We do this re-generation of the `.wast` files because the interpreters we use exit on the first failed assertion per `.wast`, but we want to guarantee this property _for all assertions_.
3. **Ensure all assertions pass after instrumenting** with `whamm`.
   - Run the specified `whamm` script on the module per set of assertions.
   - Output a new `.wast` file with the instrumented variation of the module with the respective assertions.

## Supported Interpreters ##

The harness generates `*.bin.wast` files to run on a list of engines, e.g. `wizeng` and the spec interpreter.

See the repo's `README.md` for how to set up the interpreters to run with our test harness.

## Some Ideas for Future Improvements ##

### Report Variables ###
```webassembly
;; Use something like below to assert on the values of some report variable dynamically.
;; REPORT_TRACE(ID) --> 1, 3, 5, 6, 7

;; Use something like below to assert on report variable values!
;; WITH_WHAMM --> (assert_return (invoke "get_report_var" (i32.const 1)) (i32.const 7))
```
