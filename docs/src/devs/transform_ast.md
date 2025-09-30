# Phase 3: Transforming the AST #

This is done by first collecting metadata about each probe in an AST traversal via the [`MetadataCollector`].
This collector tracks the following (this list may extend in the future):
- The user library functions that were actually used in a script.
- The bound functions that were actually used in a script.
- Each data type that will need to be flushed at report time.
- The strings that are necessary to exist for the script to work.
- Whether any probe has state that needs to be initialized.

All of this state tracking helps emit better code in the compiler backend.
It also summarizes the statically and dynamically bound variables for each probe.
This list is used in bytecode rewriting to define the necessary static data at compile time and to emit the necessary code to define dynamic data at runtime.
For the `wei` backend, each of these bound variables is requested from the engine so that the engine passes their value to the probe, or predicate, callback function.

[`MetadataCollector`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/metadata_collector.rs

When bytecode rewriting, the AST resulting from the metadata collection step is then translated into a [`SimpleAst`].

[`SimpleAst`]: https://github.com/ejrgilbert/whamm/blob/master/src/generator/rewriting/simple_ast.rs