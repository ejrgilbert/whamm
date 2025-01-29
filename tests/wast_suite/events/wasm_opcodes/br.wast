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
    (func $more_nesting (param i32) (result i32)
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

    (func $start
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
    )

    (export "get_global_var0" (func $get_global_var0))
    (export "get_global_var1" (func $get_global_var1))
    (memory (;0;) 1)
    (start $start)
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> var count: i32; wasm:opcode:br:before { count++; }
(assert_return (invoke "get_count") (i32.const 3)) ;; matches two br's (hit 3x)
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; -------------------------------
;; ==== IMMS, predicate, imm0 ====
;; WHAMM --> var count: i32; wasm:opcode:br:before / imm0 == 4 / { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; predicate == false
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; WHAMM --> var count: i32; wasm:opcode:br:before / imm0 == 1 / { count++; }
(assert_return (invoke "get_count") (i32.const 2)) ;; predicate == true (hit 3x)
;; WHAMM --> var count: i32; wasm:opcode:br:before / imm0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; predicate == true (hit 1x)
;; WHAMM --> var count: i32; wasm:opcode:br:alt / imm0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1))
(assert_return (invoke "get_global_var0") (i32.const 0))

;; --------------------------
;; ==== IMMS, body, imm0 ====
;; WHAMM --> var count: i32; wasm:opcode:br:before { count = imm0 as i32; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:br:alt { count = imm0 as i32; }
(assert_return (invoke "get_count") (i32.const 1))
(assert_return (invoke "get_global_var0") (i32.const 0))

;; after mode
;; WHAMM --> var count: i32; wasm:opcode:br:after { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; never reached (immediately following a br)

;; at_target mode
;; WHAMM --> var count: i32; wasm:opcode:br:at_target { count = imm0 as i32; }
(assert_return (invoke "get_count") (i32.const 1))
;; WHAMM --> var count: i32; wasm:opcode:br:at_target / imm0 == 0 / { count++; }
(assert_return (invoke "get_count") (i32.const 1)) ;; predicate == true (hit 1x)
