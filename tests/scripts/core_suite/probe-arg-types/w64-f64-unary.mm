report var c: i32;
wasm:opcode:f64.sqrt:before {
    if (arg0 > 0.0) { c = c + 1; }
}
