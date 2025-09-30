# Maps #

`Whamm` provides maps for storage of key-value pairs.
This is similar to `java`'s `Map` and `python`'s `dict` types.
In fact, it is exactly `Rust`'s `HashMap` type...since `Whamm` leverages this `Rust` type under-the-hood!
If you are interested in how this works, see the Developer's [Core Library](../../devs/core_lib.md) documentation.

## Instantiation ##

```
// No need to instantiate a map, it is automatically created as an empty map.
map<i32, i32> a;
```

## Reading and writing elements ##

Reading and writing elements of maps uses the `[ ... ]` syntax like maps in many other languages.
```
map<i32, i32> a;
a[0] = 3; // map write
i32 b = a[0]; // map read

// maps can also contain tuples!
map<(i32, i32, i32), i32> c;
c[(0, 0, 0)] = 3; // map write
i32 b = c[(0, 0, 0)]; // map read
```

## Bounds and null checks ##

Accesses of `Whamm` maps are dynamically checked against the bounds.

```
map<i32, i32> a;
i32 b = a[0]; // produces Wasm trap through Rust
```
