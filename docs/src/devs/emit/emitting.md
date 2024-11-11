# Phase 5: Emit #

Here is documentation describing how we _emit_ `.mm` scripts.

## Some Helpful Concepts ##

**What is a `generator`?**
A `generator` is used to traverse some representation of logic in an abstract way.
It then calls the `emitter` when appropriate to actually emit the code in the target representation.

**What is an `emitter`?**
The `emitter` exposes an API that can be called to emit code in the target representation.
There will be as many emitters as there are target representations supported by the language.

## The Injection Strategies ##

The code that is emitted, and the methodology in which emitting happens, depends on the injection strategy specified by the user.

There are currently two supported injection strategies:
1. [Bytecode Rewriting](./engine_target.md)
2. Interfacing with an [engine](./rewriting_target.md)
