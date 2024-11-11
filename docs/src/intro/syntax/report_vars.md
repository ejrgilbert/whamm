# `report` Variables #

The `report` keyword specifies variables whose values should be flushed.
When monitoring an application execution, data describing observations should be stored in a `report` variable to make use of this flushing feature.

Using `report` is really shorthand for `report unshared`, see [[`unshared`] variable](./unshared_vars.md) documentation.

The default behavior of this "flush" is to print to the console (the core `Whamm!` library uses WASI to do this).
Currently, this will be done _on each write_ to at least one of the `report` variables; however, there are plans to make this configurable.

[//]: # (When using bytecode rewriting, this will be done _on each write_ to at least one of the `report` variables.)
[//]: # (When using an engine, this will be done _at the end of program execution_ with the final values of the variables.)

For example:
```
report i32 count;
wasm:opcode:call:before {
    // count the number of times the `call` opcode was used during the application's dynamic execution.
    // (a single global count)
    count++;
}
```
