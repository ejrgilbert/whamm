# System API #

## Reflection

Since the engine will already need to decode the module bytes, we can ask it to provide some information about the module OR have it pass bytes for a specific item.
The `Whamm` module can then decode this subset of the application using its own library provided as a linked module.
This would simplify the instrumentation module and make it quite readable by splitting out the bytecode decoding into a library.

```
get_num_funcs();
get_num_globals();
get_num_memories();
get_num_...();
get_memory_size(mid: i32);
get_func_name(fid: i32) -> (start: i32, end: i32);
get_func_code(fid: i32, region: (i32, i32)) -> (start: i32, end: i32);

// places declarations/signatures in my memory to decode
get_mem_decl(mid: i32) -> (start: i32, end: i32)
get_global_decl(gid: i32) -> (start: i32, end: i32)
get_..._decl(...) -> (start: i32, end: i32)
get_func_sig(fid: i32) -> tid: i32
get_type_def(tid: i32) -> (start: i32, end: i32)
```

## Get state ##
Call these to get some state from the application module.
They will basically create a _wormhole_ into the application to be called when needed.
They can be used in bytecode-level or VM-level probe callbacks.

```
// use this to get values of global data
add_func(sig:i32,(start:i32, end:i32)) -> (funcref)

// use these to get access to specific global data
// (keeps from having to add_func for each of these)
global_accessor(gid:i32) -> funcref
memory_accessor(mid:i32) -> funcref: ((offset:i32) -> value:i32)
..._accessor(...) -> funcref
```

## Dynamically Add/Remove Probes ##
The API to dynamically add and remove probes from the co-module.

```
// BASE CASE
// uses standardized API for FrameAccessor (passed to callback)
insert_probe(fid:i32,pc:i32,funcref)
remove_probe(fid:i32,pc:i32,funcref)

// OPTIMIZED CASE (one of the two following options)
insert_whamm_probe_with_args(fid:i32,pc:i32,funcref,vec<WhammArg>)
// passes wasm bytes that execute as a trampoline (constructs the args), callback MUST RETURN requested argN
// can consume all or none of the arguments
// can also use custom opcodes for stack manipulation (dup, swap) to manage argument passing
insert_whamm_probe_with_trampoline(fid:i32,pc:i32,funcref,(start:i32,end:i32))
```
