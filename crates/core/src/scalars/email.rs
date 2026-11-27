use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use email_address::{EmailAddress, Error, Options};
use unicode_normalization::UnicodeNormalization;

// The `email_address` crate decides the shape, by RFC 5321, RFC 5322 and
// RFC 6531. No regex states that grammar, so the def declares no pattern.
// A regex refused addresses that receive mail: `o'brien@example.com`,
// `user@example.xn--p1ai` and an address with a letter outside ASCII.
//
// - The domain needs a top-level domain, so `ana@gmail` is refused.
// - An address literal such as `[192.0.2.1]` is refused.
// - Display text such as `Ana <ana@example.com>` is refused. The crate
//   accepts it by default, and a stored value must be the address alone.
//
// - A local part over 64 bytes is refused.
//
// Two rules are this scalar's own, in `parse`. A quoted local part is refused.
// The crate accepts `"a@b, c"@example.com`, which holds a space, a comma and a
// second `@`. A stored address has never held those characters, because the
// pattern this parser replaced refused them.
//
// A character outside ASCII must be a letter or a digit. From 0.2.10 the crate
// takes any character outside ASCII as atext, so an emoji, a no-break space, a
// zero-width space or a bidi control would pass in the local part or inside a
// domain label. 0.2.9 refused those, and this rule keeps its accept set.
//
// The crate's domain rule is the RFC 5322 dot-atom, which is looser than a
// host name: `ana@exam_ple.com` passes.
const OPTIONS: Options = Options {
    minimum_sub_domains: 2,
    allow_domain_literal: false,
    allow_display_text: false,
    allow_long_local_parts: false,
};

// 0.2.9 compared each code point with the UTF-8 byte patterns, so it took the
// unassigned U+D7A4..=U+D7AF as atext. They stay accepted, so the accept set
// is exactly 0.2.9's.
fn is_refused_char(c: char) -> bool {
    !c.is_ascii() && !c.is_alphanumeric() && !('\u{D7A4}'..='\u{D7AF}').contains(&c)
}

const MAX_LENGTH: usize = 255;

pub struct Email;

impl Email {
    // Lowercase, then NFC, so one address has one stored form. An accented
    // letter can arrive as one code point or as two.
    fn canonical(input: &str) -> String {
        input.trim().to_lowercase().nfc().collect()
    }
}

impl Scalar for Email {
    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(Self::canonical(input))
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let normalized = Self::canonical(input);
        if normalized.is_empty() {
            return Err(ScalarError::new(
                ErrorKind::Empty,
                "email must not be empty",
            ));
        }
        if normalized.chars().count() > MAX_LENGTH {
            return Err(ScalarError::new(
                ErrorKind::Length,
                format!("email exceeds {MAX_LENGTH} characters"),
            ));
        }
        // Only a quoted local part can hold a `"`.
        if normalized.contains('"') {
            return Err(ScalarError::new(
                ErrorKind::Parse,
                "invalid email address: a quoted name is not accepted",
            ));
        }
        if normalized.chars().any(is_refused_char) {
            return Err(ScalarError::new(
                ErrorKind::Parse,
                format!("invalid email address: {}", Error::InvalidCharacter),
            ));
        }
        // The message carries the crate's reason and never the input, which is
        // a person's address.
        if let Err(reason) = EmailAddress::parse_with_options(&normalized, OPTIONS) {
            return Err(ScalarError::new(
                ErrorKind::Parse,
                format!("invalid email address: {reason}"),
            ));
        }
        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    #[test]
    fn refuses_a_character_outside_ascii_that_is_not_a_letter_or_digit() {
        let registry = Registry::builtin();
        for input in [
            "ana\u{1F600}@example.com",
            "ana\u{00A0}lopez@example.com",
            "ana\u{200B}@example.com",
            "ana\u{202E}@example.com",
            "ana@exa\u{1F600}mple.com",
        ] {
            let err = Email.parse(registry, input).unwrap_err();
            assert_eq!(err.kind, ErrorKind::Parse, "{input:?}");
            assert_eq!(err.message, "invalid email address: Invalid character.");
        }
    }

    // The accept set 0.2.9 had: U+D7A4..=U+D7AF passed, its neighbours
    // U+D7A3 (a Hangul syllable) and U+D7B0 (a Hangul jamo) are letters.
    #[test]
    fn keeps_the_unassigned_code_points_0_2_9_accepted() {
        let registry = Registry::builtin();
        for c in '\u{D7A3}'..='\u{D7B0}' {
            let input = format!("a{c}@example.com");
            assert!(Email.parse(registry, &input).is_ok(), "{input:?}");
        }
    }
}
