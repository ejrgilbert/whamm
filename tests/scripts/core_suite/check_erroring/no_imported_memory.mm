use whamm_core;
wasm:opcode:call:before {
    report var m: u32;
    m = memid(whamm_core);
}
