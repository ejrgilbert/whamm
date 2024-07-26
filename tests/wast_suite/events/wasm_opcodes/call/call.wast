;; Test `wasm:opcode:call` event

(module
    ;; Auxiliary definitions
    (func $dummy (param i32) (result i32) (local.get 0))

    ;; Test case functions
    (func $instrument_me (result i32)
        (call $dummy (i32.const 0))
    )
    (export "instrument_me" (func $instrument_me))
    (memory (;0;) 1)
)

;; NOTE: For wizard, don't do manipulations that change arg* (that requires the frame accessor). Instead change global state for now?
;; WHAMM --> wasm:opcode:call:before { arg0 = 1; }

;; Use something like below to assert on the values of some report variable dynamically.
;; REPORT_TRACE(ID) --> 1, 3, 5, 6, 7

(assert_return (invoke "instrument_me") (i32.const 1))
;; Use something like below to assert on report variable values!
;; WITH_WHAMM --> (assert_return (invoke "get_report_var" (i32.const 1)) (i32.const 7))
(assert_return (invoke "instrument_me") (i32.const 1))

;; WHAMM --> wasm:opcode:call:before { arg0 = 1; }
(assert_return (invoke "instrument_me") (i32.const 1))
