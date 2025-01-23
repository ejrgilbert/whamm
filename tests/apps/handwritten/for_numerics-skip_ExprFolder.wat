(module
    ;; Call target, parameters are used as type values:
    ;; (u8, i8, u16, i16, u32, i32, u64, i64, f32, f64)
    (func $call_target (param i32) (param i32) (param i32) (param i32) (param i32) (param i32) (param i64) (param i64) (param f32) (param f64) (result i32)
        local.get 0
    )
    (func $start (export "main")
        (call $call_target (i32.const 1) (i32.const 1) (i32.const 1) (i32.const 1) (i32.const 1) (i32.const 1) (i64.const 1) (i64.const 1) (f32.const 1) (f64.const 1))
        drop
    )
    (start $start)
)