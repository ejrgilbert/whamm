
(module
    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Test case functions
    (func $basic_br (param i64) (result i32)
        block $eq               ;; 0,   1
            block $neq          ;; 1,   1
                local.get 0     ;; 2,   1
                i64.const 1     ;; 3,   1
                i64.eq          ;; 4,   1
                br_if $eq       ;; 5,   1
                br $neq         ;; 6,   1
            end                 ;; 7,   0
            i32.const 0         ;; 8,   1
            return              ;; 9,  1
        end                     ;; 10,  0
        i32.const 1             ;; 11,  0
        return                  ;; 12,  0
    )                           ;; 13,  0
    (func $more_nesting (param i32) (result i32)
        block $gt               ;; 0,   1
            block $neq          ;; 1,   1
                block $eq       ;; 2,   1
                    local.get 0 ;; 3,   1
                    i32.const 0 ;; 4,   1
                    i32.eq      ;; 5,   1
                    br_if $eq   ;; 6,   1
                    local.get 0 ;; 7,   1
                    i32.const 2 ;; 8,   1
                    i32.gt_u    ;; 9,   1
                    br_if $gt   ;; 10,  1
                    br $neq     ;; 11,  1
                end             ;; 12,  0
                i32.const 1     ;; 13,  1
                return          ;; 14,  1
            end                 ;; 15,  0
            i32.const 0         ;; 16,  1
            return              ;; 17,  1
        end                     ;; 18,  0
        i32.const 2             ;; 19,  1
        return                  ;; 20,  1
    )                           ;; 21,  0
    (func $br_table (param i32) (result i32)
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
    (func $if_stmt (param i32) (result i32)
        local.get 0
        i32.const 1
        i32.eq
        if
            i32.const 1
            return
        else
            i32.const 0
            return
        end
        i32.const 0
    )
    (func $select_stmt (param i32) (result i32)
        i32.const 1
        i32.const 0
        local.get 0
        i32.eqz
        select
    )

    (func $main (export "main")
        (call $basic_br (i64.const 0))
        global.set $var0
        (call $more_nesting (i32.const 0)) ;; eq
        global.get $var1
        i32.add
        global.set $var1
        (call $more_nesting (i32.const 1)) ;; neq
        global.get $var1
        i32.add
        global.set $var1
        (call $more_nesting (i32.const 1)) ;; neq
        global.get $var1
        i32.add
        global.set $var1
        (call $more_nesting (i32.const 3)) ;; gt 2
        global.get $var1
        i32.add
        global.set $var1
        (call $br_table (i32.const 0))
        drop
        (call $br_table (i32.const 1))
        drop
        (call $br_table (i32.const 2))
        drop
        (call $if_stmt (i32.const 0))
        drop
        (call $select_stmt (i32.const 0))
        drop
    )

    ;; hacky thing to make this work on BOTH wasmtime and Wizard
    (func $start (export "_start")
        call $main
    )
)