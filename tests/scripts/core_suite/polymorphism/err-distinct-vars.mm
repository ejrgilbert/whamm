// Two distinct type variables cannot be combined in one operation.
wasm:opcode:call<T: numeric, U: numeric>(arg0: T, arg1: U):before {
    report var acc: T;
    acc = arg0 + arg1;
}
