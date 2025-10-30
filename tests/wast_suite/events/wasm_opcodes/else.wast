;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat

;; @instrument
(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )
    (func $get_global_var1 (result i32)
        (global.get $var1)
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
            i32.const 0
            if
                (global.set $var1 (i32.const 10))
            else
                (global.set $var1 (i32.const 5))
            end
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
    (export "get_global_var1" (func $get_global_var1))
    (start $start) ;; run the first function automatically
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> var count: i32; wasm:opcode:else:before { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))
(assert_return (invoke "get_count") (i32.const 2))

;; target a specific `if` using `fn_id`/`fname`/`opidx`
;; WHAMM --> var count: i32; wasm:opcode:else:before /fid == 2 && opidx == 3/ { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))
(assert_return (invoke "get_count") (i32.const 1)) ;; if is true in this func
;; WHAMM --> var count: i32; wasm:opcode:else:before /fid == 3 && opidx == 3/ { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))
(assert_return (invoke "get_count") (i32.const 0)) ;; if not true in this func

;; entry mode
;; WHAMM --> var count: i32; wasm:opcode:else:entry { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 4)) ;; the if is only true 2 of the 3 times
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))

;; exit mode
;; WHAMM --> var count: i32; wasm:opcode:else:exit { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 4)) ;; the if is only true 2 of the 3 times
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))

;; after mode
;; WHAMM --> var count: i32; wasm:opcode:else:after { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 8)) ;; 4 else's
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 5))

;; alt mode
;; WHAMM --> var count: i32; wasm:opcode:else:alt /fid == 3 && opidx == 8/ { count = count + 2; }
(assert_return (invoke "get_count") (i32.const 0)) ;; never entered!
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_global_var1") (i32.const 0)) ;; never entered!

;; WHAMM --> wasm:opcode:if:alt /fid == 3 && opidx == 5/ { drop_args(); }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 1))
(assert_return (invoke "get_global_var1") (i32.const 0)) ;; never entered!
