wasm:opcode:drop:before {
    report var m: map<u32,i32>;
    m[1 as u32] = 42 as i32;
    report var r: i32 = m[1 as u32];
}
