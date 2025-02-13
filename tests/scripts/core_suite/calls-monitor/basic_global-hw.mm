report var count: u32;

wasm:opcode:call(arg0: i32):before /
    fid == 3
/ {
    count = count + 1;
}
