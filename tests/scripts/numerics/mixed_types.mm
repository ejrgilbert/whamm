wasm:opcode:call:before {
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    i8 t_i8 = 1;
    i8 f_i8 = 0;
    u8 t_u8 = 1;
    u8 f_u8 = 0;
    i16 t_i16 = 1;
    i16 f_i16 = 0;
    u16 t_u16 = 1;
    u16 f_u16 = 0;
    i32 t_i32 = 1;
    i32 f_i32 = 0;
    u32 t_u32 = 1;
    u32 f_u32 = 0;
    i64 t_i64 = 1;
    i64 f_i64 = 0;
    u64 t_u64 = 1;
    u64 f_u64 = 0;
    f32 t_f32 = 1;
    f32 f_f32 = 0;
    f64 t_f64 = 1;
    f64 f_f64 = 0;

    // TEST: and
    report bool and_test0;
    and_test0 = t_i8 as bool && f_f32 as bool;
    report bool and_test1;
    and_test1 = t_u8 as bool && f_u32 as bool;
    report bool and_test2;
    and_test2 = t_u16 as bool && f_f64 as bool;

    // TEST: or
    report bool or_test0;
    or_test0 = t_i16 as bool || f_u64 as bool;
    report bool or_test1;
    or_test1 = t_u64 as bool || f_f32 as bool;
    report bool or_test2;
    or_test2 = t_u32 as bool || f_i8 as bool;

    // --- relational operators ---
    // setup
    i8 v_i8 = 1;
    u8 v_u8 = 1;
    i16 v_i16 = 1;
    u16 v_u16 = 1;
    i32 v_i32 = 1;
    u32 v_u32 = 1;
    i64 v_i64 = 1;
    u64 v_u64 = 1;
    f32 v_f32 = 1;
    f64 v_f64 = 1;

    // TEST: ==

    report bool eq_test0;
    eq_test0 = v_i8 as i32 == v_i32;
    report bool eq_test1;
    eq_test1 = v_i64 == (v_f64 as i64);

    report bool eq_test2;
    eq_test2 = v_u32 == (v_f32 as u32);
    report bool eq_test3;
    eq_test3 = v_i16 == v_u8 as i16;

    // TEST: !=

    report bool ne_test0;
    ne_test0 = v_u8 != (v_f64 as u8);
    report bool ne_test1;
    ne_test1 = v_u16 != (v_f32 as u16);

    report bool ne_test2;
    ne_test2 = v_i8 != (v_u8 as i8);
    report bool ne_test3;
    ne_test3 = (v_u32 as i16) != v_i16;

    // TEST: >=
    report bool ge_test0;
    ge_test0 = (v_u32 as i64) >= v_i64;
    report bool ge_test1;
    ge_test1 = v_u16 >= (v_f32 as u16);

    report bool ge_test2;
    ge_test2 = v_f64 >= (v_i16 as f64);
    report bool ge_test3;
    ge_test3 = v_i16 >= (v_u8 as i16);

    // TEST: >
    report bool gt_test0;
    gt_test0 = v_u32 > (v_i32 as u32);
    report bool gt_test1;
    gt_test1 = v_i8 > (v_u16 as i8);

    report bool gt_test2;
    gt_test2 = v_i16 > (v_f32 as i16);
    report bool gt_test3;
    gt_test3 = (v_f64 as i32) > v_i32;

    // TEST: <=
    report bool le_test0;
    le_test0 = v_u32 <= (v_i8 as u32);
    report bool le_test1;
    le_test1 = v_i32 <= (v_u32 as i32);

    report bool le_test2;
    le_test2 = v_i8 <= (v_f64 as i8);
    report bool le_test3;
    le_test3 = v_f64 <= (v_u16 as f64);

    // TEST: <
    report bool lt_test0;
    lt_test0 = v_i8 < (v_u8 as i8);
    report bool lt_test1;
    lt_test1 = (v_i8 as f64) < v_f64;

    report bool lt_test2;
    lt_test2 = (v_u8 as f32) < v_f32;
    report bool lt_test3;
    lt_test3 = (v_i16 as f64) < v_f64;

    // --- arithmetic operators ---
    // setup

    u8 U8_MIN = 0;
    u64 U64_MAX = 18_446_744_073_709_551_615;

    // TEST: +
    report u8 add_test0;
    add_test0 = (v_f64 as u8) + v_u8;
    report u8 add_test1;
    add_test1 = (v_f64 as u8) + (v_u16 as u8);

    // TEST: -
    report f32 sub_test0;
    sub_test0 = v_f32 - (v_i32 as f32);
    report f64 sub_test1;
    sub_test1 = v_f64 - (v_u16 as f64);

    report i8 sub_test2;
    sub_test2 = v_i8 - (v_i32 as i8);
    report i16 sub_test3;
    sub_test3 = v_i16 - (v_i8 as i16);

    // TEST: *
    report u16 mul_test0;
    mul_test0 = v_u16 * (v_f32 as u16);
    report u8 mul_test1;
    mul_test1 = v_u8 * (v_u32 as u8);

    report f64 mul_test2;
    mul_test2 = v_f64 * (v_i64 as f64);
    report u64 mul_test3;
    mul_test3 = v_u64 * (v_f64 as u64);

    // TEST: /
    report u8 div_test0;
    div_test0 = v_u8 / (v_i64 as u8);
    report f64 div_test1;
    div_test1 = (v_u64 as f64) / (v_f32 as f64);

    report u32 div_test2;
    div_test2 = v_u32 / (v_i64 as u32);
    report i16 div_test3;
    div_test3 = (v_u8 as i16) / v_i16;

    // TEST: %
    report f32 mod_test0;
    mod_test0 = (v_u8 as f32) % v_f32;
    report u64 mod_test1;
    mod_test1 = v_u64 % (v_f64 as u64);

    report i8 mod_test2;
    mod_test2 = v_i8 % (v_f32 as i8);
    report i64 mod_test3;
    mod_test3 = (v_u16 as i64) % v_i64;

    // ==== UNOP ====
    // --- casts ---

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation
    report f32 trunc_test0;
    trunc_test0 = (U64_MAX as f32) + (U8_MIN as f32);
    report i64 trunc_test1;
    trunc_test1 = (U8_MIN as i64) * 2;

    // TEST: extension
    report i16 ext_test0;
    ext_test0 = ((-1 as u8) as i16);
    report i16 ext_test1;
    ext_test1 = ((-1 as i8) as i16);

    report u16 ext_test2;
    ext_test2 = ((-1 as u8) as u16);
    report u16 ext_test3;
    ext_test3 = ((-1 as i8) as u16);

    report i32 ext_test4;
    ext_test4 = ((-1 as u8) as i32);
    report i32 ext_test5;
    ext_test5 = ((-1 as i8) as i32);

    report u32 ext_test6;
    ext_test6 = ((-1 as u8) as u32);
    report u32 ext_test7;
    ext_test7 = ((-1 as i8) as u32);

    report i64 ext_test8;
    ext_test8 = ((-1 as u8) as i64);
    report i64 ext_test9;
    ext_test9 = ((-1 as i8) as i64);

    report u64 ext_test10;
    ext_test10 = ((-1 as u8) as u64);
    report u64 ext_test11;
    ext_test11 = ((-1 as i8) as u64);

    report f32 ext_test12;
    ext_test12 = ((-1 as u8) as f32);
    report f32 ext_test13;
    ext_test13 = ((-1 as i8) as f32);

    report f64 ext_test14;
    ext_test14 = ((-1 as u8) as f64);
    report f64 ext_test15;
    ext_test15 = ((-1 as i8) as f64);
}