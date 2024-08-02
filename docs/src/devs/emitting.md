# Phase 4: Emit #

Here is documentation describing how we _emit_ `.mm` scripts.

## Some Helpful Concepts ##

**What is a `generator`?**
A `generator` is used to traverse some representation of logic in an abstract way.
It then calls the `emitter` when appropriate to actually emit the code in the target representation.

**What is an `emitter`?**
The `emitter` exposes an API that can be called to emit code in the target representation.
There will be as many emitters as there are target representations supported by the language.

In the context of `whamm!`, there are two `generator`s.
Each of these generators are used for a specific reason while emitting instrumentation.
The `InitGenerator` is run first to emit the parts of the `.mm` script that need to exist _before_ any probe actions are emitted, such as functions and global state.
The `InstrGenerator` is run second to emit the probes. 

Both of these generators use the `emitter` that emits instrumentation as configured by the end-user (either via _bytecode rewriting_ or emitting a `.v3` file that interfaces with an engine with direct support for instrumentation).

## 4.1 `InitGenerator` ##

The [`init_generator.rs`] traverses the AST to emit functions and globals that need to exist before emitting probes.
The `run` function is the entrypoint for this generator.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

Consider _bytecode rewriting_.
This generator emits new Wasm functions and globals into the program with associated Wasm IDs.
These IDs are stored in the `SymbolTable` for use while running the `InstrGenerator`.
When emitting an instruction that either calls an emitted function or does some operation with an emitted global, the name of that symbol is looked up in the `SymbolTable` to then use the saved ID in the emitted instruction.

[`init_generator.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/init_generator.rs

## 4.2 `InstrGenerator` ##

The [`instr_generator.rs`] traverses the `BehaviorTree` which encodes the logic of the instrumentation to emit.
The `run` function is the entrypoint for this generator.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

This `generator` calls into the `emitter` to gradually traverse the program in search for the locations corresponding to each probe.

[`instr_generator.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/instr_generator.rs

### Constant Propagation and Folding!! ###

Constant propagation and folding are a compiler optimizations that serve a special purpose in `whamm!`.
There are lots of resources online explaining these concepts if that would be useful to the reader.

The `whamm info` command helps users see various globals that are in scope when using various probe specifications.
All of these global variables are defined by `whamm!`'s compiler and _should only be emitted as constant literals_.
If the variable were ever emitted into an instrumented program or `.v3` monitor, the program would fail to execute since the variable _would not be defined_.

`whamm!` uses constant propagation and folding to remedy this situation!

The `define_*` functions in [`emitters.rs`] are examples of **how compiler constants are defined**.
These specific globals are defined in the emitter since their definitions are tied to locations in the Wasm program being instrumented.

The `ExprFolder` in [`types.rs`] performs constant propagation and folding on expressions.

When considering a _predicated probe_, this behavior can be quite interesting.
Take the following probe definition for example:
```
wasm:bytecode:call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_perform"
/ { ... }
```

All three of the globals used in the `predicate` are statically defined by the compiler and are provided by the `call` event.
This means that _all_ of these variable uses will be replaced by constants and the `predicate` will fold to a `true` or `false`.
If the `predicate` folds to `true`, the probe actions can be emitted at the found location without condition.
If the `predicate` folds to `false`, the probe _should not be emitted_.

Now, take the next probe definition example:
```
wasm:bytecode:call:alt /
    target_fn_type == "import" &&
    target_imp_module == "ic0" &&
    target_fn_name == "call_new" &&
    strcmp((arg0, arg1), "bookings") &&
    strcmp((arg2, arg3), "record")
/ { ... }
```

The `predicate` of this probe now includes both variables that are defined _statically_ and variables that are defined _dynamically_, which is totally valid semantically!

So, what happens here?
The first three globals will be propagated away to constants, the expression will be folded, and those constant equivalence checks will evaluate to either `true` or `false`.
This reduced value will then be _and_-ed together with the following dynamically-defined portion of the expression.
So, the same goal will be accomplished here as in the previous example (the probe either will or will not be emitted at that bytecode location based on statically determined information).
However, this time the actions emitted _will retain a conditional_, but it will be the folded conditional that only includes the dynamic portion of the original `predicate`.

Pretty cool, right??

[`emitters.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/emitters.rs
[`types.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/types.rs
