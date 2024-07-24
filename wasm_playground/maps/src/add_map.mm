wasm:opcode:call:after  {
    map<i32, i32> a;
    i32 b = 3;
    a[1] = 2;
    a[b] = b + 1;
}