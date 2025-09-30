# Libraries #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

Libraries are used to define instrumentation behavior when it goes beyond the scope of the core DSL grammar.
In fact, `Whamm` itself depends on the Whamm core library, named `whamm_core.wasm`, to provide printing support and a map implementation.

## Building and using custom libraries ##

To build your own custom library:
1) Design your API in a way that can be interacted with using basic Wasm primitives (e.g. i32, f32, etc.).
   Note that Whamm does not currently support libraries that return multiple results.
2) Write your library in a language that compiles to Wasm.
   Make sure that the API functions are exported using filenames that are compatible with the `Whamm` DSL keywords.
3) Compile your library to Wasm.
4) Inspect your library function exports using `wasm-tools`.

Now that you have a library binary, you can use the library in your `Whamm` script.
To do so, you will import the library into your script with the `use` keyword.
At that point, the library can then be called using the syntax: `lib_name.func_name()`.

Here's an example script using a Whamm library (it also gets around Whamm not supporting returning multiple results via bit-packing):

```
// import the library that simulates a cache
use cache;

// instrument all load and store opcodes
wasm:opcode:*load*|*store*:before {
    report unshared var hit: u32;
    report unshared var miss: u32;

    // call the library `check_access` function and pass bound variables as parameters
    var result: i32 = cache.check_access(effective_addr as i32, data_size as i32);
    var num_hits: i32 = (result & 0xFFFF0000) >> 16;
    var num_misses: i32 = (result & 0x0000FFFF);

    hit = hit + (num_hits as u32);
    miss = miss + (num_misses as u32);
}
```
