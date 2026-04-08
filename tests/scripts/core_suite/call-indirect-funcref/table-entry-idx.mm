wasm:opcode:*call_indirect:before {
    report unshared var idx: i32;
    idx = table_entry_idx;
}
