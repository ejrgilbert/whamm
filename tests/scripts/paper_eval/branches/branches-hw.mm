// Matches _if and br_if events
wasm::*if:before {
  report unshared var taken: i32;
  report unshared var total: i32;

  // which branch was taken?
  var was_taken: bool = arg0 != 0;
  taken = taken + (was_taken as i32);
  total++;
}

wasm::br_table:before {
  report unshared var taken_branches: map<u32, u32>;

  // which branch was taken?
  // default branch is at 'num_targets' in the map
  var index: u32 = arg0 <= (num_targets - 1) ? arg0 : num_targets;

  // count stores an array of counters
  taken_branches[index]++;
}

wasm::select(arg0: i32):before {
  report unshared var selected_first: u32;
  report unshared var total: u32;

  // which branch was taken?
  var was_taken: bool = arg0 != 0;
  selected_first = selected_first + (was_taken as u32);
  total++;
}
