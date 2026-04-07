use toggle;

@init toggle.get_value();

wasm:opcode:*:before {
    @init toggle.get_value();

    report var i: u32 = 1;
}