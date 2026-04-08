wasm:opcode:*call_indirect:before {
    report unshared var target_fid0: i32;
    target_fid0 = resolved_fid;

    report unshared var target_fid1: i32;
    target_fid1 = resolve_funcref(target_funcref, table_entry_idx);
}
