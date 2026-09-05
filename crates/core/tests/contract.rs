//! Bug-fix contract gates (PR #3671 review findings). These guard invariants
//! that the v2 parity corpus does not exercise directly:
//!   - P1 Design.Color: the full CSS named-color palette resolves (not just the
//!     legacy 7-entry subset).
//!   - P3 Temporal.Duration: the 2-byte UTF-8 micro spellings are accepted by
//!     the scanner and never panic.
//!
//! The P2 slug round-trip gate moved with its scalar to the extension that
//! owns it.

use superscalar::{scalar_for, Registry, ScalarId};

#[test]
fn color_resolves_full_css_named_palette() {
    let color = scalar_for(ScalarId::DESIGN_COLOR);
    let cases = [
        ("aqua", "#00FFFFFF"),
        ("navy", "#000080FF"),
        ("orange", "#FFA500FF"),
        ("coral", "#FF7F50FF"),
        ("gray", "#808080FF"),
        ("grey", "#808080FF"),
        ("cornflowerblue", "#6495EDFF"),
        ("darkolivegreen", "#556B2FFF"),
        ("rebeccapurple", "#663399FF"),
        ("transparent", "#00000000"),
        // Case-insensitive: normalize lowercases before lookup.
        ("REBECCAPURPLE", "#663399FF"),
        ("Navy", "#000080FF"),
    ];
    for (input, expected) in cases {
        let got = color
            .parse(Registry::builtin(), input)
            .unwrap_or_else(|e| panic!("color {input:?} should parse: {e}"));
        assert_eq!(got, expected, "color {input:?}");
    }
}

#[test]
fn duration_accepts_micro_sign_spellings() {
    let duration = scalar_for(ScalarId::TEMPORAL_DURATION);
    // Micro sign U+00B5 and Greek small letter mu U+03BC are both valid units
    // (Go's time.ParseDuration accepts them); canonical output uses ASCII "us".
    for input in ["500us", "500\u{00b5}s", "500\u{03bc}s"] {
        let got = duration
            .parse(Registry::builtin(), input)
            .unwrap_or_else(|e| panic!("duration {input:?} should parse: {e}"));
        assert_eq!(got, "500us", "duration {input:?}");
    }
    // Combined units with a micro component round-trip too: 1ms + 500us =
    // 1_500_000ns, which canonicalizes to "1500us" (sub-millisecond remainder
    // means the microsecond spelling wins in format_duration_nanos).
    assert_eq!(
        duration
            .parse(Registry::builtin(), "1ms500\u{00b5}s")
            .expect("micro combo parses"),
        "1500us"
    );
}

#[test]
fn duration_does_not_panic_on_truncated_micro_bytes() {
    // A raw 0xC2 byte (the lead byte of U+00B5) cannot appear inside a &str, but
    // exercise inputs that previously risked mid-codepoint slicing to confirm
    // the char-based scanner is panic-free by contract.
    let duration = scalar_for(ScalarId::TEMPORAL_DURATION);
    for input in ["\u{00b5}s", "5\u{00b5}", "\u{03bc}", "5\u{03bc}s10ns"] {
        // Either Ok or Err is acceptable; the only failure is a panic.
        let _ = duration.parse(Registry::builtin(), input);
        let _ = duration.validate(Registry::builtin(), input);
    }
}
