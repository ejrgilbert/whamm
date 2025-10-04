wasm:opcode:*:before / at_func_end || opname != "end" / {
    report unshared var reached: bool;
    reached = true;
}

wasm:opcode:end:after / !at_func_end / {
    // The `end` opcode is a special case, we can tell if it's been
    // reached by fallthrough OR branch-to by emitting an AFTER probe.
    // EXCEPT for the end of a function, then we do a BEFORE.
    report unshared var reached: bool;
    reached = true;
}