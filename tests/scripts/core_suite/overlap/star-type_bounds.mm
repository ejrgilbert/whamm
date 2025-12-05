wasm:opcode:*:before {
    report var no_bounds: i32 = 1;
}
wasm:opcode:*(arg0: i32):before {
    report var with_bounds: i32 = 2;
}