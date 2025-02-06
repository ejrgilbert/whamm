wasm::br:before {
    report var taken: i32;
    // branch always taken for `br`
    taken++;
}

wasm::br_if:before {
    report var taken: i32;
    report var not_taken: i32;

    // which branch was taken?
    if (arg0 != 0) {
        taken++;
    } else {
        not_taken++;
    }
}
