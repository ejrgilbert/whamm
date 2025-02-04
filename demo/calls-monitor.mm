wasm:opcode:call(arg0: i32):before
/ imm0 == 2 && arg0 == 0 /
{
    report var count: i32;

    count++;
}