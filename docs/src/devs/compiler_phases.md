# The Four Phases of Compilation #

First, what is meant by the term "compilation" depends on the selected injection strategy.

For **bytecode rewriting**, compilation means generating a new _instrumented_ variation of the passed program.

For **direct engine support**, compilation means compiling the `.mm` script to a `.v3` program that interfaces with an engine to instrument the program dynamically.
The original program _is not touched_ when using this strategy.

The first three phases of `whamm!` compilation are identical for both strategies.
The final `emit` phase is where the variation lies.
This is because "emitting" for **bytecode rewriting** means using the `walrus` library to insert new instructions into the program.
Whereas "emitting" for **direct engine support** means emitting `Virgil` code to specify the instrumentation probes in a new format that leverages the target engine's instrumentation API.

These are the four phases of compilation:
1. [Parse](parsing.md)
2. [Verify](verifying.md)
3. [Encode as a `BehaviorTree`](behavior_tree.md)
4. [Emit](emitting.md)
