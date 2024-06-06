# Primitives #

`whamm!` offers a small set of primitive types that are useful for performing arithmetic and representing data.

## Booleans ##

With only two values, `true`, and `false`, booleans are represented in `whamm!` with the type `bool`.

```
bool x; // default == false
x = true;
x = false;
```

## Integers ##

Right now, `whamm!` supports `i32` integers (_signed_ 32-bit values), but will be supporting all numeric types provided by Wasm in the future.

```
// with declared types
i32 d; // default == 0
d = 0;
d = 9993;
d = -42;
```

The minimum decimal value for type `i32` is `-2147483648` (equal to `-2^31`) and the maximum value is `2147483647` (equal to `2^31 - 1`).
