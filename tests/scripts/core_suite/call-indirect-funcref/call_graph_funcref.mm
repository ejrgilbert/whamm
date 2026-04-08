use whamm_core;

// (from, to) -> count
report var call_graph: map<(u32, u32), u32>;

wasm:opcode:call|return_call:before {
    call_graph[(fid, imm0)]++;
}

wasm:opcode:*call_indirect:before {
    call_graph[(fid, resolve_funcref(target_funcref, table_entry_idx) as u32)]++;
}

wasm:report {
    // this is the ID of the map above...need to make this less hard-coded.
    // maybe: `call_graph.id()`
    whamm_core.print_map_as_csv(0);
}
