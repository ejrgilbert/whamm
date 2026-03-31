wasm:opcode:call(res0: f32, res1: i64, res2: i32):after {
    report var val0: f32;
    val0 = res0;
    report var val1: i64;
    val1 = res1;
    report var val2: i32;
    val2 = res2;
}

wasm:opcode:call_indirect(res0: i32):after {
    report var val0: i32;
    val0 = res0;
}