report i32 blah;
report map<i32, i32> m0;
// wasm:opcode:call:before /
//     target_fn_name == "bar"
// /
wasm:opcode:call:before
 {
    alloc i32 count;
    report i32 rep_count;

    count++;
//     if(strcmp((0, 1), "lsdjflaksjdf")) {
//         report i32 c;
//     }
// //     report i32 a;
// //     report map<i32, i32> m;
// //     a = 5;
//     m0[1] = 2;
// //     m[1] = 2;
// //     m[2] = 3;
//     blah = 3;
//     arg0 = 5;

    rep_count = count;
}
// wasm:opcode:call:after /
//     target_fn_name == "foo"
// /
// wasm:opcode:call:after
//  {
//     report i32 b;
//     b = 3;
// }