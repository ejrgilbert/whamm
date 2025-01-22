wasm:opcode:call:before {
    // ==== binary operations ====
    
    // --- <<
    report u8 shl_test0;
    shl_test0 = 1 << 2;
    report u8 shl_test1;
    shl_test1 = -1 << 2;
    report i8 shl_test2;
    shl_test2 = 1 << 2;
    report i8 shl_test3;
    shl_test3 = -1 << 2;
    
    report u16 shl_test4;
    shl_test4 = 1 << 2;
    report u16 shl_test5;
    shl_test5 = -1 << 2;
    report i16 shl_test6;
    shl_test6 = 1 << 2;
    report i16 shl_test7;
    shl_test7 = -1 << 2;
    
    report u32 shl_test8;
    shl_test8 = 1 << 2;
    report u32 shl_test9;
    shl_test9 = -1 << 2;
    report i32 shl_test10;
    shl_test10 = 1 << 2;
    report i32 shl_test11;
    shl_test11 = -1 << 2;
    
    report u64 shl_test12;
    shl_test12 = 1 << 2;
    report u64 shl_test13;
    shl_test13 = -1 << 2;
    report i64 shl_test14;
    shl_test14 = 1 << 2;
    report i64 shl_test15;
    shl_test15 = -1 << 2;
    
    // --- >>
    report u8 shr_test0;
    shr_test0 = 1 >> 2;
    report u8 shr_test1;
    shr_test1 = -1 >> 2;
    report i8 shr_test2;
    shr_test2 = 1 >> 2;
    report i8 shr_test3;
    shr_test3 = -1 >> 2;
    
    report u16 shr_test4;
    shr_test4 = 1 >> 2;
    report u16 shr_test5;
    shr_test5 = -1 >> 2;
    report i16 shr_test6;
    shr_test6 = 1 >> 2;
    report i16 shr_test7;
    shr_test7 = -1 >> 2;
    
    report u32 shr_test8;
    shr_test8 = 1 >> 2;
    report u32 shr_test9;
    shr_test9 = -1 >> 2;
    report i32 shr_test10;
    shr_test10 = 1 >> 2;
    report i32 shr_test11;
    shr_test11 = -1 >> 2;
    
    report u64 shr_test12;
    shr_test12 = 1 >> 2;
    report u64 shr_test13;
    shr_test13 = -1 >> 2;
    report i64 shr_test14;
    shr_test14 = 1 >> 2;
    report i64 shr_test15;
    shr_test15 = -1 >> 2;
    
    // --- &
    
    report u8 and_test0;
    and_test0 = 1 & 1;
    report u8 and_test1;
    and_test1 = -1 & 1;
    report i8 and_test2;
    and_test2 = 1 & 1;
    report i8 and_test3;
    and_test3 = -1 & 1;
    
    report u16 and_test4;
    and_test4 = 1 & 1;
    report u16 and_test5;
    and_test5 = -1 & 1;
    report i16 and_test6;
    and_test6 = 1 & 1;
    report i16 and_test7;
    and_test7 = -1 & 1;
    
    report u32 and_test8;
    and_test8 = 1 & 1;
    report u32 and_test9;
    and_test9 = -1 & 1;
    report i32 and_test10;
    and_test10 = 1 & 1;
    report i32 and_test11;
    and_test11 = -1 & 1;
    
    report u64 and_test12;
    and_test12 = 1 & 1;
    report u64 and_test13;
    and_test13 = -1 & 1;
    report i64 and_test14;
    and_test14 = 1 & 1;
    report i64 and_test15;
    and_test15 = -1 & 1;
    
    // --- |
    
    report u8 or_test0;
    or_test0 = 1 | 0xF;
    report u8 or_test1;
    or_test1 = -1 | 0xF;
    report i8 or_test2;
    or_test2 = 1 | 0xF;
    report i8 or_test3;
    or_test3 = -1 | 0xF;
    
    report u16 or_test4;
    or_test4 = 1 | 0xF;
    report u16 or_test5;
    or_test5 = -1 | 0xF;
    report i16 or_test6;
    or_test6 = 1 | 0xF;
    report i16 or_test7;
    or_test7 = -1 | 0xF;
    
    report u32 or_test8;
    or_test8 = 1 | 0xF;
    report u32 or_test9;
    or_test9 = -1 | 0xF;
    report i32 or_test10;
    or_test10 = 1 | 0xF;
    report i32 or_test11;
    or_test11 = -1 | 0xF;
    
    report u64 or_test12;
    or_test12 = 1 | 0xF;
    report u64 or_test13;
    or_test13 = -1 | 0xF;
    report i64 or_test14;
    or_test14 = 1 | 0xF;
    report i64 or_test15;
    or_test15 = -1 | 0xF;
    
    // --- ^
    
    report u8 xor_test0;
    xor_test0 = 1 ^ 0xF;
    report u8 xor_test1;
    xor_test1 = -1 ^ 0xF;
    report i8 xor_test2;
    xor_test2 = 1 ^ 0xF;
    report i8 xor_test3;
    xor_test3 = -1 ^ 0xF;
    
    report u16 xor_test4;
    xor_test4 = 1 ^ 0xF;
    report u16 xor_test5;
    xor_test5 = -1 ^ 0xF;
    report i16 xor_test6;
    xor_test6 = 1 ^ 0xF;
    report i16 xor_test7;
    xor_test7 = -1 ^ 0xF;
    
    report u32 xor_test8;
    xor_test8 = 1 ^ 0xF;
    report u32 xor_test9;
    xor_test9 = -1 ^ 0xF;
    report i32 xor_test10;
    xor_test10 = 1 ^ 0xF;
    report i32 xor_test11;
    xor_test11 = -1 ^ 0xF;
    
    report u64 xor_test12;
    xor_test12 = 1 ^ 0xF;
    report u64 xor_test13;
    xor_test13 = -1 ^ 0xF;
    report i64 xor_test14;
    xor_test14 = 1 ^ 0xF;
    report i64 xor_test15;
    xor_test15 = -1 ^ 0xF;

    // --- ~

    report u8 bwise_not_test0;
    bwise_not_test0 = ~ 1;
    report u8 bwise_not_test1;
    bwise_not_test1 = ~ -1;
    report i8 bwise_not_test2;
    bwise_not_test2 = ~ 1;
    report i8 bwise_not_test3;
    bwise_not_test3 = ~ -1;

    report u16 bwise_not_test4;
    bwise_not_test4 = ~ 1;
    report u16 bwise_not_test5;
    bwise_not_test5 = ~ -1;
    report i16 bwise_not_test6;
    bwise_not_test6 = ~ 1;
    report i16 bwise_not_test7;
    bwise_not_test7 = ~ -1;

    report u32 bwise_not_test8;
    bwise_not_test8 = ~ 1;
    report u32 bwise_not_test9;
    bwise_not_test9 = ~ -1;
    report i32 bwise_not_test10;
    bwise_not_test10 = ~ 1;
    report i32 bwise_not_test11;
    bwise_not_test11 = ~ -1;

    report u64 bwise_not_test12;
    bwise_not_test12 = ~ 1;
    report u64 bwise_not_test13;
    bwise_not_test13 = ~ -1;
    report i64 bwise_not_test14;
    bwise_not_test14 = ~ 1;
    report i64 bwise_not_test15;
    bwise_not_test15 = ~ -1;
}