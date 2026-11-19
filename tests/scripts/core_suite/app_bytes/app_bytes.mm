wasm:opcode:nop:before {
    report var module_len: u32 = app_bytes.len();
    report var module_addr: u32 = app_bytes.addr();
}
