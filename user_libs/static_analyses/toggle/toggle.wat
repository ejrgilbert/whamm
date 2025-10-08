(module
    (global $toggle (mut i32) (i32.const 0))

    ;; This function just toggles between the match being `true` and `false`
    ;; on each check against it.
    (func $should_inject (export "should_inject") (param i32 i32) (result i32)
        (local $was i32)
        (local.set $was (global.get $toggle))
        (if (global.get $toggle)
            (then
                ;; next time, toggle should be false
                (global.set $toggle (i32.const 0))
            )
            (else
                ;; next time, toggle should be true
                (global.set $toggle (i32.const 1))
            )
        )

        local.get $was
    )
)