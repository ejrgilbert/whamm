(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32)))
  (type (;5;) (func (param i32 i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32)))
  (type (;10;) (func (param i64)))
  (type (;11;) (func (param f32)))
  (type (;12;) (func (param f64)))
  (type (;13;) (func))
  (type (;14;) (func (param i64 i32) (result i32)))
  (import "whamm_core" "memory" (memory (;0;) 0))
  (import "whamm_core" "create_i32_i32_with_id" (func $create_i32_i32_with_id (type 2)))
  (import "whamm_core" "create_i32_i32" (func $create_i32_i32 (type 1)))
  (import "whamm_core" "create_i32_bool_with_id" (func $create_i32_bool_with_id (type 2)))
  (import "whamm_core" "create_i32_bool" (func $create_i32_bool (type 1)))
  (import "whamm_core" "create_i32_string_with_id" (func $create_i32_string_with_id (type 2)))
  (import "whamm_core" "create_i32_string" (func $create_i32_string (type 1)))
  (import "whamm_core" "create_i32_tuple_with_id" (func $create_i32_tuple_with_id (type 2)))
  (import "whamm_core" "create_i32_tuple" (func $create_i32_tuple (type 1)))
  (import "whamm_core" "create_i32_map_with_id" (func $create_i32_map_with_id (type 2)))
  (import "whamm_core" "create_i32_map" (func $create_i32_map (type 1)))
  (import "whamm_core" "create_string_i32_with_id" (func $create_string_i32_with_id (type 2)))
  (import "whamm_core" "create_string_i32" (func $create_string_i32 (type 1)))
  (import "whamm_core" "create_string_bool_with_id" (func $create_string_bool_with_id (type 2)))
  (import "whamm_core" "create_string_bool" (func $create_string_bool (type 1)))
  (import "whamm_core" "create_string_string_with_id" (func $create_string_string_with_id (type 2)))
  (import "whamm_core" "create_string_string" (func $create_string_string (type 1)))
  (import "whamm_core" "create_string_tuple_with_id" (func $create_string_tuple_with_id (type 2)))
  (import "whamm_core" "create_string_tuple" (func $create_string_tuple (type 1)))
  (import "whamm_core" "create_string_map_with_id" (func $create_string_map_with_id (type 2)))
  (import "whamm_core" "create_string_map" (func $create_string_map (type 1)))
  (import "whamm_core" "create_bool_i32_with_id" (func $create_bool_i32_with_id (type 2)))
  (import "whamm_core" "create_bool_i32" (func $create_bool_i32 (type 1)))
  (import "whamm_core" "create_bool_bool_with_id" (func $create_bool_bool_with_id (type 2)))
  (import "whamm_core" "create_bool_bool" (func $create_bool_bool (type 1)))
  (import "whamm_core" "create_bool_string_with_id" (func $create_bool_string_with_id (type 2)))
  (import "whamm_core" "create_bool_string" (func $create_bool_string (type 1)))
  (import "whamm_core" "create_bool_tuple_with_id" (func $create_bool_tuple_with_id (type 2)))
  (import "whamm_core" "create_bool_tuple" (func $create_bool_tuple (type 1)))
  (import "whamm_core" "create_bool_map_with_id" (func $create_bool_map_with_id (type 2)))
  (import "whamm_core" "create_bool_map" (func $create_bool_map (type 1)))
  (import "whamm_core" "create_tuple_i32_with_id" (func $create_tuple_i32_with_id (type 2)))
  (import "whamm_core" "create_tuple_i32" (func $create_tuple_i32 (type 1)))
  (import "whamm_core" "create_tuple_bool_with_id" (func $create_tuple_bool_with_id (type 2)))
  (import "whamm_core" "create_tuple_bool" (func $create_tuple_bool (type 1)))
  (import "whamm_core" "create_tuple_string_with_id" (func $create_tuple_string_with_id (type 2)))
  (import "whamm_core" "create_tuple_string" (func $create_tuple_string (type 1)))
  (import "whamm_core" "create_tuple_tuple_with_id" (func $create_tuple_tuple_with_id (type 2)))
  (import "whamm_core" "create_tuple_tuple" (func $create_tuple_tuple (type 1)))
  (import "whamm_core" "create_tuple_map_with_id" (func $create_tuple_map_with_id (type 2)))
  (import "whamm_core" "create_tuple_map" (func $create_tuple_map (type 1)))
  (import "whamm_core" "insert_i32_i32" (func $insert_i32_i32 (type 3)))
  (import "whamm_core" "insert_i32_string" (func $insert_i32_string (type 4)))
  (import "whamm_core" "insert_string_i32" (func $insert_string_i32 (type 4)))
  (import "whamm_core" "insert_i32i32tuple_i32" (func $insert_i32i32tuple_i32 (type 4)))
  (import "whamm_core" "insert_i32booltuple_i32" (func $insert_i32booltuple_i32 (type 4)))
  (import "whamm_core" "insert_i32i32i32tuple_i32" (func $insert_i32i32i32tuple_i32 (type 5)))
  (import "whamm_core" "get_i32_i32" (func $get_i32_i32 (type 6)))
  (import "whamm_core" "get_i32_string" (func $get_i32_string (type 3)))
  (import "whamm_core" "get_string_i32" (func $get_string_i32 (type 7)))
  (import "whamm_core" "get_i32i32tuple_i32" (func $get_i32i32tuple_i32 (type 7)))
  (import "whamm_core" "get_i32booltuple_i32" (func $get_i32booltuple_i32 (type 7)))
  (import "whamm_core" "get_i32i32i32tuple_i32" (func $get_i32i32i32tuple_i32 (type 8)))
  (import "whamm_core" "print_map" (func $print_map (type 2)))
  (import "whamm_core" "putc" (func $putc (type 2)))
  (import "whamm_core" "puts" (func $puts (type 9)))
  (import "whamm_core" "putu8" (func $putu8 (type 2)))
  (import "whamm_core" "puti8" (func $puti8 (type 2)))
  (import "whamm_core" "putu16" (func $putu16 (type 2)))
  (import "whamm_core" "puti16" (func $puti16 (type 2)))
  (import "whamm_core" "putu32" (func $putu32 (type 2)))
  (import "whamm_core" "puti32" (func $puti32 (type 2)))
  (import "whamm_core" "putu64" (func $putu64 (type 10)))
  (import "whamm_core" "puti64" (func $puti64 (type 10)))
  (import "whamm_core" "putf32" (func $putf32 (type 11)))
  (import "whamm_core" "putf64" (func $putf64 (type 12)))
  (import "whamm_core" "putbool" (func $putbool (type 2)))
  (func (;66;) (type 0) (param i32) (result i32)
    local.get 0
    i32.const 50
    i32.add)
  (func (;67;) (type 1) (result i32)
    (local i32 i32 i32 i32)
    block (result i32)  ;; label = @1
      local.get 0
      local.set 2
      i32.const 0
      local.set 3
      local.get 2
      i32.const 0
      i32.ne
      if  ;; label = @2
        local.get 3
        local.get 3
        i32.load 2 offset=28 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=28 align=1
      else
        local.get 3
        local.get 3
        i32.load 2 offset=60 align=1
        i32.const 1
        i32.add
        i32.store 2 offset=60 align=1
      end
      local.get 2
      if  ;; label = @2
        i32.const 4
        local.set 0
        local.get 1
        local.set 2
        i32.const 64
        local.set 3
        local.get 2
        i32.const 0
        i32.ne
        if  ;; label = @3
          local.get 3
          local.get 3
          i32.load 2 offset=28 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=28 align=1
        else
          local.get 3
          local.get 3
          i32.load 2 offset=60 align=1
          i32.const 1
          i32.add
          i32.store 2 offset=60 align=1
        end
        local.get 2
        if  ;; label = @3
          i32.const 100
          local.set 0
          local.get 0
          call 66
          local.set 0
        else
          i32.const 1234
          local.set 0
        end
      else
        i32.const 1111
        local.set 0
      end
      local.get 0
    end
    call $on_exit)
  (func $puts_internal (type 9) (param i32 i32)
    (local i32)
    loop  ;; label = @1
      local.get 2
      local.get 1
      i32.lt_u
      i32.eqz
      br_if 1 (;@0;)
      local.get 0
      local.get 2
      i32.add
      i32.load8_u 1
      call $putc
      local.get 2
      i32.const 1
      i32.add
      local.set 2
      br 0 (;@1;)
    end)
  (func $instr_init (type 13))
  (func $flush_var_metadata (type 14) (param i64 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.load 2 align=1
    local.set 2
    local.get 1
    i32.load8_u 2 offset=4
    local.set 3
    local.get 1
    i32.load 2 offset=5 align=1
    local.set 4
    local.get 1
    i32.load 2 offset=9 align=1
    local.set 5
    local.get 1
    i32.load 2 offset=13 align=1
    local.set 6
    local.get 1
    i32.load8_u 2 offset=17
    local.set 7
    local.get 1
    i32.load8_u 2 offset=18
    local.set 8
    local.get 1
    i32.load 2 offset=19 align=1
    local.set 9
    local.get 1
    i32.load8_u 2 offset=23
    local.set 10
    local.get 1
    call $puti32
    i32.const 215
    i32.const 11
    call $puts_internal
    local.get 6
    local.get 7
    call $puts_internal
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 0
    i64.const 7505040001133402106
    i64.eq
    if  ;; label = @1
      i32.const 234
      i32.const 5
      call $puts_internal
      i32.const 234
      i32.const 5
      call $puts_internal
    else
      unreachable
    end
    i32.const 228
    i32.const 6
    call $puts_internal
    local.get 8
    call $puti32
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 2
    local.get 3
    call $puts_internal
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 4
    call $puti32
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 5
    call $puti32
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 9
    local.get 10
    call $puts_internal
    i32.const 226
    i32.const 2
    call $puts_internal
    local.get 1
    i32.const 24
    i32.add)
  (func $flush_i32_vars (type 13)
    (local i32 i32)
    block  ;; label = @1
      global.get 2
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      global.get 2
      local.set 0
      loop  ;; label = @2
        local.get 0
        i32.load 2 align=1
        local.tee 1
        i32.const -1
        i32.ne
        if  ;; label = @3
          local.get 1
          i32.eqz
          if  ;; label = @4
            unreachable ;; Reaches this one, should not
          end
          local.get 0
          local.get 1
          i32.add
          local.set 1
        end
        i32.const 4
        local.get 0
        i32.add
        local.set 0
        i64.const 7505040001133402106
        local.get 0
        call $flush_var_metadata
        local.tee 0
        i32.load 2 align=1
        call $puti32
        i32.const 10
        call $putc
        local.get 1
        i32.const -1
        i32.ne
        if  ;; label = @3
          local.get 1
          local.set 0
          br 1 (;@2;)
        end
      end
    end)
  (func $flush_reports (type 13)
    i32.const 39
    i32.const 176
    call $puts_internal
    i32.const 10
    call $putc
    call $flush_i32_vars)
  (func $on_exit (type 13)
    call $flush_reports)
  (func (;74;) (type 13)
    call $instr_init)
  (memory (;1;) 1)
  (memory (;2;) 1)
  (global (;0;) (mut i32) (i32.const 239))
  (global (;1;) (mut i32) (i32.const 1))
  (global (;2;) (mut i32) (i32.const 0))
  (export "main" (func 67))
  (start 74)
  (data (;0;) 1 (i32.const 0) "taken")
  (data (;1;) 1 (i32.const 5) "0_wasm:opcode:if:before")
  (data (;2;) 1 (i32.const 28) "not_taken")
  (data (;3;) 1 (i32.const 37) "#1")
  (data (;4;) 1 (i32.const 39) "\0a================================= REPORT CSV FLUSH ====================================\0aid, id_type, name, whamm_type, wasm_type, script_id, fname, fid, pc, probe_id, value(s)")
  (data (;5;) 1 (i32.const 215) ", memaddr, ")
  (data (;6;) 1 (i32.const 226) ", ")
  (data (;7;) 1 (i32.const 228) "script")
  (data (;8;) 1 (i32.const 234) "i32, ")
  (data (;9;) 2 (i32.const 0) " \00\00\00%\00\00\00\02\01\00\00\00\01\00\00\00\00\00\00\00\05\00\05\00\00\00\17\00\00\00\00 \00\00\00%\00\00\00\02\01\00\00\00\01\00\00\00\1c\00\00\00\09\00\05\00\00\00\17\00\00\00\00 \00\00\00%\00\00\00\02\01\00\00\00\05\00\00\00\00\00\00\00\05\00\05\00\00\00\17\00\00\00\00\ff\ff\ff\ff%\00\00\00\02\01\00\00\00\05\00\00\00\1c\00\00\00\09\00\05\00\00\00\17\00\00\00\00"))
