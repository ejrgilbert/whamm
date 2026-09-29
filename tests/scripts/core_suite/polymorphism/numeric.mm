// Generic probe: monomorphizes to the concrete operand type at each `local.set` site.
// Fires on all four numeric local.set sites (i32/i64/f32/f64); `acc` is reported per type.
wasm:opcode:local.set<T: numeric>(arg0: T):before {
    report var acc: T;
    acc = arg0;
}
