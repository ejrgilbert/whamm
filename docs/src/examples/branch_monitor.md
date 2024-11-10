# Branch Monitor #

Here is an example monitor that can be written in `Whamm!`, it does not require an instrumentation library.
Rather, it uses the DSL to express all of its monitoring logic.

```
wasm::br:before {
    report unshared i32 taken;
    // branch always taken for `br`
    // count stores an array of counters
    taken++;
}

wasm::br_if:before {
    report unshared i32 taken;
    report unshared i32 not_taken;
    
    // which branch was taken?
    if (arg0 != 0) { // arg0: Wasm top-of-stack
        taken++;
    } else {
        not_taken++;
    }
}

wasm::br_table:before {
    report unshared map<i32, i32> taken_branches;
    
    // which branch was taken?
    i32 index;
    
    // arg0: the Wasm top-of-stack
    // num_targets: the number of targets for this br_table
    // targets: the branches that can be targeted by this br_table
    // default_target: the default target for this br_table
    index = arg0 < (num_targets - 1) ? targets[arg0] : default_target;
    
    taken_branches[index]++;
}
```
