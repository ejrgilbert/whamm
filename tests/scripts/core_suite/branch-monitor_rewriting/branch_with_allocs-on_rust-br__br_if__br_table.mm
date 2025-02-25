// TODO -- cannot flush global report variables yet on Wizard target
// TODO -- support pulling fname on Wizard target!

wasm::br:before /
    fname == "calc" ||
    fname == "print_x" ||
    fname == "opt_str"
/ {
  // branch always taken for `br`
  report unshared var taken: i32;
  taken++;
}

wasm::br_if:before /
    fid == 27 ||        // calc
    fid == 28 ||        // print_x
    fid == 29           // opt_str
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

wasm::br_table:before /
    fid == 27 ||        // calc
    fid == 28 ||        // print_x
    fid == 29           // opt_str
/ {
  report unshared var taken_branches: map<u32, u32>;

  // which branch was taken?
  var index: u32;
  index = arg0 <= (num_targets - 1) ? targets[arg0] : default_target;

  // count stores an array of counters
  taken_branches[index]++;
}
