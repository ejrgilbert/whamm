use whamm_core;
report var c: i32;
wasm:opcode:*:before {
    c++;
}
wasm:report {
    whamm_core.puti32(c);
    whamm_core.putc(10);
}
