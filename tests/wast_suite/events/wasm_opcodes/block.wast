;; Test `wasm:opcode:block` event

;; @instrument
(module
    ;; Globals
    (global $var0 (mut i32) (i32.const 0))
    (global $var1 (mut i32) (i32.const 0))
    (global $var2 (mut i32) (i32.const 0))

    ;; Global getters
    (func $get_global_var0 (result i32)
        (global.get $var0)
    )
    (func $get_global_var1 (result i32)
        (global.get $var1)
    )
    (func $get_global_var2 (result i32)
        global.get $var2
    )

    ;; Test case functions
    (func $basic_br (param i32) (result i32)
        block ;; this gets replaced
            (i32.add (global.get $var2) (i32.const 1))
            global.set $var2
        end
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

    (func (;5;) (param i32) (result i32)
        block (result i32)
            local.get 0
        end
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

        (call 5 (i32.const 5))
        drop
    )

    (export "get_global_var0" (func $get_global_var0))
    (export "get_global_var1" (func $get_global_var1))
    (export "get_global_var2" (func $get_global_var2))
    (memory (;0;) 1)
    (start $start)
)

;; ----------------------
;; ==== unpredicated ====
;; WHAMM --> var count: i32; wasm:opcode:block:before { count++; }
(assert_return (invoke "get_count") (i32.const 16))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var2") (i32.const 1)) ;; sanity check

;; target a specific `block` using `fn_id`/`fname`/`pc`
;; WHAMM --> var count: i32; wasm:opcode:block:before /fid == 3 && pc == 6/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var2") (i32.const 1)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:block:before /fid == 3 && pc == 8/ { count++; }
(assert_return (invoke "get_count") (i32.const 0)) ;; location DNE
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var2") (i32.const 1)) ;; sanity check

;; WHAMM --> var count: i32; wasm:opcode:block:before /fname == "more_nesting" && pc == 2/ { count++; }
(assert_return (invoke "get_count") (i32.const 4))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var2") (i32.const 1)) ;; sanity check

;; name of func 4 isn't set! so it is ""
;; WHAMM --> var count: i32; wasm:opcode:block:before /fname == "" && fid == 5/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; entry mode
;; WHAMM --> var count: i32; wasm:opcode:block:entry { count++; }
(assert_return (invoke "get_count") (i32.const 16))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; exit mode
;; WHAMM --> var count: i32; wasm:opcode:block:exit { count++; }
(assert_return (invoke "get_count") (i32.const 2))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; after mode
;; WHAMM --> var count: i32; wasm:opcode:block:after { count++; }
(assert_return (invoke "get_count") (i32.const 7))
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; alt mode
;; WHAMM --> var count: i32; wasm:opcode:block:alt /fid == 3 && pc == 6/ { count++; }
(assert_return (invoke "get_count") (i32.const 1))
(assert_return (invoke "get_global_var0") (i32.const 1))
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check

;; if empty alt body, remove original!
;; WHAMM --> wasm:opcode:block:alt /fid == 3 && pc == 0/ {}
;; @passes_uninstr
(assert_return (invoke "get_global_var0") (i32.const 0)) ;; sanity check
;; @passes_uninstr
(assert_return (invoke "get_global_var1") (i32.const 3)) ;; sanity check
(assert_return (invoke "get_global_var2") (i32.const 0)) ;; never entered!

;; TODO -- `BlockType` struct? Issue #139
