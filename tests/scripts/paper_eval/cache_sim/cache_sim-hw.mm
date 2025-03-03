use cache_sim;

wasm:opcode:i32_load:before {
    report var val: i32;
    val = cache_sim.load(effective_addr as i32);
}