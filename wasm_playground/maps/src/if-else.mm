//works as intended - add to testing suite when you can
wasm:opcode:call:before / 
    target_fn_name == "inner_fn"
/
 {
    i32 a;
    if(true) {
        a = 3;
    } else {
        a = 4;
    };
}