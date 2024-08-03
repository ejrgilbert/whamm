;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat

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

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> i32 count; wasm:opcode:_loop:before { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; TODO -- disable `alt` on block
;; TODO -- target a specific `block` using `fn_id`/`pc`

;; TODO -- `after` mode, find the end of the `block` and emit the body there! (Issue#132)
;;         that semantically makes sense for the after of a `block`
;; TODO -- `entry`/`exit` of block?
;; TODO -- `BlockType` struct?
