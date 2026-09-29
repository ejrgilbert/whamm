// A float literal compared against a type-var operand can't be monomorphized per site.
wasm:opcode:local.set<T: numeric>(arg0: T):before / arg0 > 1.5 / {
    report var acc: T;
    acc = arg0;
}
