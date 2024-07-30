;; Test `wasm:opcode:call` event

;; @instrument
(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    ;; Auxiliary definitions
    (func $dummy_five_params (param i32 i32 i32 i32 i32)
        local.get 0
        local.get 1
        i32.add
        local.get 2
        i32.add
        local.get 3
        i32.add
        local.get 4
        i32.add
        global.set $var
    )
    (func $dummy_no_params
        ;; function with no params and no returns
        i32.const 1
        drop
    )

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i32.const 4))
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
;; WHAMM --> i32 count; wasm:opcode:call:before { count = 1; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
