use dyninstr;

wasm:opcode:*:before {
    report unshared var reached: bool;
    reached = true;

    dyninstr.remove_probe(probe_id);
}
