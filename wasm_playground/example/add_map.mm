// report i32 blah;
map<i32, i32> m0;
wasm:opcode:call:before /
    target_fn_name == "bar"
/
 {
//     if(true) {
//         report i32 c;
//     };
//     report i32 a;
// //     map<i32, i32> m;
//     report map<i32, i32> m;
//     a = 5;
    m0[1] = 2;
//     m[1] = 2;
//     m[2] = 3;
//     blah = 3;
//     arg0 = 5;
}
// wasm:opcode:call:after /
//     target_fn_name == "foo"
// /
//  {
//     report i32 b;
//     b = 3;
// }