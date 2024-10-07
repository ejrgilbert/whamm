;; Test `if` control flow

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

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i32.const 4))
    )

    (start $five_params)
    (export "five_params" (func $five_params))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
)

;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "dummy_five_params" / { if(true) { count = 3; } }
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "dummy_five_params" / { if(false) { count = 3; } }
(assert_return (invoke "get_count") (i32.const 0))

;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "dummy_five_params" / { i32 a; if(a == 0) { count = 3; } }
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "dummy_five_params" / { i32 a; if(a != 0) { count = 3; } }
(assert_return (invoke "get_count") (i32.const 0))
