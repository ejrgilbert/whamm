wasm:opcode:call:before {
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    f32 t_32 = 1;
    f32 f_32 = 0;
    f64 t_64 = 1;
    f64 f_64 = 0;

    // TEST: and
    report var and_test0: bool;
    and_test0 = t_32 as bool && f_32 as bool;
    report var and_test1: bool;
    and_test1 = t_64 as bool && f_64 as bool;

    // TEST: or
    report var or_test0: bool;
    or_test0 = t_32 as bool || f_32 as bool;
    report var or_test1: bool;
    or_test1 = t_64 as bool || f_64 as bool;

    // --- relational operators ---
    // setup
    var v32: f32 = 1;
    var v64: f64 = 1;

    // TEST: ==

    report var eq_test0: bool;
    eq_test0 = v32 == v32;
    report var eq_test1: bool;
    eq_test1 = v32 == (v64 as f32);

    report var eq_test2: bool;
    eq_test2 = v64 == (v32 as f64);
    report var eq_test3: bool;
    eq_test3 = v64 == v64;

    report var eq_test4: bool;
    eq_test4 = 1 as f32 == 0;
    report var eq_test5: bool;
    eq_test5 = 1 as f64 == 0;

    // TEST: !=

    report var ne_test0: bool;
    ne_test0 = v32 != v32;
    report var ne_test1: bool;
    ne_test1 = v32 != (v64 as f32);

    report var ne_test2: bool;
    ne_test2 = v64 != (v32 as f64);
    report var ne_test3: bool;
    ne_test3 = v64 != v64;

    report var ne_test4: bool;
    ne_test4 = 1 as f32 != 0;
    report var ne_test5: bool;
    ne_test5 = 1 as f64 != 0;

    // TEST: >=
    report var ge_test0: bool;
    ge_test0 = v32 >= v32;
    report var ge_test1: bool;
    ge_test1 = v32 >= (v64 as f32);

    report var ge_test2: bool;
    ge_test2 = v64 >= (v32 as f64);
    report var ge_test3: bool;
    ge_test3 = v64 >= v64;

    report var ge_test4: bool;
    ge_test4 = 1 as f32 >= 0;
    report var ge_test5: bool;
    ge_test5 = 1 as f64 >= 0;

    // TEST: >
    report var gt_test0: bool;
    gt_test0 = v32 > v32;
    report var gt_test1: bool;
    gt_test1 = v32 > (v64 as f32);

    report var gt_test2: bool;
    gt_test2 = v64 > (v32 as f64);
    report var gt_test3: bool;
    gt_test3 = v64 > v64;

    report var gt_test4: bool;
    gt_test4 = 1 as f32 > 0;
    report var gt_test5: bool;
    gt_test5 = 1 as f64 > 0;

    // TEST: <=
    report var le_test0: bool;
    le_test0 = v32 <= v32;
    report var le_test1: bool;
    le_test1 = v32 <= (v64 as f32);

    report var le_test2: bool;
    le_test2 = v64 <= (v32 as f64);
    report var le_test3: bool;
    le_test3 = v64 <= v64;

    report var le_test4: bool;
    le_test4 = 1 as f32 <= 0;
    report var le_test5: bool;
    le_test5 = 1 as f64 <= 0;

    // TEST: <
    report var lt_test0: bool;
    lt_test0 = v32 < v32;
    report var lt_test1: bool;
    lt_test1 = v32 < (v64 as f32);

    report var lt_test2: bool;
    lt_test2 = v64 < (v32 as f64);
    report var lt_test3: bool;
    lt_test3 = v64 < v64;

    report var lt_test4: bool;
    lt_test4 = 1 as f32 < 0;
    report var lt_test5: bool;
    lt_test5 = 1 as f64 < 0;

    // --- arithmetic operators ---
    // setup
    var F32_MIN: f32 = -3.40282347E+38;
    var F32_MAX: f32 = 3.40282347E+38;

    var F64_MIN: f64 = -1.7976931348623157E+308;
    var F64_MAX: f64 = 1.7976931348623157E+308;

    // TEST: +
    report var add_test0: f32;
    add_test0 = v32 + v32;
    report var add_test1: f32;
    add_test1 = v32 + (v64 as f32);

    report var add_test2: f64;
    add_test2 = v64 + (v32 as f64);
    report var add_test3: f64;
    add_test3 = v64 + v64;

    report var add_test4: f32;
    add_test4 = 1 as f32 + 0;
    report var add_test5: f64;
    add_test5 = 1 as f64 + 0;

    report var add_test8: f32;
    add_test8 = F32_MAX + 1;
    report var add_test9: f64;
    add_test9 = F64_MAX + 1;

    // TEST: -
    report var sub_test0: f32;
    sub_test0 = v32 - v32;
    report var sub_test1: f32;
    sub_test1 = v32 - (v64 as f32);

    report var sub_test2: f64;
    sub_test2 = v64 - (v32 as f64);
    report var sub_test3: f64;
    sub_test3 = v64 - v64;

    report var sub_test4: f32;
    sub_test4 = 1 as f32 - 0;
    report var sub_test5: f64;
    sub_test5 = 1 as f64 - 0;

    report var sub_test8: f32;
    sub_test8 = F32_MIN - 1;
    report var sub_test9: f64;
    sub_test9 = F64_MIN - 1;

    // TEST: *
    report var mul_test0: f32;
    mul_test0 = v32 * v32;
    report var mul_test1: f32;
    mul_test1 = v32 * (v64 as f32);

    report var mul_test2: f64;
    mul_test2 = v64 * (v32 as f64);
    report var mul_test3: f64;
    mul_test3 = v64 * v64;

    report var mul_test4: f32;
    mul_test4 = 1 as f32 * 0;
    report var mul_test5: f64;
    mul_test5 = 1 as f64 * 0;

    report var mul_test8: f32;
    mul_test8 = F32_MIN * 1;
    report var mul_test9: f64;
    mul_test9 = F64_MIN * 1;

    report var mul_test10: f32;
    mul_test10 = F32_MIN * 2;
    report var mul_test11: f64;
    mul_test11 = F64_MIN * 2;

    report var mul_test12: f32;
    mul_test12 = F32_MAX * 1;
    report var mul_test13: f64;
    mul_test13 = F64_MAX * 1;

    report var mul_test14: f32;
    mul_test14 = F32_MAX * 2;
    report var mul_test15: f64;
    mul_test15 = F64_MAX * 2;

    // TEST: /
    report var div_test0: f32;
    div_test0 = v32 / v32;
    report var div_test1: f32;
    div_test1 = v32 / (v64 as f32);

    report var div_test2: f64;
    div_test2 = v64 / (v32 as f64);
    report var div_test3: f64;
    div_test3 = v64 / v64;

    report var div_test8: f32;
    div_test8 = F32_MIN / 1;
    report var div_test9: f64;
    div_test9 = F64_MIN / 1;

    // TEST: %
    report var mod_test0: f32;
    mod_test0 = v32 % v32;
    report var mod_test1: f32;
    mod_test1 = v32 % (v64 as f32);

    report var mod_test2: f64;
    mod_test2 = v64 % (v32 as f64);
    report var mod_test3: f64;
    mod_test3 = v64 % v64;

    report var mod_test8: f32;
    mod_test8 = F32_MIN % 1;
    report var mod_test9: f64;
    mod_test9 = F64_MIN % 1;

    // ==== UNOP ====
    // --- casts ---

    // TEST: implicit with primitive
    report var imp_test0: f32;
    imp_test0 = v32 + 1;
    report var imp_test1: f64;
    imp_test1 = v64 + 1;

    report var imp_test4: f32;
    imp_test4 = 1 + v32;
    report var imp_test5: f64;
    imp_test5 = 1 + v64;

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation (unsigned)
    report var trunc_test0: f32;
    trunc_test0 = F64_MAX as f32;
    // TEST: truncation (signed)
    report var trunc_test1: f32;
    trunc_test1 = F64_MIN as f32;

    // TEST: extension (unsigned)
    report var trunc_test2: f64;
    trunc_test2 = F32_MAX as f64;
    // TEST: extension (signed)
    report var trunc_test3: f64;
    trunc_test3 = F32_MIN as f64;
}