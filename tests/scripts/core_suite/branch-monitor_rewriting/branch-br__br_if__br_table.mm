// TODO -- cannot flush global report variables yet on Wizard target
// TODO -- support pulling fname on Wizard target!
// TODO -- support flushing map types on Wizard target
report var count: map<(u32, u32, u32), i32>;

wasm::br:before /
    fname == "calc" ||
    fname == "print_x" ||
    fname == "opt_str"
/ {
  // branch always taken for `br`
  // count stores an array of counters
  count[(fid, pc, 1)]++;
}

// wasm::br_if:before /
//     fid == 27 ||        // calc
//     fid == 28 ||        // print_x
//     fid == 29           // opt_str
// / {
//   // which branch was taken?
//   var index: u32;
//   index = arg0 != 0 ? 1 : 0;
//
//   // count stores an array of counters
//   count[(fid, pc, index)]++;
// }

// wasm::br_table:before /
//     fname == "calc" ||
//     fname == "print_x" ||
//     fname == "opt_str"
// / {
//   // which branch was taken?
//   var index: u32;
//   index = arg0 <= (num_targets - 1) ? targets[arg0] : default_target;
//
//   // count stores an array of counters
//   count[(fid, pc, index)]++;
// }
