# Getting Started #

Here you will find information on how to begin writing instrumentation in the `whamm!` DSL.

# Installation #
The current way to install `whamm!` is to clone the repository, build the source yourself, and add the created binary to your `PATH` variable.
In the future, users will be able to download pre-built binaries on the GH releases page as we have stable, tagged releases of `whamm!`.

Steps:
1. Clone the [`whamm!` repo](https://github.com/ejrgilbert/whamm)
2. Build the source code with `cargo build`
3. Add the built binary to your `PATH`.
   This binary should be located at `target/debug/whamm`<sup>[1](#why_target)</sup>.

## Basic Test ##
A basic test you can run to make sure that the `whamm!` binary is on your path and working as-expected is running the following command: `whamm --help`. The CLI will provide information on various commands and options available for use.

# Wasm monitors and manipulators #

As mentioned in the [introduction](../intro.md), `whamm!` can be used to either monitor **OR** manipulate a program's execution.

What we mean by **monitor** execution is _collect some information_ about a program's dynamic behavior.
This is commonly used for debugging, logging, and metric collection.

What we mean by **manipulate** execution is to literally _change_ the program's dynamic behavior.
Consider a specific feature of many debugger tools: using a debugger, a developer can set a breakpoint, inspect the current application state, and _change the values of variables_.
This is an example of manipulating an application's dynamic behavior through changing the state and something we will support doing in `whamm!`.

Continue reading through this book's "getting started" content for how to write such _monitors_ and _manipulators_.

# Helpful Tools #

Here are some tools that may help when working with Wasm:
1. [`wabt`](https://github.com/WebAssembly/wabt), aka the WebAssembly Binary Toolkit
2. [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools)

<a name="why_target">1</a>: We recommend adding the binary built inside `target/` to your path as this will enable you to pull the latest changes on `master`, build the latest version, and automatically have the latest binary on your `PATH`.