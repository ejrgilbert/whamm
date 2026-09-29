// Narrower bound: only the integer local.set sites (i32/i64) are admitted; the f32/f64
// sites are skipped (negative admission), so only integer `acc` values are reported.
wasm:opcode:local.set<T: int>(arg0: T):before {
    report var acc: T;
    acc = arg0;
}
