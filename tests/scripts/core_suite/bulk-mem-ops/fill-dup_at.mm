use whamm_core;

wasm:opcode:memory.fill:before {
    var l: i32 = arg0;
    var ptr: i32 = whamm_core.mem_alloc(l);

    dup_at(memid(whamm_core), ptr as u32);
    whamm_core.puts(ptr, l as i32);
    whamm_core.mem_free(ptr);

    whamm_core.putc(10); // write a newline so it prints!
}
