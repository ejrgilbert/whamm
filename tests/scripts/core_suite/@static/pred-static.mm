use toggle;

wasm:opcode:i32.load:before / @static toggle.should_inject(fid as i32, pc as i32) as bool / {
    report var val: i32;
}