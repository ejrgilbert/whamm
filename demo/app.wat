(module
    (import "wizeng" "puts" (func $puts (param i32 i32)))
    (func $main (export "main")
        (call $call_target (i32.const 0))
    )
    (func $call_target (param i32) (result i32)
        (block
            (br_if 0 (local.get 0))
            call $foo
        )
        call $bar
    )
    (func $foo
        (call $puts (i32.const 0) (i32.const 3))
        br 0
    )
    (func $bar
        (call $puts (i32.const 3) (i32.const 3))
        br 0
    )
    (memory 1)
    (data (;0;) (i32.const 0) "foo")
    (data (;1;) (i32.const 3) "bar")
)
