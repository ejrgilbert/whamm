wasm:opcode:call:before {
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    i8 t_8 = 1;
    i8 f_8 = 0;
    i16 t_16 = 1;
    i16 f16 = 0;
    i32 t_32 = 1;
    i32 f_32 = 0;
    i64 t_64 = 1;
    i64 f_64 = 0;

    // TEST: and
    report bool and_test0;
    and_test0 = t_8 as bool && f_8 as bool;
    report bool and_test1;
    and_test1 = t_16 as bool && f16 as bool;
    report bool and_test2;
    and_test2 = t_32 as bool && f_32 as bool;
    report bool and_test3;
    and_test3 = t_64 as bool && f_64 as bool;
    report bool and_test4;
    and_test4 = 1 as bool && 0 as bool;

    // TEST: or
    report bool or_test0;
    or_test0 = t_8 as bool || f_8 as bool;
    report bool or_test1;
    or_test1 = t_16 as bool || f16 as bool;
    report bool or_test2;
    or_test2 = t_32 as bool || f_32 as bool;
    report bool or_test3;
    or_test3 = t_64 as bool || f_64 as bool;
    report bool or_test4;
    or_test4 = 1 as bool || 0 as bool;

    // --- relational operators ---
    // setup
    i8 v8 = 1;
    i16 v16 = 1;
    i32 v32 = 1;
    i64 v64 = 1;

    // TEST: ==
    report bool eq_test0;
    eq_test0 = v8 == v8;
    report bool eq_test1;
    eq_test1 = v8 == (v16 as i8);
    report bool eq_test2;
    eq_test2 = v8 == (v32 as i8);
    report bool eq_test3;
    eq_test3 = v8 == (v64 as i8);

    report bool eq_test4;
    eq_test4 = v16 == (v8 as i16);
    report bool eq_test5;
    eq_test5 = v16 == v16;
    report bool eq_test6;
    eq_test6 = v16 == (v32 as i8);
    report bool eq_test7;
    eq_test7 = v16 == (v64 as i8);

    report bool eq_test8;
    eq_test8 = v32 == (v8 as i32);
    report bool eq_test9;
    eq_test9 = v32 == (v16 as i8);
    report bool eq_test10;
    eq_test10 = v32 == v32;
    report bool eq_test11;
    eq_test11 = v32 == (v64 as i8);

    report bool eq_test12;
    eq_test12 = v64 == (v8 as i64);
    report bool eq_test13;
    eq_test13 = v64 == (v16 as i8);
    report bool eq_test14;
    eq_test14 = v64 == (v32 as i8);
    report bool eq_test15;
    eq_test15 = v64 == v64;

    report bool eq_test16;
    eq_test16 = 1 as i8 == 0 as i64;
    report bool eq_test17;
    eq_test17 = 1 as i16 == 0 as i64;
    report bool eq_test18;
    eq_test18 = 1 as i32 == 0 as i64;
    report bool eq_test19;
    eq_test19 = 1 as i64 == 0 as i64;
    
    // TEST: !=
    report bool ne_test0;
    ne_test0 = v8 != v8;
    report bool ne_test1;
    ne_test1 = v8 != (v16 as i8);
    report bool ne_test2;
    ne_test2 = v8 != (v32 as i8);
    report bool ne_test3;
    ne_test3 = v8 != (v64 as i8);

    report bool ne_test4;
    ne_test4 = v16 != (v8 as i16);
    report bool ne_test5;
    ne_test5 = v16 != v16;
    report bool ne_test6;
    ne_test6 = v16 != (v32 as i8);
    report bool ne_test7;
    ne_test7 = v16 != (v64 as i8);

    report bool ne_test8;
    ne_test8 = v32 != (v8 as i32);
    report bool ne_test9;
    ne_test9 = v32 != (v16 as i8);
    report bool ne_test10;
    ne_test10 = v32 != v32;
    report bool ne_test11;
    ne_test11 = v32 != (v64 as i8);

    report bool ne_test12;
    ne_test12 = v64 != (v8 as i64);
    report bool ne_test13;
    ne_test13 = v64 != (v16 as i8);
    report bool ne_test14;
    ne_test14 = v64 != (v32 as i8);
    report bool ne_test15;
    ne_test15 = v64 != v64;

    report bool ne_test16;
    ne_test16 = 1 as i8 != 0 as i64;
    report bool ne_test17;
    ne_test17 = 1 as i16 != 0 as i64;
    report bool ne_test18;
    ne_test18 = 1 as i32 != 0 as i64;
    report bool ne_test19;
    ne_test19 = 1 as i64 != 0 as i64;
    
    // TEST: >=
    report bool ge_test0;
    ge_test0 = v8 >= v8;
    report bool ge_test1;
    ge_test1 = v8 >= (v16 as i8);
    report bool ge_test2;
    ge_test2 = v8 >= (v32 as i8);
    report bool ge_test3;
    ge_test3 = v8 >= (v64 as i8);

    report bool ge_test4;
    ge_test4 = v16 >= (v8 as i16);
    report bool ge_test5;
    ge_test5 = v16 >= v16;
    report bool ge_test6;
    ge_test6 = v16 >= (v32 as i8);
    report bool ge_test7;
    ge_test7 = v16 >= (v64 as i8);

    report bool ge_test8;
    ge_test8 = v32 >= (v8 as i32);
    report bool ge_test9;
    ge_test9 = v32 >= (v16 as i8);
    report bool ge_test10;
    ge_test10 = v32 >= v32;
    report bool ge_test11;
    ge_test11 = v32 >= (v64 as i8);

    report bool ge_test12;
    ge_test12 = v64 >= (v8 as i64);
    report bool ge_test13;
    ge_test13 = v64 >= (v16 as i8);
    report bool ge_test14;
    ge_test14 = v64 >= (v32 as i8);
    report bool ge_test15;
    ge_test15 = v64 >= v64;

    report bool ge_test16;
    ge_test16 = 1 as i8 >= 0 as i64;
    report bool ge_test17;
    ge_test17 = 1 as i16 >= 0 as i64;
    report bool ge_test18;
    ge_test18 = 1 as i32 >= 0 as i64;
    report bool ge_test19;
    ge_test19 = 1 as i64 >= 0 as i64;
    
    // TEST: >
    report bool gt_test0;
    gt_test0 = v8 > v8;
    report bool gt_test1;
    gt_test1 = v8 > (v16 as i8);
    report bool gt_test2;
    gt_test2 = v8 > (v32 as i8);
    report bool gt_test3;
    gt_test3 = v8 > (v64 as i8);

    report bool gt_test4;
    gt_test4 = v16 > (v8 as i16);
    report bool gt_test5;
    gt_test5 = v16 > v16;
    report bool gt_test6;
    gt_test6 = v16 > (v32 as i8);
    report bool gt_test7;
    gt_test7 = v16 > (v64 as i8);

    report bool gt_test8;
    gt_test8 = v32 > (v8 as i32);
    report bool gt_test9;
    gt_test9 = v32 > (v16 as i8);
    report bool gt_test10;
    gt_test10 = v32 > v32;
    report bool gt_test11;
    gt_test11 = v32 > (v64 as i8);

    report bool gt_test12;
    gt_test12 = v64 > (v8 as i64);
    report bool gt_test13;
    gt_test13 = v64 > (v16 as i8);
    report bool gt_test14;
    gt_test14 = v64 > (v32 as i8);
    report bool gt_test15;
    gt_test15 = v64 > v64;

    report bool gt_test16;
    gt_test16 = 1 as i8 > 0 as i64;
    report bool gt_test17;
    gt_test17 = 1 as i16 > 0 as i64;
    report bool gt_test18;
    gt_test18 = 1 as i32 > 0 as i64;
    report bool gt_test19;
    gt_test19 = 1 as i64 > 0 as i64;
    
    // TEST: <=
    report bool le_test0;
    le_test0 = v8 <= v8;
    report bool le_test1;
    le_test1 = v8 <= (v16 as i8);
    report bool le_test2;
    le_test2 = v8 <= (v32 as i8);
    report bool le_test3;
    le_test3 = v8 <= (v64 as i8);

    report bool le_test4;
    le_test4 = v16 <= (v8 as i16);
    report bool le_test5;
    le_test5 = v16 <= v16;
    report bool le_test6;
    le_test6 = v16 <= (v32 as i8);
    report bool le_test7;
    le_test7 = v16 <= (v64 as i8);

    report bool le_test8;
    le_test8 = v32 <= (v8 as i32);
    report bool le_test9;
    le_test9 = v32 <= (v16 as i8);
    report bool le_test10;
    le_test10 = v32 <= v32;
    report bool le_test11;
    le_test11 = v32 <= (v64 as i8);

    report bool le_test12;
    le_test12 = v64 <= (v8 as i64);
    report bool le_test13;
    le_test13 = v64 <= (v16 as i8);
    report bool le_test14;
    le_test14 = v64 <= (v32 as i8);
    report bool le_test15;
    le_test15 = v64 <= v64;

    report bool le_test16;
    le_test16 = 1 as i8 <= 0 as i64;
    report bool le_test17;
    le_test17 = 1 as i16 <= 0 as i64;
    report bool le_test18;
    le_test18 = 1 as i32 <= 0 as i64;
    report bool le_test19;
    le_test19 = 1 as i64 <= 0 as i64;
    
    // TEST: <
    report bool lt_test0;
    lt_test0 = v8 < v8;
    report bool lt_test1;
    lt_test1 = v8 < (v16 as i8);
    report bool lt_test2;
    lt_test2 = v8 < (v32 as i8);
    report bool lt_test3;
    lt_test3 = v8 < (v64 as i8);

    report bool lt_test4;
    lt_test4 = v16 < (v8 as i16);
    report bool lt_test5;
    lt_test5 = v16 < v16;
    report bool lt_test6;
    lt_test6 = v16 < (v32 as i8);
    report bool lt_test7;
    lt_test7 = v16 < (v64 as i8);

    report bool lt_test8;
    lt_test8 = v32 < (v8 as i32);
    report bool lt_test9;
    lt_test9 = v32 < (v16 as i8);
    report bool lt_test10;
    lt_test10 = v32 < v32;
    report bool lt_test11;
    lt_test11 = v32 < (v64 as i8);

    report bool lt_test12;
    lt_test12 = v64 < (v8 as i64);
    report bool lt_test13;
    lt_test13 = v64 < (v16 as i8);
    report bool lt_test14;
    lt_test14 = v64 < (v32 as i8);
    report bool lt_test15;
    lt_test15 = v64 < v64;

    report bool lt_test16;
    lt_test16 = 1 as i8 < 0 as i64;
    report bool lt_test17;
    lt_test17 = 1 as i16 < 0 as i64;
    report bool lt_test18;
    lt_test18 = 1 as i32 < 0 as i64;
    report bool lt_test19;
    lt_test19 = 1 as i64 < 0 as i64;

    // --- arithmetic operators ---
    // setup
    i8 I8_MIN = -128;
    i8 I8_MAX = 127;

    i16 I16_MIN = -32_768;
    i16 I16_MAX = 32_767;

    i32 I32_MIN = -2_147_483_648;
    i32 I32_MAX = 2_147_483_647;

    i64 I64_MIN = -9_223_372_036_854_775_808;
    i64 I64_MAX = 9_223_372_036_854_775_807;

    // TEST: +
    report i8 add_test0;
    add_test0 = v8 + v8;
    report i8 add_test1;
    add_test1 = v8 + (v16 as i8);
    report i8 add_test2;
    add_test2 = v8 + (v32 as i8);
    report i8 add_test3;
    add_test3 = v8 + (v64 as i8);

    report i16 add_test4;
    add_test4 = v16 + (v8 as i16);
    report i16 add_test5;
    add_test5 = v16 + v16;
    report i16 add_test6;
    add_test6 = v16 + (v32 as i8);
    report i16 add_test7;
    add_test7 = v16 + (v64 as i8);

    report i32 add_test8;
    add_test8 = v32 + (v8 as i32);
    report i32 add_test9;
    add_test9 = v32 + (v16 as i8);
    report i32 add_test10;
    add_test10 = v32 + v32;
    report i32 add_test11;
    add_test11 = v32 + (v64 as i8);

    report i64 add_test12;
    add_test12 = v64 + (v8 as i64);
    report i64 add_test13;
    add_test13 = v64 + (v16 as i8);
    report i64 add_test14;
    add_test14 = v64 + (v32 as i8);
    report i64 add_test15;
    add_test15 = v64 + v64;

    report i8 add_test16;
    add_test16 = 1 as i8 + 0 as i64;
    report i16 add_test17;
    add_test17 = 1 as i16 + 0 as i64;
    report i32 add_test18;
    add_test18 = 1 as i32 + 0 as i64;
    report i64 add_test19;
    add_test19 = 1 as i64 + 0 as i64;

    report i8 add_test20;
    add_test20 = I8_MAX + 1;
    report i16 add_test21;
    add_test21 = I16_MAX + 1;
    report i32 add_test22;
    add_test22 = I32_MAX + 1;
    report i64 add_test23;
    add_test23 = I64_MAX + 1;

    // TEST: -
    report i8 sub_test0;
    sub_test0 = v8 - v8;
    report i8 sub_test1;
    sub_test1 = v8 - (v16 as i8);
    report i8 sub_test2;
    sub_test2 = v8 - (v32 as i8);
    report i8 sub_test3;
    sub_test3 = v8 - (v64 as i8);

    report i16 sub_test4;
    sub_test4 = v16 - (v8 as i16);
    report i16 sub_test5;
    sub_test5 = v16 - v16;
    report i16 sub_test6;
    sub_test6 = v16 - (v32 as i8);
    report i16 sub_test7;
    sub_test7 = v16 - (v64 as i8);

    report i32 sub_test8;
    sub_test8 = v32 - (v8 as i32);
    report i32 sub_test9;
    sub_test9 = v32 - (v16 as i8);
    report i32 sub_test10;
    sub_test10 = v32 - v32;
    report i32 sub_test11;
    sub_test11 = v32 - (v64 as i8);

    report i64 sub_test12;
    sub_test12 = v64 - (v8 as i64);
    report i64 sub_test13;
    sub_test13 = v64 - (v16 as i8);
    report i64 sub_test14;
    sub_test14 = v64 - (v32 as i8);
    report i64 sub_test15;
    sub_test15 = v64 - v64;

    report i8 sub_test16;
    sub_test16 = 1 as i8 - 0 as i64;
    report i16 sub_test17;
    sub_test17 = 1 as i16 - 0 as i64;
    report i32 sub_test18;
    sub_test18 = 1 as i32 - 0 as i64;
    report i64 sub_test19;
    sub_test19 = 1 as i64 - 0 as i64;

    report i8 sub_test20;
    sub_test20 = I8_MIN - 1;
    report i16 sub_test21;
    sub_test21 = I16_MIN - 1;
    report i32 sub_test22;
    sub_test22 = I32_MIN - 1;
    report i64 sub_test23;
    sub_test23 = I64_MIN - 1;

    // TEST: *
    report i8 mul_test0;
    mul_test0 = v8 * v8;
    report i8 mul_test1;
    mul_test1 = v8 * (v16 as i8);
    report i8 mul_test2;
    mul_test2 = v8 * (v32 as i8);
    report i8 mul_test3;
    mul_test3 = v8 * (v64 as i8);

    report i16 mul_test4;
    mul_test4 = v16 * (v8 as i16);
    report i16 mul_test5;
    mul_test5 = v16 * v16;
    report i16 mul_test6;
    mul_test6 = v16 * (v32 as i8);
    report i16 mul_test7;
    mul_test7 = v16 * (v64 as i8);

    report i32 mul_test8;
    mul_test8 = v32 * (v8 as i32);
    report i32 mul_test9;
    mul_test9 = v32 * (v16 as i8);
    report i32 mul_test10;
    mul_test10 = v32 * v32;
    report i32 mul_test11;
    mul_test11 = v32 * (v64 as i8);

    report i64 mul_test12;
    mul_test12 = v64 * (v8 as i64);
    report i64 mul_test13;
    mul_test13 = v64 * (v16 as i8);
    report i64 mul_test14;
    mul_test14 = v64 * (v32 as i8);
    report i64 mul_test15;
    mul_test15 = v64 * v64;

    report i8 mul_test16;
    mul_test16 = 1 as i8 * 0 as i64;
    report i16 mul_test17;
    mul_test17 = 1 as i16 * 0 as i64;
    report i32 mul_test18;
    mul_test18 = 1 as i32 * 0 as i64;
    report i64 mul_test19;
    mul_test19 = 1 as i64 * 0 as i64;

    report i8 mul_test20;
    mul_test20 = I8_MIN * 1;
    report i16 mul_test21;
    mul_test21 = I16_MIN * 1;
    report i32 mul_test22;
    mul_test22 = I32_MIN * 1;
    report i64 mul_test23;
    mul_test23 = I64_MIN * 1;
    
    report i8 mul_test24;
    mul_test24 = I8_MIN * 2;
    report i16 mul_test25;
    mul_test25 = I16_MIN * 2;
    report i32 mul_test26;
    mul_test26 = I32_MIN * 2;
    report i64 mul_test27;
    mul_test27 = I64_MIN * 2;

    report i8 mul_test20;
    mul_test20 = I8_MAX * 1;
    report i16 mul_test21;
    mul_test21 = I16_MAX * 1;
    report i32 mul_test22;
    mul_test22 = I32_MAX * 1;
    report i64 mul_test23;
    mul_test23 = I64_MAX * 1;
    
    report i8 mul_test24;
    mul_test24 = I8_MAX * 2;
    report i16 mul_test25;
    mul_test25 = I16_MAX * 2;
    report i32 mul_test26;
    mul_test26 = I32_MAX * 2;
    report i64 mul_test27;
    mul_test27 = I64_MAX * 2;

    // TEST: /
    report i8 div_test0;
    div_test0 = v8 / v8;
    report i8 div_test1;
    div_test1 = v8 / (v16 as i8);
    report i8 div_test2;
    div_test2 = v8 / (v32 as i8);
    report i8 div_test3;
    div_test3 = v8 / (v64 as i8);

    report i16 div_test4;
    div_test4 = v16 / (v8 as i16);
    report i16 div_test5;
    div_test5 = v16 / v16;
    report i16 div_test6;
    div_test6 = v16 / (v32 as i8);
    report i16 div_test7;
    div_test7 = v16 / (v64 as i8);

    report i32 div_test8;
    div_test8 = v32 / (v8 as i32);
    report i32 div_test9;
    div_test9 = v32 / (v16 as i8);
    report i32 div_test10;
    div_test10 = v32 / v32;
    report i32 div_test11;
    div_test11 = v32 / (v64 as i8);

    report i64 div_test12;
    div_test12 = v64 / (v8 as i64);
    report i64 div_test13;
    div_test13 = v64 / (v16 as i8);
    report i64 div_test14;
    div_test14 = v64 / (v32 as i8);
    report i64 div_test15;
    div_test15 = v64 / v64;

    report i8 div_test16;
    div_test16 = 1 as i8 / 0 as i64;
    report i16 div_test17;
    div_test17 = 1 as i16 / 0 as i64;
    report i32 div_test18;
    div_test18 = 1 as i32 / 0 as i64;
    report i64 div_test19;
    div_test19 = 1 as i64 / 0 as i64;

    report i8 div_test20;
    div_test20 = I8_MIN / 1;
    report i16 div_test21;
    div_test21 = I16_MIN / 1;
    report i32 div_test22;
    div_test22 = I32_MIN / 1;
    report i64 div_test23;
    div_test23 = I64_MIN / 1;
    
    // TEST: %
    report i8 mod_test0;
    mod_test0 = v8 % v8;
    report i8 mod_test1;
    mod_test1 = v8 % (v16 as i8);
    report i8 mod_test2;
    mod_test2 = v8 % (v32 as i8);
    report i8 mod_test3;
    mod_test3 = v8 % (v64 as i8);

    report i16 mod_test4;
    mod_test4 = v16 % (v8 as i16);
    report i16 mod_test5;
    mod_test5 = v16 % v16;
    report i16 mod_test6;
    mod_test6 = v16 % (v32 as i8);
    report i16 mod_test7;
    mod_test7 = v16 % (v64 as i8);

    report i32 mod_test8;
    mod_test8 = v32 % (v8 as i32);
    report i32 mod_test9;
    mod_test9 = v32 % (v16 as i8);
    report i32 mod_test10;
    mod_test10 = v32 % v32;
    report i32 mod_test11;
    mod_test11 = v32 % (v64 as i8);

    report i64 mod_test12;
    mod_test12 = v64 % (v8 as i64);
    report i64 mod_test13;
    mod_test13 = v64 % (v16 as i8);
    report i64 mod_test14;
    mod_test14 = v64 % (v32 as i8);
    report i64 mod_test15;
    mod_test15 = v64 % v64;

    report i8 mod_test16;
    mod_test16 = 1 as i8 % 0 as i64;
    report i16 mod_test17;
    mod_test17 = 1 as i16 % 0 as i64;
    report i32 mod_test18;
    mod_test18 = 1 as i32 % 0 as i64;
    report i64 mod_test19;
    mod_test19 = 1 as i64 % 0 as i64;

    report i8 mod_test20;
    mod_test20 = I8_MIN % 1;
    report i16 mod_test21;
    mod_test21 = I16_MIN % 1;
    report i32 mod_test22;
    mod_test22 = I32_MIN % 1;
    report i64 mod_test23;
    mod_test23 = I64_MIN % 1;

    // ==== UNOP ====
    // --- casts ---

    // TEST: implicit with primitive
    report i8 imp_test0;
    imp_test0 = v8 + 1;
    report i16 imp_test1;
    imp_test1 = v16 + 1;
    report i32 imp_test2;
    imp_test2 = v32 + 1;
    report i64 imp_test3;
    imp_test3 = v64 + 1;

    report i8 imp_test4;
    imp_test4 = 1 + v8;
    report i16 imp_test5;
    imp_test5 = 1 + v16;
    report i32 imp_test6;
    imp_test6 = 1 + v32;
    report i64 imp_test7;
    imp_test7 = 1 + v64;

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation
    report i8 trunc_test0;
    trunc_test0 = I16_MAX as i8;
    report i16 trunc_test1;
    trunc_test1 = I32_MAX as i16;
    report i32 trunc_test2;
    trunc_test2 = I64_MAX as i32;

    // TEST: extension (unsigned)
    report i16 trunc_test0;
    trunc_test0 = I8_MAX as i16;
    report i32 trunc_test1;
    trunc_test1 = I16_MAX as i32;
    report i64 trunc_test2;
    trunc_test2 = I32_MAX as i64;

    // TEST: extension (signed)
    report i16 trunc_test0;
    trunc_test0 = I8_MIN as i16;
    report i32 trunc_test1;
    trunc_test1 = I16_MIN as i32;
    report i64 trunc_test2;
    trunc_test2 = I32_MIN as i64;
}