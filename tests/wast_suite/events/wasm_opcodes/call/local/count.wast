;; Test `wasm:opcode:call` event
;; modified from https://github.com/titzer/wizard-engine/blob/master/test/monitors/profile_monitor0.wat
;; `wizeng --monitors=opcodes test/monitors/profile_monitor0.wasm`

;; TODO: the behavior here is slightly differnt from wizard
;; since the reference interpreter starts executiing this module from
;; the `_start` function, specified by `(start $start)`
;; However, wizard will always start from the `main` function, and will run
;; the `start` function specified by `(start $start)`
;; So wizard results in 2051 calls instead of 1250 calls

;; @instrument
(module
  (type (;0;) (func))
  (func $start (export "_start")
    (local $cnt i32)
    (local.set $cnt (i32.const 50))
    (loop $l
      call $foo
      call $bar
      call $foo
      call $baz
      call $foo
      call $foo
      call $foo
      call $bar
      call $bar
      call $foo
        (local.set $cnt (i32.sub (local.get $cnt) (i32.const 1)))
	(br_if $l (local.get $cnt))
    )
  )
  (func $foo (type 0)
    call $bar)
  (func $bar (type 0)
    call $baz)
  (func $baz (type 0))
  (memory 1)
  (start $start)
)

;; =================================
;; ---- `CALL`: local functions ----
;; =================================

;; -------------------------------
;; ==== ARGS, predicate, arg0 ====
;; WHAMM --> i32 count; wasm:opcode:call:before { count++; }
(assert_return (invoke "get_count") (i32.const 1250))
