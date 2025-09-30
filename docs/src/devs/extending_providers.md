# Extending Provider Definitions #

(This is rather tedious at the moment. We plan to change the way this is done in the future.)

`Whamm` providers are defined via `.yaml` files located in the [`providers/`] folder.
They are processed by the [`provider_handler.rs`] to find matching events based on probe match rules.

The reason this is so tedious is that all `.yaml` files supplied to `Whamm` are concatenated into one giant `.yaml` file to be compatible with Rust's yaml parsing library.
Further, this library has a bug when handling `yaml` anchors, which makes what we have to write very redundant.
Since this library has been recently archived, we've really got to move to something more stable.

Anyway, to extend what we have at the moment, you can simply edit the files you find in the [`providers/`] folder.
To test your changes, use the `whamm info` utility, it should be able to show you changes immediately.
If you get an error during parsing the `yaml`, there's something wrong with your changes.

Further, remember that the `whamm` CLI supports re-routing where it looks for the [`providers/`] folder.
To do so, use the `--defs-path` option.

## Binding the data for Rewriting

If you're wanting to support your `provider` changes in the rewriting backend of `Whamm`, you'll need to define what the state actually is in the compiler.
All of this is managed in [`emitter/rewriting/rules/mod.rs`].
This file matches on the probe match rule (e.g. "wasm:opcode:call:before") and then takes a look at what application state the probe needs to work.
It then defines this state and allows the compiler to continue on with its probe insertions.

[`emitter/rewriting/rules/mod.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/emitter/rewriting/rules/mod.rs

## Provider Defs + `wei` = Awesome

This functionality can even let you define totally new providers for `Whamm`, letting you instrument anything with Wasm code!
Maybe you want to instrument linux syscalls with Wasm instrumentation (look out eBPF).
To do this, you would need to create provider definitions for this and then create something that supports `wei`.
This `wei` thing would do the same thing a Wasm engine does on your monitor module, finding relevant exported functions, performing predication, and remembering the state to pass on callback invocation.
Then it would need to somehow insert logic into the kernel that calls the callbacks and passes the relevant state to the Wasm module.

This really lets you instrument anything with Wasm using a simple, extensible interface: `wei`.

[`providers/`]: https://github.com/ejrgilbert/whamm/tree/master/providers
[`provider_handler.rs`]: https://github.com/ejrgilbert/whamm/blob/master/src/parser/provider_handler.rs