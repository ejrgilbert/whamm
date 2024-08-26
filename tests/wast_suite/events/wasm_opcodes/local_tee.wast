;; Test `wasm:opcode:block` event

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
        (local i32)
        block $eq
            block $neq
                local.get 0
                i32.const 1
                local.tee 1
                i32.eq
                br_if $eq
                br $neq
            end
            i32.const 0
            return ;; they are not equal, return '0'
        end
        i32.const 1
        return ;; they are equal, return '1'
    )
    (func $more_nesting (param i32) (result i32)
        (local i32 i32)
        block $gt
            block $neq
                block $eq
                    local.get 0
                    i32.const 0
                    local.tee 1
                    i32.eq
                    br_if $eq

                    local.get 0
                    i32.const 2
                    local.tee 2
                    i32.gt_u
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
;; WHAMM --> i32 count; wasm:opcode:local_tee:before { count++; }
(assert_return (invoke "get_count") (i32.const 8))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; target a specific `block` using `fn_id`/`fname`/`pc`
;; WHAMM --> i32 count; wasm:opcode:local_tee:before /imm0 == 3/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; WHAMM --> i32 count; wasm:opcode:local_tee:before /imm0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> i32 count; wasm:opcode:local_tee:before /imm0 == 1/ { count++; }
(assert_return (invoke "get_count") (i32.const 5))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> i32 count; wasm:opcode:local_tee:before /fid == 2 && pc == 4/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> i32 count; wasm:opcode:local_tee:before /fid == 2/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> i32 count; wasm:opcode:local_tee:before /fname == "basic_br" && pc == 4/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; after mode
;; WHAMM --> i32 count; wasm:opcode:local_tee:after /fname == "basic_br" && pc == 4/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;;;;;; TODO -- what to do about assigning to static data?
;;;;;; alt mode
;;;;;; WHAMM --> wasm:opcode:local_tee:alt /fname == "more_nesting" && pc == 1/ { imm0 = 2; }
;;;;;; @passes_uninstr
;;;;(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;;;;;; @passes_uninstr
;;;;(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
