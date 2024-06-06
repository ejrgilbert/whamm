# Injection Strategies #

Debugging and profiling programs are an integral part of engineering software.
This is done through instrumenting the program under observation (inserting instructions that provide insight into dynamic execution).

The most-common instrumentation techniques, such as **bytecode rewriting**, _inject instructions directly into the application code_.
While this method enables instrumentation to support any application domain, it _intrudes_ on the program state space (possibly introducing bugs), complicates the implementation, limits the scope of observation, and cannot dynamically adapt to program behavior.

Instead, one can remedy these issues with bytecode rewriting by **interfacing with a runtime engine** that _directly supports instrumentation_.
This technique can bring powerful capabilities into play as demonstrated by the Wizard research engine, in the ASPLOS paper [Flexible Non-intrusive Dynamic Instrumentation for WebAssembly](https://dl.acm.org/doi/10.1145/3620666.3651338).
This paper demonstrated how to build instrumentation support that protects the application-under-observation, provides consistency guarantees to enable composable tooling, applies JIT optimizations specific to instrumentation that make some tools run even faster than bytecode rewriting, and more.
However, this technique is not as widely-used as bytecode rewriting since it limits a tool's scope to applications that can run on such engines.

This is where `whamm!` comes in.
This DSL abstracts above the instrumentation technique to enable developer tooling to support a broad domain of applications while leveraging runtime capabilities as-available without reimplementation.
With `whamm!` you can _write instrumentation once_ and _support wide domain_ of apps.
_Use engine instrumentation_ capabilities _as available_.
_Use bytecode rewriting_ to support _everything else_.

## Bytecode Rewriting ##
[walrus](https://github.com/rustwasm/walrus)

To perform the bytecode rewriting injection strategy, `whamm!` leverages the `walrus` Rust library.
This library loads a Wasm module into an AST representation that can then be traversed and manipulated to inject the instrumentation logic.
Read more about the low-level details in the [developers documentation](../devs/intro.md).

## Direct Engine Support ##
[Flexible Non-intrusive Dynamic Instrumentation for WebAssembly](https://dl.acm.org/doi/10.1145/3620666.3651338)

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!
