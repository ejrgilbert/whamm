// `<<` requires `int`, but T is declared `numeric` -> must declare the stronger bound.
wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before {
    report var acc: T;
    acc = arg0 << arg1;
}
