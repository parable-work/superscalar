use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use email_address::{EmailAddress, Options};
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
// One rule is this scalar's own, in `parse`: a quoted local part is refused.
// The crate accepts `"a@b, c"@example.com`, which holds a space, a comma and a
// second `@`. A stored address has never held those characters, because the
// pattern this parser replaced refused them.
//
// The crate's domain rule is the RFC 5322 dot-atom, which is looser than a
// host name: `ana@exam_ple.com` passes.
const OPTIONS: Options = Options {
    minimum_sub_domains: 2,
    allow_domain_literal: false,
    allow_display_text: false,
};

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
