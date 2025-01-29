wasm:opcode:call(arg0: u8, arg2: u16, arg4: u32, arg6: u64):before {
    // Call target, parameters are used as type values (all are set to '1'):
    // (u8, i8, u16, i16, u32, i32, u64, i64, f32, f64)
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    var t_8: u8 = arg0;
    var f_8: u8 = 0;
    var t_16: u16 = arg2;
    var f_16: u16 = 0;
    var t_32: u32 = arg4;
    var f_32: u32 = 0;
    var t_64: u64 = arg6;
    var f_64: u64 = 0;

    // TEST: and
    report var and_test0: bool;
    and_test0 = t_8 as bool && f_8 as bool;
    report var and_test1: bool;
    and_test1 = t_16 as bool && f_16 as bool;
    report var and_test2: bool;
    and_test2 = t_32 as bool && f_32 as bool;
    report var and_test3: bool;
    and_test3 = t_64 as bool && f_64 as bool;
    report var and_test4: bool;
    and_test4 = t_32 as bool && 0 as bool;

    // TEST: or
    report var or_test0: bool;
    or_test0 = t_8 as bool || f_8 as bool;
    report var or_test1: bool;
    or_test1 = t_16 as bool || f_16 as bool;
    report var or_test2: bool;
    or_test2 = t_32 as bool || f_32 as bool;
    report var or_test3: bool;
    or_test3 = t_64 as bool || f_64 as bool;
    report var or_test4: bool;
    or_test4 = t_32 as bool || 0 as bool;

    // --- relational operators ---
    // setup
    var v8: u8 = arg0;
    var v16: u16 = arg2;
    var v32: u32 = arg4;
    var v64: u64 = arg6;

    // TEST: ==
    report var eq_test0: bool;
    eq_test0 = v8 == v8;
    report var eq_test1: bool;
    eq_test1 = v8 == (v16 as u8);
    report var eq_test2: bool;
    eq_test2 = v8 == (v32 as u8);
    report var eq_test3: bool;
    eq_test3 = v8 == (v64 as u8);

    report var eq_test4: bool;
    eq_test4 = v16 == (v8 as u16);
    report var eq_test5: bool;
    eq_test5 = v16 == v16;
    report var eq_test6: bool;
    eq_test6 = v16 == (v32 as u16);
    report var eq_test7: bool;
    eq_test7 = v16 == (v64 as u16);

    report var eq_test8: bool;
    eq_test8 = v32 == (v8 as u32);
    report var eq_test9: bool;
    eq_test9 = v32 == (v16 as u32);
    report var eq_test10: bool;
    eq_test10 = v32 == v32;
    report var eq_test11: bool;
    eq_test11 = v32 == (v64 as u32);

    report var eq_test12: bool;
    eq_test12 = v64 == (v8 as u64);
    report var eq_test13: bool;
    eq_test13 = v64 == (v16 as u64);
    report var eq_test14: bool;
    eq_test14 = v64 == (v32 as u64);
    report var eq_test15: bool;
    eq_test15 = v64 == v64;

    report var eq_test16: bool;
    eq_test16 = v32 as u8 == 0;
    report var eq_test17: bool;
    eq_test17 = v32 as u16 == 0;
    report var eq_test18: bool;
    eq_test18 = v32 as u32 == 0;
    report var eq_test19: bool;
    eq_test19 = v32 as u64 == 0;
    
    // TEST: !=
    report var ne_test0: bool;
    ne_test0 = v8 != v8;
    report var ne_test1: bool;
    ne_test1 = v8 != (v16 as u8);
    report var ne_test2: bool;
    ne_test2 = v8 != (v32 as u8);
    report var ne_test3: bool;
    ne_test3 = v8 != (v64 as u8);

    report var ne_test4: bool;
    ne_test4 = v16 != (v8 as u16);
    report var ne_test5: bool;
    ne_test5 = v16 != v16;
    report var ne_test6: bool;
    ne_test6 = v16 != (v32 as u16);
    report var ne_test7: bool;
    ne_test7 = v16 != (v64 as u16);

    report var ne_test8: bool;
    ne_test8 = v32 != (v8 as u32);
    report var ne_test9: bool;
    ne_test9 = v32 != (v16 as u32);
    report var ne_test10: bool;
    ne_test10 = v32 != v32;
    report var ne_test11: bool;
    ne_test11 = v32 != (v64 as u32);

    report var ne_test12: bool;
    ne_test12 = v64 != (v8 as u64);
    report var ne_test13: bool;
    ne_test13 = v64 != (v16 as u64);
    report var ne_test14: bool;
    ne_test14 = v64 != (v32 as u64);
    report var ne_test15: bool;
    ne_test15 = v64 != v64;

    report var ne_test16: bool;
    ne_test16 = v32 as u8 != 0;
    report var ne_test17: bool;
    ne_test17 = v32 as u16 != 0;
    report var ne_test18: bool;
    ne_test18 = v32 as u32 != 0;
    report var ne_test19: bool;
    ne_test19 = v32 as u64 != 0;
    
    // TEST: >=
    report var ge_test0: bool;
    ge_test0 = v8 >= v8;
    report var ge_test1: bool;
    ge_test1 = v8 >= (v16 as u8);
    report var ge_test2: bool;
    ge_test2 = v8 >= (v32 as u8);
    report var ge_test3: bool;
    ge_test3 = v8 >= (v64 as u8);

    report var ge_test4: bool;
    ge_test4 = v16 >= (v8 as u16);
    report var ge_test5: bool;
    ge_test5 = v16 >= v16;
    report var ge_test6: bool;
    ge_test6 = v16 >= (v32 as u16);
    report var ge_test7: bool;
    ge_test7 = v16 >= (v64 as u16);

    report var ge_test8: bool;
    ge_test8 = v32 >= (v8 as u32);
    report var ge_test9: bool;
    ge_test9 = v32 >= (v16 as u32);
    report var ge_test10: bool;
    ge_test10 = v32 >= v32;
    report var ge_test11: bool;
    ge_test11 = v32 >= (v64 as u32);

    report var ge_test12: bool;
    ge_test12 = v64 >= (v8 as u64);
    report var ge_test13: bool;
    ge_test13 = v64 >= (v16 as u64);
    report var ge_test14: bool;
    ge_test14 = v64 >= (v32 as u64);
    report var ge_test15: bool;
    ge_test15 = v64 >= v64;

    report var ge_test16: bool;
    ge_test16 = v32 as u8 >= 0;
    report var ge_test17: bool;
    ge_test17 = v32 as u16 >= 0;
    report var ge_test18: bool;
    ge_test18 = v32 as u32 >= 0;
    report var ge_test19: bool;
    ge_test19 = v32 as u64 >= 0;
    
    // TEST: >
    report var gt_test0: bool;
    gt_test0 = v8 > v8;
    report var gt_test1: bool;
    gt_test1 = v8 > (v16 as u8);
    report var gt_test2: bool;
    gt_test2 = v8 > (v32 as u8);
    report var gt_test3: bool;
    gt_test3 = v8 > (v64 as u8);

    report var gt_test4: bool;
    gt_test4 = v16 > (v8 as u16);
    report var gt_test5: bool;
    gt_test5 = v16 > v16;
    report var gt_test6: bool;
    gt_test6 = v16 > (v32 as u16);
    report var gt_test7: bool;
    gt_test7 = v16 > (v64 as u16);

    report var gt_test8: bool;
    gt_test8 = v32 > (v8 as u32);
    report var gt_test9: bool;
    gt_test9 = v32 > (v16 as u32);
    report var gt_test10: bool;
    gt_test10 = v32 > v32;
    report var gt_test11: bool;
    gt_test11 = v32 > (v64 as u32);

    report var gt_test12: bool;
    gt_test12 = v64 > (v8 as u64);
    report var gt_test13: bool;
    gt_test13 = v64 > (v16 as u64);
    report var gt_test14: bool;
    gt_test14 = v64 > (v32 as u64);
    report var gt_test15: bool;
    gt_test15 = v64 > v64;

    report var gt_test16: bool;
    gt_test16 = v32 as u8 > 0;
    report var gt_test17: bool;
    gt_test17 = v32 as u16 > 0;
    report var gt_test18: bool;
    gt_test18 = v32 as u32 > 0;
    report var gt_test19: bool;
    gt_test19 = v32 as u64 > 0;
    
    // TEST: <=
    report var le_test0: bool;
    le_test0 = v8 <= v8;
    report var le_test1: bool;
    le_test1 = v8 <= (v16 as u8);
    report var le_test2: bool;
    le_test2 = v8 <= (v32 as u8);
    report var le_test3: bool;
    le_test3 = v8 <= (v64 as u8);

    report var le_test4: bool;
    le_test4 = v16 <= (v8 as u16);
    report var le_test5: bool;
    le_test5 = v16 <= v16;
    report var le_test6: bool;
    le_test6 = v16 <= (v32 as u16);
    report var le_test7: bool;
    le_test7 = v16 <= (v64 as u16);

    report var le_test8: bool;
    le_test8 = v32 <= (v8 as u32);
    report var le_test9: bool;
    le_test9 = v32 <= (v16 as u32);
    report var le_test10: bool;
    le_test10 = v32 <= v32;
    report var le_test11: bool;
    le_test11 = v32 <= (v64 as u32);

    report var le_test12: bool;
    le_test12 = v64 <= (v8 as u64);
    report var le_test13: bool;
    le_test13 = v64 <= (v16 as u64);
    report var le_test14: bool;
    le_test14 = v64 <= (v32 as u64);
    report var le_test15: bool;
    le_test15 = v64 <= v64;

    report var le_test16: bool;
    le_test16 = v32 as u8 <= 0;
    report var le_test17: bool;
    le_test17 = v32 as u16 <= 0;
    report var le_test18: bool;
    le_test18 = v32 as u32 <= 0;
    report var le_test19: bool;
    le_test19 = v32 as u64 <= 0;
    
    // TEST: <
    report var lt_test0: bool;
    lt_test0 = v8 < v8;
    report var lt_test1: bool;
    lt_test1 = v8 < (v16 as u8);
    report var lt_test2: bool;
    lt_test2 = v8 < (v32 as u8);
    report var lt_test3: bool;
    lt_test3 = v8 < (v64 as u8);

    report var lt_test4: bool;
    lt_test4 = v16 < (v8 as u16);
    report var lt_test5: bool;
    lt_test5 = v16 < v16;
    report var lt_test6: bool;
    lt_test6 = v16 < (v32 as u16);
    report var lt_test7: bool;
    lt_test7 = v16 < (v64 as u16);

    report var lt_test8: bool;
    lt_test8 = v32 < (v8 as u32);
    report var lt_test9: bool;
    lt_test9 = v32 < (v16 as u32);
    report var lt_test10: bool;
    lt_test10 = v32 < v32;
    report var lt_test11: bool;
    lt_test11 = v32 < (v64 as u32);

    report var lt_test12: bool;
    lt_test12 = v64 < (v8 as u64);
    report var lt_test13: bool;
    lt_test13 = v64 < (v16 as u64);
    report var lt_test14: bool;
    lt_test14 = v64 < (v32 as u64);
    report var lt_test15: bool;
    lt_test15 = v64 < v64;

    report var lt_test16: bool;
    lt_test16 = v32 as u8 < 0;
    report var lt_test17: bool;
    lt_test17 = v32 as u16 < 0;
    report var lt_test18: bool;
    lt_test18 = v32 as u32 < 0;
    report var lt_test19: bool;
    lt_test19 = v32 as u64 < 0;

    // --- arithmetic operators ---
    // setup
    var U8_MIN: u8 = 0;
    var U8_MAX: u8 = 255;

    var U16_MIN: u16 = 0;
    var U16_MAX: u16 = 65_535;

    var U32_MIN: u32 = 0;
    var U32_MAX: u32 = 4_294_967_295;

    var U64_MIN: u64 = 0;
    var U64_MAX: u64 = 18_446_744_073_709_551_615;

    // TEST: +
    report var add_test0: u8;
    add_test0 = v8 + v8;
    report var add_test1: u8;
    add_test1 = v8 + (v16 as u8);
    report var add_test2: u8;
    add_test2 = v8 + (v32 as u8);
    report var add_test3: u8;
    add_test3 = v8 + (v64 as u8);

    report var add_test4: u16;
    add_test4 = v16 + (v8 as u16);
    report var add_test5: u16;
    add_test5 = v16 + v16;
    report var add_test6: u16;
    add_test6 = v16 + (v32 as u16);
    report var add_test7: u16;
    add_test7 = v16 + (v64 as u16);

    report var add_test8: u32;
    add_test8 = v32 + (v8 as u32);
    report var add_test9: u32;
    add_test9 = v32 + (v16 as u32);
    report var add_test10: u32;
    add_test10 = v32 + v32;
    report var add_test11: u32;
    add_test11 = v32 + (v64 as u32);

    report var add_test12: u64;
    add_test12 = v64 + (v8 as u64);
    report var add_test13: u64;
    add_test13 = v64 + (v16 as u64);
    report var add_test14: u64;
    add_test14 = v64 + (v32 as u64);
    report var add_test15: u64;
    add_test15 = v64 + v64;

    report var add_test16: u8;
    add_test16 = v32 as u8 + 0;
    report var add_test17: u16;
    add_test17 = v32 as u16 + 0;
    report var add_test18: u32;
    add_test18 = v32 as u32 + 0;
    report var add_test19: u64;
    add_test19 = v32 as u64 + 0;

    report var add_test20: u8;
    add_test20 = U8_MAX + (v32 as u8);
    report var add_test21: u16;
    add_test21 = U16_MAX + (v32 as u16);
    report var add_test22: u32;
    add_test22 = U32_MAX + v32;
    report var add_test23: u64;
    add_test23 = U64_MAX + (v32 as u64);

    // TEST: -
    report var sub_test0: u8;
    sub_test0 = v8 - v8;
    report var sub_test1: u8;
    sub_test1 = v8 - (v16 as u8);
    report var sub_test2: u8;
    sub_test2 = v8 - (v32 as u8);
    report var sub_test3: u8;
    sub_test3 = v8 - (v64 as u8);

    report var sub_test4: u16;
    sub_test4 = v16 - (v8 as u16);
    report var sub_test5: u16;
    sub_test5 = v16 - v16;
    report var sub_test6: u16;
    sub_test6 = v16 - (v32 as u16);
    report var sub_test7: u16;
    sub_test7 = v16 - (v64 as u16);

    report var sub_test8: u32;
    sub_test8 = v32 - (v8 as u32);
    report var sub_test9: u32;
    sub_test9 = v32 - (v16 as u32);
    report var sub_test10: u32;
    sub_test10 = v32 - v32;
    report var sub_test11: u32;
    sub_test11 = v32 - (v64 as u32);

    report var sub_test12: u64;
    sub_test12 = v64 - (v8 as u64);
    report var sub_test13: u64;
    sub_test13 = v64 - (v16 as u64);
    report var sub_test14: u64;
    sub_test14 = v64 - (v32 as u64);
    report var sub_test15: u64;
    sub_test15 = v64 - v64;

    report var sub_test16: u8;
    sub_test16 = v32 as u8 - 0;
    report var sub_test17: u16;
    sub_test17 = v32 as u16 - 0;
    report var sub_test18: u32;
    sub_test18 = v32 as u32 - 0;
    report var sub_test19: u64;
    sub_test19 = v32 as u64 - 0;

    report var sub_test20: u8;
    sub_test20 = U8_MIN - (v32 as u8);
    report var sub_test21: u16;
    sub_test21 = U16_MIN - (v32 as u16);
    report var sub_test22: u32;
    sub_test22 = U32_MIN - v32;
    report var sub_test23: u64;
    sub_test23 = U64_MIN - (v32 as u64);

    // TEST: *
    report var mul_test0: u8;
    mul_test0 = v8 * v8;
    report var mul_test1: u8;
    mul_test1 = v8 * (v16 as u8);
    report var mul_test2: u8;
    mul_test2 = v8 * (v32 as u8);
    report var mul_test3: u8;
    mul_test3 = v8 * (v64 as u8);

    report var mul_test4: u16;
    mul_test4 = v16 * (v8 as u16);
    report var mul_test5: u16;
    mul_test5 = v16 * v16;
    report var mul_test6: u16;
    mul_test6 = v16 * (v32 as u16);
    report var mul_test7: u16;
    mul_test7 = v16 * (v64 as u16);

    report var mul_test8: u32;
    mul_test8 = v32 * (v8 as u32);
    report var mul_test9: u32;
    mul_test9 = v32 * (v16 as u32);
    report var mul_test10: u32;
    mul_test10 = v32 * v32;
    report var mul_test11: u32;
    mul_test11 = v32 * (v64 as u32);

    report var mul_test12: u64;
    mul_test12 = v64 * (v8 as u64);
    report var mul_test13: u64;
    mul_test13 = v64 * (v16 as u64);
    report var mul_test14: u64;
    mul_test14 = v64 * (v32 as u64);
    report var mul_test15: u64;
    mul_test15 = v64 * v64;

    report var mul_test16: u8;
    mul_test16 = v32 as u8 * 0;
    report var mul_test17: u16;
    mul_test17 = v32 as u16 * 0;
    report var mul_test18: u32;
    mul_test18 = v32 as u32 * 0;
    report var mul_test19: u64;
    mul_test19 = v32 as u64 * 0;

    report var mul_test20: u8;
    mul_test20 = U8_MIN * (v32 as u8);
    report var mul_test21: u16;
    mul_test21 = U16_MIN * (v32 as u16);
    report var mul_test22: u32;
    mul_test22 = U32_MIN * v32;
    report var mul_test23: u64;
    mul_test23 = U64_MIN * (v32 as u64);
    
    report var mul_test24: u8;
    mul_test24 = U8_MIN * 2;
    report var mul_test25: u16;
    mul_test25 = U16_MIN * 2;
    report var mul_test26: u32;
    mul_test26 = U32_MIN * 2;
    report var mul_test27: u64;
    mul_test27 = U64_MIN * 2;

    report var mul_test28: u8;
    mul_test28 = U8_MAX * (v32 as u8);
    report var mul_test29: u16;
    mul_test29 = U16_MAX * (v32 as u16);
    report var mul_test30: u32;
    mul_test30 = U32_MAX * v32;
    report var mul_test31: u64;
    mul_test31 = U64_MAX * (v32 as u64);
    
    report var mul_test32: u8;
    mul_test32 = U8_MAX * 2;
    report var mul_test33: u16;
    mul_test33 = U16_MAX * 2;
    report var mul_test34: u32;
    mul_test34 = U32_MAX * 2;
    report var mul_test35: u64;
    mul_test35 = U64_MAX * 2;

    // TEST: /
    report var div_test0: u8;
    div_test0 = v8 / v8;
    report var div_test1: u8;
    div_test1 = v8 / (v16 as u8);
    report var div_test2: u8;
    div_test2 = v8 / (v32 as u8);
    report var div_test3: u8;
    div_test3 = v8 / (v64 as u8);

    report var div_test4: u16;
    div_test4 = v16 / (v8 as u16);
    report var div_test5: u16;
    div_test5 = v16 / v16;
    report var div_test6: u16;
    div_test6 = v16 / (v32 as u16);
    report var div_test7: u16;
    div_test7 = v16 / (v64 as u16);

    report var div_test8: u32;
    div_test8 = v32 / (v8 as u32);
    report var div_test9: u32;
    div_test9 = v32 / (v16 as u32);
    report var div_test10: u32;
    div_test10 = v32 / v32;
    report var div_test11: u32;
    div_test11 = v32 / (v64 as u32);

    report var div_test12: u64;
    div_test12 = v64 / (v8 as u64);
    report var div_test13: u64;
    div_test13 = v64 / (v16 as u64);
    report var div_test14: u64;
    div_test14 = v64 / (v32 as u64);
    report var div_test15: u64;
    div_test15 = v64 / v64;

    report var div_test20: u8;
    div_test20 = U8_MIN / (v32 as u8);
    report var div_test21: u16;
    div_test21 = U16_MIN / (v32 as u16);
    report var div_test22: u32;
    div_test22 = U32_MIN / v32;
    report var div_test23: u64;
    div_test23 = U64_MIN / (v32 as u64);
    
    // TEST: %
    report var mod_test0: u8;
    mod_test0 = v8 % v8;
    report var mod_test1: u8;
    mod_test1 = v8 % (v16 as u8);
    report var mod_test2: u8;
    mod_test2 = v8 % (v32 as u8);
    report var mod_test3: u8;
    mod_test3 = v8 % (v64 as u8);

    report var mod_test4: u16;
    mod_test4 = v16 % (v8 as u16);
    report var mod_test5: u16;
    mod_test5 = v16 % v16;
    report var mod_test6: u16;
    mod_test6 = v16 % (v32 as u16);
    report var mod_test7: u16;
    mod_test7 = v16 % (v64 as u16);

    report var mod_test8: u32;
    mod_test8 = v32 % (v8 as u32);
    report var mod_test9: u32;
    mod_test9 = v32 % (v16 as u32);
    report var mod_test10: u32;
    mod_test10 = v32 % v32;
    report var mod_test11: u32;
    mod_test11 = v32 % (v64 as u32);

    report var mod_test12: u64;
    mod_test12 = v64 % (v8 as u64);
    report var mod_test13: u64;
    mod_test13 = v64 % (v16 as u64);
    report var mod_test14: u64;
    mod_test14 = v64 % (v32 as u64);
    report var mod_test15: u64;
    mod_test15 = v64 % v64;

    report var mod_test20: u8;
    mod_test20 = U8_MIN % (v32 as u8);
    report var mod_test21: u16;
    mod_test21 = U16_MIN % (v32 as u16);
    report var mod_test22: u32;
    mod_test22 = U32_MIN % v32;
    report var mod_test23: u64;
    mod_test23 = U64_MIN % (v32 as u64);

    // ==== UNOP ====
    // --- casts ---

    // TEST: implicit with primitive
    report var imp_test0: u8;
    imp_test0 = v8 + (v32 as u8);
    report var imp_test1: u16;
    imp_test1 = v16 + (v32 as u16);
    report var imp_test2: u32;
    imp_test2 = v32 + v32;
    report var imp_test3: u64;
    imp_test3 = v64 + (v32 as u64);

    report var imp_test4: u8;
    imp_test4 = (v32 as u8) + v8;
    report var imp_test5: u16;
    imp_test5 = (v32 as u16) + v16;
    report var imp_test6: u32;
    imp_test6 = v32 + v32;
    report var imp_test7: u64;
    imp_test7 = (v32 as u64) + v64;

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation
    report var trunc_test0: u8;
    trunc_test0 = U16_MAX as u8;
    report var trunc_test1: u16;
    trunc_test1 = U32_MAX as u16;
    report var trunc_test2: u32;
    trunc_test2 = U64_MAX as u32;


    // TEST: extension (unsigned)
    report var trunc_test3: u16;
    trunc_test3 = U8_MAX as u16;
    report var trunc_test4: u32;
    trunc_test4 = U16_MAX as u32;
    report var trunc_test5: u64;
    trunc_test5 = U32_MAX as u64;
}