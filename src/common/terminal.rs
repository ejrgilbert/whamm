use std::io::Write;
use termcolor::{Buffer, Color, ColorSpec, WriteColor};

// ===========================
// = Terminal Printing Logic =
// ===========================

const WRITE_ERR: &str = "Uh oh, something went wrong while printing to terminal";

pub fn color(s: String, buffer: &mut Buffer, bold: bool, italics: bool, c: Color) {
    buffer
        .set_color(
            ColorSpec::new()
                .set_bold(bold)
                .set_italic(italics)
                .set_fg(Some(c)),
        )
        .expect(WRITE_ERR);
    write!(buffer, "{}", s.as_str()).expect(WRITE_ERR);
}

pub fn black(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Black)
}
pub fn blue(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Blue)
}
pub fn cyan(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Cyan)
}
pub fn green(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Green)
}
pub fn magenta(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Magenta)
}
pub fn magenta_italics(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, true, Color::Magenta)
}
pub fn red(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Red)
}
pub fn white(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Rgb(193, 193, 193))
}
pub fn grey(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::White)
}
pub fn grey_italics(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, true, Color::White)
}
pub fn yellow(bold: bool, s: String, buffer: &mut Buffer) {
    color(s, buffer, bold, false, Color::Yellow)
}
pub fn long_line(buffer: &mut Buffer) {
    let s = "--".repeat(50);
    color(s, buffer, false, false, Color::White);
}
