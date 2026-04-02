use whamm_core;

whamm_core.mem_alloc(1);

wasm:opcode:call:before {
    report var i: i32 = 1;
    whamm_core.mem_alloc(1);
}