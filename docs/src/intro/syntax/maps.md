# Maps #

`Whamm` provides maps for storage of key-value pairs.
This is similar to `java`'s `Map` and `python`'s `dict` types.
In fact, it is exactly `Rust`'s `HashMap` type...since `Whamm` leverages this `Rust` type under-the-hood!

## Instantiation ##

```
// No need to instantiate a map, it is automatically created as an empty map.
var a: map<i32, i32>;
```

## Reading and writing elements ##

Reading and writing elements of maps uses the `[ ... ]` syntax like maps in many other languages.
```
var a: map<i32, i32>;
a[0] = 3; // map write
var b: i32 = a[0]; // map read

// maps can also contain tuples!
var c: map<(i32, i32, i32), i32>;
c[(0, 0, 0)] = 3; // map write
var b: i32 = c[(0, 0, 0)]; // map read
```

## Bounds and null checks ##

Accesses of `Whamm` maps are dynamically checked against the bounds.

```
var a: map<i32, i32>;
var b: i32 = a[0]; // produces Wasm trap through Rust
```
