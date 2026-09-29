// `arg0` and `arg1` share type parameter `T`, so the probe fires only where both call args
// have the same concrete type; a site whose args differ is skipped (per-site self-consistency).
wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before {
    report var acc: T;
    acc = arg0 + arg1;
}
