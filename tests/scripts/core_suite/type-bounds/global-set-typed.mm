use whamm_core;

report var i32_sets: i32;
report var i64_sets: i32;

wasm:opcode:global.set(arg0: i32):before { var x: i32 = arg0; i32_sets = i32_sets + 1; }
wasm:opcode:global.set(arg0: i64):before { var x: i64 = arg0; i64_sets = i64_sets + 1; }

wasm:report {
    whamm_core.puti32(i32_sets);
    whamm_core.putc(10); // newline
    whamm_core.puti32(i64_sets);
    whamm_core.putc(10); // newline
}
