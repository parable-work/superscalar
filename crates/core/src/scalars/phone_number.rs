use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;
use phonenumber::{parse, PhoneNumber};
use regex::Regex;

fn err(msg: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Custom, msg)
}

fn parse_international(input: &str) -> Result<PhoneNumber, ScalarError> {
    parse(None, input).map_err(|e| {
        err(format!(
            "Invalid phone number format - must include country code: {e}"
        ))
    })
}

// The shape of the input, checked before phonenumber reads it. phonenumber
// alone takes the first number out of free text, so "Phone: +1 415 555 2671"
// passes. It also reads letters as keypad digits, so "+1 800 FLOWERS" passes.
// The input is a `+` (after an optional `(`), then digits and separators, then
// an optional extension. The one other form is a `tel:` URI (RFC 3966).
static INPUT_SHAPE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        r"(?i)^(?:",
        // phonenumber reads the URI's parameters, such as `;ext=3` or
        // `;phone-context=+1`.
        r"tel:\+?[0-9().-]+(?:;\S*)?",
        r"|",
        // The separators are whitespace, `(`, `)`, `.`, `-` and the other
        // hyphen and dash characters. A `/`, a `[` or a `~` is not one.
        r"\(?\+[0-9().\s\x{2010}-\x{2015}\x{2212}-]+",
        // An extension, such as `ext. 3`, `x3` or `#3`.
        r"(?:[\s,]*(?:ext\.?|extn\.?|extension|x|#)[\s.:]*[0-9]+)?",
        r")$",
    ))
    .expect("phone input shape compiles")
});

// Copy and paste and CSV exports add these, and `str::trim` keeps them.
fn is_invisible_mark(c: char) -> bool {
    matches!(
        c,
        '\u{200B}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}

fn normalize_phone_number(input: &str) -> Result<String, ScalarError> {
    let visible: String = input.chars().filter(|&c| !is_invisible_mark(c)).collect();
    let input = visible.trim();
    if !INPUT_SHAPE.is_match(input) {
        return Err(err(
            "Invalid phone number format - must be only the number, starting with +",
        ));
    }
    let num = parse_international(input)?;
    if !num.is_valid() {
        return Err(err("Invalid phone number - number failed validation"));
    }
    if num.code().value() == 0 {
        return Err(err("Invalid phone number - missing country code"));
    }
    Ok(num.format().mode(phonenumber::Mode::E164).to_string())
}

pub struct PhoneNumberScalar;

impl Scalar for PhoneNumberScalar {
    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_phone_number(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_phone_number(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_phone_number(input).map(|_| ())
    }
}
