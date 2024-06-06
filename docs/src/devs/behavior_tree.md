# Phase 3: Encode as a `BehaviorTree` #

Here is documentation describing how we encode `.mm` scripts _as a `BehaviorTree`_.
NOTE: This is only used for bytecode rewriting!

## What is a Behavior Tree? ##

The following pages are great resources describing behavior trees and how they can be used:
1. [What is a behavior tree?]
2. [Introduction to BTs]

[What is a behavior tree?]: https://www.behaviortree.dev/docs/intro#what-is-a-behavior-tree
[Introduction to BTs]: https://www.behaviortree.dev/docs/learn-the-basics/BT_basics

## Why even do this? ##

That's a great question, I'm glad you asked!

The first implementation of this DSL required traversing lots of the AST inside the emitter rather than building something (e.g. a `generator`) that generically traversed the AST and calling the emitter to do low-level instruction emission events.
Consider what the semantic implications of a probe definition.
A probe specifies locations in a program to insert some instrumentation code.
One probe could result in 0 to many matched locations.
This means, to emit a probe, the program must be traversed gradually, if a location is found, stop, check if the predicate tells us to emit the probe, if it does, we emit the probe's actions that that point.

If we think about this logic at a high level _without_ the use of the behavior tree, there is a weird fuzzy layer between the `generator` and `emitter`.
Either the `generator` needs to have some `emitter` logic in it (traversing the program), or the `emitter` needs to have some `generator` logic in it (traversing the AST).
The `BehaviorTree` decouples the two since the `generator` logic is now in terms of the `BehaviorTree`'s control flow which encodes the decisions and actions to be taken while instrumenting the program rather than hardcoding those decisions and actions while traversing the AST!

This also makes adding new instrumentable `event`s easier since the instrumentation logic can be encoded in the `BehaviorTree` instead of hardcoding yet another conditional block to support the new functionality. 

## Visualization for Debugging ##
`whamm vis-script --help`

The `whamm` CLI provides an easy way to generate a visualization of the `BehaviorTree` to make it easier-to-debug the control flow of instrumentation.

## Building the `BehaviorTree` ##

The [`builder_visitor.rs`] file builds the `BehaviorTree` from the script's AST.
The `visit_whamm` function is the entrypoint for this action.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

While building the `BehaviorTree` a simpler version of the script's AST is also built (see `SimpleAST` in [`builder_visitor.rs`]).
This new representation makes it easier to lookup relevant pieces of information that will be relevant to the logic performed in `instr_generator.rs`.

[`builder_visitor.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/behavior/builder_visitor.rs

## Using the `BehaviorTree` ##

The [`instr_generator.rs`] file actually uses the `BehaviorTree` to follow the logic necessary to make decisions about emitting a probe into a program.
The `run` function is the entrypoint for this action.
This follows the visitor software design pattern.
There are great resources online that teach about the visitor pattern if that is helpful for any readers.

[`instr_generator.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/instr_generator.rs
