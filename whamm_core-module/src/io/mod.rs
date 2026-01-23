use whamm_core_impl::io::print::{putc as impl_putc, puts as impl_puts, putu8 as impl_putu8, puti8 as impl_puti8, putu16 as impl_putu16, puti16 as impl_puti16, putu32 as impl_putu32, puti32 as impl_puti32, putu64 as impl_putu64, puti64 as impl_puti64, putf32 as impl_putf32, putf64 as impl_putf64, putbool as impl_putbool};

#[unsafe(no_mangle)]
pub fn putc(c: u8) {
    impl_putc(c)
}

#[unsafe(no_mangle)]
pub unsafe fn puts(start: i32, len: i32) {
    unsafe { impl_puts(start, len) }
}

#[unsafe(no_mangle)]
pub fn putu8(i: u8) {
    impl_putu8(i)
}

#[unsafe(no_mangle)]
pub fn puti8(i: i8) {
    impl_puti8(i)
}

#[unsafe(no_mangle)]
pub fn putu16(i: u16) {
    impl_putu16(i)
}

#[unsafe(no_mangle)]
pub fn puti16(i: i16) {
    impl_puti16(i)
}

#[unsafe(no_mangle)]
pub fn putu32(i: u32) {
    impl_putu32(i)
}

#[unsafe(no_mangle)]
pub fn puti32(i: i32) {
    impl_puti32(i)
}

#[unsafe(no_mangle)]
pub fn putu64(i: u64) {
    impl_putu64(i)
}

#[unsafe(no_mangle)]
pub fn puti64(i: i64) {
    impl_puti64(i)
}

#[unsafe(no_mangle)]
pub fn putf32(f: f32) {
    impl_putf32(f)
}

#[unsafe(no_mangle)]
pub fn putf64(f: f64) {
    impl_putf64(f)
}

#[unsafe(no_mangle)]
pub fn putbool(i: i32) {
    impl_putbool(i)
}
