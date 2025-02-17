// TODO -- support pulling fname on Wizard target!

wasm::br:before /
//     fname == "calc" ||
//     fname == "print_x"
    fid == 61 ||        // calc
    fid == 62           // print_x
/ {
    report unshared var taken: i32;
    // branch always taken for `br`
    taken++;
}

wasm::br_if:before /
    fid == 61 ||        // calc
    fid == 62           // print_x
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
