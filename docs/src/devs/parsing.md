# Phase 1: Parse #

Here is documentation describing how we _parse_ `.mm` scripts.

## The Grammar ##
[whamm.pest]

`whamm!`'s grammar is written using the [Pest] parser generator Rust library, which uses Parsing Expression Grammars (PEG) as input.
Reading the [Pest] book first will inform how to read the [whamm.pest] grammar.

Pest parses a passed `.mm` script and creates a set of matched `Rule`s that are then traversed in the [`whamm_parser.rs`] to generate `whamm!`'s Abstract Syntax Tree (AST).
These `Rule`s correspond to the naming used in the `whamm.pest` grammar.

The logic for creating the AST from the Pest `Rule`s can be followed by starting at the parsing entrypoint: the `parse_script` function found in the [`whamm_parser.rs`] file.

[whamm.pest]: https://github.com/ejrgilbert/whamm/blob/master/src/parser/whamm.pest
[Pest]: https://pest.rs/book/

[`whamm_parser.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/parser/whamm_parser.rs

## The Abstract Syntax Tree (AST) ##

We use an AST to represent the `.mm` script after parsing.
This AST is leveraged in different ways for each of the subsequent compiler phases.

During [**verification**](verifying.md), the AST is used to build the `SymbolTable` and perform type checking.

While [**building the behavior tree**](behavior_tree.md), the AST is used to inform what the behavior should be as instrumentation is being injected into the target program (for bytecode rewriting).
Since the AST encodes the events utilized by the instrumentation and the predicates that must be partially evaluated during injection, the built behavior tree encodes a flow of actions customized to the instrumentation to be emitted.
While building the behavior tree, a _simpler variation of the AST_ is created to optimize the lookup of information that is relevant during the emit phase.

While [**emitting**](emitting.md), the _simpler AST variation_ mentioned above is used to lookup global statements and iterate over probe definitions to inject them into locations-of-interest in the Wasm program.
