wasm:opcode:br_table:before {
    // invalid: shadows br_table bound var with a probe-local
    var targets: i32;
    targets = 1;
}
