// Only instrument if it's calling 'print_x' or 'calc'
// TODO -- support pulling fname on Wizard target!
wasm:opcode:call:before /
    imm0 == 38
/ {
    report unshared var count: u32;
    count = count + 1;
}
