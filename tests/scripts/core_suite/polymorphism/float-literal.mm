// A float literal is admissible when the parameter can only be a float type; the per-site
// monomorphizer casts it to each concrete float type.
wasm:opcode:local.set<T: float>(arg0: T):before / arg0 > 1.5 / {
    report var acc: T;
    acc = arg0;
}
