wasm(local0: i32, local1: i32):opcode:*:before / opidx == 0 && fname == "with_params" / {
    report var x: i32;
    x = local0;
}

wasm:opcode:*:before / opidx == 0 && fname == "no_params" / {
    report var y: i32;
    y = 42;
}