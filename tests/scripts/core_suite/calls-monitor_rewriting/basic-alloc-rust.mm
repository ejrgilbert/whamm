// Only instrument if it's calling 'print_x' or 'calc'
// TODO -- support pulling fname on Wizard target!
wasm:opcode:call(arg0: i32, arg1: i32):before /
    target_fn_name == "print_x" ||
    target_fn_name == "calc"
//     imm0 == 61 ||        // calc
//     imm0 == 62           // print_x
/ {
    report var count: u32;
    if (arg0 == 0 || arg1 == 1) {
       count++;
    }
}