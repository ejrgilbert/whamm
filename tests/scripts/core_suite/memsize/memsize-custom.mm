wasm:opcode:memory.size:before {
    report var page0: u32 = page_size(APP_MEMID);

    report var page1: u32;
    page1 = page_size(APP_MEMID);

    report var mem0: u32 = mem_size(APP_MEMID);

    report var mem1: u32;
    mem1 = mem_size(APP_MEMID);
}