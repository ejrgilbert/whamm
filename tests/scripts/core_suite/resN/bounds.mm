wasm:opcode:call(res0: i32):after / imm0 == 2 / {
    report var x: i32;
    x = res0;
}
wasm:opcode:call(res0: f64):after / imm0 == 3 / {
    report var y: f64;
    y = res0;
}
