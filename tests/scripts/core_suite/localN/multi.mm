wasm:opcode:i32.store:before { } // empty

wasm(local0: i32):func:entry / fname == "more_nesting" / {
    report var y: i32;
    y = local0;
}