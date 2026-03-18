use whamm_core;

report var h: str = "hello global\n";
var hl: u32 = h.len();
var hptr: i32 = whamm_core.mem_alloc(hl as i32);
memcpy(0, h.addr(), memid(whamm_core), hptr as u32, hl);
whamm_core.puts(hptr, hl as i32);

wasm:opcode:drop:before {
    whamm_core.puts(hptr, hl as i32);
}
