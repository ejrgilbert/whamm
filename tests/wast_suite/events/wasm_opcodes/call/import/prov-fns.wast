;; Test `wasm:opcode:call` event

;; Auxiliary module to import from

(module
    (func (export "add_all") (param i32 i32 i32 i32 i32) (result i32)
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
    (import "test" "add_all" (func $add_all (type 0)))

    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    (func $mult_all (type 0)
;;        local.get 0 ;; ignore to avoid result being 0
        local.get 1
        local.get 2
        i32.mul
        local.get 3
        i32.mul
        local.get 4
        i32.mul
    )

    ;; Test case functions
    (func $five_params
        (call $add_all (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i32.const 4))
        global.set $var
    )

    (start $five_params)
    (export "five_params" (func $five_params))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
)

;; -------------------------------------------------
;; ==== FUNCS, with predicate, `alt_call_by_id` ====
;; WHAMM --> wasm:opcode:call(arg4: i32):alt / arg4 == 0 / { alt_call_by_id(2); }
(assert_return (invoke "get_global_var") (i32.const 24)) ;; global should be what's calculated by the new func

;; ----------------------------------------------------
;; ==== FUNCS, without predicate, `alt_call_by_id` ====
;; WHAMM --> wasm:opcode:call:alt { alt_call_by_id(2); }
(assert_return (invoke "get_global_var") (i32.const 24)) ;; global should be what's calculated by the new func

;; ---------------------------------------------------
;; ==== FUNCS, with predicate, `alt_call_by_name` ====
;; WHAMM --> wasm:opcode:call(arg4: i32):alt / arg4 == 0 / { alt_call_by_name("mult_all"); }
(assert_return (invoke "get_global_var") (i32.const 24)) ;; global should be what's calculated by the new func

;; ------------------------------------------------------
;; ==== FUNCS, without predicate, `alt_call_by_name` ====
;; WHAMM --> wasm:opcode:call:alt { alt_call_by_name("mult_all"); }
(assert_return (invoke "get_global_var") (i32.const 24)) ;; global should be what's calculated by the new func
