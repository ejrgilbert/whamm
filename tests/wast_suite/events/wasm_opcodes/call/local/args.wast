;; Test `wasm:opcode:call` event

;; @instrument
(module
    ;; Globals
    (global $var (mut i64) (i64.const 0))

    ;; Global getters
    (func $get_global_var (result i64)
        (global.get $var)
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
    )

    ;; Test case functions
    (func $five_params
        (call $dummy_five_params (i32.const 0) (i32.const 1) (i32.const 2) (i32.const 3) (i64.const 4))
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
;; WHAMM --> var count: i32; wasm:opcode:call:before / arg0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:after / arg0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg0 == 0 / { count = 5; }
(assert_return (invoke "get_global_var") (i64.const 0)) ;; alt, so global should not change
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg0 == 2 / { count = 5; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10)) ;; pred == false, so global should change
(assert_return (invoke "get_count") (i32.const 0))

;; TODO -- uncomment after we support multiple numeric types (https://github.com/ejrgilbert/whamm/issues/141)
;;;; ---------------------------------
;;;; ==== ARGS, predicate, argLEN ====
;;;; WHAMM --> var count: i32; wasm:opcode:call:before / arg4 == 4 / { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; WHAMM --> var count: i32; wasm:opcode:call:after / arg4 == 4 / { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg4 == 4 / { count = 5; }
;;(assert_return (invoke "get_global_var") (i64.const 0)) ;; alt, so global should not change
;;(assert_return (invoke "get_count") (i32.const 5))
;;;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg4 == 2 / { count = 5; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i64.const 10)) ;; pred == false, so global should change
;;(assert_return (invoke "get_count") (i32.const 0))

;; ---------------------------------
;; ==== ARGS, predicate, argMID ====
;; WHAMM --> var count: i32; wasm:opcode:call:before / arg2 == 2 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:after / arg2 == 2 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg2 == 2 / { count = 5; }
(assert_return (invoke "get_global_var") (i64.const 0)) ;; alt, so global should not change
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg2 == 3 / { count = 5; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10)) ;; pred == false, so global should change
(assert_return (invoke "get_count") (i32.const 0))

;; -----------------------------------
;; ==== ARGS, predicate, argMID+1 ====
;; WHAMM --> var count: i32; wasm:opcode:call:before / arg3 == 3 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:after / arg3 == 3 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg3 == 3 / { count = 5; }
(assert_return (invoke "get_global_var") (i64.const 0)) ;; alt, so global should not change
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg3 == 0 / { count = 5; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10)) ;; pred == false, so global should change
(assert_return (invoke "get_count") (i32.const 0))

;; -----------------------------------
;; ==== ARGS, predicate, argMID-1 ====
;; WHAMM --> var count: i32; wasm:opcode:call:before / arg1 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:after / arg1 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg1 == 1 / { count = 5; }
(assert_return (invoke "get_global_var") (i64.const 0)) ;; alt, so global should not change
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call:alt / arg1 == 2 / { count = 5; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10)) ;; pred == false, so global should change
(assert_return (invoke "get_count") (i32.const 0))

;; --------------------------
;; ==== ARGS, body, arg0 ====
;; WHAMM --> wasm:opcode:call:before { arg0 = 1; }
(assert_return (invoke "get_global_var") (i64.const 11))
;; WHAMM --> var count: i32; wasm:opcode:call:before { count = arg0; }
(assert_return (invoke "get_count") (i32.const 0))
;; WHAMM --> var count: i32; wasm:opcode:call:after { count = arg0; }
(assert_return (invoke "get_count") (i32.const 0))
;; WHAMM --> var count: i32; wasm:opcode:call:alt { count = 5; }
(assert_return (invoke "get_count") (i32.const 5))

;; TODO -- uncomment after we support multiple numeric types (https://github.com/ejrgilbert/whamm/issues/141)
;;;; ----------------------------
;;;; ==== ARGS, body, argLEN ====
;;;; WHAMM --> wasm:opcode:call:before { arg4 = 1; }
;;(assert_return (invoke "get_global_var") (i64.const 7))
;;;; WHAMM --> var count: i32; wasm:opcode:call:before { count = arg4; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i64.const 10))
;;(assert_return (invoke "get_count") (i32.const 4))
;;;; WHAMM --> var count: i32; wasm:opcode:call:after { count = arg4; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i64.const 10))
;;(assert_return (invoke "get_count") (i32.const 4))
;;;; WHAMM --> var count: i32; wasm:opcode:call:alt { count = arg4; }
;;(assert_return (invoke "get_global_var") (i64.const 0))
;;(assert_return (invoke "get_count") (i32.const 4))

;; ----------------------------
;; ==== ARGS, body, argMID ====
;; WHAMM --> wasm:opcode:call:before { arg2 = 1; }
(assert_return (invoke "get_global_var") (i64.const 9))
;; WHAMM --> var count: i32; wasm:opcode:call:before { count = arg2; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 2))
;; WHAMM --> var count: i32; wasm:opcode:call:after { count = arg2; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 2))
;; WHAMM --> var count: i32; wasm:opcode:call:alt { count = arg2; }
(assert_return (invoke "get_global_var") (i64.const 0))
(assert_return (invoke "get_count") (i32.const 2))

;; ------------------------------
;; ==== ARGS, body, argMID+1 ====
;; WHAMM --> wasm:opcode:call:before { arg3 = 1; }
(assert_return (invoke "get_global_var") (i64.const 8))
;; WHAMM --> var count: i32; wasm:opcode:call:before { count = arg3; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:call:after { count = arg3; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:call:alt { count = arg3; }
(assert_return (invoke "get_global_var") (i64.const 0))
(assert_return (invoke "get_count") (i32.const 3))

;; ------------------------------
;; ==== ARGS, body, argMID-1 ====
;; WHAMM --> wasm:opcode:call:before { arg1 = 2; }
(assert_return (invoke "get_global_var") (i64.const 11))
;; WHAMM --> var count: i32; wasm:opcode:call:before { count = arg1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:after { count = arg1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i64.const 10))
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call:alt { count = arg1; }
(assert_return (invoke "get_global_var") (i64.const 0))
(assert_return (invoke "get_count") (i32.const 1))
