wasm:opcode:call:before {
    unshared i32 count;
    report i32 rep_count;

    count++;

    rep_count = count;
}
