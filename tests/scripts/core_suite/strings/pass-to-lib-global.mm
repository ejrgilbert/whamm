use whamm_core;

report var h: str = "hello global\n";
var hl: u32 = len(h);
var hptr: i32 = whamm_core.mem_alloc(hl as i32);
write_str(memid(whamm_core), hptr, h);
whamm_core.puts(hptr, hl as i32);

wasm:opcode:drop:before {
    whamm_core.puts(hptr, hl as i32);
}
