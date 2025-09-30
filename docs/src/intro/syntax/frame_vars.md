# `shared` Variables #

NOTE: This functionality hasn't been fully implemented! More docs to come post-implementation!

If a variable is marked as `frame`, a single instance of this variable, the variable itself is stored on a function's active frame.
So, the variable is in-scope for each probe in that active frame.
The value of this variable will be stable across probe accesses while a specific function's frame is active.

This can be used to collect data for some probed event _during a specific function's execution_.

For example:
```
// Tracks all of the runtimes of each function invocation across program execution.
use time;
wasm:func:entry {
    frame var start: u32;
    start = time.now();
}
wasm:func:exit {
    frame var start: u32;
    unshared var elapsed: vec<u32>;
    elapsed.push(time.now() - start);
}
```
