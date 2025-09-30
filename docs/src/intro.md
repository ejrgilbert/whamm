# Introduction

Debugging Wasm? Put some `Whamm` on it!

If you're building a new _dynamic analysis_ for Wasm and are looking for a framework to support you, you're in the right place!
`Whamm` is a framework for "Wasm Application Monitoring and Manipulation"<sup>[1](#silent_h)</sup>.
This book will help you [get on the right track](intro/getting_started.md) when working with the language.

Tools in the `Whamm` framework are expressed using its domain-specific language (DSL).
The DSL is inspired by [Dtrace's `D` language](https://illumos.org/books/dtrace).
The `Whamm` DSL enables Wasm tool implementers to express their instrumentation using high-level abstractions of program events or locations at various levels of granularity.

Here are some of the goals of the `Whamm` DSL:
1. is **high-level** and **intuitive**
2. instrumentation is **easy-to-build**, [**testable**](intro/testing.md), and **debuggable**
3. express instrumentation in terms of [**predicated probes**](intro/language.md)
4. can instrument **[events](intro/events.md) of different granularity**
5. provide **[behavior](intro/libraries.md) as Wasm functions**, say where to call them in `Whamm`
6. **write instrumentation once**, `Whamm` takes care of the [injection strategy](intro/injection_strategies.md).

A tool written in the `Whamm` framework can be applied using an engine's dynamic instrumentation capabilities or bytecode rewriting (if such capabilities aren't available).
You can use the same tool for either strategy, Whamm takes care of the injection!
Through providing such portability, Whamm broadens the applicability of tools, reducing tooling fragmentation across source languages (since Wasm is a polyglot) and Wasm engine (since it has the bytecode rewriting fallback that runs anywhere).

Let's take a moment to consider the scale of impact that this DSL could have on developer tooling by considering the following facts:
1. WebAssembly is growing to _use cases beyond the browser_.
2. Many languages can compile to Wasm. 
3. With `Whamm` **write instrumentation once** to support wide domain of apps.
   - Use **engine** instrumentation _capabilities as available_.
   - Use **bytecode rewriting** to _support everything_ else.

This means that developer tools written in `Whamm` could support a **vast domain of applications**, making WebAssembly the **future target platform for debugging**.

## Resources ##

Injection strategies:
- Paper describing the _non-intrusive_ injection strategy used by `Whamm`: [Flexible Non-intrusive Dynamic Instrumentation for WebAssembly](https://dl.acm.org/doi/10.1145/3620666.3651338)
- The library used to do the _intrusive_ injection strategy (bytecode rewriting): [wirm](https://github.com/composablesys/wirm)

## Some helpful terms and concepts ##

1. **[WebAssembly (Wasm)](https://webassembly.org/)**:
   WebAssembly is a binary instruction format for a stack-based virtual machine.
   It is designed as a portable compilation target for programming languages.
2. **Instrumentation**:
   When we say we are "_instrumenting_ a program," at a high-level we mean we are "_injecting some code_ into a program’s execution to _do some operation_."
   This definition is intentionally generic since instrumentation can really do anything we can imagine!
   You can use instrumentation to build debuggers, dynamic analyses, telemetry generators, and more.
3. **Dynamic analysis**:
   A dynamic analysis is something that analyzes a program as it is executing (in contrast to a static analysis which analyzes a program that is not running).
   This type of analysis can gain useful insights into a program as it is able to access information that is not available statically (such as hot code locations, memory accesses over time, code coverage of test suites, etc.).
4. **Bytecode rewriting**:
   This is an example strategy for injecting instrumentation logic into the application.
   It injects instrumentation through literally inserting new instructions into the application bytecode.

<a name="silent_h">1</a>: The 'h' is silent.
 