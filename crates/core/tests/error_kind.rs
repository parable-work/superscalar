//! Pins the stable string contract of `ErrorKind`: these tokens are the parity
//! vectors' `validator` values, so a drift here silently invalidates parity.

use superscalar::{ErrorKind, ScalarError};

/// Expected stable token per variant. The exhaustive `match` (no wildcard) makes
/// adding an `ErrorKind` variant fail to compile until this contract is updated.
fn expected_token(kind: ErrorKind) -> &'static str {
    match kind {
        ErrorKind::Parse => "parse",
        ErrorKind::Pattern => "pattern",
        ErrorKind::Length => "length",
        ErrorKind::Range => "range",
        ErrorKind::Enum => "enum",
        ErrorKind::Custom => "custom",
        ErrorKind::Empty => "empty",
    }
}

#[test]
fn as_str_matches_stable_tokens() {
    let all = [
        ErrorKind::Parse,
        ErrorKind::Pattern,
        ErrorKind::Length,
        ErrorKind::Range,
        ErrorKind::Enum,
        ErrorKind::Custom,
        ErrorKind::Empty,
    ];
    for kind in all {
        assert_eq!(kind.as_str(), expected_token(kind), "{kind:?}");
    }
}

#[test]
fn display_is_category_colon_message() {
    let err = ScalarError::new(ErrorKind::Parse, "not an integer: x");
    assert_eq!(err.to_string(), "parse: not an integer: x");
}
