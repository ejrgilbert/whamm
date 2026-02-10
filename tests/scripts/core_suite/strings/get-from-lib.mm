use whamm_core;
use alpha;

wasm:opcode:drop:before {
    // allocate some space in memory to store the library's string
    var MAX: i32 = 100;
    var ptr: i32 = alpha.mem_alloc(MAX);

    // write a string to memory, return the length of the string written
    // (should also get flushed at the end of execution)
    var l: i32 = alpha.write_alphabet(ptr, MAX);
    // TODO: make report var l work!

    // read the string from the library's memory
    // TODO: make `report var string` work!
    var s: str = read_str(memid(alpha), ptr, l as u32);

    // free the memory we've just used
    alpha.mem_free(ptr);

    ptr = whamm_core.mem_alloc(l as i32);
    write_str(memid(whamm_core), ptr, s);
    whamm_core.puts(ptr, l as i32);
    whamm_core.mem_free(ptr);
}
