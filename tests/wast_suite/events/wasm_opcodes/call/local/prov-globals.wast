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

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i32.const 4))
    )

    (start $five_params)
    (export "five_params" (func $five_params))
    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 27)
)

;; ----------------------------------------------
;; ==== GLOBALS, predicate, `target_fn_type` ====
;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_type == "import" / { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; predicate is 'false'
;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_type == "local" / { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; predicate is 'true'

;; -------------------------------------------------
;; ==== GLOBALS, predicate, `target_imp_module` ====
;; WHAMM --> i32 count; wasm:opcode:call:before / target_imp_module == "test" / { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; predicate is 'false'

;; -----------------------------------------------
;; ==== GLOBALS, predicate, `target_fn_name` ====
;; TODO -- uncomment when we've merged in this PR: https://github.com/ejrgilbert/whamm/pull/112
;;;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "wrong" / { count++; }
;;(assert_return (invoke "get_count") (i32.const 0)) ;; predicate is 'false'
;;;; WHAMM --> i32 count; wasm:opcode:call:before / target_fn_name == "dummy_five_params" / { count++; }
;;(assert_return (invoke "get_count") (i32.const 1)) ;; predicate is 'true'

;; -----------------------------------------
;; ==== GLOBALS, body, `target_fn_type` ====
;; WHAMM --> i32 count; wasm:opcode:call:before { count = target_fn_type == "import" ? 1 : 0; }
(assert_return (invoke "get_count") (i32.const 0)) ;; condition is 'false'
;; WHAMM --> i32 count; wasm:opcode:call:before { count = target_fn_type == "local" ? 1 : 0; }
(assert_return (invoke "get_count") (i32.const 1)) ;; predicate is 'true'

;; --------------------------------------------
;; ==== GLOBALS, body, `target_imp_module` ====
;; WHAMM --> i32 count; wasm:opcode:call:before { count = target_imp_module == "import" ? 1 : 0; }
(assert_return (invoke "get_count") (i32.const 0)) ;; condition is 'false'

;; ------------------------------------------
;; ==== GLOBALS, body, `target_fn_name` ====
;; TODO -- uncomment when we've merged in this PR: https://github.com/ejrgilbert/whamm/pull/112
;;;; WHAMM --> i32 count; wasm:opcode:call:before { count = target_fn_name == "wrong" ? 1 : 0; }
;;(assert_return (invoke "get_count") (i32.const 0)) ;; condition is 'false'
;;;; WHAMM --> i32 count; wasm:opcode:call:before { count = target_fn_name == "dummy_five_params" ? 1 : 0; }
;;(assert_return (invoke "get_count") (i32.const 1)) ;; condition is 'true'
