wasm:opcode:call(arg0: i32):before /
    fid == 3
/ {
    report var count: u32;
    if (arg0 == 0) {
       count++;
    }
}