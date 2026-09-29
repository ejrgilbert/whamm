// The cast may sit on either operand: here it bridges the left operand (`arg0 as T`).
// `arg0: i32` restricts matching to the site whose topmost operand is i32, so `T` (bound from
// `arg1`) monomorphizes to i32 and the probe fires once.
wasm:opcode:call<T: numeric>(arg0: i32, arg1: T):before {
    report var acc: T;
    acc = (arg0 as T) + arg1;
}
