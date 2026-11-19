use whamm_core;

wasm:opcode:drop:before {
    // The app embeds a smiley (😀 = 0xF0 0x9F 0x98 0x80) in a data section.
    // `app_bytes` holds the whole target module, so those 4 bytes sit at a
    // fixed offset (52) from `app_bytes.addr()`. Copy them into the library's
    // memory and print them -- should output the smiley.
//     report var module_len: u32 = app_bytes.len();
    var ptr: i32 = whamm_core.mem_alloc(4);
    memcpy(1, app_bytes.addr() + 52, memid(whamm_core), ptr as u32, 4);
    whamm_core.puts(ptr, 4);
    whamm_core.mem_free(ptr);
    whamm_core.putc(10);
}
