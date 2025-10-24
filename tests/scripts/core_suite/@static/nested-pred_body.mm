use toggle;

// The second call to `should_inject` takes the 'false', so the predicate is always 'true'
wasm:opcode:*:before / @static toggle.should_inject(fid as i32, @static toggle.get_value()) as bool / {
    report var val: i32;
    val = @static toggle.should_inject(fid as i32, @static toggle.get_value());
}