# Introduction

Debugging Wasm? Put some `whamm!` on it!

`whamm!` is a tool for "Wasm Application Monitoring and Manipulation"[^note], a domain-specific language (DSL) inspired by [Dtrace's `D` language](https://illumos.org/books/dtrace).
If you're building a new _dynamic analysis_ for Wasm and are looking for a framework to support you, you're in the right place!
This book will help you [get on the right track](intro/getting_started.md) when working with the language.

`whamm!` enables Wasm tool implementers to express their instrumentation using high-level abstractions of program events or locations at various levels of granularity, increasing the expressiveness, intuitiveness, and maintainability of the tool.

Here are some of the goals of `whamm!`:
1. is **high-level** and **intuitive**
2. instrumentation is **easy-to-build**, [**testable**](intro/testing.md), and **debuggable**
3. express instrumentation in terms of [**predicated probes**](intro/grammar.md)
4. can instrument **[events](intro/events.md) of different granularity**
5. provide **[behavior](intro/libraries.md) as Wasm functions**, say where to call them in `whamm!`
6. **write instrumentation once**, `whamm!` takes care of the [injection strategy](intro/injection_strategies.md).

# Who would use this DSL? #
TODO

# What is instrumentation? #
TODO

# What is dynamic analysis? #
TODO


Instrumentation that is built using  is easy

Parsers that use pest are easy to design and maintain due to the use of Parsing Expression Grammars, or PEGs. And, because of Rust's zero-cost abstractions, pest parsers can be very fast.

The target end-users o

This DSL enables tool implementers to express their instrumentation in terms of program events and corresponding predicated actions; "When _this event_ occurs during program execution, do _these actions_ if _this predicate_ (or conditional) evaluates to true."
This abstraction provides a high-level and intuitive syntax that can target events at various granularities in the instrumented program.

We use the term `probe` to refer to this triple of `event`, `predicate` and `actions`.

The overarching goal of this DSL is to enable tool implementers to write instrumentation in an intuitive way, by express
At a high level, we wish to insert probes into a WebAssembly application, to gain some insights into its execution.
A probe is a location or activity to which Whamm can bind a request to perform a set of actions, like recording a stack trace, a timestamp, or the argument to a function.
Probes are like programmable sensors scattered all over your wasm application in interesting places.

For a comprehensive guide on using DTrace and the D language, see [the Dynamic Tracing Guide](https://illumos.org/books/dtrace/bookinfo.html).

[^note] The 'h' is silent.
 