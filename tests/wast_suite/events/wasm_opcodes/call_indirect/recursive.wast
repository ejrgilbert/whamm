;; Repro for https://github.com/ejrgilbert/whamm/issues/314
;; `resolved_fid` at a `call_indirect:before` probe should not cause the
;; instrumented module to trap, even when the call_indirect has arguments
;; and recurses through itself.

;; @instrument
(module
    (type $void_void (func))
    (type $i32_void (func (param i32)))
    (table 1 1 funcref)
    (elem (i32.const 0) func $f)

    (func $f (param $n i32)
        local.get $n
        i32.eqz
        if return end
        local.get $n
        i32.const 1
        i32.sub
        i32.const 0
        call_indirect (type $i32_void)
    )

    (func $main
        i32.const 5
        i32.const 0
        call_indirect (type $i32_void)
    )

    (start $main)
    (memory (;0;) 1)
)

;; --------------------------------------------------------------
;; Reading `resolved_fid` must not trap. The recursive descent
;; from $main -> $f(5) -> ... -> $f(1) -> $f(0) executes call_indirect
;; six times (1 in $main + 5 in $f), so `count` must be 6.
;; WHAMM --> var count: i32; wasm:opcode:call_indirect:before { var f: i32 = resolved_fid; count++; }
(assert_return (invoke "get_count") (i32.const 6))
