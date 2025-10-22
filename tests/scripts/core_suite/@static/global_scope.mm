use toggle;

report var val: i32;
val = @static toggle.get_nonzero();

wasm:opcode:nop:before {
    report var pval: i32;
}