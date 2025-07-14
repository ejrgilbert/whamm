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
            block (result i32)
                global.get $var
            end
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
;; WHAMM --> var count: i32; wasm:opcode:end:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_count") (i32.const 5))
(assert_return (invoke "get_global_var") (i32.const 3))
;; WHAMM --> var count: i32; wasm:opcode:end:after { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 6))

;; WHAMM --> var count: i32; wasm:opcode:end:before /fid == 1 && pc == 5/ { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 3))
(assert_return (invoke "get_count") (i32.const 1)) ;; only enter else 1 out of 2 times
