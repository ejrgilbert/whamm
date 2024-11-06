/* Facts:
 * Bytecode names are probe types
 * Bound variables
 * - probe_func  : (whamm standard) is the Wasm function
 * - tos         : Wasm top-of-stack
 * - pc          : Wasm program counter
 * - local0...N  : Wasm locals
 * -
 * need to access top-of-stack, locals, program counter
 * need a handle to the function ("func")
*/

report map<(u32, u32, i32), i32> count;

wasm::br:before / fname == "calc" || fname == "print_x" / {
  // branch always taken for `br`
  // count stores an array of counters
  count[(fid, pc, 1)]++;
}

wasm::br_if:before / fname == "calc" || fname == "print_x" / {
  // which branch was taken?
  i32 index;
  index = arg0 != 0 ? 1 : 0;

  // count stores an array of counters
  count[(fid, pc, index)]++;
}

wasm::br_table:before / fname == "calc" || fname == "print_x" / {
  // which branch was taken?
  i32 index;
  index = arg0 < (num_targets - 1) ? targets[arg0] : default_target;

  // count stores an array of counters
  count[(fid, pc, index)]++;
}
