use whamm_core;

// Call graph built using `resolved_fid` directly at the indirect call site,
// instead of the callee-side `wasm:func:entry` workaround in call_graph.mm.
// (from, to) -> count
report var call_graph: map<(u32, u32), u32>;

wasm:opcode:call|return_call:before {
    call_graph[(fid, imm0)]++;
}

wasm:opcode:*call_indirect:before {
    // resolved_fid is i32 (negative = unknown). Cast to u32 so the unknown
    // sentinel surfaces as 0xFFFFFFFF in the call graph rather than mixing
    // signed/unsigned in one map.
    call_graph[(fid, resolved_fid as u32)]++;
}

// TODO: also handle *call_ref here once resolved_fid is exposed on it.
// The original call_graph.mm covers call_ref via the
// `tracking_target` + `wasm:func:entry` callee-tagging workaround.

wasm:report {
    whamm_core.print_map_as_csv(0);
}
