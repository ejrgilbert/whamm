// `<<` requires `int`; a `float`-bounded var can never satisfy it.
wasm:opcode:call<T: float>(arg0: T, arg1: T):before {
    report var acc: T;
    acc = arg0 << arg1;
}
