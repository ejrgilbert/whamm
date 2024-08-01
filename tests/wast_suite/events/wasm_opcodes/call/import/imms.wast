;; Test `wasm:opcode:call` event

;; Auxiliary module to import from

(module
    (func (export "dummy_five_params") (param i32 i32 i32 i32 i32) (result i32)
        local.get 0
        local.get 1
        i32.add
        local.get 2
        i32.add
        local.get 3
        i32.add
        local.get 4
        i32.add
    )
)

(register "test")

;; @instrument
(module
    ;; Imports
    (type (;0;) (func (param i32 i32 i32 i32 i32) (result i32)))
    (import "test" "dummy_five_params" (func $dummy_five_params (type 0)))

    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i32.const 4))
        global.set $var
    )

    (start $five_params)
    (export "five_params" (func $five_params))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
)

;; ---------------------------------
;; ==== IMMS, predicate, `imm0` ====
;; WHAMM --> i32 count; wasm:opcode:call:before / imm0 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; predicate is 'false'
;; WHAMM --> i32 count; wasm:opcode:call:before / imm0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; predicate is 'true'

;; ----------------------------
;; ==== IMMS, body, `imm0` ====
;; WHAMM --> i32 count; wasm:opcode:call:before { count = imm0 == 1 ? 1 : 0; }
(assert_return (invoke "get_count") (i32.const 0)) ;; condition is 'false'
;; WHAMM --> i32 count; wasm:opcode:call:before { count = imm0 == 0 ? 1 : 0; }
(assert_return (invoke "get_count") (i32.const 1)) ;; condition is 'true'
