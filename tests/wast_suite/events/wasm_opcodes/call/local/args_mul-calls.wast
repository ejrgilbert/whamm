;; Test `wasm:opcode:call` event

;; @instrument
(module
    ;; Globals
    (global $var (mut i64) (i64.const 0))

    ;; Global getters
    (func $get_global_var (result i64)
        (global.get $var)
    )

    (func $other (param i32)
        local.get 0
        drop
    )

    ;; Auxiliary definitions
    (func $dummy_five_params (param i32 i32 i32 i32 i64)
        local.get 0
        local.get 1
        i32.add
        local.get 2
        i32.add
        local.get 3
        i32.add
        i64.extend_i32_s
        local.get 4
        i64.add
        global.set $var
        (call $other (local.get 0))
    )
    (func $dummy_no_params
        ;; function with no params and no returns
        i32.const 1
        drop
    )

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i64.const 4))
        (call $dummy_no_params)
    )

    (start $five_params)
    (export "five_params" (func $five_params))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
)

;; =================================
;; ---- `CALL`: local functions ----
;; =================================

;; -------------------------------
;; ==== ARGS, predicate, arg0 ====
;; WHAMM --> var count: i32; wasm:opcode:call:before { count = 1; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
