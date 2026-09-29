// Two independent type parameters, each bound from its own operand at the site.
wasm:opcode:call<T: numeric, U: numeric>(arg0: T, arg1: U):before {
    report var a: T;
    report var b: U;
    a = arg0;
    b = arg1;
}
