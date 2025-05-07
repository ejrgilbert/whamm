# Frame Variables #

Consider the following Whamm script:
```
map<i32, i32> total_time;
i32 enter_time;           // NEW IDEA: `frame i32 enter_time;`

wasm:func:entry {
   enter_time = clock();
}

wasm:func:exit {
   total_time[fid] += (clock() - enter_time);
}
```

This actually wouldn't work correctly for recursive functions since the `total_time` is dependent on global state.
This also wouldn't work using a local variable inside a probe body since it wouldn't be accessible by another probe...

Thus enter `frame` variables.
This type of variable actually attaches its scope to the function that an event matches.
Meaning, that for all probe bodies executing inside a specific function, they all have access to that function's `frame` variables.
The implementation for bytecode rewriting would be straightforward since it can just be a Wasm `local` variable.
However, the implementation in Wizard would be a bit more complex since we would need a new datastructure to store the state that hangs off a function.

Another example:
```
wasm:func:entry {
    frame var start: u32;
    start = time.now();
}
wasm:func:exit {
    frame var start: u32;
    report var profiles: vec<u32>;
    profiles.push(time.now() - start);
}
```
