use std::iter::zip;

#[no_mangle]
pub fn calc(a: i32, b: i32) -> i32 {
    if a > 5 {
        a + b
    } else {
        a * b
    }
}

#[no_mangle]
pub fn print_x(s: &str, x: u32) {
    for _ in 0..x {
        println!("{s}")
    }
}

#[no_mangle]
fn main() {
    println!("==== CALC ====");
    let times = 10;
    let aspan = (0..times).rev();
    let bspan = 0..times;
    for (a, b) in zip(aspan, bspan) {
        // a: times to 0
        // b: 0 to times
        println!("calc({a}, {b}) -> {}", calc(a, b));
    }

    println!("\n==== PRINT_X ====");
    print_x("hello world!", times as u32)

}