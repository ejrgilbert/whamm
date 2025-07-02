// (from, to) -> count
report var call_graph: map<(u32, u32), u32>;
var tracking_target: bool;
var callee: u32;

wasm:opcode:call|return_call:before {
    call_graph[(fid, imm0)]++;
}

// TODO:
// - support `wasm:func:entry`
// - fix wizard not initializing a global map

// wasm:opcode:*call_indirect|*call_ref:before {
//     tracking_target = true;
//     callee = fid;
// }
//
// wasm:func:entry {
//     if (tracking_target) {
//         call_graph[(callee, fid)]++;
//         tracking_target = false;
//     }
// }
