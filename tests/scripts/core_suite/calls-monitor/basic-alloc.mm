// Only instrument if it's calling 'print_x' or 'calc'
// TODO -- support pulling fname on Wizard target!
wasm:opcode:call(arg0: i32, arg1: i32):before /
    target_fn_name == "print_x" ||
    target_fn_name == "calc"
//     imm0 == 38 ||
//     imm0 == 39
/ {
    report var count: u32;
    if (arg0 == 0 || arg1 == 1) {
       count++;
    }
}