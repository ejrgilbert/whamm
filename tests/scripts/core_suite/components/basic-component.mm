wasm:opcode:call(arg0: i32):before {
    report unshared var count: u32;
    count = count + 1;
}
