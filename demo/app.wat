(module
    (import "wizeng" "puts" (func $puts (param i32 i32)))
    (func $main (export "main")
        (local i32)
        (local.set 0 (call $call_target (i32.const 0)))
        (block
            (br_if 1 (local.get 0))
        )
    )
    (func $call_target (param i32) (result i32)
        (block
            (br_if 0 (local.get 0))
            call $foo
        )
        call $bar
        (local.get 0)
    )
    (func $foo
        (call $puts (i32.const 0) (i32.const 4))
        br 0
    )
    (func $bar
        (call $puts (i32.const 4) (i32.const 4))
        br 0
    )
    (memory 1)
    (export "memory" (memory 0))
    (data (;0;) (i32.const 0) "foo\n")
    (data (;1;) (i32.const 4) "bar\n")
)
