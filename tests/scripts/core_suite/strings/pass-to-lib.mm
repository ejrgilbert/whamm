use whamm_core;

// TODO allow non-static calls to libraries in global statements, then:
// var h: str = "hello global\n";
// var hl: u32 = len(h);
// var hptr: i32 = whamm_core.mem_alloc(hl as i32);
// write_str(mem(whamm_core), hptr, h);
// whamm_core.puts(hptr, hl as i32);
// whamm_core.mem_free(hptr);

wasm:opcode:drop:before {
    // should print: "hello drop😀😀A"
    var s: str = "hello drop\u{1F600}😀\x41\n";
    var l: u32 = len(s);
    var ptr: i32 = whamm_core.mem_alloc(l as i32);
    write_str(mem(whamm_core), ptr, s);
    whamm_core.puts(ptr, l as i32);
    whamm_core.mem_free(ptr);
}
