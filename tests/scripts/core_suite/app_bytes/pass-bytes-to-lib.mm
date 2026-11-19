use whamm_core;

wasm:opcode:drop:before {
    // Print the app's embedded smiley
    var ptr: i32 = whamm_core.mem_alloc(4);
    memcpy(memid(app_bytes), app_bytes.addr() + 52, memid(whamm_core), ptr as u32, 4);
    whamm_core.puts(ptr, 4);
    whamm_core.mem_free(ptr);
    whamm_core.putc(10);
}
