wasm:opcode:call:before {
    // Call target, parameters are used as type values (all are set to '1'):
    // (u8, i8, u16, i16, u32, i32, u64, i64, f32, f64)
    
    u8 v_u8 = arg0;
    i8 v_i8 = arg1;
    u16 v_u16 = arg2;
    i16 v_i16 = arg3;
    u32 v_u32 = arg4;
    i32 v_i32 = arg5;
    u64 v_u64 = arg6;
    i64 v_i64 = arg7;
    f32 v_f32 = arg8;
    f64 v_f64 = arg9;
    
    u8 TWO = v_u8 + v_u8;
    i8 NEG = 0 - v_i8;
    
    // ==== binary operations ====

    // --- <<
    report u8 shl_test0;
    shl_test0 = v_u8 << TWO;
    report u8 shl_test1;
    shl_test1 = (NEG as u8) << TWO;
    report i8 shl_test2;
    shl_test2 = v_i8 << (TWO as i8);
    report i8 shl_test3;
    shl_test3 = (NEG as i8) << (TWO as i8);

    report u16 shl_test4;
    shl_test4 = v_u16 << (TWO as u16);
    report u16 shl_test5;
    shl_test5 = (NEG as u16) << (TWO as u16);
    report i16 shl_test6;
    shl_test6 = v_i16 << (TWO as i16);
    report i16 shl_test7;
    shl_test7 = (NEG as i16) << (TWO as i16);

    report u32 shl_test8;
    shl_test8 = v_u32 << (TWO as u32);
    report u32 shl_test9;
    shl_test9 = (NEG as u32) << (TWO as u32);
    report i32 shl_test10;
    shl_test10 = v_i32 << (TWO as i32);
    report i32 shl_test11;
    shl_test11 = (NEG as i32) << (TWO as i32);

    report u64 shl_test12;
    shl_test12 = v_u64 << (TWO as u64);
    report u64 shl_test13;
    shl_test13 = (NEG as u64) << (TWO as u64);
    report i64 shl_test14;
    shl_test14 = v_i64 << (TWO as i64);
    report i64 shl_test15;
    shl_test15 = (NEG as i64) << (TWO as i64);

    // --- >>
    report u8 shr_test0;
    shr_test0 = v_u8 >> (TWO as u8);
    report u8 shr_test1;
    shr_test1 = (NEG as u8) >> (TWO as u8);
    report i8 shr_test2;
    shr_test2 = v_i8 >> (TWO as i8);
    report i8 shr_test3;
    shr_test3 = NEG >> (TWO as i8);
    
    report u16 shr_test4;
    shr_test4 = v_u16 >> (TWO as u16);
    report u16 shr_test5;
    shr_test5 = (NEG as u16) >> (TWO as u16);
    report i16 shr_test6;
    shr_test6 = v_i16 >> (TWO as i16);
    report i16 shr_test7;
    shr_test7 = (NEG as i16) >> (TWO as i16);
    
    report u32 shr_test8;
    shr_test8 = v_u32 >> (TWO as u32);
    report u32 shr_test9;
    shr_test9 = (NEG as u32) >> (TWO as u32);
    report i32 shr_test10;
    shr_test10 = v_i32 >> (TWO as i32);
    report i32 shr_test11;
    shr_test11 = (NEG as i32) >> (TWO as i32);
    
    report u64 shr_test12;
    shr_test12 = v_u64 >> (TWO as u64);
    report u64 shr_test13;
    shr_test13 = (NEG as u64) >> (TWO as u64);
    report i64 shr_test14;
    shr_test14 = v_i64 >> (TWO as i64);
    report i64 shr_test15;
    shr_test15 = (NEG as i64) >> (TWO as i64);
    
    // --- &
    
    report u8 and_test0;
    and_test0 = v_u8 & v_u8;
    report u8 and_test1;
    and_test1 = (NEG as u8) & v_u8;
    report i8 and_test2;
    and_test2 = v_i8 & v_i8;
    report i8 and_test3;
    and_test3 = (NEG as i8) & v_i8;
    
    report u16 and_test4;
    and_test4 = v_u16 & v_u16;
    report u16 and_test5;
    and_test5 = (NEG as u16) & v_u16;
    report i16 and_test6;
    and_test6 = v_i16 & v_i16;
    report i16 and_test7;
    and_test7 = (NEG as i16) & v_i16;
    
    report u32 and_test8;
    and_test8 = v_u32 & v_u32;
    report u32 and_test9;
    and_test9 = (NEG as u32) & v_u32;
    report i32 and_test10;
    and_test10 = v_i32 & v_i32;
    report i32 and_test11;
    and_test11 = (NEG as i32) & v_i32;
    
    report u64 and_test12;
    and_test12 = v_u64 & v_u64;
    report u64 and_test13;
    and_test13 = (NEG as u64) & v_u64;
    report i64 and_test14;
    and_test14 = v_i64 & v_i64;
    report i64 and_test15;
    and_test15 = (NEG as i64) & v_i64;
    
    // --- |
    
    report u8 or_test0;
    or_test0 = v_u8 | 0xF;
    report u8 or_test1;
    or_test1 = (NEG as u8) | 0xF;
    report i8 or_test2;
    or_test2 = v_i8 | 0xF;
    report i8 or_test3;
    or_test3 = (NEG as i8) | 0xF;
    
    report u16 or_test4;
    or_test4 = v_u16 | 0xF;
    report u16 or_test5;
    or_test5 = (NEG as u16) | 0xF;
    report i16 or_test6;
    or_test6 = v_i16 | 0xF;
    report i16 or_test7;
    or_test7 = (NEG as i16) | 0xF;
    
    report u32 or_test8;
    or_test8 = v_u32 | 0xF;
    report u32 or_test9;
    or_test9 = (NEG as u32) | 0xF;
    report i32 or_test10;
    or_test10 = v_i32 | 0xF;
    report i32 or_test11;
    or_test11 = (NEG as i32) | 0xF;
    
    report u64 or_test12;
    or_test12 = v_u64 | 0xF;
    report u64 or_test13;
    or_test13 = (NEG as u64) | 0xF;
    report i64 or_test14;
    or_test14 = v_i64 | 0xF;
    report i64 or_test15;
    or_test15 = (NEG as i64) | 0xF;
    
    // --- ^
    
    report u8 xor_test0;
    xor_test0 = v_u8 ^ 0xF;
    report u8 xor_test1;
    xor_test1 = (NEG as u8) ^ 0xF;
    report i8 xor_test2;
    xor_test2 = v_i8 ^ 0xF;
    report i8 xor_test3;
    xor_test3 = (NEG as i8) ^ 0xF;
    
    report u16 xor_test4;
    xor_test4 = v_u16 ^ 0xF;
    report u16 xor_test5;
    xor_test5 = (NEG as u16) ^ 0xF;
    report i16 xor_test6;
    xor_test6 = v_i16 ^ 0xF;
    report i16 xor_test7;
    xor_test7 = (NEG as i16) ^ 0xF;
    
    report u32 xor_test8;
    xor_test8 = v_u32 ^ 0xF;
    report u32 xor_test9;
    xor_test9 = (NEG as u32) ^ 0xF;
    report i32 xor_test10;
    xor_test10 = v_i32 ^ 0xF;
    report i32 xor_test11;
    xor_test11 = (NEG as i32) ^ 0xF;
    
    report u64 xor_test12;
    xor_test12 = v_u64 ^ 0xF;
    report u64 xor_test13;
    xor_test13 = (NEG as u64) ^ 0xF;
    report i64 xor_test14;
    xor_test14 = v_i64 ^ 0xF;
    report i64 xor_test15;
    xor_test15 = (NEG as i64) ^ 0xF;
}