use cache_sim;

wasm:opcode:i32_load:before {
    cache_sim.load(effective_addr);
}