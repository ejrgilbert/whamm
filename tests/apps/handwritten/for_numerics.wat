(module
    ;; Test case functions
    (func $call_target (param i32) (result i32)
        local.get 0
    )
    (func $start (export "main")
        (call $call_target (i32.const 0))
        drop
    )
    (start $start)
)