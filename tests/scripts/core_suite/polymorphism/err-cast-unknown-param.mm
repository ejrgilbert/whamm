// Casting to a type parameter that is not declared in the generic header is a type error.
wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
    report var acc: T;
    acc = arg0 + (arg1 as Z);
}
