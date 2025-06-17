// Matches _if and br_if events
wasm::*if:before {
    report unshared var taken: i32;
    report unshared var not_taken: i32;

    // which branch was taken?
    if (arg0 != 0) {
        taken++;
    } else {
        not_taken++;
    }
}

wasm:opcode:br_table:before {
    report unshared var taken_branches: map<u32, u32>;

    // which branch was taken?
    // default branch is at 'num_targets' in the map
    var index: u32 = arg0 <= (num_targets - 1) ? arg0 : num_targets;

    // count stores an array of counters
    taken_branches[index]++;
}

wasm::select(arg0: i32):before {
    report unshared var selected_first: u32;
    report unshared var selected_second: u32;

    // which branch was taken?
    if (arg0 != 0) {
        selected_first++;
    } else {
        selected_second++;
    }
}