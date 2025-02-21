wasm:opcode:call(arg0: i32):before /
    fid == 5
/ {
    report unshared var count: u32;
    count = count + 1;
}
