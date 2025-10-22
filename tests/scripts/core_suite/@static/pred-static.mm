use toggle;

wasm:opcode:*:before / @static toggle.should_inject(fid as i32, pc as i32) as bool / {
    report var val: i32;
    val++; // todo: remove this line
}