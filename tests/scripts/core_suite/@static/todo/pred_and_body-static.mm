use toggle;

// - static data used in @static call in predicate
//   - dynamic data in @static call in predicate errors!
// - static data used in @static call in body
//   - dynamic data in @static call in body errors!

wasm:opcode:*:before / @static toggle.should_inject(fid as i32, pc as i32) as bool / {
    report var val: i32;
    val = @static toggle.get_value();
    report var val1: i32;
    val1 = @static toggle.should_inject(fid as i32, pc as i32);
}