// TODO -- support pulling fname on Wizard target!
// TODO -- support maps on Wizard target!
report var count: map<(u32, u32, u32), i32>;

wasm::br:before /
    fid == 0 ||
    fid == 1 ||
    fid == 2
/ {
  // branch always taken for `br`
  // count stores an array of counters
  count[(fid, pc, 1)]++;
}

wasm::br_if:before /
    fid == 0 ||
    fid == 1 ||
    fid == 2
/ {
  // which branch was taken?
  var index: u32;
  index = arg0 != 0 ? 1 : 0;

  // count stores an array of counters
  count[(fid, pc, index)]++;
}
