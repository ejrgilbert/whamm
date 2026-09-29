# Type Bounds #

Some Wasm opcodes, such as `local.set`, have polymorphic arguments.
Users can restrict matching such probe locations to a specific type using a _type bound_.

Here's an example using a type bound:
```
wasm:opcode:call(arg0: i32):before {
    report unshared var all_arg0s: map<i32, i32>;
    all_arg0s[arg0]++;
}
```

This probe only matches `call` sites whose first argument is an `i32`; to also handle `i64`,
`f32`, etc., you would write one type-bound probe per type.

To instead match _multiple_ types with a single probe by binding an operand's type to a type
parameter that is resolved per match site, see [Polymorphism](./polymorphism.md).