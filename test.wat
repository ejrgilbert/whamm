(module
  (type (;0;) (func (result i64)))
  (type (;1;) (func (param i64) (result i64)))
  (type (;2;) (func))
  (type (;3;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (result i32)))
  (func $get_global_var0 (;0;) (type 0) (result i64)
    global.get $var0
  )
  (func $get_global_var1 (;1;) (type 0) (result i64)
    global.get $var1
  )
  (func $basic_br (;2;) (type 1) (param i64) (result i64)
    block $eq
      block $neq
        local.get 0
        global.get 2
        i32.const 1
        i32.add
        global.set 2
        i64.const 1
        i64.eq
        br_if $eq
        br $neq
      end
      global.get 2
      i32.const 1
      i32.add
      global.set 2
      i64.const 0
      return
    end
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 1
    return
  )
  (func $more_nesting (;3;) (type 1) (param i64) (result i64)
    block $gt
      block $neq
        block $eq
          local.get 0
          global.get 2
          i32.const 1
          i32.add
          global.set 2
          i64.const 0
          i64.eq
          br_if $eq
          local.get 0
          global.get 2
          i32.const 1
          i32.add
          global.set 2
          i64.const 2
          i64.gt_u
          br_if $gt
          br $neq
        end
        global.get 2
        i32.const 1
        i32.add
        global.set 2
        i64.const 1
        return
      end
      global.get 2
      i32.const 1
      i32.add
      global.set 2
      i64.const 0
      return
    end
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 2
    return
  )
  (func $start (;4;) (type 2)
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 0
    call $basic_br
    global.set $var0
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 0
    call $more_nesting
    global.get $var1
    i64.add
    global.set $var1
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 1
    call $more_nesting
    global.get $var1
    i64.add
    global.set $var1
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 1
    call $more_nesting
    global.get $var1
    i64.add
    global.set $var1
    global.get 2
    i32.const 1
    i32.add
    global.set 2
    i64.const 3
    call $more_nesting
    global.get $var1
    i64.add
    global.set $var1
  )
  (func $strcmp (;5;) (type 3) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        local.get 3
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i32.eq
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        loop ;; label = @3
          local.get 4
          local.get 1
          i32.lt_u
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 4
          i32.add
          i32.load8_u
          local.set 5
          local.get 2
          local.get 4
          i32.add
          i32.load8_u
          local.set 6
          local.get 5
          local.get 6
          i32.ne
          br_if 2 (;@1;)
          local.get 4
          i32.const 1
          i32.add
          local.set 4
          br 0 (;@3;)
        end
        br 0 (;@2;)
      end
      i32.const 1
      return
    end
    i32.const 0
    return
  )
  (func (;6;) (type 4) (result i32)
    global.get 2
  )
  (memory (;0;) 1)
  (global $var0 (;0;) (mut i32) i64.const 0)
  (global $var1 (;1;) (mut i32) i64.const 0)
  (global (;2;) (mut i32) i32.const 0)
  (export "get_global_var0" (func $get_global_var0))
  (export "get_global_var1" (func $get_global_var1))
  (export "get_count" (func 6))
  (start $start)
)
