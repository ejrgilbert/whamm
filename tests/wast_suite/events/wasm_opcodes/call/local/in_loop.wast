;; Test `wasm:opcode:call` event
;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat
;; `wizeng --monitors=opcodes test/monitors/profile_monitor0.wasm`

;; @instrument
(module
    (type (;0;) (func))
    (func $foo (type 0)
        call $bar
    )
    (func $bar (type 0)
        call $baz
    )
    (func $baz (type 0))

    (func $start
        (local $cnt i32)
        (local.set $cnt (i32.const 50))
        loop $l
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
    (memory 1)
    (start $start)
)

;; =================================
;; ---- `CALL`: local functions ----
;; =================================

;; WHAMM --> var count: i32; wasm:opcode:call:before { count++; }
(assert_return (invoke "get_count") (i32.const 1250))