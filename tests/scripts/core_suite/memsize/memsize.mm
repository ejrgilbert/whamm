wasm:opcode:memory.fill:before {
    report var size0: u32 = page_size(APP_MEMID);

    report var size1: u32;
    size1 = page_size(APP_MEMID);
}