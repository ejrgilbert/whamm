use alpha;

wasm:opcode:drop:before {
    // allocate some space in memory to store the library's string
    var MAX: i32 = 100;
    var ptr: i32 = alpha.mem_alloc(MAX);

    // write a string to memory, return the length of the string written
    // (should also get flushed at the end of execution)
    report var l: i32;
    l = alpha.write_alphabet(ptr, MAX);

    // read the string from the library's memory
    // (should also get flushed at the end of execution)
    report var s: str;
    s = read_str(memid(alpha), ptr, l as u32);

    report var length: u32;
    length = s.len();                       // call happens at runtime

    report var start_exp_true: bool;
    start_exp_true = s.starts_with("abc");
    report var start_exp_false: bool;
    start_exp_false = s.starts_with("wxyz");

    report var end_exp_true: bool;
    end_exp_true = s.ends_with("wxyz\n");
    report var end_exp_false: bool;
    end_exp_false = s.ends_with("abc");

//     report var contains_exp_true0: bool = s.contains("def");
//     report var contains_exp_true1: bool = s.contains("abc");
//     report var contains_exp_true2: bool = s.contains("xyz");
//     report var contains_exp_false: bool = s.contains("asdf ");

    // free the memory we've just used
    alpha.mem_free(ptr);
}
