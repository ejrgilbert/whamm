wasm:opcode:memory.fill:before {
    report var pages: u32 = mem_size(APP_MEMID);
    report var ps: u32 = page_size(APP_MEMID);
    report var dstart: u32 = active_data_start(APP_MEMID);
    report var dlen: u32 = active_data_len(APP_MEMID);
}
