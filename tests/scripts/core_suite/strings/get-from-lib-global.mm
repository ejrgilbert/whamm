use whamm_core;
use alpha;

// allocate some space in memory to store the library's string
var MAX: i32 = 100;
var ptr: i32 = alpha.mem_alloc(MAX);

// write a string to memory, return the length of the string written
// (should also get flushed at the end of execution)
var l: i32 = alpha.write_alphabet(ptr, MAX);

// read the string from the library's memory
// TODO: make `report var string` work!
var s: str = read_str(memid(alpha), ptr, l as u32);

var core_ptr: i32 = whamm_core.mem_alloc(l as i32);
write_str(memid(whamm_core), core_ptr, s);
whamm_core.puts(core_ptr, l as i32);
whamm_core.mem_free(core_ptr);

wasm:opcode:drop:before {
    // read the string from the library's memory that should still be live
    // TODO: make `report var string` work!
    var s1: str = read_str(memid(alpha), ptr, l as u32);


    core_ptr = whamm_core.mem_alloc(l as i32);
    write_str(memid(whamm_core), core_ptr, s1);
    whamm_core.puts(core_ptr, l as i32);
    whamm_core.mem_free(core_ptr);
}
