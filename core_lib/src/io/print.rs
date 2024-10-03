#[no_mangle]
pub fn putc(c: u8) {
    let str = String::from_utf8([c].to_vec()).expect("Our bytes should be valid utf8");
    print!("{str}");
}

#[no_mangle]
pub fn puti(i: i32) {
    print!("{i}");
}
