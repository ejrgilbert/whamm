use dyninstr;

wasm:block:end {
    report unshared var reached: bool;
    reached = true;

    dyninstr.remove_probe(probe_id);
}
