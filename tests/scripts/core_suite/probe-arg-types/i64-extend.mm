report var c: i32;
wasm:opcode:i64.extend8_s:before {
    if (arg0 > 0) { c = c + 1; }
}
