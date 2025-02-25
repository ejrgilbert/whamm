// Only instrument if it's calling 'print_x' or 'calc'

wasm:opcode:call:before /
    imm0 == 27 ||        // calc
    imm0 == 28           // print_x
/ {
    report unshared var count: u32;
    count = count + 1;
}
