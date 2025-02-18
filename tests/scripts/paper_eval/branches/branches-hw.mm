wasm::br_if:before {
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
    fid == 0 ||
    fid == 1 ||
    fid == 2
/ {
  report unshared var taken_branches: map<u32, u32>;

  // which branch was taken?
  // default branch is at 'num_targets' in the map
  var index: u32 = arg0 <= (num_targets - 1) ? arg0 : num_targets;

  // count stores an array of counters
  taken_branches[index]++;
}
