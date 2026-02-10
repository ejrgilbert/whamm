report unshared var m: map<i32, str>;
wasm:opcode:drop:before {
    m[pc] = "drop"
}
