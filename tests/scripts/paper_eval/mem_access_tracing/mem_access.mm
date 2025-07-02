report var accesses: map<u32, u32>;

wasm:opcode:*load*|*store*:before {
    accesses[effective_addr]++;
}
