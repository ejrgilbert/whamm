use toggle;

report var val: i32 = @static toggle.get_nonzero_nested(1, @static toggle.get_value());

wasm:opcode:*:before {
//     report var val: i32;
}