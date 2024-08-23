(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    (func $check (param i32) (result i32)
        local.get 0
;;        if (result i32)
;;            i32.const 1
;;        else
;;            i32.const 0
;;        end
        if (result i32)
            i32.const 1
            global.get $var1
            i32.const 2
            i32.add
            global.set $var1
        end
    )

    (func $other_check (param i32) (result i32)
        local.get 0
        if (result i32)
            i32.const 1
        else
            i32.const 1
            if
                nop
            end
            i32.const 0
        end
    )

    (func $start
        i32.const 1 ;; true
        call $check
        global.get $var
        i32.add
        global.set $var

        i32.const 0 ;; false
        call $other_check
        global.get $var
        i32.add
        global.set $var
    )

    (memory 1)
    (export "get_global_var" (func $get_global_var))
    (start $start) ;; run the first function automatically
)
