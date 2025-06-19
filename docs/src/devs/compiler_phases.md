# The Four Phases of Compilation #

First, what is meant by the term "compilation" depends on the selected injection strategy.

For **bytecode rewriting**, compilation means generating a new _instrumented_ variation of the application's bytecode.

For **direct engine support**, compilation means compiling the `.mm` script to a new Wasm module that interfaces with an engine to instrument the program dynamically.
The original program is _not touched_ **and** _not provided_ when using this strategy.

The first three phases of `whamm!` compilation are identical for both strategies.
The `translate` and `emit` phases vary between injection strategy.
This is because "emitting" for **bytecode rewriting** means using the `orca` library to insert new instructions into the program.
Whereas "emitting" for **direct engine support** means emitting a Wasm module encoding _where to instrument_ and the callbacks to attach at the probed sites by interfacing with the engine at application runtime.

These are the four phases of compilation:
1. [Parse](parsing.md)
2. Configure the `Whamm!` [Core Library](./core_lib.md) (if needed)
3. [Verify](verifying.md)
4. [Translate](translate.md) AST into the injection strategy's representation
5. [Emit](emit/emitting.md)
