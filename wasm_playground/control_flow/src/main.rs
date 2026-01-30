use std::iter::zip;

#[unsafe(no_mangle)]
pub fn calc(a: i32, b: i32) -> i32 {
    if a > 5 { a + b } else { a * b }
}

#[unsafe(no_mangle)]
fn print_x(opt: Opt, x: u32) {
    for _ in 0..x {
        println!("{} world!", opt_str(&opt))
    }
}

#[unsafe(no_mangle)]
fn opt_str(opt: &Opt) -> String {
    match opt {
        Opt::Hi => "hi".to_string(),
        Opt::Hello => "hello".to_string(),
        Opt::Sup => "'sup".to_string(),
    }
}

#[unsafe(no_mangle)]
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

    println!("\n==== PRINT ====");
    print_x(Opt::Hi, times as u32);
    print_x(Opt::Hello, times as u32);
    print_x(Opt::Sup, times as u32);
}

enum Opt {
    Hi,
    Hello,
    Sup,
}
