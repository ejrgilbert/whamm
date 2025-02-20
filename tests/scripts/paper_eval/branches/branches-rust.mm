// Matches _if and br_if events
wasm::*_if:before {
  report unshared var taken: i32;
  report unshared var not_taken: i32;

  // which branch was taken?
  if (arg0 != 0) {
    taken++;
  } else {
    not_taken++;
  }
}

wasm::br_table:before {
  report unshared var taken_branches: map<u32, u32>;

  // which branch was taken?
  // default branch is at 'num_targets' in the map
  var index: u32 = arg0 <= (num_targets - 1) ? arg0 : num_targets;

  // count stores an array of counters
  taken_branches[index]++;
}

// TODO -- need a virtual stack to support this! (even if I don't type bound the arguments)
// This is necessary in ALL cases since there is no way to do stack manipulation to keep around
// the stack arguments without saving them off to a local...buuuut I don't know what the local type
// would need to be! Hence, keeping a virtual stack around while visiting a function...yikes...what
// about branching???
// wasm::select(arg0: i32, arg1: i32, arg2: i32):before {
//   report unshared var selected_first: u32;
//   report unshared var selected_second: u32;
//
//   // which branch was taken?
//   if (arg2 != 0) {
//     selected_first++;
//   } else {
//     selected_second++;
//   }
// }