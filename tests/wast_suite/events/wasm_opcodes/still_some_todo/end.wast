;; @instrument
(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    (func $cond_logic (param i32)
        local.get 0
        if
            i32.const 1
            global.get $var
            i32.add
            global.set $var
        else
            i32.const 2
            global.get $var
            i32.add
            global.set $var
        end
    )

    (func $run
        i32.const 1 ;; true
        call $cond_logic

        i32.const 0 ;; false
        call $cond_logic
    )

    (memory 1)
    (export "get_global_var" (func $get_global_var))
    (start $run) ;; run the first function automatically
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> i32 count; wasm:opcode:end:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 3))
(assert_return (invoke "get_count") (i32.const 1)) ;; only enter else 1 out of 2 times
;; WHAMM --> i32 count; wasm:opcode:end:after { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 4))

;; TODO -- target a specific `end` using `fn_id`/`pc`
