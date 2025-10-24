use toggle;

wasm:opcode:*:before / toggle.should_inject(fid as i32, @static toggle.get_value()) as bool / {
    report var val: i32;
    val = toggle.should_inject(fid as i32, @static toggle.get_value());
}