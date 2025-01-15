wasm:opcode:call:before {
    // ==== BINOP ====

    // --- logical operators ---
    // setup
    f32 t_32 = 1;
    f32 f_32 = 0;
    f64 t_64 = 1;
    f64 f_64 = 0;

    // TEST: and
    report bool and_test0;
    and_test0 = t_32 as bool && f_32 as bool;
    report bool and_test1;
    and_test1 = t_64 as bool && f_64 as bool;

    // TEST: or
    report bool or_test0;
    or_test0 = t_32 as bool || f_32 as bool;
    report bool or_test1;
    or_test1 = t_64 as bool || f_64 as bool;

    // --- relational operators ---
    // setup
    f32 v32 = 1;
    f64 v64 = 1;

    // TEST: ==

    report bool eq_test0;
    eq_test0 = v32 == v32;
    report bool eq_test1;
    eq_test1 = v32 == (v64 as v32);

    report bool eq_test2;
    eq_test2 = v64 == (v32 as u64);
    report bool eq_test3;
    eq_test3 = v64 == v64;

    report bool eq_test4;
    eq_test4 = 1 as f32 == 0 as f32;
    report bool eq_test5;
    eq_test5 = 1 as f64 == 0 as f64;
    report bool eq_test6;
    eq_test6 = 1 as f32 == 0 as f64;
    report bool eq_test7;
    eq_test7 = 1 as f64 == 0 as f64;
    
    // TEST: !=

    report bool ne_test0;
    ne_test10 = v32 != v32;
    report bool ne_test1;
    ne_test11 = v32 != (v64 as f32);

    report bool ne_test2;
    ne_test2 = v64 != (v32 as f64);
    report bool ne_test3;
    ne_test3 = v64 != v64;

    report bool ne_test4;
    ne_test4 = 1 as f32 != 0 as f32;
    report bool ne_test5;
    ne_test5 = 1 as f64 != 0 as f64;
    report bool ne_test6;
    ne_test6 = 1 as f32 != 0 as f64;
    report bool ne_test7;
    ne_test7 = 1 as f64 != 0 as f64;
    
    // TEST: >=
    report bool ge_test0;
    ge_test10 = v32 >= v32;
    report bool ge_test1;
    ge_test11 = v32 >= (v64 as f32);

    report bool ge_test2;
    ge_test2 = v64 >= (v32 as f64);
    report bool ge_test3;
    ge_test3 = v64 >= v64;

    report bool ge_test4;
    ge_test4 = 1 as f32 >= 0 as f32;
    report bool ge_test5;
    ge_test5 = 1 as f64 >= 0 as f64;
    report bool ge_test6;
    ge_test6 = 1 as f32 >= 0 as f64;
    report bool ge_test7;
    ge_test7 = 1 as f64 >= 0 as f64;
    
    // TEST: >
    report bool gt_test0;
    gt_test10 = v32 > v32;
    report bool gt_test1;
    gt_test11 = v32 > (v64 as f32);

    report bool gt_test2;
    gt_test2 = v64 > (v32 as f64);
    report bool gt_test3;
    gt_test3 = v64 > v64;

    report bool gt_test4;
    gt_test4 = 1 as f32 > 0 as f32;
    report bool gt_test5;
    gt_test5 = 1 as f64 > 0 as f64;
    report bool gt_test6;
    gt_test6 = 1 as f32 > 0 as f64;
    report bool gt_test7;
    gt_test7 = 1 as f64 > 0 as f64;
    
    // TEST: <=
    report bool le_test0;
    le_test10 = v32 <= v32;
    report bool le_test1;
    le_test11 = v32 <= (v64 as f32);

    report bool le_test2;
    le_test2 = v64 <= (v32 as f64);
    report bool le_test3;
    le_test3 = v64 <= v64;

    report bool le_test4;
    le_test4 = 1 as f32 <= 0 as f32;
    report bool le_test5;
    le_test5 = 1 as f64 <= 0 as f64;
    report bool le_test6;
    le_test6 = 1 as f32 <= 0 as f64;
    report bool le_test7;
    le_test7 = 1 as f64 <= 0 as f64;
    
    // TEST: <
    report bool lt_test0;
    lt_test10 = v32 < v32;
    report bool lt_test1;
    lt_test11 = v32 < (v64 as f32);

    report bool lt_test2;
    lt_test2 = v64 < (v32 as f64);
    report bool lt_test3;
    lt_test3 = v64 < v64;

    report bool lt_test4;
    lt_test4 = 1 as f32 < 0 as f32;
    report bool lt_test5;
    lt_test5 = 1 as f64 < 0 as f64;
    report bool lt_test6;
    lt_test6 = 1 as f32 < 0 as f64;
    report bool lt_test7;
    lt_test7 = 1 as f64 < 0 as f64;

    // --- arithmetic operators ---
    // setup
    f32 F32_MIN = -3.40282347E+38;
    f32 F32_MAX = 3.40282347E+38;

    f64 F64_MIN = -1.7976931348623157E+308;
    f64 F64_MAX = 1.7976931348623157E+308;

    // TEST: +
    report bool add_test0;
    add_test10 = v32 + v32;
    report bool add_test1;
    add_test11 = v32 + (v64 as f32);

    report bool add_test2;
    add_test2 = v64 + (v32 as f64);
    report bool add_test3;
    add_test3 = v64 + v64;

    report bool add_test4;
    add_test4 = 1 as f32 + 0 as f32;
    report bool add_test5;
    add_test5 = 1 as f64 + 0 as f64;
    report bool add_test6;
    add_test6 = 1 as f32 + 0 as f64;
    report bool add_test7;
    add_test7 = 1 as f64 + 0 as f64;

    report f32 add_test8;
    add_test8 = F32_MAX + 1;
    report f64 add_test9;
    add_test9 = F64_MAX + 1;

    // TEST: -
    report bool sub_test0;
    sub_test10 = v32 - v32;
    report bool sub_test1;
    sub_test11 = v32 - (v64 as f32);

    report bool sub_test2;
    sub_test2 = v64 - (v32 as f64);
    report bool sub_test3;
    sub_test3 = v64 - v64;

    report bool sub_test4;
    sub_test4 = 1 as f32 - 0 as f32;
    report bool sub_test5;
    sub_test5 = 1 as f64 - 0 as f64;
    report bool sub_test6;
    sub_test6 = 1 as f32 - 0 as f64;
    report bool sub_test7;
    sub_test7 = 1 as f64 - 0 as f64;

    report f32 sub_test8;
    sub_test8 = F32_MIN - 1;
    report f64 sub_test9;
    sub_test9 = F64_MIN - 1;

    // TEST: *
    report bool mul_test0;
    mul_test10 = v32 * v32;
    report bool mul_test1;
    mul_test11 = v32 * (v64 as f32);

    report bool mul_test2;
    mul_test2 = v64 * (v32 as f64);
    report bool mul_test3;
    mul_test3 = v64 * v64;

    report bool mul_test4;
    mul_test4 = 1 as f32 * 0 as f32;
    report bool mul_test5;
    mul_test5 = 1 as f64 * 0 as f64;
    report bool mul_test6;
    mul_test6 = 1 as f32 * 0 as f64;
    report bool mul_test7;
    mul_test7 = 1 as f64 * 0 as f64;

    report f32 mul_test8;
    mul_test8 = F32_MIN * 1;
    report f64 mul_test9;
    mul_test9 = F64_MIN * 1;
    
    report f32 mul_test8;
    mul_test8 = F32_MIN * 2;
    report f64 mul_test9;
    mul_test9 = F64_MIN * 2;
    
    report f32 mul_test8;
    mul_test8 = F32_MAX * 1;
    report f64 mul_test9;
    mul_test9 = F64_MAX * 1;
    
    report f32 mul_test8;
    mul_test8 = F32_MAX * 2;
    report f64 mul_test9;
    mul_test9 = F64_MAX * 2;

    // TEST: /
    report bool div_test0;
    div_test10 = v32 / v32;
    report bool div_test1;
    div_test11 = v32 / (v64 as f32);

    report bool div_test2;
    div_test2 = v64 / (v32 as f64);
    report bool div_test3;
    div_test3 = v64 / v64;

    report bool div_test4;
    div_test4 = 1 as f32 / 0 as f32;
    report bool div_test5;
    div_test5 = 1 as f64 / 0 as f64;
    report bool div_test6;
    div_test6 = 1 as f32 / 0 as f64;
    report bool div_test7;
    div_test7 = 1 as f64 / 0 as f64;

    report f32 div_test8;
    div_test8 = F32_MIN / 1;
    report f64 div_test9;
    div_test9 = F64_MIN / 1;
    
    // TEST: %
    report bool mod_test0;
    mod_test10 = v32 % v32;
    report bool mod_test1;
    mod_test11 = v32 % (v64 as f32);

    report bool mod_test2;
    mod_test2 = v64 % (v32 as f64);
    report bool mod_test3;
    mod_test3 = v64 % v64;

    report bool mod_test4;
    mod_test4 = 1 as f32 % 0 as f32;
    report bool mod_test5;
    mod_test5 = 1 as f64 % 0 as f64;
    report bool mod_test6;
    mod_test6 = 1 as f32 % 0 as f64;
    report bool mod_test7;
    mod_test7 = 1 as f64 % 0 as f64;

    report f32 mod_test8;
    mod_test8 = F32_MIN % 1;
    report f64 mod_test9;
    mod_test9 = F64_MIN % 1;

    // ==== UNOP ====
    // --- casts ---

    // TEST: implicit with primitive
    report f32 imp_test0;
    imp_test0 = v32 + 1;
    report f64 imp_test1;
    imp_test1 = v64 + 1;

    report f32 imp_test4;
    imp_test4 = 1 + v32;
    report f64 imp_test5;
    imp_test5 = 1 + v64;

    // TEST: explicit (done in above BINOP tests)

    // TEST: truncation (unsigned)
    report f32 trunc_test0;
    trunc_test0 = F64_MAX as f32;
    // TEST: truncation (signed)
    report f64 trunc_test0;
    trunc_test0 = F64_MIN as f32;

    // TEST: extension (unsigned)
    report f64 trunc_test0;
    trunc_test0 = F32_MAX as f64;
    // TEST: extension (signed)
    report f64 trunc_test0;
    trunc_test0 = F32_MIN as f64;
}