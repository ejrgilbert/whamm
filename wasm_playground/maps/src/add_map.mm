report i32 whatever;
wasm:opcode:call:before / 
    target_fn_name == "inner_fn"
/
 {
    if(true) {
        report i32 c;
    };
    report i32 a;
    map<i32, i32> m;
    a = 5;
    whatever = 3;
}
wasm:opcode:call:after / 
    target_fn_name == "foo"
/
 {
    a = 3;
}