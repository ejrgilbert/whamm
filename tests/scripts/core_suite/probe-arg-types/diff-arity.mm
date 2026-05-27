report var count_one: i32;
report var count_two: i32;

wasm:opcode:call(arg0: i32):before /
    target_fn_name == "one_arg"
/ {
    var a: i32 = arg0;
    count_one = count_one + 1;
}

wasm:opcode:call(arg0: i32, arg1: i32):before /
    target_fn_name == "two_args"
/ {
    var a: i32 = arg0;
    var b: i32 = arg1;
    count_two = count_two + 1;
}
