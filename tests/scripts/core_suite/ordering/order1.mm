var i: i32 = 0;

wasm:opcode:call:before {
    report var on_call0: i32;
    on_call0 = i;
    i++;
}

wasm:opcode:call:before {
    report var on_call1: i32;
    on_call1 = i;
    i++;
}
