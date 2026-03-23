use toggle;

wasm:opcode:*:before {
    report var val: i32;
    val = @static toggle.get_value();
}