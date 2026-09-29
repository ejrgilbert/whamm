# Polymorphism #

Some Wasm opcodes have operands whose type varies from one program location to the next
(e.g. `local.set` sets a local of any type; `call` passes arguments of the callee's types).
Rather than writing a separate [type-bound](./type_bounds.md) probe for every concrete type,
you can write a single **generic probe** that is _monomorphized_ to the concrete operand type
at each match site.

> **Target support.** Generic probes are currently only supported by bytecode-rewriting backend.
> The `wei` monitor-module backend does not yet support them.

## Type parameters ##

A probe declares **type parameters** in a generic header,`<...>`, placed after the rule part,
then it can use them in type bounds, in variable declarations, and in the body:

```
wasm:opcode:local.set<T: numeric>(arg0: T):before {
    report var acc: T;
    acc = arg0;
}
```

Here `T` is a type parameter. At each `local.set`, `T` is monomorphized to that operand's concrete
type: the `i32` sites are instrumented with `T = i32`, the `i64` sites with `T = i64`, and so
on. The `report var acc: T` becomes a distinct accumulator per concrete type, so the report
shows one `acc` for each type that actually occurred.

Type-parameter names are capitalized identifiers (`T`, `U`, `Elem`, ...).

Numeric literals may be combined with a type parameter and are resolved to its concrete type at
each site. For example, `acc = arg0 + 1` or a predicate `/ arg0 > 0 /`. A _float_ literal
(`arg0 > 1.5`) is only allowed when the parameter can be nothing but a float type (e.g.
`T: float`), since a float literal cannot be resolved to an integer instantiation.

## Generic constraints ##

A type parameter may be constrained to a class of types with `T: _constraint_`:

| Constraint | Admits                                  |
|------------|-----------------------------------------|
| `numeric`  | `i32`, `i64`, `f32`, `f64`              |
| `int`      | `i32`, `i64`                            |
| `float`    | `f32`, `f64`                            |
| _(none)_   | inferred from how the parameter is used |

```
wasm:opcode:call<T: int, U: numeric, V>(arg0: T, arg1: U, arg2: V):before { }
```

A probe only fires at a site whose operand types satisfy the parameters' constraints.
For example, a `<T: int>` probe skips `f32`/`f64` sites.

An **unconstrained** parameter (bare `<V>`) has its constraint _inferred_ from how it is used.
For example, this infers `V: int`, because `<<` requires an integer:

```
wasm:opcode:call<V>(arg0: V):before {
    report var shifted: V;
    shifted = arg0 << 1;
}
```

### The constraint is a contract ###

The declared constraint must be strong enough for every operation performed on the parameter.
If the body uses an operator the constraint cannot guarantee, it is a compile-time error.
This forces you to declare, up front, exactly which sites the probe applies to:

```
wasm:opcode:call<T: numeric>(arg0: T, arg1: T):before {
    report var acc: T;
    acc = arg0 << arg1;   // ERROR: `<<` requires `int`, but `T` is declared `numeric`.
}                         //        Declare `T: int`.
```

## Sharing a type across operands ##

Reuse a parameter to require that operands share a type: `(arg0: T, arg1: T)` matches only where
both operands have the _same_ concrete type, while a site whose operands differ is skipped. Two
_distinct_ parameters cannot be combined in a single operation (`arg0: T + arg1: U` is an error,
since that would require them to be equal); reuse one parameter if the operands must match.

## Future work ##

Combining a type parameter with a concretely-typed operand (`arg0: T + arg1: i32`) is not yet
supported; an explicit cast to bridge the two is planned.
