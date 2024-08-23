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

    (func $other_check (param i32) (result i32)
        local.get 0
        if (result i32)
            i32.const 1
        else
            i32.const 1
            if
                nop
            else
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
;; WHAMM --> i32 count; wasm:opcode:_else:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 2)) ;; the if is only true 1 of the 2 times

;; target a specific `if` using `fn_id`/`fname`/`pc`
;; WHAMM --> i32 count; wasm:opcode:_else:before /fid == 1 && pc == 3/ { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 1)) ;; if is true in this func
;; WHAMM --> i32 count; wasm:opcode:_else:before /fid == 2 && pc == 3/ { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 0)) ;; if not true in this func

;; entry mode
;; WHAMM --> i32 count; wasm:opcode:_else:entry { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 2)) ;; the if is only true 1 of the 2 times

;; exit mode
;; WHAMM --> i32 count; wasm:opcode:_else:exit { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 2)) ;; the if is only true 1 of the 2 times

;; after mode
;; WHAMM --> i32 count; wasm:opcode:_else:after { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 6)) ;; 3 else's

;; alt mode
;; WHAMM --> i32 count; wasm:opcode:_else:alt /fid == 2 && pc == 7/ { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 2))
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
;; TODO -- if empty alt body, remove original!
;;;; WHAMM --> wasm:opcode:_else:alt /fid == 2 && pc == 3/ {  }
;;(assert_return (invoke "get_global_var") (i32.const 2))
