use toggle;

wasm:opcode:*:before {
    report var val: i32;
    val = @static toggle.should_inject(fid as i32, @static toggle.get_value());
}