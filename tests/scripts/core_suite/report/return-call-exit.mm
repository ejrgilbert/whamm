use whamm_core;

report var c: i32 = 0;

wasm:opcode:drop:before {
    c = c + 1;
}

wasm:report {
    whamm_core.puti32(c);
    whamm_core.putc(10); // newline
}
