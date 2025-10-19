# The Whamm Engine Interface (`wei`) #

For `wei`, there is one `generator` named `WeiGenerator` in [`generator/mod.rs`].
This `generator` visits the [`MetadataCollector`]'s AST and emits a self-contained, portable monitor against `wei`.

This generators uses the `emitter` that emits Wasm code.
The `emitter` uses utilities that centralize the Wasm emitting logic found at [`utils.rs`]

[`generator/mod.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/mod.rs
[`MetadataCollector`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/metadata_collector.rs
[`utils.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/emitter/utils.rs


## The `WeiGenerator` ##

The `generator` traverses the AST to emit functions and variables that need to exist before emitting probes.
The `run` function is the entrypoint for this generator.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

This generator emits new Wasm functions and variables into the program with associated Wasm IDs.
These IDs are stored in the `SymbolTable` for use while running the `InstrGenerator`.
When emitting an instruction that either calls an emitted function or does some operation with an emitted global, the name of that symbol is looked up in the `SymbolTable` to then use the saved ID in the emitted instruction.

Remember that when targeting `wei`, `Whamm` does not have access to the application during compilation.
Rather, `Whamm` must encode requests for application state via the interface.
The generator encodes these requests in Wasm function export names.
The requested state is then passed by the engine to the monitor via function arguments.

## `wei` exports ##

Probe predicates and bodies are compiled to Wasm and leveraged as callbacks to the engine.
The engine is made aware of these special functions through encoding metadata in their export names.
This metadata follows a simple grammar:

```
⟨export-name⟩   ::=     ⟨match-rule⟩⟨predicate⟩? ⟨params⟩?
⟨match-rule⟩    ::=     'wasm':'opcode':('OPCODE'|'*')
                        'wasm':'func':('entry'|'exit')
                        'wasm':'block':('start'|'end')
                        'wasm':'exit'
⟨predicate⟩     ::=     ‘/’ ⟨call⟩‘/’
⟨call⟩          ::=     ‘$’ ID ⟨params⟩?
⟨params⟩        ::=     ‘(’ ‘)’
                        | ‘(’ ⟨param⟩( ‘,’ ⟨param⟩)* ‘)’
⟨param⟩         ::=     ‘argN’ | ‘immN’ | ‘localN’ | ⟨call⟩| ‘pc’ | ‘fid’ | ‘frame’
```

The engine will identify these special, exported functions and find their respective match locations in the target application.
If the match is further predicated, it will call the exported predicate function and pass its requested data.
If the predicate evaluates to true (non-zero response), the probe is dynamically inserted at that application location.
If the predicate evaluates to false (zero response), the probe is not inserted at that location.

Many bound variables must be requested from the engine explicitly.
However, some variables are derived from other bound variables, these request the data that they are derived from to keep the engine from supporting more and more state.
For example, the `effective_addr` for a load is derived by adding `arg0` and `offset`.
So, the compiler will simply request `arg0` and the static `offset` state, then derive the `effective_addr` through adding the two.

Here is an example probe function export:
```webassembly
(export "wasm:opcode:array.new / $20(is_func_end, opname) / ($21(fname), fid, pc)" (func 22))
```
Seeing this, the engine will find all `array.new` opcodes in a target application.
It will evaluate the predicate function, exported with the name "$20", and pass the `is_func_end`, and `op_name` state.
If it evaluates to true, the engine would then execute "$21" function, passing the requested state and remember this function's results.
The probe will be attached to this `array.new` location and the engine will pass the result of the execution of "$21" as well as `fid`, and `pc`, in that order.

There are many optimizations that the engine can perform to make the instrumentation run fast when using `wei`.
These optimizations are discussed in this paper: https://doi.org/10.1145/3763124

You might also notice that `wei` does not support general glob matching.
To keep from engines needing to implement this feature, `Whamm` expands globs to their lowest event match.
