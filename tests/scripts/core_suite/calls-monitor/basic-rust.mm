// Only instrument if it's calling 'print_x' or 'calc'

wasm:opcode:call:before /
    imm0 == 29 ||        // calc
    imm0 == 30           // print_x
/ {
    report unshared var count: u32;
    count = count + 1;
}
