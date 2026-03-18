use whamm_core;

wasm:opcode:drop:before {
    // should print: "hello drop😀😀A"
    report var s: str = "hello drop\u{1F600}😀\x41\n";
    var l: u32 = s.len();
    var ptr: i32 = whamm_core.mem_alloc(l as i32);
    memcpy(0, s.addr(), memid(whamm_core), ptr as u32, l);
    whamm_core.puts(ptr, l as i32);
    whamm_core.mem_free(ptr);
}
