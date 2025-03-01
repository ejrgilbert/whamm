wasm:opcode:call(arg0: f64, arg1: f32, arg2: i64, arg3: u64, arg4: i32, arg5: u32, arg6: i16, arg7: u16, arg8: i8, arg9: u8):before /
    arg9 == 1
/ {
    // Call target, parameters are used as type values (all are set to '1'):
    // type signature: (u8, i8, u16, i16, u32, i32, u64, i64, f32, f64)
    // stack order: (f64, f32, i64, u64, i32, u32, i16, u16, i8, u8)

    var v_u8: u8 = arg9;
    var v_i8: i8 = arg8;
    var v_u16: u16 = arg7;
    var v_i16: i16 = arg6;
    var v_u32: u32 = arg5;
    var v_i32: i32 = arg4;
    var v_u64: u64 = arg3;
    var v_i64: i64 = arg2;
    var v_f32: f32 = arg1;
    var v_f64: f64 = arg0;

    var TWO: u8 = v_u8 + v_u8;
    var NEG: i8 = 0 - v_i8;
    
    // ==== binary operations ====

    // --- <<
    report var shl_test0: u8;
    shl_test0 = v_u8 << TWO;
    report var shl_test1: u8;
    shl_test1 = (NEG as u8) << TWO;
    report var shl_test2: i8;
    shl_test2 = v_i8 << (TWO as i8);
    report var shl_test3: i8;
    shl_test3 = (NEG as i8) << (TWO as i8);

    report var shl_test4: u16;
    shl_test4 = v_u16 << (TWO as u16);
    report var shl_test5: u16;
    shl_test5 = (NEG as u16) << (TWO as u16);
    report var shl_test6: i16;
    shl_test6 = v_i16 << (TWO as i16);
    report var shl_test7: i16;
    shl_test7 = (NEG as i16) << (TWO as i16);

    report var shl_test8: u32;
    shl_test8 = v_u32 << (TWO as u32);
    report var shl_test9: u32;
    shl_test9 = (NEG as u32) << (TWO as u32);
    report var shl_test10: i32;
    shl_test10 = v_i32 << (TWO as i32);
    report var shl_test11: i32;
    shl_test11 = (NEG as i32) << (TWO as i32);

    report var shl_test12: u64;
    shl_test12 = v_u64 << (TWO as u64);
    report var shl_test13: u64;
    shl_test13 = (NEG as u64) << (TWO as u64);
    report var shl_test14: i64;
    shl_test14 = v_i64 << (TWO as i64);
    report var shl_test15: i64;
    shl_test15 = (NEG as i64) << (TWO as i64);

    // --- >>
    report var shr_test0: u8;
    shr_test0 = v_u8 >> (TWO as u8);
    report var shr_test1: u8;
    shr_test1 = (NEG as u8) >> (TWO as u8);
    report var shr_test2: i8;
    shr_test2 = v_i8 >> (TWO as i8);
    report var shr_test3: i8;
    shr_test3 = NEG >> (TWO as i8);

    report var shr_test4: u16;
    shr_test4 = v_u16 >> (TWO as u16);
    report var shr_test5: u16;
    shr_test5 = (NEG as u16) >> (TWO as u16);
    report var shr_test6: i16;
    shr_test6 = v_i16 >> (TWO as i16);
    report var shr_test7: i16;
    shr_test7 = (NEG as i16) >> (TWO as i16);

    report var shr_test8: u32;
    shr_test8 = v_u32 >> (TWO as u32);
    report var shr_test9: u32;
    shr_test9 = (NEG as u32) >> (TWO as u32);
    report var shr_test10: i32;
    shr_test10 = v_i32 >> (TWO as i32);
    report var shr_test11: i32;
    shr_test11 = (NEG as i32) >> (TWO as i32);

    report var shr_test12: u64;
    shr_test12 = v_u64 >> (TWO as u64);
    report var shr_test13: u64;
    shr_test13 = (NEG as u64) >> (TWO as u64);
    report var shr_test14: i64;
    shr_test14 = v_i64 >> (TWO as i64);
    report var shr_test15: i64;
    shr_test15 = (NEG as i64) >> (TWO as i64);

    // --- &

    report var and_test0: u8;
    and_test0 = v_u8 & v_u8;
    report var and_test1: u8;
    and_test1 = (NEG as u8) & v_u8;
    report var and_test2: i8;
    and_test2 = v_i8 & v_i8;
    report var and_test3: i8;
    and_test3 = (NEG as i8) & v_i8;

    report var and_test4: u16;
    and_test4 = v_u16 & v_u16;
    report var and_test5: u16;
    and_test5 = (NEG as u16) & v_u16;
    report var and_test6: i16;
    and_test6 = v_i16 & v_i16;
    report var and_test7: i16;
    and_test7 = (NEG as i16) & v_i16;

    report var and_test8: u32;
    and_test8 = v_u32 & v_u32;
    report var and_test9: u32;
    and_test9 = (NEG as u32) & v_u32;
    report var and_test10: i32;
    and_test10 = v_i32 & v_i32;
    report var and_test11: i32;
    and_test11 = (NEG as i32) & v_i32;

    report var and_test12: u64;
    and_test12 = v_u64 & v_u64;
    report var and_test13: u64;
    and_test13 = (NEG as u64) & v_u64;
    report var and_test14: i64;
    and_test14 = v_i64 & v_i64;
    report var and_test15: i64;
    and_test15 = (NEG as i64) & v_i64;

    // --- |

    report var or_test0: u8;
    or_test0 = v_u8 | 0xF;
    report var or_test1: u8;
    or_test1 = (NEG as u8) | 0xF;
    report var or_test2: i8;
    or_test2 = v_i8 | 0xF;
    report var or_test3: i8;
    or_test3 = (NEG as i8) | 0xF;

    report var or_test4: u16;
    or_test4 = v_u16 | 0xF;
    report var or_test5: u16;
    or_test5 = (NEG as u16) | 0xF;
    report var or_test6: i16;
    or_test6 = v_i16 | 0xF;
    report var or_test7: i16;
    or_test7 = (NEG as i16) | 0xF;

    report var or_test8: u32;
    or_test8 = v_u32 | 0xF;
    report var or_test9: u32;
    or_test9 = (NEG as u32) | 0xF;
    report var or_test10: i32;
    or_test10 = v_i32 | 0xF;
    report var or_test11: i32;
    or_test11 = (NEG as i32) | 0xF;

    report var or_test12: u64;
    or_test12 = v_u64 | 0xF;
    report var or_test13: u64;
    or_test13 = (NEG as u64) | 0xF;
    report var or_test14: i64;
    or_test14 = v_i64 | 0xF;
    report var or_test15: i64;
    or_test15 = (NEG as i64) | 0xF;

    // --- ^

    report var xor_test0: u8;
    xor_test0 = v_u8 ^ 0xF;
    report var xor_test1: u8;
    xor_test1 = (NEG as u8) ^ 0xF;
    report var xor_test2: i8;
    xor_test2 = v_i8 ^ 0xF;
    report var xor_test3: i8;
    xor_test3 = (NEG as i8) ^ 0xF;

    report var xor_test4: u16;
    xor_test4 = v_u16 ^ 0xF;
    report var xor_test5: u16;
    xor_test5 = (NEG as u16) ^ 0xF;
    report var xor_test6: i16;
    xor_test6 = v_i16 ^ 0xF;
    report var xor_test7: i16;
    xor_test7 = (NEG as i16) ^ 0xF;

    report var xor_test8: u32;
    xor_test8 = v_u32 ^ 0xF;
    report var xor_test9: u32;
    xor_test9 = (NEG as u32) ^ 0xF;
    report var xor_test10: i32;
    xor_test10 = v_i32 ^ 0xF;
    report var xor_test11: i32;
    xor_test11 = (NEG as i32) ^ 0xF;

    report var xor_test12: u64;
    xor_test12 = v_u64 ^ 0xF;
    report var xor_test13: u64;
    xor_test13 = (NEG as u64) ^ 0xF;
    report var xor_test14: i64;
    xor_test14 = v_i64 ^ 0xF;
    report var xor_test15: i64;
    xor_test15 = (NEG as i64) ^ 0xF;

    // --- ~

    report var bwise_not_test0: u8;
    bwise_not_test0 = ~ v_u8;
    report var bwise_not_test1: u8;
    bwise_not_test1 = ~ (NEG as u8);
    report var bwise_not_test2: i8;
    bwise_not_test2 = ~ v_i8;
    report var bwise_not_test3: i8;
    bwise_not_test3 = ~ NEG;

    report var bwise_not_test4: u16;
    bwise_not_test4 = ~ v_u16;
    report var bwise_not_test5: u16;
    bwise_not_test5 = ~ (NEG as u16);
    report var bwise_not_test6: i16;
    bwise_not_test6 = ~ v_i16;
    report var bwise_not_test7: i16;
    bwise_not_test7 = ~ (NEG as i16);

    report var bwise_not_test8: u32;
    bwise_not_test8 = ~ v_u32;
    report var bwise_not_test9: u32;
    bwise_not_test9 = ~ (NEG as u32);
    report var bwise_not_test10: i32;
    bwise_not_test10 = ~ v_i32;
    report var bwise_not_test11: i32;
    bwise_not_test11 = ~ (NEG as i32);

    report var bwise_not_test12: u64;
    bwise_not_test12 = ~ v_u64;
    report var bwise_not_test13: u64;
    bwise_not_test13 = ~ (NEG as u64);
    report var bwise_not_test14: i64;
    bwise_not_test14 = ~ v_i64;
    report var bwise_not_test15: i64;
    bwise_not_test15 = ~ (NEG as i64);
}