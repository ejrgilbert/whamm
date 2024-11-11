# `unshared` Variables #

If a variable is marked as `unshared`, an instance of this variable is made available to _every match site_ for a probe.
The scope of this variable is limited to the specific _match site_.
The value of this variable will be stable on each entry into the probe's logic, meaning that it will not be reinitialized each time.

This can be used to collect data at a specific program point _over time_.

For example:
```
wasm:opcode:call:before {
    // collect the number of times each `call` opcode is executed during dynamic execution.
    // (as many counts as there are `call` opcodes in the program)
    // This variable will not be reinitialized each time this probe's body is executed,
    // rather, it will be the value it was the last time it ran!
    unshared i32 count;
    count++;
    
    // This variable will be reinitialized to 0 each time this probe's body is executed
    i32 local_variable;
    local_variable++;
}
```
