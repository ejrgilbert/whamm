wasm:opcode:call:before {
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    var t_i8: i8 = 1;
    var f_i8: i8 = 0;
    var t_u8: u8 = 1;
    var f_u8: u8 = 0;
    var t_i16: i16 = 1;
    var f_i16: i16 = 0;
    var t_u16: u16 = 1;
    var f_u16: u16 = 0;
    var t_i32: i32 = 1;
    var f_i32: i32 = 0;
    var t_u32: u32 = 1;
    var f_u32: u32 = 0;
    var t_i64: i64 = 1;
    var f_i64: i64 = 0;
    var t_u64: u64 = 1;
    var f_u64: u64 = 0;
    var t_f32: f32 = 1;
    var f_f32: f32 = 0;
    var t_f64: i8 = 1;
    var f_f64: f64 = 0;

    // TEST: and
    report var and_test0: bool;
    and_test0 = t_i8 as bool && f_f32 as bool;
    report var and_test1: bool;
    and_test1 = t_u8 as bool && f_u32 as bool;
    report var and_test2: bool;
    and_test2 = t_u16 as bool && f_f64 as bool;

    // TEST: or
    report var or_test0: bool;
    or_test0 = t_i16 as bool || f_u64 as bool;
    report var or_test1: bool;
    or_test1 = t_u64 as bool || f_f32 as bool;
    report var or_test2: bool;
    or_test2 = t_u32 as bool || f_i8 as bool;

    // --- relational operators ---
    // setup
    var v_i8: i8 = 1;
    var v_u8: u8 = 1;
    var v_i16: i16 = 1;
    var v_u16: u16 = 1;
    var v_i32: i32 = 1;
    var v_u32: u32 = 1;
    var v_i64: i64 = 1;
    var v_u64: u64 = 1;
    var v_f32: f32 = 1;
    var v_f64: f64 = 1;

    // TEST: ==

    report var eq_test0: bool;
    eq_test0 = v_i8 as i32 == v_i32;
    report var eq_test1: bool;
    eq_test1 = v_i64 == (v_f64 as i64);

    report var eq_test2: bool;
    eq_test2 = v_u32 == (v_f32 as u32);
    report var eq_test3: bool;
    eq_test3 = v_i16 == v_u8 as i16;

    // TEST: !=

    report var ne_test0: bool;
    ne_test0 = v_u8 != (v_f64 as u8);
    report var ne_test1: bool;
    ne_test1 = v_u16 != (v_f32 as u16);

    report var ne_test2: bool;
    ne_test2 = v_i8 != (v_u8 as i8);
    report var ne_test3: bool;
    ne_test3 = (v_u32 as i16) != v_i16;

    // TEST: >=
    report var ge_test0: bool;
    ge_test0 = (v_u32 as i64) >= v_i64;
    report var ge_test1: bool;
    ge_test1 = v_u16 >= (v_f32 as u16);

    report var ge_test2: bool;
    ge_test2 = v_f64 >= (v_i16 as f64);
    report var ge_test3: bool;
    ge_test3 = v_i16 >= (v_u8 as i16);

    // TEST: >
    report var gt_test0: bool;
    gt_test0 = v_u32 > (v_i32 as u32);
    report var gt_test1: bool;
    gt_test1 = v_i8 > (v_u16 as i8);

    report var gt_test2: bool;
    gt_test2 = v_i16 > (v_f32 as i16);
    report var gt_test3: bool;
    gt_test3 = (v_f64 as i32) > v_i32;

    // TEST: <=
    report var le_test0: bool;
    le_test0 = v_u32 <= (v_i8 as u32);
    report var le_test1: bool;
    le_test1 = v_i32 <= (v_u32 as i32);

    report var le_test2: bool;
    le_test2 = v_i8 <= (v_f64 as i8);
    report var le_test3: bool;
    le_test3 = v_f64 <= (v_u16 as f64);

    // TEST: <
    report var lt_test0: bool;
    lt_test0 = v_i8 < (v_u8 as i8);
    report var lt_test1: bool;
    lt_test1 = (v_i8 as f64) < v_f64;

    report var lt_test2: bool;
    lt_test2 = (v_u8 as f32) < v_f32;
    report var lt_test3: bool;
    lt_test3 = (v_i16 as f64) < v_f64;

    // --- arithmetic operators ---
    // setup

    var U8_MIN: u8 = 0;
    var U64_MAX: u64 = 18_446_744_073_709_551_615;

    // TEST: +
    report var add_test0: u8;
    add_test0 = (v_f64 as u8) + v_u8;
    report var add_test1: u8;
    add_test1 = (v_f64 as u8) + (v_u16 as u8);

    // TEST: -
    report var sub_test0: f32;
    sub_test0 = v_f32 - (v_i32 as f32);
    report var sub_test1: f64;
    sub_test1 = v_f64 - (v_u16 as f64);

    report var sub_test2: i8;
    sub_test2 = v_i8 - (v_i32 as i8);
    report var sub_test3: i16;
    sub_test3 = v_i16 - (v_i8 as i16);

    // TEST: *
    report var mul_test0: u16;
    mul_test0 = v_u16 * (v_f32 as u16);
    report var mul_test1: u8;
    mul_test1 = v_u8 * (v_u32 as u8);

    report var mul_test2: f64;
    mul_test2 = v_f64 * (v_i64 as f64);
    report var mul_test3: u64;
    mul_test3 = v_u64 * (v_f64 as u64);

    // TEST: /
    report var div_test0: u8;
    div_test0 = v_u8 / (v_i64 as u8);
    report var div_test1: f64;
    div_test1 = (v_u64 as f64) / (v_f32 as f64);

    report var div_test2: u32;
    div_test2 = v_u32 / (v_i64 as u32);
    report var div_test3: i16;
    div_test3 = (v_u8 as i16) / v_i16;

    // TEST: %
    report var mod_test0: f32;
    mod_test0 = (v_u8 as f32) % v_f32;
    report var mod_test1: u64;
    mod_test1 = v_u64 % (v_f64 as u64);

    report var mod_test2: i8;
    mod_test2 = v_i8 % (v_f32 as i8);
    report var mod_test3: i64;
    mod_test3 = (v_u16 as i64) % v_i64;

    // ==== UNOP ====
    // --- casts ---

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation
    report var trunc_test0: f32;
    trunc_test0 = (U64_MAX as f32) + (U8_MIN as f32);
    report var trunc_test1: i64;
    trunc_test1 = (U8_MIN as i64) * 2;

    // TEST: extension
    report var ext_test0: i16;
    ext_test0 = ((-1 as u8) as i16);
    report var ext_test1: i16;
    ext_test1 = ((-1 as i8) as i16);

    report var ext_test2: u16;
    ext_test2 = ((-1 as u8) as u16);
    report var ext_test3: u16;
    ext_test3 = ((-1 as i8) as u16);

    report var ext_test4: i32;
    ext_test4 = ((-1 as u8) as i32);
    report var ext_test5: i32;
    ext_test5 = ((-1 as i8) as i32);

    report var ext_test6: u32;
    ext_test6 = ((-1 as u8) as u32);
    report var ext_test7: u32;
    ext_test7 = ((-1 as i8) as u32);

    report var ext_test8: i64;
    ext_test8 = ((-1 as u8) as i64);
    report var ext_test9: i64;
    ext_test9 = ((-1 as i8) as i64);

    report var ext_test10: u64;
    ext_test10 = ((-1 as u8) as u64);
    report var ext_test11: u64;
    ext_test11 = ((-1 as i8) as u64);

    report var ext_test12: f32;
    ext_test12 = ((-1 as u8) as f32);
    report var ext_test13: f32;
    ext_test13 = ((-1 as i8) as f32);

    report var ext_test14: f64;
    ext_test14 = ((-1 as u8) as f64);
    report var ext_test15: f64;
    ext_test15 = ((-1 as i8) as f64);
}