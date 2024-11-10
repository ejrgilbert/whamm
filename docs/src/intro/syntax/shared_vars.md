# `shared` Variables #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

If a variable is marked as `shared`, a single instance of this variable is available to _every match site_ for a probe.
The scope of this variable is limited to _all match sites_ of a probe.
The value of this variable will be stable on each entry into the probe's logic, meaning that it will not be reinitialized each time.

This can be used to collect data for some probed event _over time_.

For example:
```
wasm:opcode:call:before {
    // collect the number of times the `call` opcode is executed during dynamic execution.
    // (a single count tied to the wasm:opcode:call event in the program)
    // This variable will not be reinitialized each time this probe's body is executed,
    // rather, it will be the value it was the last time it ran!
    shared i32 count;
    count++;
    
    // This variable will be reinitialized to 0 each time this probe's body is executed
    i32 local_variable;
    local_variable++;
}
```
