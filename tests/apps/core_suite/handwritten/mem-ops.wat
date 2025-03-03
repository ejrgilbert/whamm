
(module
    (func $main (export "main")
        (i32.load (i32.const 0))
        i32.load offset=8
        drop
    )

    ;; hacky thing to make this work on BOTH wasmtime and Wizard
    (func $start (export "_start")
        call $main
    )

    (memory 1)
)