use whamm_core;
report var accesses: map<u32, u32>;

wasm:opcode:*load*|*store*:before {
    accesses[effective_addr]++;
}

wasm:report {
    // this is the ID of the map above...need to make this less bad.
    whamm_core.print_map_as_csv(0);
}
