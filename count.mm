// Implements a opcode monitor for call

i32 count;
wasm:opcode:call:before {
    count++;
}
