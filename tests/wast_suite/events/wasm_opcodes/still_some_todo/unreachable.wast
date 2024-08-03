;; Test `wasm:opcode:br` event

;; @instrument
(module
    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var0 (result i32)
        (global.get $var0)
    )
    (func $get_global_var1 (result i32)
        (global.get $var1)
    )

    ;; Test case functions
    (func $basic_br (param i32) (result i32)
        block $eq
            block $neq
                (i32.eq (local.get 0) (i32.const 1))
                br_if $eq
;;                br $neq
                unreachable
            end
            ;; they are not equal, return '0'
            i32.const 0
            return
        end
        ;; they are equal, return '1'
        i32.const 1
        return
    )

    (func $start
        (call $basic_br (i32.const 0))
        global.set $var0
    )

    (export "get_global_var0" (func $get_global_var0))
    (export "get_global_var1" (func $get_global_var1))
    (memory (;0;) 1)
    (start $start)
)

;; TODO -- See Issue#132
;; WHAMM --> i32 count; wasm:opcode:unreachable:alt { count++; }
(assert_return (invoke "get_count") (i32.const 1))