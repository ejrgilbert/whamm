use whamm_core;

// TODO(wei): this produces a wei module with dupe exports (no differentiator
// on type bounds except for the type of the param) -- invalid module produced!!
report var i32_probe: i32;
report var i64_probe: i32;

wasm:opcode:*(arg0: i32):before { i32_probe = i32_probe + 1; }
wasm:opcode:*(arg0: i64):before { i64_probe = i64_probe + 1; }

wasm:report {
    whamm_core.puti32(i32_probe);
    whamm_core.putc(10); // newline
    whamm_core.puti32(i64_probe);
    whamm_core.putc(10); // newline
}
