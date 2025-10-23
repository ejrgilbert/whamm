# Serialization of State from `@static` Library Call #

In bytecode rewriting, the state of library calls annotated with `@static` is thrown away after match time has completed.
This means that if a monitor is dependent on any initialization that occurs during match time is not persistent to run time.

We could add a new capability where the Wasm state accrued during match time is serialized and then reloaded on startup.
Wasmtime has a feature like this called [core-dumps](https://docs.wasmtime.dev/examples-core-dumps.html).
There's also a tool called [`wizer`](https://github.com/bytecodealliance/wizer) that may be of use.
Maybe we can reuse some of its code to do this state serialization?
