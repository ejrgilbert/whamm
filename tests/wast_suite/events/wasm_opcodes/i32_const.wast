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
        block $eq
            block $neq
                local.get 0
                i32.const 1
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
;; WHAMM --> var count: i32; wasm:opcode:i32.const:before { count++; }
(assert_return (invoke "get_count") (i32.const 18))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; target a specific `block` using `fn_id`/`fname`/`opidx`
;; WHAMM --> var count: i32; wasm:opcode:i32.const:before /imm0 == 4/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:i32.const:before /imm0 == 0/ { count++; }
(assert_return (invoke "get_count") (i32.const 9))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:i32.const:before /fid == 2 && opidx == 8/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:i32.const:before /fid == 2/ { count++; }
(assert_return (invoke "get_count") (i32.const 2))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:i32.const:before /fname == "basic_br" && opidx == 8/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; after mode
;; WHAMM --> var count: i32; wasm:opcode:i32.const:after /fname == "basic_br" && opidx == 8/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; alt mode
;; WHAMM --> wasm:opcode:i32.const:alt /fname == "basic_br" && opidx == 8/ { return 1; }
(assert_return (invoke "get_global_var0") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
