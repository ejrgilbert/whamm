
(module
    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Test case functions
    (func $f0 (param i64) (result i32)
        block $eq
            block $neq
                (i64.eq (local.get 0) (i64.const 1))
                br_if $eq
                br $neq
            end
            ;; they are not equal, return '0'
            i32.const 0
            return
        end
        ;; they are equal, return '1'
        i32.const 1
        return
    )
    (func $f1 (param i32) (result i32)
        block $gt
            block $neq
                block $eq
                    (i32.eq (local.get 0) (i32.const 0))
                    br_if $eq

                    (i32.gt_u (local.get 0) (i32.const 2))
                    br_if $gt

                    br $neq
                end
                ;; they are equal, return '1'
                i32.const 1
                return
            end
            ;; they are not equal, return '0'
            i32.const 0
            return
        end
        ;; param is greater than 2, return 2
        i32.const 2
        return
    )
    (func $f2 (param i32) (result i32)
        block $two
            block $one
                block $zero
                    local.get 0
                    br_table $zero $one $two $two
                end
                i32.const 0
                return
            end
            i32.const 1
            return
        end
        i32.const 2
    )
    (func $f3 (param i32) (result i32)
        (if (i32.eq (local.get 0) (i32.const 1))
            (then
                ;; they are equal, return '1'
                i32.const 1
                return
            )
            (else
                ;; they are not equal, return '0'
                i32.const 0
                return
            )
        )
        i32.const 0
    )
    (func $f4 (param i32) (result i32)
        (select (i32.const 1) (i32.const 0) (i32.eqz (local.get 0)))
    )

    (func $_start (export "_start")
        (call $f0 (i64.const 0))
        global.set $var0
        (call $f1 (i32.const 0)) ;; eq
        global.get $var1
        i32.add
        global.set $var1
        (call $f1 (i32.const 1)) ;; neq
        global.get $var1
        i32.add
        global.set $var1
        (call $f1 (i32.const 1)) ;; neq
        global.get $var1
        i32.add
        global.set $var1
        (call $f1 (i32.const 3)) ;; gt 2
        global.get $var1
        i32.add
        global.set $var1
        (call $f2 (i32.const 0))
        drop
        (call $f2 (i32.const 1))
        drop
        (call $f2 (i32.const 2))
        drop
        (call $f3 (i32.const 0))
        drop
        (call $f4 (i32.const 0))
        drop

        i32.const 1
        i32.const 2
        call_indirect (type 1)
        drop
    )

    (table (;0;) 5 5 funcref)
    (elem (;0;) (i32.const 0) func $f1 $f2 $f3 $f4)
)