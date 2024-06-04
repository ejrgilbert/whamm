# Introduction

whamm! is a tool for "Wasm Application Monitoring and Manipulation"[^note], a DSL inspired by the D language.


In a high level, we wish to insert probes into a WebAssembly application, to gain some insights into its execution. A probe is a location or activity to which Whamm can bind a request to perform a set of actions, like recording a stack trace, a timestamp, or the argument to a function. Probes are like programmable sensors scattered all over your wasm application in interesting places.

For a comprehensive guide on using DTrace and the D language, see [the Dynamic Tracing Guide](https://illumos.org/books/dtrace/bookinfo.html).


[^note]: The 'h' is silent.
 