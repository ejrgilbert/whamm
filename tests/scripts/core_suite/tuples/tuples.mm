wasm:opcode:call(arg0: i32):before {
    var t0: (i32, i32, i32) = (0, 1, 2);
    var t1: (i32, i32, i32);
    t1 = (0, 1, 2);

    var a: (bool, i32);
    a = (true, 78);

    report var t2: (i32, i32, i32) = (0, 1, 2);
    report var t3: (i32, i32, i32);
    t3 = (0, 1, 2);


    report var t4: (u8, i32, i64) = (0, 1, -2);
    report var t5: (i8, str) = (-10, "wassup");
    report var t6: (str, f32);
    t6 = ("BOOOOOOOOOOOOOOYAH", 1000.18);

    // Access values
    report var t7: (i32, i32, i32);
    t7 = t1;

    report var t8: (u8, i32, i64);
    t8 = (t0.0 as u8, t1.1, t3.2 as i64);

    report var t9: (i8, str);
    t9 = (t5.0 + 5, t5.1);

    report var t10: (str, f32);
    t10 = (t5.1, t6.1);

    report var t11: i32;
    t11 = t7.2 + (t8.0 as i32) + (t9.0 as i32) + (t10.1 as i32);

    report var t12: bool;
    t12 = a.0;

    // nested tuples
    report var b: (i32, (i32, i32));
    b = (1, (12, 13));

    report var c: i32;
    c = b.1.0;  // == 12

    // set tuple
    t0.0 = 24;
    a.0 = false;
    t5.1 = "other str";

    // use bound vars in the tuple
    report var bound0: (u32, u32) = (fid, pc);
    report var bound1: (i32, u32);
    bound1 = (arg0, pc);
}
