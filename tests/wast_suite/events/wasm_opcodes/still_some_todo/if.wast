;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat

;; @instrument
(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    (func $check (param i32) (result i32)
        local.get 0
        if (result i32)
            i32.const 1
        else
            i32.const 0
        end
    )

    (func $start
        i32.const 1 ;; true
        call $check
        global.get $var
        i32.add
        global.set $var

        i32.const 0 ;; false
        call $check
        global.get $var
        i32.add
        global.set $var
    )

    (memory 1)
    (export "get_global_var" (func $get_global_var))
    (start $start) ;; run the first function automatically
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> i32 count; wasm:opcode:_if:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 2))
;; WHAMM --> i32 count; wasm:opcode:_if:entry { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; the if is only true 1 of the 2 times

;; -------------------------------
;; ==== arg0, predicate ====
;; -------------------------------
;; WHAMM --> i32 count; wasm:opcode:_if:before /arg0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only false 1 of the 2 times
;; WHAMM --> i32 count; wasm:opcode:_if:before /arg0 != 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only true 1 of the 2 times
;; WHAMM --> i32 count; wasm:opcode:_if:before /arg0 > 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only true 1 of the 2 times
;; WHAMM --> i32 count; wasm:opcode:_if:entry /arg0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; should never be executed! (doesn't enter the if block!)
;; WHAMM --> i32 count; wasm:opcode:_if:entry /arg0 != 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only true 1 of the 2 times
;; WHAMM --> i32 count; wasm:opcode:_if:entry /arg0 > 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only true 1 of the 2 times

;; --------------------
;; ==== arg0, body ====
;; --------------------
;; WHAMM --> wasm:opcode:_if:before { arg0 = 1; }
(assert_return (invoke "get_global_var") (i32.const 2)) ;; should now always be true!
;; WHAMM --> wasm:opcode:_if:entry { arg0 = 1; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1)) ;; arg change doesn't matter at this point!
;; WHAMM --> i32 count; wasm:opcode:_if:before { count = count + arg0; }
(assert_return (invoke "get_count") (i32.const 1))


;; TODO -- disable `alt`
;; TODO -- target a specific `if` using `fn_id`/`pc`

;; TODO -- `after` mode, find the end of the `if` and emit the body there! ...but what to do about else? maybe it goes after the else? (Issue#132)
;;         that semantically makes sense for the after of a `block`
;; TODO -- `entry`/`exit` of if?
;; TODO -- `BlockType` struct?