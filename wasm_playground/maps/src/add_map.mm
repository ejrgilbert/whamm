report i32 hello;
wasm:opcode:call:after / 
    target_fn_name == "do_something"
/
 {
    hello = 3;
    map<i32, i32> a;
    i32 b = 3;
    a[1] = 2;
    a[b] = b + 1;
    i32 c = a[1];
    a[c + a[1]] = a[b] + 2;
}