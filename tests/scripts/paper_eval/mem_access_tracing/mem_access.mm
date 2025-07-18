use whamm_core;
report var accesses: map<(u32, bool), u32>;

wasm:opcode:*load*|*store*|*atomic_rmw*:before {
    accesses[(effective_addr, is_write)]++;
}

wasm:report {
    // this is the ID of the map above...need to make this less hard-coded.
    // maybe: `accesses.id()`
    whamm_core.print_map_as_csv(0);
}
