var targets: map<i32, i32>;

wasm:opcode:br_table:before {
    // invalid: shadows br_table bound var
    targets[0] = 1;
}
