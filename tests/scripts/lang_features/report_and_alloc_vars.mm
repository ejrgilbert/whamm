wasm:opcode:call:before {
    unshared var count: i32;
    report var rep_count;

    count++;

    rep_count = count;
}
