(module
  (type (;0;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;1;) (func))
  (func (;0;) (type 0) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
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
        loop  ;; label = @3
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
    return)
  (func (;1;) (type 1)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        call 0
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 1
        i32.const 0
        i32.const 1
        call 0
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 1
        i32.const 1
        i32.const 1
        call 0
        i32.const 0
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 3
        i32.const 3
        i32.const 3
        call 0
        i32.const 0
        i32.ne
        br_if 1 (;@1;)
        br 0 (;@2;)
      end
      return
    end
    unreachable
    return)
  (memory (;0;) 1)
  (export "memory" (memory 0))
  (start 1)
  (data (;0;) (i32.const 0) "abcabd"))
