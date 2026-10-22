use whamm_core;
wasm:opcode:i32.const:before {
    var m: u32;
    m = mem_size(APP_MEMID);
    var s: u32 = mem_size(m);
}
