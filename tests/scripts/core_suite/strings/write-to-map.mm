report var m: map<u32, str>;
wasm:opcode:drop:before {
    m[pc] = "drop";
}
