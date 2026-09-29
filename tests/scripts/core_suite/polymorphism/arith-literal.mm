wasm:opcode:local.set<T: numeric>(arg0: T):before {
    report var acc: T;
    acc = arg0 + 1;
}
