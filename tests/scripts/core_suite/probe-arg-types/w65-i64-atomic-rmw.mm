report var c: i32;
wasm:opcode:i64.atomic.rmw.add:before {
    if (arg0 > 0) { c = c + 1; }
}
