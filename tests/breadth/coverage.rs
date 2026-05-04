// Coverage check: every opcode probed by breadth.mm / breadth_wei.mm must have at
// least one static occurrence in breadth_target.wat. This catches both directions of
// drift — a new provider opcode added without a matching site in the wat, or an
// opcode dropped from the wat while a probe still references it.
//
// Implementation note: we search the .wat text directly (rather than walking the
// compiled .wasm) because opcode names in the YAML providers and probe rules are
// the canonical WAT mnemonics. A token-boundary check avoids false positives from
// substring matches like `i32.load` inside `i32.load8_s`. A handful of probe-rule
// names have no literal WAT mnemonic and need dedicated patterns:
//   * `typed_select` — surfaces as `select (result …)`.
//   * `ref.test_null` / `ref.cast_null` — the nullable-target variants of
//     `ref.test` / `ref.cast`, written in WAT as `ref.test (ref null $T)` and
//     `ref.cast (ref null $T)`.

use std::collections::BTreeSet;
use std::fs;

const BREADTH_MM: &str = "tests/breadth/breadth.mm";
const BREADTH_WEI_MM: &str = "tests/breadth/breadth_wei.mm";
const BREADTH_TARGET_WAT: &str = "tests/breadth/breadth_target.wat";

#[test]
fn breadth_target_covers_every_opcode_probe() {
    let probed = collect_probed_opcodes(&[BREADTH_MM, BREADTH_WEI_MM]);
    let wat = strip_line_comments(&fs::read_to_string(BREADTH_TARGET_WAT).unwrap());

    let mut missing: Vec<&str> = probed
        .iter()
        .filter(|op| !contains_opcode(&wat, op))
        .map(String::as_str)
        .collect();
    missing.sort();

    assert!(
        missing.is_empty(),
        "breadth_target.wat is missing static occurrences of {} opcode(s) probed by \
         breadth.mm / breadth_wei.mm:\n  - {}\n\
         add a static occurrence for each so the breadth probes have somewhere to attach.",
        missing.len(),
        missing.join("\n  - ")
    );
}

/// Pull every `<op>` out of probe rules of the form `wasm:opcode:<op>:<mode>`.
fn collect_probed_opcodes(paths: &[&str]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for path in paths {
        let src = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {}", path, e));
        for line in src.lines() {
            let line = line.trim_start();
            let Some(rest) = line.strip_prefix("wasm:opcode:") else {
                continue;
            };
            // rest looks like `<op>:<mode> { ... }` — opcode is up to the next colon.
            let Some(colon) = rest.find(':') else { continue };
            out.insert(rest[..colon].to_string());
        }
    }
    out
}

/// Drop everything from `;;` to end-of-line. Line-comments only — the wat doesn't use
/// block comments.
fn strip_line_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    for line in src.lines() {
        let visible = match line.find(";;") {
            Some(i) => &line[..i],
            None => line,
        };
        out.push_str(visible);
        out.push('\n');
    }
    out
}

fn contains_opcode(wat: &str, op: &str) -> bool {
    match op {
        "typed_select" => contains_with_suffix(wat, "select", "(result"),
        "ref.test_null" => contains_with_suffix(wat, "ref.test", "(ref null"),
        "ref.cast_null" => contains_with_suffix(wat, "ref.cast", "(ref null"),
        _ => contains_token(wat, op),
    }
}

fn contains_token(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    let needle_b = needle.as_bytes();
    let mut i = 0;
    while i + needle_b.len() <= bytes.len() {
        if &bytes[i..i + needle_b.len()] == needle_b {
            let before_ok = i == 0 || !is_token_char(bytes[i - 1]);
            let after = i + needle_b.len();
            let after_ok = after == bytes.len() || !is_token_char(bytes[after]);
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn is_token_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
}

fn contains_with_suffix(wat: &str, head: &str, suffix_first_token: &str) -> bool {
    for line in wat.lines() {
        let trimmed = line.trim_start();
        let Some(after_head) = trimmed.strip_prefix(head) else {
            continue;
        };
        // Make sure `head` ended at a token boundary (so `select` doesn't match `selected`).
        let next = after_head.as_bytes().first().copied().unwrap_or(b' ');
        if is_token_char(next) {
            continue;
        }
        if after_head.trim_start().starts_with(suffix_first_token) {
            return true;
        }
    }
    false
}
