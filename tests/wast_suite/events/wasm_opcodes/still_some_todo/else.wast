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
;; WHAMM --> i32 count; wasm:opcode:_else:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_count") (i32.const 1)) ;; the if is only true 1 of the 2 times
;; WHAMM --> i32 count; wasm:opcode:_else:entry { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 2)) ;; the if is only true 1 of the 2 times


;; TODO -- disable `alt`
;; TODO -- target a specific `_else` using `fn_id`/`pc`

;; TODO -- `after` mode, find the end of the `_else` and emit the body there!
;;         that semantically makes sense for the after of a `block`
;; TODO -- `entry`/`exit` of _else?
