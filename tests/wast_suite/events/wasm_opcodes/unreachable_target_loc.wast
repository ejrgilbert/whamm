;; Test `wasm:opcode:br` event

;; @instrument
(module
    ;; Globals
    (global $var (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var (result i32)
        (global.get $var)
    )

    ;; Test case functions
    (func $target_func (param i32) (result i32)
        block $eq
            block $neq
                (i32.eq (local.get 0) (i32.const 1))
                unreachable
                br_if $eq
            end
            ;; they are not equal, return '0'
            i32.const 0
            return
        end
        ;; they are equal, return '1'
        i32.const 1
        return
    )

    ;; Test case functions
    (func $basic_br (param i32) (result i32)
        block $eq
            block $neq
                (i32.eq (local.get 0) (i32.const 1))
                br_if $eq
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
        global.set $var
        (call $target_func (i32.const 1))
        global.get $var
        i32.add
        global.set $var
        (call $target_func (i32.const 1))
        global.get $var
        i32.add
        global.set $var
    )

    (export "get_global_var" (func $get_global_var))
    (memory (;0;) 1)
    (start $start)
)

;; WHAMM --> var count: i32; wasm:opcode:unreachable:alt { count++; }
;; @passes_uninstr
(assert_return (invoke "get_global_var") (i32.const 2))
(assert_return (invoke "get_count") (i32.const 3))

;; TODO with targeting (fid), see Issue#106
;;;; WHAMM --> var count: i32; wasm:opcode:unreachable:alt /fid == 1/ { count++; } wasm:opcode:unreachable:alt /fid == 2/ { count = count + 2; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 2))
;;(assert_return (invoke "get_count") (i32.const 4))

;; TODO with targeting (pc), see Issue#106
;;;; WHAMM --> var count: i32; wasm:opcode:unreachable:alt /pc == 4/ { count++; } wasm:opcode:unreachable:alt /pc == 5/ { count = count + 2; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 2))
;;(assert_return (invoke "get_count") (i32.const 4))

;; TODO with targeting (fid/pc), see Issue#106
;;;; WHAMM --> var count: i32; wasm:opcode:unreachable:alt /fid == 1 && pc == 4/ { count++; } wasm:opcode:unreachable:alt /fid == 2 && pc == 5/ { count = count + 2; }
;;;; @passes_uninstr
;;(assert_return (invoke "get_global_var") (i32.const 2))
;;(assert_return (invoke "get_count") (i32.const 4))
