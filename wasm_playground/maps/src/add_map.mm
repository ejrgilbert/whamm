report i32 whatever;
wasm:opcode:call:before / 
    target_fn_name == "inner_fn"
/
 {
    if(true) {
        report i32 c;
    };
    report i32 a;
    a = 5;
    report map<i32, i32> m;
    m[1] = 2;
    whatever = 3;
}
wasm:opcode:call:after / 
    target_fn_name == "foo"
/
 {
    report i32 b;
    a = 3;
}