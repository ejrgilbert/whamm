#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    let res = a + b;

    return res;
}

fn main() {
    let a = 0;
    let mut b = 3;
    let c = add(a, b);

    if c > 0 {
        b = 5;
    }
}