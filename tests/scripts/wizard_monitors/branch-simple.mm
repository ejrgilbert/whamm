report map<(u32, u32, i32), i32> count;
wasm::br_if:before {
  i32 index;
  // "tos" is defined as the top-of-stack
  index = arg0 != 0 ? 1 : 0;
  // count stores an array of counters
  count[(fid, pc, index)]++;
}
