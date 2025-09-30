# Type Bounds #

Some Wasm opcodes, such as `local.set`, have polymorphic arguments.
Users can further predicate matching such probe locations by type using _type bounds_.

For now, this also avoids needing to implement polymorphism in `Whamm`.
Necessitating users to create one _type bound_ probe per potential type.

Here's an example using type bounds:
```
wasm:opcode:call(arg0: i32):before {
    report unshared var all_arg0s: map<i32, i32>;
    all_arg0s[arg0]++;
}
```
