#[no_mangle]
pub fn foo(a: i32) -> i32 {
    a - 3
}

#[no_mangle]
pub fn bar(a: i32) -> i32 {
    let b = foo(a);
    for i in 0..b {
        println!("hello: {i}")
    }

    b
}

#[no_mangle]
fn main() {
    let b = bar(15);
    println!("b = {b}");
}
