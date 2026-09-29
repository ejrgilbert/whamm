// Integer literal in a predicate comparison against a type-var operand: the literal is
// resolved to the site's concrete type, so this is valid at every numeric site.
wasm:opcode:local.set<T: numeric>(arg0: T):before / arg0 > 0 / {
    report var acc: T;
    acc = arg0;
}
