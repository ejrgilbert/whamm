// should not match
wasm:opcode:call(arg0: i64):before / imm0 == 1 / {
    report var b: i64 = 2;
}