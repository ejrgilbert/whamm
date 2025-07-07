use whamm_core;

// (from, to) -> count
report var call_graph: map<(u32, u32), u32>;
var tracking_target: bool;
var caller: u32;

wasm:opcode:call|return_call:before {
    call_graph[(fid, imm0)]++;
}

wasm:opcode:*call_indirect|*call_ref:before {
    tracking_target = true;
    caller = fid;
}

wasm:func:entry {
    if (tracking_target) {
        call_graph[(caller, fid)]++;
        tracking_target = false;
    }
}

wasm:report {
    // this is the ID of the map above...need to make this less hard-coded.
    // maybe: `call_graph.id()`
    whamm_core.print_map_as_csv(0);
}
