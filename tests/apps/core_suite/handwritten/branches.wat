
(module
    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Test case functions
    ;; control: 6
    ;; const: 2
    ;; compare: 1
    ;; local: 1
    (func $basic_br (param i32) (result i32)
        block $eq               ;; 1
            block $neq          ;; 1
                (i32.eq (local.get 0) (i32.const 1))    ;; 1
                br_if $eq       ;; 1
                br $neq         ;; 1
            end                 ;; 1
            ;; they are not equal, return '0'
            i32.const 0         ;; 1
            return              ;; 1
        end                     ;; 0
        ;; they are equal, return '1'
        i32.const 1             ;; 0
        return                  ;; 0
    )
    ;; control: 29
    ;; local: 7
    ;; const: 11
    ;; compare: 7
    (func $more_nesting (param i32) (result i32)
        block $gt           ;; |
            block $neq      ;; |
                block $eq   ;; |
                    (i32.eq (local.get 0) (i32.const 0)) ;; |
                    br_if $eq   ;; |

                    (i32.gt_u (local.get 0) (i32.const 2)) ;; |
                    br_if $gt   ;; |

                    br $neq     ;; |
                end             ;; |
                ;; they are equal, return '1'
                i32.const 1     ;; |
                return          ;; |
            end                 ;; |
            ;; they are not equal, return '0'
            i32.const 0         ;; |
            return              ;; |
        end
        ;; param is greater than 2, return 2
        i32.const 2
        return
    )
    ;; control: 17
    ;; local: 3
    ;; const: 3
    (func $br_table (param i32) (result i32)
        block $two;;
            block $one;;
                block $zero;;
                    local.get 0 ;;
                    br_table $zero $one $two $two
                end ;;
                i32.const 0 ;;
                return ;;
            end ;;
            i32.const 1 ;;
            return ;;
        end ;;
        i32.const 2
    ) ;; 1
    ;; control: 3
    ;; local: 1
    ;; const: 2
    ;; compare: 1
    (func $if_stmt (param i32) (result i32)
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
    ;; control: 1
    ;; local: 1
    ;; const: 2
    ;; compare: 1
    (func $select_stmt (param i32) (result i32)
        (select (i32.const 1) (i32.const 0) (i32.eqz (local.get 0)))
    ) ;; 1

    ;; control: 10
    ;; const: 10
    ;; global: 9
    ;; arith: 4
    ;; misc: 5
    (func $main (export "main")
        (call $basic_br (i32.const 0))
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
    ;; control: 1
    (func $start (export "_start")
        call $main
    )
)