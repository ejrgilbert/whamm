wasm:opcode:call:after{
    map<(i32, i32, i32), i32> my_map;
    i32 b = 5;
    my_map[(b, 2, 3)] = 2;
    i32 a = my_map[(a, 2, 3)];
}