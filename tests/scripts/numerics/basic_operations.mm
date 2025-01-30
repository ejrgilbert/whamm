wasm:opcode:call:before /
    imm0 == 0
/ {
    // ==== binary operations ====
    
    // --- <<
    report unshared var shl_test0: u8;
    shl_test0 = 1 << 2;
    report unshared var shl_test1: u8;
    shl_test1 = -1 << 2;
    report unshared var shl_test2: i8;
    shl_test2 = 1 << 2;
    report unshared var shl_test3: i8;
    shl_test3 = -1 << 2;
    
    report unshared var shl_test4: u16;
    shl_test4 = 1 << 2;
    report unshared var shl_test5: u16;
    shl_test5 = -1 << 2;
    report unshared var shl_test6: i16;
    shl_test6 = 1 << 2;
    report unshared var shl_test7: i16;
    shl_test7 = -1 << 2;
    
    report unshared var shl_test8: u32;
    shl_test8 = 1 << 2;
    report unshared var shl_test9: u32;
    shl_test9 = -1 << 2;
    report unshared var shl_test10: i32;
    shl_test10 = 1 << 2;
    report unshared var shl_test11: i32;
    shl_test11 = -1 << 2;
    
    report unshared var shl_test12: u64;
    shl_test12 = 1 << 2;
    report unshared var shl_test13: u64;
    shl_test13 = -1 << 2;
    report unshared var shl_test14: i64;
    shl_test14 = 1 << 2;
    report unshared var shl_test15: i64;
    shl_test15 = -1 << 2;
    
    // --- >>
    report unshared var shr_test0: u8;
    shr_test0 = 1 >> 2;
    report unshared var shr_test1: u8;
    shr_test1 = -1 >> 2;
    report unshared var shr_test2: i8;
    shr_test2 = 1 >> 2;
    report unshared var shr_test3: i8;
    shr_test3 = -1 >> 2;
    
    report unshared var shr_test4: u16;
    shr_test4 = 1 >> 2;
    report unshared var shr_test5: u16;
    shr_test5 = -1 >> 2;
    report unshared var shr_test6: i16;
    shr_test6 = 1 >> 2;
    report unshared var shr_test7: i16;
    shr_test7 = -1 >> 2;
    
    report unshared var shr_test8: u32;
    shr_test8 = 1 >> 2;
    report unshared var shr_test9: u32;
    shr_test9 = -1 >> 2;
    report unshared var shr_test10: i32;
    shr_test10 = 1 >> 2;
    report unshared var shr_test11: i32;
    shr_test11 = -1 >> 2;
    
    report unshared var shr_test12: u64;
    shr_test12 = 1 >> 2;
    report unshared var shr_test13: u64;
    shr_test13 = -1 >> 2;
    report unshared var shr_test14: i64;
    shr_test14 = 1 >> 2;
    report unshared var shr_test15: i64;
    shr_test15 = -1 >> 2;
    
    // --- &
    
    report unshared var and_test0: u8;
    and_test0 = 1 & 1;
    report unshared var and_test1: u8;
    and_test1 = -1 & 1;
    report unshared var and_test2: i8;
    and_test2 = 1 & 1;
    report unshared var and_test3: i8;
    and_test3 = -1 & 1;
    
    report unshared var and_test4: u16;
    and_test4 = 1 & 1;
    report unshared var and_test5: u16;
    and_test5 = -1 & 1;
    report unshared var and_test6: i16;
    and_test6 = 1 & 1;
    report unshared var and_test7: i16;
    and_test7 = -1 & 1;
    
    report unshared var and_test8: u32;
    and_test8 = 1 & 1;
    report unshared var and_test9: u32;
    and_test9 = -1 & 1;
    report unshared var and_test10: i32;
    and_test10 = 1 & 1;
    report unshared var and_test11: i32;
    and_test11 = -1 & 1;
    
    report unshared var and_test12: u64;
    and_test12 = 1 & 1;
    report unshared var and_test13: u64;
    and_test13 = -1 & 1;
    report unshared var and_test14: i64;
    and_test14 = 1 & 1;
    report unshared var and_test15: i64;
    and_test15 = -1 & 1;
    
    // --- |
    
    report unshared var or_test0: u8;
    or_test0 = 1 | 0xF;
    report unshared var or_test1: u8;
    or_test1 = -1 | 0xF;
    report unshared var or_test2: i8;
    or_test2 = 1 | 0xF;
    report unshared var or_test3: i8;
    or_test3 = -1 | 0xF;
    
    report unshared var or_test4: u16;
    or_test4 = 1 | 0xF;
    report unshared var or_test5: u16;
    or_test5 = -1 | 0xF;
    report unshared var or_test6: i16;
    or_test6 = 1 | 0xF;
    report unshared var or_test7: i16;
    or_test7 = -1 | 0xF;
    
    report unshared var or_test8: u32;
    or_test8 = 1 | 0xF;
    report unshared var or_test9: u32;
    or_test9 = -1 | 0xF;
    report unshared var or_test10: i32;
    or_test10 = 1 | 0xF;
    report unshared var or_test11: i32;
    or_test11 = -1 | 0xF;
    
    report unshared var or_test12: u64;
    or_test12 = 1 | 0xF;
    report unshared var or_test13: u64;
    or_test13 = -1 | 0xF;
    report unshared var or_test14: i64;
    or_test14 = 1 | 0xF;
    report unshared var or_test15: i64;
    or_test15 = -1 | 0xF;
    
    // --- ^
    
    report unshared var xor_test0: u8;
    xor_test0 = 1 ^ 0xF;
    report unshared var xor_test1: u8;
    xor_test1 = -1 ^ 0xF;
    report unshared var xor_test2: i8;
    xor_test2 = 1 ^ 0xF;
    report unshared var xor_test3: i8;
    xor_test3 = -1 ^ 0xF;
    
    report unshared var xor_test4: u16;
    xor_test4 = 1 ^ 0xF;
    report unshared var xor_test5: u16;
    xor_test5 = -1 ^ 0xF;
    report unshared var xor_test6: i16;
    xor_test6 = 1 ^ 0xF;
    report unshared var xor_test7: i16;
    xor_test7 = -1 ^ 0xF;
    
    report unshared var xor_test8: u32;
    xor_test8 = 1 ^ 0xF;
    report unshared var xor_test9: u32;
    xor_test9 = -1 ^ 0xF;
    report unshared var xor_test10: i32;
    xor_test10 = 1 ^ 0xF;
    report unshared var xor_test11: i32;
    xor_test11 = -1 ^ 0xF;
    
    report unshared var xor_test12: u64;
    xor_test12 = 1 ^ 0xF;
    report unshared var xor_test13: u64;
    xor_test13 = -1 ^ 0xF;
    report unshared var xor_test14: i64;
    xor_test14 = 1 ^ 0xF;
    report unshared var xor_test15: i64;
    xor_test15 = -1 ^ 0xF;

    // --- ~

    report unshared var bwise_not_test0: u8;
    bwise_not_test0 = ~ 1;
    report unshared var bwise_not_test1: u8;
    bwise_not_test1 = ~ -1;
    report unshared var bwise_not_test2: i8;
    bwise_not_test2 = ~ 1;
    report unshared var bwise_not_test3: i8;
    bwise_not_test3 = ~ -1;

    report unshared var bwise_not_test4: u16;
    bwise_not_test4 = ~ 1;
    report unshared var bwise_not_test5: u16;
    bwise_not_test5 = ~ -1;
    report unshared var bwise_not_test6: i16;
    bwise_not_test6 = ~ 1;
    report unshared var bwise_not_test7: i16;
    bwise_not_test7 = ~ -1;

    report unshared var bwise_not_test8: u32;
    bwise_not_test8 = ~ 1;
    report unshared var bwise_not_test9: u32;
    bwise_not_test9 = ~ -1;
    report unshared var bwise_not_test10: i32;
    bwise_not_test10 = ~ 1;
    report unshared var bwise_not_test11: i32;
    bwise_not_test11 = ~ -1;

    report unshared var bwise_not_test12: u64;
    bwise_not_test12 = ~ 1;
    report unshared var bwise_not_test13: u64;
    bwise_not_test13 = ~ -1;
    report unshared var bwise_not_test14: i64;
    bwise_not_test14 = ~ 1;
    report unshared var bwise_not_test15: i64;
    bwise_not_test15 = ~ -1;
}