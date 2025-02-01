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

wasm::br:before / fname == "calc" || fname == "print_x" / {
    report unshared var taken: i32;
    // branch always taken for `br`
    // count stores an array of counters
    taken++;
}

wasm::br_if:before / fname == "calc" || fname == "print_x" / {
    report unshared var taken: i32;
    report unshared var not_taken: i32;

    // which branch was taken?
    if (arg0 != 0) {
        taken++;
    } else {
        not_taken++;
    }
}

// wasm::br_table:before / fname == "calc" || fname == "print_x" / {
//     report unshared map<i32, i32> taken_branches;
//     // which branch was taken?
//     i32 index;
//     index = arg0 < (num_targets - 1) ? targets[arg0] : default_target;
//
//     // count stores an array of counters
//     taken_branches[index]++;
// }
