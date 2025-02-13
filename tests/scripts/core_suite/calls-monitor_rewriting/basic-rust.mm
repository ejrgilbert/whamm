// Only instrument if it's calling 'print_x' or 'calc'
// TODO -- support pulling fname on Wizard target!
wasm:opcode:call:before /
//     target_fn_name == "print_x" ||
//     target_fn_name == "calc"
    imm0 == 61 ||        // calc
    imm0 == 62           // print_x
/ {
    report unshared var count: u32;
    count = count + 1;
}
