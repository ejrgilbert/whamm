
(module
    (func $main (export "_start")
        (i64.store (i32.const 10) (i64.const 1))
        (i32.load (i32.const 0))
        i32.load offset=3
        drop
    )

    (memory 1)
    (data (i32.const 0) "\01\00\00\00\02\00\00\00")  ;; Store i32 1, then i32 2
)