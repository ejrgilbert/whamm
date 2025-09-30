# Primitives #

`Whamm` offers a small set of primitive types that are useful for performing arithmetic and representing data.

## Booleans ##

With only two values, `true`, and `false`, booleans are represented in `Whamm` with the type `bool`.

```
var x: bool; // default == false
x = true;
x = false;
```

## Integers ##

Right now, `Whamm` supports the following types:

| type  | bits | description             | min                                 | max                                                |
|-------|------|-------------------------|-------------------------------------|----------------------------------------------------|
| `u8`  | 8    | _unsigned_ values       | `0`                                 | `255` (equal to `2^8 - 1`)                         |
| `i8`  | 8    | _signed_ values         | `-128` (equal to `-2^7`)            | `127` (equal to `2^7 - 1`)                         |
| `u16` | 16   | _unsigned_ values       | `0`                                 | `65_535` (equal to `2^16 - 1`)                     |
| `i16` | 16   | _signed_ values         | `-32_768` (equal to `-2^15`)        | `32_767` (equal to `2^15 - 1`)                     |
| `u32` | 32   | _unsigned_ values       | `0`                                 | `4_294_967_295` (equal to `2^32 - 1`)              |
| `i32` | 32   | _signed_ values         | `-2_147_483_648` (equal to `-2^31`) | `2_147_483_647` (equal to `2^31 - 1`)              |
| `f32` | 32   | _floating point_ values | `-3.40282347E+38`                   | `3.40282347E+38`                                   |
| `u64` | 64   | _unsigned_ values       | `0`                                 | `4_294_967_295` (equal to `2^32 - 1`)              |
| `i64` | 64   | _signed_ values         | `-2_147_483_648` (equal to `-2^31`) | `18_446_744_073_709_551_615` (equal to `2^63 - 1`) |
| `f64` | 64   | _floating point_ values | `-1.7976931348623157E+308`          | `1.7976931348623157E+308`                          |

```
// with declared types
var d: i32; // default == 0
d = 0;
d = 9993;
d = -42;
```
