// Bridge a concrete-typed operand and a type parameter with an explicit cast (`arg1 as T`).
// `T` binds from `arg0` (the topmost call operand: i32 at the first site, i64 at the second),
// so the cast is a no-op i32->i32 at the first site and a real i32->i64 widening at the second.
wasm:opcode:call<T: numeric>(arg0: T, arg1: i32):before {
    report var acc: T;
    acc = arg0 + (arg1 as T);
}
