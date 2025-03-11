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
        nop
    )

    (func $other_check (param i32) (result i32)
        local.get 0
        if (result i32)
            i32.const 1
        else
            i32.const 1
            if
                nop
            end
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
        call $other_check
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
;; WHAMM --> var count: i32; wasm:opcode:_if:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:_if:entry { count++; }
(assert_return (invoke "get_count") (i32.const 2)) ;; the if is only true 2 of the 3 times

;; -------------------------------
;; ==== arg0, predicate ====
;; -------------------------------
;; WHAMM --> var count: i32; wasm:opcode:_if:before /arg0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; only false 1 of the 3 times
;; WHAMM --> var count: i32; wasm:opcode:_if:before /arg0 != 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 2)) ;; only true 2 of the 3 times
;; WHAMM --> var count: i32; wasm:opcode:_if:before /arg0 > 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 2)) ;; only true 2 of the 3 times
;; WHAMM --> var count: i32; wasm:opcode:_if:entry /arg0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; should never be executed! (doesn't enter the if block!)
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /arg0 != 0/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 2)) ;; only true 2 of the 3 times
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /arg0 > 0/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 2)) ;; only true 2 of the 3 times

;;;; --------------------
;;;; ==== arg0, body ====
;;;; --------------------
;;;; WHAMM --> wasm:opcode:_if:before { arg0 = 1; }
;;(assert_return (invoke "get_global_var") (i32.const 2)) ;; should now always be true!
;;;; WHAMM --> wasm:opcode:_if:entry { arg0 = 1; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; arg change doesn't matter at this point!
;;;; WHAMM --> var count: i32; wasm:opcode:_if:before { count = count + arg0; }
;;(assert_return (invoke "get_count") (i32.const 2))
;;
;;;; target a specific `if` using `fn_id`/`fname`/`pc`
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /fid == 1 && pc == 1/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /fid == 1 && pc == 0/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 0)) ;; loc DNE
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /fid == 1 && pc == 2/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 0)) ;; loc DNE
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;;; WHAMM --> var count: i32; wasm:opcode:_if:entry /fname == "check" && pc == 1/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;
;;;; exit mode
;;;; WHAMM --> var count: i32; wasm:opcode:_if:exit /fid == 1 && pc == 1/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;;; WHAMM --> var count: i32; wasm:opcode:_if:exit /fid == 2 && pc == 1/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 0))
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;
;;;; after mode
;;;; WHAMM --> var count: i32; wasm:opcode:_if:after /fid == 1 && pc == 1/ { count++; }
;;(assert_return (invoke "get_count") (i32.const 1))
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 1)) ;; sanity check
;;
;;;; alt mode
;;;; WHAMM --> wasm:opcode:_if:alt /fid == 1 && pc == 1/ { drop_args(); return 0; }
;;(assert_return (invoke "get_global_var") (i32.const 0))
;;
;;;; WHAMM --> wasm:opcode:_if:alt /fid == 1 && pc == 1/ { drop_args(); } wasm:opcode:nop:before /fid == 1 && pc == 6/ { return 2; }
;;(assert_return (invoke "get_global_var") (i32.const 2))
;;
;;;; TODO -- `BlockType` struct? Issue #139