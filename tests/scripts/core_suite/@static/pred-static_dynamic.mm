use toggle;

// In wizard (not rewriting): This probe still gets inserted since we evaluate the
// static check at runtime. (Pushed down because of the use of `addr`)
wasm:opcode:i32.load:before /
    addr == 0
    && @static toggle.should_inject(fid as i32, pc as i32) as bool
/ {
    report var val: i32;
    val++;
}