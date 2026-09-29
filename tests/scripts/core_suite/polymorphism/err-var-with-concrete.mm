// A type variable cannot (yet) be combined with a concrete-typed operand.
wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
    report var acc: T;
    acc = arg0 + arg1;
}
