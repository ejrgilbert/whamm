# Libraries #

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

## Using Libraries at Match Time ##

Libraries can even be used to extend the match-time capabilities of the `Whamm` DSL.
This is done by simply adding the `@static` annotation to such a call.
NOTE: These calls _must_ not have side-effects that are required to persist to dynamic execution.
Such side effects will persist on `wei`, but in bytecode rewriting, the state is lost.
We plan to support persistent state for rewriting in the future, see: [`ideas/@static-serialize.md`].

[`ideas/@static-serialize.md`]: https://github.com/ejrgilbert/whamm/blob/master/ideas/@static-serialize.md

### Why is this useful? ###

Let's say you want to instrument the `i32.load` who's result is eventually used by a specific `br_table`.
You cannot express that with the current match rule capabilities of `Whamm`.
In fact, this specific situation requires abstract interpretation!
To offload such complexity, match-time decisions can leverage libraries to find such points.

Further, in bytecode rewriting, such libraries can be used to pull constants and reduce emitted code through constant propagation.
This constant propagation can even reduce `if`/`else` switches!
Consider the following example script that can be used for gas instrumentation:

```
use gas;
use analysis;

TINIT = 0;
TFILL = 0;
TCOPY = 0;

MINIT = 0;
MFILL = 0;
MCOPY = 0;

// Probes for GAS usage

wasm:opcode:*:before / @static analysis.should_inject(fid, pc) && ! @static analysis.linear_cost_at(fid, pc) == 0 / {
    var constant_cost: i32 = @static analysis.constant_cost_at(fid, pc);

    switch (@static analysis.instr_kind(fid, pc)) {
        case CONST => gas.decr_const(constant_cost);
        default => unreachable();
    }
}
wasm:opcode:*(arg0: i32):before / @static analysis.should_inject(fid, pc) && @static analysis.linear_cost_at(fid, pc) > 0  / {
    var linear_cost: i32 = @static analysis.linear_cost_at(fid, pc);
    var constant_cost: i32 = @static analysis.constant_cost_at(fid, pc);
    switch (@static analysis.instr_kind(fid, pc)) {
        case TINIT => gas.finite_wasm_table_init(arg0, linear_cost, constant_cost);
        case TFILL => gas.finite_wasm_table_fill(arg0, linear_cost, constant_cost);
        case TCOPY => gas.finite_wasm_table_copy(arg0, linear_cost, constant_cost);

        case MINIT => gas.finite_wasm_memory_init(arg0, linear_cost, constant_cost);
        case MFILL => gas.finite_wasm_memory_fill(arg0, linear_cost, constant_cost);
        case MCOPY => gas.finite_wasm_memory_copy(arg0, linear_cost, constant_cost);

        default => unreachable();
    }
}
```

### How does this work? ###

_In bytecode rewriting_, we embed `wasmtime` to call such libraries as the backend visits the target application bytecode.
Each module that is called statically is instantiated, the instance and state are held by the `WasmRegistry`.
This registry is defined in [`src/lang_features/libraries/registry.rs`].

[`src/lang_features/libraries/registry.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/lang_features/libraries/registry.rs

_In `wei`_, these calls are simply performed at the engine match time.
This guarantees that any state that should be persisted between match and runtime remains.
This also requires that running a `wei` monitor module that has been generated from a script containing `@static` calls to a library is linked at runtime.
This is not required for bytecode rewriting since those `@static` calls have already been made (thus not requiring the Wasm imports to that library).
