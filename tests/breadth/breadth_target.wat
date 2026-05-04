;; Breadth test target — a single hand-written wasm app whose surface area is meant
;; to grow until every event in providers/*.yaml has at least one static occurrence
;; here. The breadth.mm/breadth_wei.mm scripts attach a probe at every targetable
;; site, so this app's job is to give them somewhere to attach and to run end-to-end
;; once instrumented.
;;
;; Each section below covers one opcode family. As coverage grows, move opcodes from
;; the TODO list at the bottom into one of the existing functions (or add a new one).
;;
;; Constraints to keep in mind when extending:
;;   * The instrumented module must run to completion on both wasmtime (rewriting
;;     backend) and wizeng (wei backend). Avoid traps — no integer div by zero,
;;     trunc-of-NaN, OOB load, etc.
;;   * Each new feature family may need a feature flag added to the helper runner
;;     before the test will pass. Confirm uninstrumented support on both engines
;;     before checking a TODO off.
;;   * Some opcodes (`unreachable`, `throw`, …) cannot run end-to-end. Place those
;;     in a never-taken branch so they are statically present but skipped at runtime.
(module
  ;; ===== Type definitions =====
  (type $sig_void (func))
  (type $sig_i32_i32 (func (param i32) (result i32)))

  ;; GC types — packed i8 field exercises struct.get_{s,u} / array.get_{s,u}
  (type $packed_struct (struct (field $b (mut i8)) (field $w (mut i32))))
  (type $i32_array (array (mut i32)))
  (type $i8_array (array (mut i8)))
  (type $funcref_array (array (mut funcref)))

  ;; ===== Memory =====
  ;; Shared so atomic ops can target it. Atomic ops require explicit alignment matching
  ;; the access size; non-atomic ops elsewhere stay address-compatible.
  (memory $mem 1 1 shared)

  ;; ===== Tags (exceptions) =====
  (tag $exn (param i32))

  ;; ===== Tables =====
  (table $tbl 4 funcref)
  (elem (i32.const 0) func $callee_a $callee_b)
  (elem $passive_elem func $callee_a $callee_b)

  ;; ===== Globals =====
  (global $g_i32 (mut i32) (i32.const 0))
  (global $g_const_i32 i32 (i32.const 7))

  ;; ===== Passive data segment (bulk memory) =====
  (data $passive "hello, breadth")
  (data $arr_data "\01\02\03\04\05\06\07\08")
  (elem $arr_elem func $callee_a $callee_b)

  ;; ===== Helpers used by call / call_indirect =====
  (func $callee_a (type $sig_i32_i32)
    local.get 0
    i32.const 1
    i32.add
  )
  (func $callee_b (type $sig_i32_i32)
    local.get 0
    i32.const 2
    i32.mul
  )

  ;; ===== Control flow: block, loop, if/else, br, br_if, br_table, return, drop, select, typed_select, nop =====
  (func $ctrl_flow (param i32) (result i32) (local $sum i32) (local $i i32)
    nop

    ;; if/else with an early return path
    local.get 0
    i32.const 0
    i32.eq
    if (result i32)
      i32.const 100
    else
      i32.const 200
    end
    local.set $sum

    ;; block + br_if exit
    block $done
      local.get $sum
      i32.const 100
      i32.eq
      br_if $done
      ;; never taken when arg == 0
      i32.const 999
      local.set $sum
    end

    ;; loop counting from 0..3
    i32.const 0
    local.set $i
    block $exit
      loop $continue
        local.get $i
        i32.const 3
        i32.ge_s
        br_if $exit
        local.get $sum
        i32.const 1
        i32.add
        local.set $sum
        local.get $i
        i32.const 1
        i32.add
        local.set $i
        br $continue
      end
    end

    ;; br_table dispatch
    block $b_two
      block $b_one
        block $b_zero
          local.get 0
          br_table $b_zero $b_one $b_two
        end
        local.get $sum
        i32.const 10
        i32.add
        local.set $sum
        br $b_two
      end
      local.get $sum
      i32.const 20
      i32.add
      local.set $sum
    end

    ;; select + typed_select
    local.get $sum
    i32.const 0
    local.get 0
    i32.eqz
    select
    drop

    local.get $sum
    i32.const 0
    local.get 0
    i32.eqz
    select (result i32)
    drop

    local.get $sum
    return
  )

  ;; ===== call, call_indirect =====
  (func $calls (result i32)
    i32.const 5
    call $callee_a       ;; -> 6

    i32.const 3
    i32.const 1          ;; table index for $callee_b
    call_indirect (type $sig_i32_i32)  ;; -> 6

    i32.add              ;; 12
  )

  ;; ===== local.get/set/tee, global.get/set =====
  (func $locals_globals (result i32) (local $x i32)
    i32.const 41
    local.set $x
    local.get $x
    i32.const 1
    i32.add
    local.tee $x         ;; $x = 42, value also left on stack
    global.set $g_i32

    global.get $g_i32
    global.get $g_const_i32
    i32.add              ;; 42 + 7 = 49
  )

  ;; ===== Memory load/store full set + memory.grow =====
  (func $memory_full (result i32)
    ;; i64.store / i64.load at addr 32
    i32.const 32
    i64.const 0x0123456789ABCDEF
    i64.store
    i32.const 32
    i64.load
    drop

    ;; narrow i32 stores
    i32.const 40
    i32.const 0x7F
    i32.store8
    i32.const 41
    i32.const 0x1234
    i32.store16

    ;; narrow i32 loads (signed/unsigned)
    i32.const 40
    i32.load8_s
    drop
    i32.const 40
    i32.load8_u
    drop
    i32.const 41
    i32.load16_s
    drop
    i32.const 41
    i32.load16_u
    drop

    ;; narrow i64 stores
    i32.const 48
    i64.const 0x7F
    i64.store8
    i32.const 49
    i64.const 0x1234
    i64.store16
    i32.const 51
    i64.const 0x12345678
    i64.store32

    ;; narrow i64 loads (signed/unsigned)
    i32.const 48
    i64.load8_s
    drop
    i32.const 48
    i64.load8_u
    drop
    i32.const 49
    i64.load16_s
    drop
    i32.const 49
    i64.load16_u
    drop
    i32.const 51
    i64.load32_s
    drop
    i32.const 51
    i64.load32_u
    drop

    ;; memory.grow by 0 pages — defined to return current size and never traps
    i32.const 0
    memory.grow
    drop

    i32.const 0
  )

  ;; ===== Bulk memory =====
  (func $bulk_memory (result i32)
    ;; memory.init from $passive at memory[64..78]
    i32.const 64       ;; dest
    i32.const 0        ;; src offset within $passive
    i32.const 14       ;; length ("hello, breadth")
    memory.init $passive

    ;; memory.copy memory[64..78] -> memory[80..94]
    i32.const 80
    i32.const 64
    i32.const 14
    memory.copy

    ;; memory.fill memory[96..112] = 0xAB
    i32.const 96
    i32.const 0xAB
    i32.const 16
    memory.fill

    ;; data.drop releases the passive segment
    data.drop $passive

    i32.const 0
  )

  ;; ===== Table ops =====
  (func $table_ops (result i32)
    table.size $tbl
    drop

    ;; table.set: put $callee_a into slot 2
    i32.const 2
    ref.func $callee_a
    table.set $tbl

    ;; table.get: read slot 2
    i32.const 2
    table.get $tbl
    drop

    ;; table.fill: 1 entry at slot 3 = ref.func $callee_b
    i32.const 3
    ref.func $callee_b
    i32.const 1
    table.fill $tbl

    ;; table.copy within $tbl: copy 1 entry from slot 0 to slot 2
    i32.const 2
    i32.const 0
    i32.const 1
    table.copy $tbl $tbl

    ;; table.init from $passive_elem: 1 entry from segment[0] -> slot 2
    i32.const 2
    i32.const 0
    i32.const 1
    table.init $tbl $passive_elem

    ;; elem.drop releases the passive elem segment
    elem.drop $passive_elem

    ;; table.grow by 0 elems — never traps and is always defined
    ref.func $callee_a
    i32.const 0
    table.grow $tbl
    drop

    i32.const 0
  )

  ;; ===== Reference + funcref ops =====
  (func $ref_ops (result i32) (local $f (ref null $sig_i32_i32))
    ;; ref.null + ref.is_null
    ref.null func
    ref.is_null
    drop

    ;; ref.func, ref.as_non_null, call_ref
    ref.func $callee_a
    local.set $f
    i32.const 5
    local.get $f
    ref.as_non_null
    call_ref $sig_i32_i32
    drop

    ;; br_on_null — never branches because $f is non-null
    block $on_null
      local.get $f
      br_on_null $on_null
      drop
    end

    ;; br_on_non_null — branches with the unwrapped ref because $f is non-null
    block $on_non_null (result (ref $sig_i32_i32))
      local.get $f
      br_on_non_null $on_non_null
      ;; null path: still has to satisfy block result type
      ref.func $callee_b
    end
    drop

    ;; ref.eq on two null eqrefs (always true)
    ref.null eq
    ref.null eq
    ref.eq
    drop

    i32.const 0
  )

  ;; ===== Tail calls =====
  (func $tail_call (param i32) (result i32)
    local.get 0
    return_call $callee_a
  )

  (func $tail_call_indirect (param i32) (result i32)
    local.get 0
    i32.const 1                 ;; table index for $callee_b
    return_call_indirect (type $sig_i32_i32)
  )

  (func $tail_call_ref (param i32) (result i32)
    local.get 0
    ref.func $callee_a
    return_call_ref $sig_i32_i32
  )

  ;; ===== GC: structs, arrays, casts, i31, extern bridge =====
  (func $gc_ops (result i32)
    (local $s (ref $packed_struct))
    (local $a (ref $i32_array))
    (local $pa (ref $i8_array))
    (local $fa (ref $funcref_array))

    ;; struct.new with explicit values
    i32.const 0x7F
    i32.const 42
    struct.new $packed_struct
    local.set $s

    ;; struct.new_default
    struct.new_default $packed_struct
    drop

    ;; struct.get_s / struct.get_u on the packed i8 field
    local.get $s
    struct.get_s $packed_struct $b
    drop
    local.get $s
    struct.get_u $packed_struct $b
    drop

    ;; struct.get on the regular i32 field
    local.get $s
    struct.get $packed_struct $w
    drop

    ;; struct.set
    local.get $s
    i32.const 99
    struct.set $packed_struct $w

    ;; array.new — 4 elems initialized to 7
    i32.const 7
    i32.const 4
    array.new $i32_array
    local.set $a

    ;; array.new_default
    i32.const 4
    array.new_default $i32_array
    drop

    ;; array.new_fixed (2 elems: 1, 2)
    i32.const 1
    i32.const 2
    array.new_fixed $i32_array 2
    drop

    ;; array.new_data — 8 packed bytes from $arr_data
    i32.const 0
    i32.const 8
    array.new_data $i8_array $arr_data
    local.set $pa

    ;; array.new_elem — 2 funcrefs from $arr_elem
    i32.const 0
    i32.const 2
    array.new_elem $funcref_array $arr_elem
    local.set $fa

    ;; array.get / array.get_s / array.get_u
    local.get $a
    i32.const 0
    array.get $i32_array
    drop
    local.get $pa
    i32.const 0
    array.get_s $i8_array
    drop
    local.get $pa
    i32.const 0
    array.get_u $i8_array
    drop

    ;; array.set
    local.get $a
    i32.const 0
    i32.const 99
    array.set $i32_array

    ;; array.len
    local.get $a
    array.len
    drop

    ;; array.fill
    local.get $a
    i32.const 0
    i32.const 0
    i32.const 4
    array.fill $i32_array

    ;; array.copy from $a to $a
    local.get $a
    i32.const 0
    local.get $a
    i32.const 0
    i32.const 4
    array.copy $i32_array $i32_array

    ;; array.init_data
    local.get $pa
    i32.const 0
    i32.const 0
    i32.const 8
    array.init_data $i8_array $arr_data

    ;; array.init_elem
    local.get $fa
    i32.const 0
    i32.const 0
    i32.const 2
    array.init_elem $funcref_array $arr_elem

    ;; ref.test / ref.test_null (ref.test with nullable target)
    local.get $s
    ref.test (ref $packed_struct)
    drop
    local.get $s
    ref.test (ref null $packed_struct)
    drop

    ;; ref.cast / ref.cast_null
    local.get $s
    ref.cast (ref $packed_struct)
    drop
    local.get $s
    ref.cast (ref null $packed_struct)
    drop

    ;; br_on_cast — cast succeeds, branches with the dst ref. On the (unreachable
    ;; in practice) fall-through, the original src ref is left on the stack which
    ;; matches the block's result type.
    block $bc_ok (result (ref $packed_struct))
      local.get $s
      br_on_cast $bc_ok (ref $packed_struct) (ref $packed_struct)
    end
    drop

    ;; br_on_cast_fail — cast succeeds, falls through with the dst ref already on stack.
    block $bcf_fail (result (ref $packed_struct))
      local.get $s
      br_on_cast_fail $bcf_fail (ref $packed_struct) (ref $packed_struct)
    end
    drop

    ;; i31 trio
    i32.const 7
    ref.i31
    drop
    i32.const 7
    ref.i31
    i31.get_s
    drop
    i32.const 7
    ref.i31
    i31.get_u
    drop

    ;; extern <-> any conversions
    local.get $s
    extern.convert_any
    any.convert_extern
    drop

    i32.const 0
  )

  ;; ===== Atomic ops (threads / shared memory required on wasmtime) =====
  ;; Addresses are kept aligned to the access size; the slot at addr 128 is 8-byte
  ;; aligned, addr 144 is also 8-aligned.
  (func $atomic_ops (result i32)
    ;; atomic.fence — no operands
    atomic.fence

    ;; ----- i32 plain atomic load/store (4-byte align) -----
    i32.const 128
    i32.const 1
    i32.atomic.store
    i32.const 128
    i32.atomic.load
    drop

    i32.const 128
    i32.const 1
    i32.atomic.store8
    i32.const 128
    i32.atomic.load8_u
    drop

    i32.const 128
    i32.const 1
    i32.atomic.store16
    i32.const 128
    i32.atomic.load16_u
    drop

    ;; ----- i32 rmw — full-width -----
    i32.const 128
    i32.const 1
    i32.atomic.rmw.add
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw.sub
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw.and
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw.or
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw.xor
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw.xchg
    drop
    i32.const 128
    i32.const 0       ;; expected
    i32.const 1       ;; replacement
    i32.atomic.rmw.cmpxchg
    drop

    ;; ----- i32 rmw8_u -----
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.add_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.sub_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.and_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.or_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.xor_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw8.xchg_u
    drop
    i32.const 128
    i32.const 0
    i32.const 1
    i32.atomic.rmw8.cmpxchg_u
    drop

    ;; ----- i32 rmw16_u -----
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.add_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.sub_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.and_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.or_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.xor_u
    drop
    i32.const 128
    i32.const 1
    i32.atomic.rmw16.xchg_u
    drop
    i32.const 128
    i32.const 0
    i32.const 1
    i32.atomic.rmw16.cmpxchg_u
    drop

    ;; ----- i64 plain atomic load/store -----
    i32.const 144
    i64.const 1
    i64.atomic.store
    i32.const 144
    i64.atomic.load
    drop

    i32.const 144
    i64.const 1
    i64.atomic.store8
    i32.const 144
    i64.atomic.load8_u
    drop

    i32.const 144
    i64.const 1
    i64.atomic.store16
    i32.const 144
    i64.atomic.load16_u
    drop

    i32.const 144
    i64.const 1
    i64.atomic.store32
    i32.const 144
    i64.atomic.load32_u
    drop

    ;; ----- i64 rmw full-width -----
    i32.const 144
    i64.const 1
    i64.atomic.rmw.add
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw.sub
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw.and
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw.or
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw.xor
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw.xchg
    drop
    i32.const 144
    i64.const 0
    i64.const 1
    i64.atomic.rmw.cmpxchg
    drop

    ;; ----- i64 rmw8_u -----
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.add_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.sub_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.and_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.or_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.xor_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw8.xchg_u
    drop
    i32.const 144
    i64.const 0
    i64.const 1
    i64.atomic.rmw8.cmpxchg_u
    drop

    ;; ----- i64 rmw16_u -----
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.add_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.sub_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.and_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.or_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.xor_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw16.xchg_u
    drop
    i32.const 144
    i64.const 0
    i64.const 1
    i64.atomic.rmw16.cmpxchg_u
    drop

    ;; ----- i64 rmw32_u -----
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.add_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.sub_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.and_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.or_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.xor_u
    drop
    i32.const 144
    i64.const 1
    i64.atomic.rmw32.xchg_u
    drop
    i32.const 144
    i64.const 0
    i64.const 1
    i64.atomic.rmw32.cmpxchg_u
    drop

    ;; ----- wait/notify -----
    ;; wait32 with mismatched expected — returns 1 (NOT_EQUAL) immediately
    i32.const 128
    i32.const 0xFFFFFFFF      ;; expected (mismatches current value, so doesn't block)
    i64.const 0               ;; timeout
    memory.atomic.wait32
    drop
    ;; wait64 likewise
    i32.const 144
    i64.const 0xFFFFFFFFFFFFFFFF
    i64.const 0
    memory.atomic.wait64
    drop
    ;; notify with count 0
    i32.const 128
    i32.const 0
    memory.atomic.notify
    drop

    i32.const 0
  )

  ;; ===== Exceptions: try_table, throw, throw_ref =====
  (func $exception_ops (result i32)
    block $caught (result i32)
      try_table (result i32) (catch $exn $caught)
        ;; never thrown — branch is statically present though
        i32.const 0
        if
          i32.const 7
          throw $exn
        end

        ;; throw_ref also lives behind a never-taken guard
        i32.const 0
        if
          ref.null exn
          throw_ref
        end

        i32.const 0
      end
    end
  )

  ;; ===== unreachable (statically present, never taken) =====
  (func $never_taken (param i32) (result i32)
    local.get 0
    if (result i32)
      unreachable
    else
      i32.const 0
    end
  )

  ;; ===== Numeric i32 long tail =====
  (func $i32_long_tail (result i32) (local $a i32) (local $b i32) (local $r i32)
    i32.const 12
    local.set $a
    i32.const 5
    local.set $b

    ;; arithmetic that wasn't already covered by callee_a / callee_b
    local.get $a
    local.get $b
    i32.sub
    drop

    ;; div / rem (positive operands keep us out of overflow + div-by-zero traps)
    local.get $a
    local.get $b
    i32.div_s
    drop
    local.get $a
    local.get $b
    i32.div_u
    drop
    local.get $a
    local.get $b
    i32.rem_s
    drop
    local.get $a
    local.get $b
    i32.rem_u
    drop

    ;; bitwise
    local.get $a
    local.get $b
    i32.and
    drop
    local.get $a
    local.get $b
    i32.or
    drop
    local.get $a
    local.get $b
    i32.xor
    drop

    ;; shifts and rotates
    local.get $a
    i32.const 1
    i32.shl
    drop
    local.get $a
    i32.const 1
    i32.shr_s
    drop
    local.get $a
    i32.const 1
    i32.shr_u
    drop
    local.get $a
    i32.const 1
    i32.rotl
    drop
    local.get $a
    i32.const 1
    i32.rotr
    drop

    ;; unary bitcount
    local.get $a
    i32.clz
    drop
    local.get $a
    i32.ctz
    drop
    local.get $a
    i32.popcnt
    drop

    ;; comparisons (eq/eqz/ge_s already exercised in $ctrl_flow)
    local.get $a
    local.get $b
    i32.ne
    drop
    local.get $a
    local.get $b
    i32.lt_s
    drop
    local.get $a
    local.get $b
    i32.lt_u
    drop
    local.get $a
    local.get $b
    i32.gt_s
    drop
    local.get $a
    local.get $b
    i32.gt_u
    drop
    local.get $a
    local.get $b
    i32.le_s
    drop
    local.get $a
    local.get $b
    i32.le_u
    drop
    local.get $a
    local.get $b
    i32.ge_u
    drop

    ;; sign-extension
    i32.const 0xFF
    i32.extend8_s
    drop
    i32.const 0xFFFF
    i32.extend16_s
    drop

    local.get $a
  )

  ;; ===== Numeric i64 full set =====
  (func $i64_ops (result i32) (local $a i64) (local $b i64)
    i64.const 100
    local.set $a
    i64.const 7
    local.set $b

    ;; eqz
    local.get $a
    i64.eqz
    drop

    ;; arithmetic
    local.get $a
    local.get $b
    i64.add
    drop
    local.get $a
    local.get $b
    i64.sub
    drop
    local.get $a
    local.get $b
    i64.mul
    drop
    local.get $a
    local.get $b
    i64.div_s
    drop
    local.get $a
    local.get $b
    i64.div_u
    drop
    local.get $a
    local.get $b
    i64.rem_s
    drop
    local.get $a
    local.get $b
    i64.rem_u
    drop

    ;; bitwise + shifts/rotates
    local.get $a
    local.get $b
    i64.and
    drop
    local.get $a
    local.get $b
    i64.or
    drop
    local.get $a
    local.get $b
    i64.xor
    drop
    local.get $a
    i64.const 1
    i64.shl
    drop
    local.get $a
    i64.const 1
    i64.shr_s
    drop
    local.get $a
    i64.const 1
    i64.shr_u
    drop
    local.get $a
    i64.const 1
    i64.rotl
    drop
    local.get $a
    i64.const 1
    i64.rotr
    drop

    ;; unary bitcount
    local.get $a
    i64.clz
    drop
    local.get $a
    i64.ctz
    drop
    local.get $a
    i64.popcnt
    drop

    ;; comparisons
    local.get $a
    local.get $b
    i64.eq
    drop
    local.get $a
    local.get $b
    i64.ne
    drop
    local.get $a
    local.get $b
    i64.lt_s
    drop
    local.get $a
    local.get $b
    i64.lt_u
    drop
    local.get $a
    local.get $b
    i64.gt_s
    drop
    local.get $a
    local.get $b
    i64.gt_u
    drop
    local.get $a
    local.get $b
    i64.le_s
    drop
    local.get $a
    local.get $b
    i64.le_u
    drop
    local.get $a
    local.get $b
    i64.ge_s
    drop
    local.get $a
    local.get $b
    i64.ge_u
    drop

    ;; sign-extensions (the i64.extend_i32_{s,u} pair lives in $conversions)
    i64.const 0xFF
    i64.extend8_s
    drop
    i64.const 0xFFFF
    i64.extend16_s
    drop
    i64.const 0xFFFFFFFF
    i64.extend32_s
    drop

    i32.const 0
  )

  ;; ===== f32 ops =====
  (func $f32_ops (result i32) (local $a f32) (local $b f32)
    f32.const 3.5
    local.set $a
    f32.const 1.25
    local.set $b

    ;; load/store via memory address 16 to keep clear of $memory_basics
    i32.const 16
    local.get $a
    f32.store
    i32.const 16
    f32.load
    drop

    ;; arithmetic
    local.get $a
    local.get $b
    f32.add
    drop
    local.get $a
    local.get $b
    f32.sub
    drop
    local.get $a
    local.get $b
    f32.mul
    drop
    local.get $a
    local.get $b
    f32.div
    drop

    ;; comparisons
    local.get $a
    local.get $b
    f32.eq
    drop
    local.get $a
    local.get $b
    f32.ne
    drop
    local.get $a
    local.get $b
    f32.lt
    drop
    local.get $a
    local.get $b
    f32.gt
    drop
    local.get $a
    local.get $b
    f32.le
    drop
    local.get $a
    local.get $b
    f32.ge
    drop

    ;; unary
    local.get $a
    f32.abs
    drop
    local.get $a
    f32.neg
    drop
    local.get $a
    f32.ceil
    drop
    local.get $a
    f32.floor
    drop
    local.get $a
    f32.trunc
    drop
    local.get $a
    f32.nearest
    drop
    local.get $a
    f32.sqrt
    drop

    ;; binary min/max/copysign
    local.get $a
    local.get $b
    f32.min
    drop
    local.get $a
    local.get $b
    f32.max
    drop
    local.get $a
    local.get $b
    f32.copysign
    drop

    i32.const 0
  )

  ;; ===== f64 ops =====
  (func $f64_ops (result i32) (local $a f64) (local $b f64)
    f64.const 6.5
    local.set $a
    f64.const 2.0
    local.set $b

    i32.const 24
    local.get $a
    f64.store
    i32.const 24
    f64.load
    drop

    local.get $a
    local.get $b
    f64.add
    drop
    local.get $a
    local.get $b
    f64.sub
    drop
    local.get $a
    local.get $b
    f64.mul
    drop
    local.get $a
    local.get $b
    f64.div
    drop

    local.get $a
    local.get $b
    f64.eq
    drop
    local.get $a
    local.get $b
    f64.ne
    drop
    local.get $a
    local.get $b
    f64.lt
    drop
    local.get $a
    local.get $b
    f64.gt
    drop
    local.get $a
    local.get $b
    f64.le
    drop
    local.get $a
    local.get $b
    f64.ge
    drop

    local.get $a
    f64.abs
    drop
    local.get $a
    f64.neg
    drop
    local.get $a
    f64.ceil
    drop
    local.get $a
    f64.floor
    drop
    local.get $a
    f64.trunc
    drop
    local.get $a
    f64.nearest
    drop
    local.get $a
    f64.sqrt
    drop

    local.get $a
    local.get $b
    f64.min
    drop
    local.get $a
    local.get $b
    f64.max
    drop
    local.get $a
    local.get $b
    f64.copysign
    drop

    i32.const 0
  )

  ;; ===== Conversions + saturating truncation =====
  (func $conversions (result i32)
    ;; i32.wrap_i64
    i64.const 0x1_0000_0007
    i32.wrap_i64
    drop

    ;; i64.extend_i32_{s,u}
    i32.const -1
    i64.extend_i32_s
    drop
    i32.const -1
    i64.extend_i32_u
    drop

    ;; i32.trunc_f32_{s,u} — non-saturating; keep value well within range
    f32.const 7.5
    i32.trunc_f32_s
    drop
    f32.const 7.5
    i32.trunc_f32_u
    drop

    ;; i32.trunc_f64_{s,u}
    f64.const 7.5
    i32.trunc_f64_s
    drop
    f64.const 7.5
    i32.trunc_f64_u
    drop

    ;; i64.trunc_f32_{s,u}
    f32.const 7.5
    i64.trunc_f32_s
    drop
    f32.const 7.5
    i64.trunc_f32_u
    drop

    ;; i64.trunc_f64_{s,u}
    f64.const 7.5
    i64.trunc_f64_s
    drop
    f64.const 7.5
    i64.trunc_f64_u
    drop

    ;; f32.convert_i32/i64_{s,u}
    i32.const 5
    f32.convert_i32_s
    drop
    i32.const 5
    f32.convert_i32_u
    drop
    i64.const 5
    f32.convert_i64_s
    drop
    i64.const 5
    f32.convert_i64_u
    drop

    ;; f64.convert_i32/i64_{s,u}
    i32.const 5
    f64.convert_i32_s
    drop
    i32.const 5
    f64.convert_i32_u
    drop
    i64.const 5
    f64.convert_i64_s
    drop
    i64.const 5
    f64.convert_i64_u
    drop

    ;; demote / promote
    f64.const 1.5
    f32.demote_f64
    drop
    f32.const 1.5
    f64.promote_f32
    drop

    ;; reinterpret round-trip
    f32.const 1.5
    i32.reinterpret_f32
    drop
    f64.const 1.5
    i64.reinterpret_f64
    drop
    i32.const 0
    f32.reinterpret_i32
    drop
    i64.const 0
    f64.reinterpret_i64
    drop

    ;; saturating truncation — safe even with NaN/inf, but we still feed it finite values
    f32.const 7.5
    i32.trunc_sat_f32_s
    drop
    f32.const 7.5
    i32.trunc_sat_f32_u
    drop
    f64.const 7.5
    i32.trunc_sat_f64_s
    drop
    f64.const 7.5
    i32.trunc_sat_f64_u
    drop
    f32.const 7.5
    i64.trunc_sat_f32_s
    drop
    f32.const 7.5
    i64.trunc_sat_f32_u
    drop
    f64.const 7.5
    i64.trunc_sat_f64_s
    drop
    f64.const 7.5
    i64.trunc_sat_f64_u
    drop

    i32.const 0
  )

  ;; ===== Memory load/store + memory.size =====
  (func $memory_basics (result i32)
    ;; store 0x12345678 at address 0
    i32.const 0
    i32.const 0x12345678
    i32.store

    memory.size          ;; 1 page
    drop

    i32.const 0
    i32.load
  )

  ;; ===== Entry point: invoke each section so probes have somewhere to fire =====
  (func $_start (export "_start")
    i32.const 0
    call $ctrl_flow
    drop

    call $calls
    drop

    call $locals_globals
    drop

    call $memory_basics
    drop

    call $i32_long_tail
    drop

    call $i64_ops
    drop

    call $f32_ops
    drop

    call $f64_ops
    drop

    call $conversions
    drop

    call $memory_full
    drop

    call $bulk_memory
    drop

    call $table_ops
    drop

    call $ref_ops
    drop

    i32.const 5
    call $tail_call
    drop

    i32.const 5
    call $tail_call_indirect
    drop

    i32.const 5
    call $tail_call_ref
    drop

    call $gc_ops
    drop

    ;; never-taken branch keeps `unreachable` statically present without trapping
    i32.const 0
    call $never_taken
    drop

    call $atomic_ops
    drop

    call $exception_ops
    drop
  )

  ;; =====================================================================
  ;; TODO — opcode families still to cover. Each item, once added, should be
  ;; verified uninstrumented on both wasmtime and wizeng before being struck.
  ;;
  ;; - [x] Numeric i32 long tail: i32.{div_s,div_u,rem_s,rem_u,and,or,xor,
  ;;       shl,shr_s,shr_u,rotl,rotr,clz,ctz,popcnt,ne,lt_s,lt_u,gt_s,gt_u,
  ;;       le_s,le_u,ge_u,extend8_s,extend16_s}
  ;; - [x] Numeric i64 full set (eqz/eq/ne/cmp/arith/bitwise/shifts/extend*)
  ;; - [x] f32 ops: const/load/store/arith/cmp/abs/neg/ceil/floor/trunc/
  ;;       nearest/sqrt/min/max/copysign
  ;; - [x] f64 ops: same set as f32
  ;; - [x] Conversions: i32.wrap_i64, i64.extend_i32_{s,u},
  ;;       i32/i64.trunc_f32/f64_{s,u}, f32/f64.convert_i32/i64_{s,u},
  ;;       f32.demote_f64, f64.promote_f32, *.reinterpret_*
  ;; - [x] Saturating truncation: i32/i64.trunc_sat_f32/f64_{s,u}
  ;; - [x] Memory loads/stores beyond i32.{load,store}: i64.load/store,
  ;;       f32/f64.load/store, *.load8_{s,u}/load16_{s,u}/load32_{s,u},
  ;;       *.store8/store16/store32, memory.grow
  ;; - [x] Bulk memory: memory.init, memory.copy, memory.fill, data.drop
  ;;       (passive data segment required)
  ;; - [x] Table ops: table.get, table.set, table.copy, table.init,
  ;;       table.fill, table.grow, table.size, elem.drop
  ;; - [x] Reference types: ref.null, ref.is_null, ref.func, ref.eq
  ;; - [x] Function references: call_ref, return_call_ref, ref.as_non_null,
  ;;       br_on_null, br_on_non_null
  ;; - [x] Tail calls: return_call, return_call_indirect, return_call_ref
  ;; - [x] GC structs: struct.new, struct.new_default, struct.{get,get_s,
  ;;       get_u,set} (packed fields needed for get_s/get_u)
  ;; - [x] GC arrays: array.new, array.new_default, array.new_fixed,
  ;;       array.new_data, array.new_elem, array.{get,get_s,get_u,set,len,
  ;;       fill,copy,init_data,init_elem}
  ;; - [x] GC casts: ref.test, ref.test_null, ref.cast, ref.cast_null,
  ;;       br_on_cast, br_on_cast_fail
  ;; - [x] i31 / extern: ref.i31, i31.get_s, i31.get_u,
  ;;       any.convert_extern, extern.convert_any
  ;; - [x] Threads / atomics: atomic.fence, memory.atomic.{notify,wait32,
  ;;       wait64}, i32/i64.atomic.{load*,store*,rmw*}
  ;; - [x] Exceptions: try_table, throw, throw_ref
  ;; - [x] `unreachable` (placed behind a never-taken branch in $never_taken)
  ;; =====================================================================
)

