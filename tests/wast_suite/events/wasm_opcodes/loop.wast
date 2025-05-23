;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat

;; @instrument
(module
    (type (;0;) (func))

    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (func $get_global_var0 (result i32)
        (global.get $var0)
    )

    (func $foo (type 0)
        loop ;; runs 1x (no branch)
            i32.const 1
            drop
        end
    )
    (func $bar (type 0)
        call $baz
    )
    (func $baz (type 0))

    (func $start
        (local $cnt i32)
        (local.set $cnt (i32.const 50))
        loop $l
            (global.set $var0 (i32.add (global.get $var0) (i32.const 1)))
            call $foo
            call $bar
            call $foo
            call $baz
            call $foo
            call $foo
            call $foo
            call $bar
            call $bar
            call $foo
            (local.set $cnt (i32.sub (local.get $cnt) (i32.const 1)))
            (br_if $l (local.get $cnt))
        end
    )

    (export "get_global_var0" (func $get_global_var0))
    (memory 1)
    (start $start)
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> var count: i32; wasm:opcode:loop:before { count++; }
(assert_return (invoke "get_count") (i32.const 301)) ;; (times $foo is called) + (times $start runs)
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; target a specific `block` using `fn_id`/`fname`/`pc`
;; WHAMM --> var count: i32; wasm:opcode:loop:before /fid == 1 && pc == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 300))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:loop:before /fid == 1 && pc == 1/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:loop:before /fname == "start" && pc == 2/ { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; entry mode
;; WHAMM --> var count: i32; wasm:opcode:loop:entry /fname == "start"/ { count++; }
(assert_return (invoke "get_count") (i32.const 50))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; exit mode
;; WHAMM --> var count: i32; wasm:opcode:loop:exit /fname == "start"/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check
;; WHAMM --> var count: i32; wasm:opcode:loop:exit /fname == "foo"/ { count++; }
(assert_return (invoke "get_count") (i32.const 300))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; after mode
;; WHAMM --> var count: i32; wasm:opcode:loop:after /fname == "start"/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check
;; WHAMM --> var count: i32; wasm:opcode:loop:after /fname == "foo"/ { count++; }
(assert_return (invoke "get_count") (i32.const 300))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check

;; alt mode
;; WHAMM --> var count: i32; wasm:opcode:loop:alt /fname == "foo"/ { count++; }
(assert_return (invoke "get_count") (i32.const 300))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 50)) ;; sanity check
;; WHAMM --> var count: i32; wasm:opcode:loop:alt /fname == "start"/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check

;; TODO -- `BlockType` struct? Issue #139
