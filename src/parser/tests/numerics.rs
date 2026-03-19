#![allow(clippy::type_complexity)]
use crate::common::error::WhammError;
use crate::parser::tests::setup_logger;
use crate::parser::types::{Expr, NumFmt, NumLit, Rule, Value, WhammParser};
use crate::parser::whamm_parser::{handle_float, handle_int};
use log::error;
use pest::iterators::Pair;
use pest::Parser;

// ==== Integer tests ====
//
// We test our own parser logic, not Rust stdlib `from_str_radix`:
//   - prefix stripping (0x/0X → hex, 0b/0B → binary)
//   - underscore removal
//   - decimal sign preservation (negative values stored as signed i128)
//   - hex/binary tokens stored as unsigned magnitudes (no sign extension)
//   - u128 fallback for values > i64::MAX (e.g. 0xFFFFFFFFFFFFFFFF)

#[test]
pub fn test_int_decimal() {
    setup_logger();
    // Basic values
    parse_int(0, "0");
    parse_int(1, "1");
    parse_int(-1, "-1");

    // Boundary values
    parse_int(i32::MAX as i128, "2147483647");
    parse_int(i32::MIN as i128, "-2147483648");
    parse_int(i64::MAX as i128, "9223372036854775807");
    parse_int(i64::MIN as i128, "-9223372036854775808");

    // Underscore separators are stripped
    parse_int(1_000_000, "1_000_000");
    parse_int(-2_147_483_648, "-2_147_483_648");
    parse_int(3_842_309_267_894_817_852, "3_842_309_267_894_817_852");
}

#[test]
pub fn test_int_hex() {
    setup_logger();
    // Basic values — hex tokens are unsigned magnitudes (no sign extension)
    parse_int(0, "0x0");
    parse_int(0, "0x000");
    parse_int(0xFF, "0xff");
    parse_int(0xFF, "0xFF"); // uppercase digits
    parse_int(0xFF, "0XFF"); // uppercase prefix
    parse_int(1, "0x0000000000000001");

    // 32-bit patterns: stored as unsigned magnitude, NOT sign-extended
    parse_int(0xFFFF_FFFF, "0xffffffff"); // 4294967295, not -1
    parse_int(0x8000_0000, "0x80000000"); // 2147483648, not i32::MIN

    // 64-bit values that fit in i64 (positive)
    parse_int(0x7FFF_FFFF_FFFF_FFFF, "0x7fffffffffffffff");

    // u128 fallback: values > i64::MAX parsed via u128 and cast to i128 bit pattern
    parse_int(0xFFFF_FFFF_FFFF_FFFFu64 as i128, "0xffffffffffffffff");
    parse_int(0x8000_0000_0000_0000u64 as i128, "0x8000000000000000");

    // Mixed case digits
    parse_int(0xDEAD_BEEF, "0xDeAdBeEf");
}

#[test]
pub fn test_int_binary() {
    setup_logger();
    // Basic values
    parse_int(0, "0b0");
    parse_int(0, "0b000");
    parse_int(1, "0b1");
    parse_int(0b1010, "0b1010");
    parse_int(0b101, "0B101"); // uppercase prefix

    // Leading zeros do not affect value
    parse_int(3, "0b0011");
    parse_int(3, "0b0000000011");

    // 32-bit all-ones: unsigned magnitude, NOT -1
    parse_int(0xFFFF_FFFF, "0b11111111111111111111111111111111");

    // u128 fallback: 64-bit all-ones
    parse_int(
        0xFFFF_FFFF_FFFF_FFFFu64 as i128,
        "0b1111111111111111111111111111111111111111111111111111111111111111",
    );
}

#[test]
pub fn test_int_format_field() {
    setup_logger();
    // Verify the `fmt` field on the produced NumericLiteral
    parse_int_fmt(42, NumFmt::Dec, "42");
    parse_int_fmt(0xFF, NumFmt::Hex, "0xff");
    parse_int_fmt(0b1010, NumFmt::Bin, "0b1010");
    parse_int_fmt(0xFF, NumFmt::Hex, "0XFF"); // uppercase prefix still → Hex
    parse_int_fmt(0b101, NumFmt::Bin, "0B101"); // uppercase prefix still → Bin
}

#[test]
#[rustfmt::skip]
pub fn test_f32s() {
    setup_logger();
    let t = test_float;

    t(NumLit::f32(9.0),             "9.0");
    t(NumLit::f32(9.),              "9.");
    t(NumLit::f32(0.0),             "0.0");
    t(NumLit::f32(0.),              "0.");
    t(NumLit::f32(8.1),             "8.1");
    t(NumLit::f32(7.22),            "7.22");
    t(NumLit::f32(6.333),           "6.333");
    t(NumLit::f32(5.4444),          "5.4444");
    t(NumLit::f32(4.55555),         "4.55555");
    t(NumLit::f32(3.666666),        "3.666666");
    t(NumLit::f32(2.7777777),       "2.7777777");
    t(NumLit::f32(1.888_888_8),      "1.88888888");
    t(NumLit::f32(-1.888_888_8),     "-1.88888888");
    t(NumLit::f32(0.999999999),     "0.999999999");
    t(NumLit::f32(-0.999999999),    "-0.999999999");

    t(NumLit::f32(0.25),      "0.25");
    t(NumLit::f32(0.1),       "0.1");
    t(NumLit::f32(0.2),       "0.2");
    t(NumLit::f32(0.3),       "0.3");
    t(NumLit::f32(0.4),       "0.4");
    t(NumLit::f32(0.5),       "0.5");
    t(NumLit::f32(0.6),       "0.6");
    t(NumLit::f32(0.7),       "0.7");
    t(NumLit::f32(0.8),       "0.8");
    t(NumLit::f32(0.9),       "0.9");
    t(NumLit::f32(1.2),       "1.2");
    t(NumLit::f32(1.3),       "1.3");
    t(NumLit::f32(1.4),       "1.4");
    t(NumLit::f32(1.5),       "1.5");
    t(NumLit::f32(1.6),       "1.6");
    t(NumLit::f32(1.7),       "1.7");
    t(NumLit::f32(1.8),       "1.8");
    t(NumLit::f32(1.9),       "1.9");
    t(NumLit::f32(0.77),      "0.77");
    t(NumLit::f32(-0.888),    "-0.888");
    t(NumLit::f32(0.055),     "0.055");
    t(NumLit::f32(-0.003),    "-0.003");
    t(NumLit::f32(0.000256),  "0.000256");

    t(NumLit::f32(-0.2),  "-0.2");
    t(NumLit::f32(-0.3),  "-0.3");
    t(NumLit::f32(-0.7),  "-0.7");
    t(NumLit::f32(1.1),   "1.1");
    t(NumLit::f32(-1.2),  "-1.2");
    t(NumLit::f32(1.3),   "1.3");
    t(NumLit::f32(1.4),   "1.4");
    t(NumLit::f32(1.44),  "1.44");
    t(NumLit::f32(2.33),  "2.33");
    t(NumLit::f32(4.11),  "4.11");
    t(NumLit::f32(5.55),  "5.55");
}

#[test]
#[rustfmt::skip]
pub fn test_exp() {
    let t = test_float;

    t(NumLit::f32(9e0),    "9e0");
    t(NumLit::f32(8e+0),   "8e+0");
    t(NumLit::f32(7e-0),   "7e-0");
    t(NumLit::f32(6E1),    "6E1");
    t(NumLit::f32(5E+1),   "5E+1");
    t(NumLit::f32(4E-1),   "4E-1");
    t(NumLit::f32(-9e10),  "-9e10");
    t(NumLit::f32(-8e+20), "-8e+20");
    t(NumLit::f32(-7e-30), "-7e-30");
    t(NumLit::f32(-4E-61), "-4E-61");

    t(NumLit::f32(9e5),     "9e5");
    t(NumLit::f32(8e+3),    "8e+3");
    t(NumLit::f32(7e-2),    "7e-2");

    t(NumLit::f32(1.0e1),    "1.0e1");
    t(NumLit::f32(-3.0e22),  "-3.0e22");

    t(NumLit::f32(1.0e-1),                "1.0e-1");
    t(NumLit::f32(3.0e-22),               "3.0e-22");
    t(NumLit::f32(5.0e-333),              "5.0e-333");
    t(NumLit::f32(7.0e-4444),             "7.0e-4444");
    t(NumLit::f32(9.0e-55555),            "9.0e-55555");
    t(NumLit::f32(22.0e-666666),          "22.0e-666666");
    t(NumLit::f32(444.0e-7777777),        "444.0e-7777777");
    t(NumLit::f32(6666.0e-88888888),      "6666.0e-88888888");
    t(NumLit::f32(88888.0e-999999999),    "88888.0e-999999999");
    t(NumLit::f32(88_888.0e-999999999),    "88_888.0e-999999999");

    // ==== f64 ====

    t(NumLit::f64(-6E41),   "-6E41");
    t(NumLit::f64(-5E+51),  "-5E+51");

    // ==== infinity/nan ====

    t(NumLit::f32(f32::INFINITY),             "5.0e333");
    t(NumLit::f32(f32::NEG_INFINITY),         "-7.0e4444");
    t(NumLit::f32(f32::INFINITY),             "9.0e55555");
    t(NumLit::f32(f32::NEG_INFINITY),         "-22.0e666666");
    t(NumLit::f32(f32::INFINITY),             "444.0e7777777");
    t(NumLit::f32(f32::NEG_INFINITY),         "-6666.0e88888888");
    t(NumLit::f32(f32::INFINITY),             "88_888.0e999999999");

    t(NumLit::f32(f32::INFINITY),       "inf");
    t(NumLit::f32(f32::INFINITY),       "infinity");
    t(NumLit::f32(f32::NEG_INFINITY),   "-inf");
    t(NumLit::f32(f32::NEG_INFINITY),   "-infinity");

    // ==== INVALID ====
    let f = fail_parse_float;

    f("--1");
}

// ==== Utilities ====

fn test_float(exp: NumLit, float: &str) {
    parse_float(exp, float);
}

fn parse_int(exp_raw: i128, token: &str) {
    match call_parser(Rule::INT, token, &handle_int) {
        Ok(Expr::Primitive {
            val: Value::NumericLiteral { raw, .. },
            ..
        }) => {
            assert_eq!(raw, exp_raw, "token: {token}");
        }
        unexp => handle_unexp_fail(unexp, token),
    }
}

fn parse_int_fmt(exp_raw: i128, exp_fmt: NumFmt, token: &str) {
    match call_parser(Rule::INT, token, &handle_int) {
        Ok(Expr::Primitive {
            val: Value::NumericLiteral { raw, fmt, .. },
            ..
        }) => {
            assert_eq!(raw, exp_raw, "token: {token}");
            assert_eq!(fmt, exp_fmt, "token: {token}");
        }
        unexp => handle_unexp_fail(unexp, token),
    }
}

fn parse_float(exp: NumLit, token: &str) {
    match call_parser(Rule::FLOAT, token, &handle_float) {
        Ok(Expr::Primitive {
            val: Value::Number { val, .. },
            ..
        }) => {
            assert_eq!(exp, val);
        }
        unexp => handle_unexp_fail(unexp, token),
    }
}

fn fail_parse_float(token: &str) {
    if let Ok(Expr::Primitive {
        val: Value::Number { val, .. },
        ..
    }) = fail_parser(Rule::FLOAT, token, &handle_float)
    {
        handle_unexp_pass(val, token);
    }
}

fn handle_unexp_fail(res: Result<Expr, Vec<WhammError>>, token: &str) {
    match res {
        Ok(expr) => {
            error!("Number did not parse correctly: {token} -> {:?}", expr);
            panic!();
        }
        Err(errs) => {
            error!("Parsing the number caused errors: {token}");
            for e in errs.iter() {
                error!("{:?}", e.ty)
            }
            panic!();
        }
    }
}

fn handle_unexp_pass(expr: NumLit, token: &str) {
    error!(
        "Number passed the parse, but it shouldn't have: {token} -> {:?}",
        expr
    );
    panic!();
}

fn call_parser(
    parse_rule: Rule,
    token: &str,
    handler: &dyn Fn(Pair<Rule>) -> Result<Expr, Vec<WhammError>>,
) -> Result<Expr, Vec<WhammError>> {
    let parse_res = WhammParser::parse(parse_rule, token);
    match parse_res {
        Ok(mut pairs) => handler(pairs.next().unwrap()),
        Err(e) => {
            error!("Parsing the number caused errors: {token}\n{:?}", e);
            panic!();
        }
    }
}

fn fail_parser(
    parse_rule: Rule,
    token: &str,
    handler: &dyn Fn(Pair<Rule>) -> Result<Expr, Vec<WhammError>>,
) -> Result<Expr, Vec<WhammError>> {
    let parse_res = WhammParser::parse(parse_rule, token);
    match parse_res {
        Ok(mut pairs) => handler(pairs.next().unwrap()),
        Err(_) => Err(vec![]),
    }
}
