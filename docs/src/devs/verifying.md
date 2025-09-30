# Phase 3: Verify #

Here is documentation describing how we _verify_ `.mm` scripts.

## The `SymbolTable` ##

During verification, first the `SymbolTable` is build from the AST.
There are great resources online that teach about symbol tables if that is helpful for any readers.

At a high-level, the `SymbolTable` stores **metadata** about source code **symbols**.
For `.mm` scripts, symbols can be parts of the probe match rule (e.g. provider, package, event, and mode), function names, and variables (local or global).
The `enum` named `Record` in the [`verifier/types.rs`] file defines these symbols and the metadata-of-interest for each of them.

The **metadata**-of-interest tends to be _type information_ and _addressing_ that corresponds to the ID assigned to the symbol after it's been emitted into the Wasm program.

_Type information_ is used when type checking the script _and_ when emitting the instrumentation (to know the type of each item being emitted).

An example of when the _addressing_ metadata is used is emitting and calling functions.
After a function defined by the instrumentation has been injected, the ID for this function would need to be stored in the `SymbolTable` to be looked up and used when there a call to the function is being emitted at some future point (see the [`InitGenerator` documentation]).

These **symbols** are contained within some **scope**.
The types of scopes present in a `.mm` script can be found in the `enum` named `ScopeType` in the [`verifier/types.rs`] file.
The concept of scopes in this context is the same as in other programming languages.
A scope defines where variables and functions are accessible based on their location in a program.

In the context of `Whamm` there are some scopes that exist but aren't accessible to the end-user.
Consider the probe match rule: `provider:package:event:mode`.
Each part of this match rule really has its own scope.
This enables each part to introduce its own helpful global variables and functions that the user can leverage to write more expressive instrumentation!
These bound variables and functions are added to the AST in the [`whamm_parser.rs`] file.
See the [probes syntax documentation] for a helpful CLI tool that enables the user to see what is in-scope for any given probe match rule.


[`verifier/types.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/verifier/types.rs
[`InitGenerator` documentation]: emit/emitting.md#parta-initgenerator
[probes syntax documentation]: ../intro/syntax/probes.md#helpful-info-in-cli
[`whamm_parser.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/parser/whamm_parser.rs

### Problems / Workarounds ###

#### 1. Ownership of `Record`s and `Scope`s. ####

When writing the `SymbolTable` structure, there were issues with pushing the ownership of `Record`s and `Scope`s down into each parent.
The workaround was to hold a `Vec` of _all_ `Record`s and `Scope`s for the entire program in the `SymbolTable` `struct`, then hold `usize` types that indexes into these `Vec`s in the `Record`s and `Scope`s.

It _is possible_ that this could be avoided by just boxing the values in the `Record`s and `Scope`s, but some experimentation needs to be done.

### Building the `SymbolTable` ###

The [`builder_visitor.rs`] file builds the `SymbolTable` from the script's AST.
The `visit_whamm` function is the entrypoint for this behavior.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

[`builder_visitor.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/verifier/builder_visitor.rs

## The `TypeChecker` ##

The type checker then visits the AST and uses the `SymbolTable` to verify that variable usage is appropriate.
It can find out-of-scope usages, invalid method invocations, misused types, etc.
