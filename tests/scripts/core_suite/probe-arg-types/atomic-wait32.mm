report var c: i32;
wasm:opcode:memory.atomic.wait32:before {
    if (arg0 == 7) { c = c + 1; }
    if (arg2 == 0) { c = c + 1; }
}
