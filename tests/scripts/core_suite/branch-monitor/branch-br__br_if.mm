// TODO -- support pulling fname on Wizard target!

wasm::br:before /
    fname == "calc" ||
    fname == "print_x"
/ {
    report unshared var taken: i32;
    // branch always taken for `br`
    taken++;
}

wasm::br_if:before /
    fid == 38 ||        // calc
    fid == 39           // print_x
/ {
    report unshared var taken: i32;
    report unshared var not_taken: i32;

    // which branch was taken?
    if (arg0 != 0) {
        taken++;
    } else {
        not_taken++;
    }
}
