use tracer;

// anchors
wasm:func:entry {
    unshared var anchor_id: i32 = @static tracer.init_anchor(fid as i32, pc as i32);
    tracer.on_anchor(anchor_id);
}
wasm:opcode:loop:before {
    unshared var anchor_id: i32 = @static tracer.init_anchor(fid as i32, pc as i32);
    tracer.on_anchor(anchor_id);
}

wasm:opcode:*if:before {
    tracer.on_if(arg0 as i32);
}

wasm:opcode:br_table:before {
    // TODO -- will save memory to use `target_label` or `target_pc` instead
    tracer.on_br_table(target as i32);
}

wasm:report {
    tracer.flush_csv();
}

// TODO: Handle GC opcodes
// export "wasm:opcode:br_on_null" def br_on_null_probe() {
// }
// export "wasm:opcode:br_on_non_null" def br_on_non_null_probe() {
// }
// export "wasm:opcode:br_on_cast" def br_on_cast_probe() {
// }
// export "wasm:opcode:br_on_cast_fail" def br_on_cast_fail_probe() {
// }
