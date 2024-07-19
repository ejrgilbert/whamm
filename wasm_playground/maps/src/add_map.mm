wasm:opcode:call:after{
    map<i32, i32> my_map;
    my_map[1] = 2;
    i32 a = my_map[1];
}