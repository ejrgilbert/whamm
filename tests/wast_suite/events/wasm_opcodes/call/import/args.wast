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

;; ====================================
;; ---- `CALL`: imported functions ----
;; ====================================

;; --------------------------
;; ==== ARGS, body, arg0 ====
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):before / arg4 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):after / arg4 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):alt / arg4 == 0 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):alt / arg4 == 2 / { count = 5; return 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10)) ;; pred == false, so global should change
(assert_return (invoke "get_count") (i32.const 0))

;; ----------------------------
;; ==== ARGS, body, argLEN ====
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):before / arg0 == 4 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):after / arg0 == 4 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):alt / arg0 == 4 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):alt / arg0 == 2 / { count = 5; return 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10)) ;; pred == false, so global should be original
(assert_return (invoke "get_count") (i32.const 0))

;; ----------------------------
;; ==== ARGS, body, argMID ====
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):before / arg2 == 2 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):after / arg2 == 2 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):alt / arg2 == 2 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):alt / arg2 == 3 / { count = 5; return 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10)) ;; pred == false, so global should be original
(assert_return (invoke "get_count") (i32.const 0))

;; ------------------------------
;; ==== ARGS, body, argMID+1 ====
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):before / arg1 == 3 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):after / arg1 == 3 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):alt / arg1 == 3 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):alt / arg1 == 0 / { count = 5; return 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10)) ;; pred == false, so global should be original
(assert_return (invoke "get_count") (i32.const 0))

;; ------------------------------
;; ==== ARGS, body, argMID-1 ====
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):before / arg3 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):after / arg3 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):alt / arg3 == 1 / { count = 5; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):alt / arg3 == 2 / { count = 5; return 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10)) ;; pred == false, so global should be original
(assert_return (invoke "get_count") (i32.const 0))

;; --------------------------
;; ==== ARGS, body, arg4 ====
;; WHAMM --> wasm:opcode:call(arg4: i32):before { arg4 = 1; }
(assert_return (invoke "get_global_var") (i32.const 11))
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):before { count = arg4; }
(assert_return (invoke "get_count") (i32.const 0))
;; WHAMM --> var count: i32; wasm:opcode:call(arg4: i32):after { count = arg4; }
(assert_return (invoke "get_count") (i32.const 0))
;; WHAMM --> var count: i32; wasm:opcode:call:alt { drop_args(); count = 5; return 1; }
;; since the script doesn't use an argN in the body, must drop_args to make it validate
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 5))

;; ----------------------------
;; ==== ARGS, body, argLEN ====
;; WHAMM --> wasm:opcode:call(arg0: i32):before { arg0 = 1; }
(assert_return (invoke "get_global_var") (i32.const 7))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):before { count = arg0; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 4))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):after { count = arg0; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 4))
;; WHAMM --> var count: i32; wasm:opcode:call(arg0: i32):alt { drop_args(); count = arg0; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 4))

;; ----------------------------
;; ==== ARGS, body, argMID ====
;; WHAMM --> wasm:opcode:call(arg2: i32):before { arg2 = 1; }
(assert_return (invoke "get_global_var") (i32.const 9))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):before { count = arg2; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 2))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):after { count = arg2; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 2))
;; WHAMM --> var count: i32; wasm:opcode:call(arg2: i32):alt { count = arg2; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1)) ;; alt, so global should be return value
(assert_return (invoke "get_count") (i32.const 2))

;; ------------------------------
;; ==== ARGS, body, argMID+1 ====
;; WHAMM --> wasm:opcode:call(arg1: i32):before { arg1 = 1; }
(assert_return (invoke "get_global_var") (i32.const 8))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):before { count = arg1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):after { count = arg1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:call(arg1: i32):alt { drop_args(); count = arg1; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 3))

;; ------------------------------
;; ==== ARGS, body, argMID-1 ====
;; WHAMM --> wasm:opcode:call(arg3: i32):before { arg3 = 2; }
(assert_return (invoke "get_global_var") (i32.const 11))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):before { count = arg3; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):after { count = arg3; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 10))
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:call(arg3: i32):alt { drop_args(); count = arg3; return 1; }
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 1))
