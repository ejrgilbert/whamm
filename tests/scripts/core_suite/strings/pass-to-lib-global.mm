use whamm_core;

// TODO allow non-static calls to libraries in global statements, then:
var h: str = "hello global\n";
var hl: u32 = len(h);
var hptr: i32 = whamm_core.mem_alloc(hl as i32);
write_str(memid(whamm_core), hptr, h);
whamm_core.puts(hptr, hl as i32);

// whamm_core.puti32(hl as i32);
// whamm_core.putc(10);
//
// whamm_core.puti32(hptr as i32);
// whamm_core.putc(33);
// whamm_core.putc(10);

wasm:opcode:drop:before {
    whamm_core.puts(hptr, hl as i32);
}
