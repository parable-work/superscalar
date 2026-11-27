//! File.MediaType: an IANA media type (a MIME type), `type/subtype` with
//! optional parameters: `image/png`, `text/csv; charset=utf-8`.
//!
//! The type and the subtype are each an RFC 6838 restricted name: 1 to 127
//! characters, an ASCII letter or digit first, then letters, digits and
//! `!#$&-^_.+`. A parameter is `;name=value` (RFC 9110): the name a token,
//! the value a token or a quoted string. The grammar is checked, not
//! membership in the IANA registry: a pinned registry would refuse the types
//! registered after it, and `application/x-…` and vendor types are in daily
//! use.
//!
//! Canonical form follows the WHATWG MIME Sniffing serialization: the type,
//! the subtype and each parameter name in lowercase, no whitespace, and each
//! parameter as `;name=value` in the order given, its value quoted only when
//! it is empty or not a token. So `Text/HTML; Charset="UTF-8"` is
//! `text/html;charset=UTF-8`. A parameter's value keeps its case, since some
//! are case-sensitive (a multipart boundary).
//!
//! Refused: a media range (`image/*`, `*/*`), a type with no subtype, a
//! parameter with no `=`, an empty parameter (`text/plain;`), a parameter
//! named twice, and anything over 255 characters.

use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

/// The longest input read, in bytes, after trimming; the canonical form is
/// never longer than its input.
const MAX_LENGTH: usize = 255;

/// The longest type or subtype (RFC 6838, section 4.2).
const MAX_NAME_LENGTH: usize = 127;

pub struct FileMediaType;

fn parse_error(message: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Parse, message)
}

/// HTTP optional whitespace, and the line ends a pasted header value carries.
fn is_whitespace(symbol: char) -> bool {
    matches!(symbol, ' ' | '\t' | '\r' | '\n')
}

/// RFC 6838 restricted-name: the shape of a type and of a subtype.
fn is_restricted_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    let first_ok = bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric());
    first_ok
        && name.len() <= MAX_NAME_LENGTH
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || b"!#$&-^_.+".contains(&byte))
}

/// RFC 9110 token: a parameter's name, and its value when not quoted.
fn is_token(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte))
}

/// Whether a quoted string may hold `symbol`.
fn is_quotable(symbol: char) -> bool {
    symbol == '\t' || !symbol.is_control()
}

/// Reads the quoted string at the front of `text` (which starts with `"`):
/// its value with the escapes resolved, and what follows the closing quote.
fn quoted(text: &str) -> Option<(String, &str)> {
    let mut value = String::new();
    let mut symbols = text.char_indices().skip(1);
    while let Some((index, symbol)) = symbols.next() {
        match symbol {
            '"' => return Some((value, &text[index + 1..])),
            // A quoted string holds a tab, a space and the visible and
            // non-ASCII characters, escaped or not; no other control
            // character.
            '\\' => value.push(
                symbols
                    .next()
                    .map(|(_, escaped)| escaped)
                    .filter(|&escaped| is_quotable(escaped))?,
            ),
            symbol if is_quotable(symbol) => value.push(symbol),
            _ => return None,
        }
    }
    None
}

fn canonical(input: &str) -> Result<String, ScalarError> {
    let trimmed = input.trim_matches(is_whitespace);
    if trimmed.is_empty() {
        return Err(ScalarError::new(ErrorKind::Empty, "media type is empty"));
    }
    if trimmed.len() > MAX_LENGTH {
        return Err(ScalarError::new(
            ErrorKind::Length,
            format!("media type is longer than {MAX_LENGTH} characters"),
        ));
    }

    let (essence, mut rest) = match trimmed.find(';') {
        Some(index) => (&trimmed[..index], &trimmed[index..]),
        None => (trimmed, ""),
    };
    let essence = essence.trim_end_matches(is_whitespace);
    let (kind, subtype) = essence
        .split_once('/')
        .filter(|(kind, subtype)| is_restricted_name(kind) && is_restricted_name(subtype))
        .ok_or_else(|| parse_error(format!("not a media type (type/subtype): {essence}")))?;
    let mut out = format!(
        "{}/{}",
        kind.to_ascii_lowercase(),
        subtype.to_ascii_lowercase()
    );

    let mut names: Vec<String> = Vec::new();
    while !rest.is_empty() {
        // `rest` starts at a `;`.
        let parameter = rest[1..].trim_start_matches(is_whitespace);
        let (name, after) = parameter
            .split_once('=')
            .filter(|(name, _)| is_token(name))
            .ok_or_else(|| {
                parse_error(format!("a media type parameter is name=value: {trimmed}"))
            })?;
        let name = name.to_ascii_lowercase();
        if names.contains(&name) {
            return Err(parse_error(format!(
                "media type parameter {name} is given twice"
            )));
        }
        let (value, after) = if after.starts_with('"') {
            quoted(after).ok_or_else(|| {
                parse_error(format!(
                    "media type parameter {name} has a bad quoted value"
                ))
            })?
        } else {
            let end = after.find(';').unwrap_or(after.len());
            let value = after[..end].trim_end_matches(is_whitespace);
            if !is_token(value) {
                return Err(parse_error(format!(
                    "media type parameter {name} has a bad value"
                )));
            }
            (value.to_string(), &after[end..])
        };
        let after = after.trim_start_matches(is_whitespace);
        if !after.is_empty() && !after.starts_with(';') {
            return Err(parse_error(format!(
                "media type parameter {name} has text after its value"
            )));
        }

        out.push(';');
        out.push_str(&name);
        out.push('=');
        if is_token(&value) {
            out.push_str(&value);
        } else {
            out.push('"');
            for symbol in value.chars() {
                if matches!(symbol, '"' | '\\') {
                    out.push('\\');
                }
                out.push(symbol);
            }
            out.push('"');
        }
        names.push(name);
        rest = after;
    }
    Ok(out)
}

impl Scalar for FileMediaType {
    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        canonical(input)
    }

    /// The canonical form of a valid value; any other value as written,
    /// because normalize does not enforce shape.
    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(canonical(input).unwrap_or_else(|_| input.to_string()))
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        canonical(input).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::FileMediaType;
    use crate::error::ErrorKind;
    use crate::registry::{Registry, Scalar};

    fn kind(input: &str) -> Option<ErrorKind> {
        FileMediaType
            .validate(Registry::builtin(), input)
            .err()
            .map(|error| error.kind)
    }

    #[test]
    fn every_accepted_form_parses_to_the_canonical_form() {
        let registry = Registry::builtin();
        let cases = [
            ("image/png", "image/png"),
            ("IMAGE/PNG", "image/png"),
            ("  text/plain\r\n", "text/plain"),
            ("application/json", "application/json"),
            ("application/vnd.api+json", "application/vnd.api+json"),
            (
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ),
            (
                "application/x-www-form-urlencoded",
                "application/x-www-form-urlencoded",
            ),
            ("image/svg+xml", "image/svg+xml"),
            ("audio/3gpp2", "audio/3gpp2"),
            ("text/csv; charset=utf-8", "text/csv;charset=utf-8"),
            ("text/csv;charset=utf-8", "text/csv;charset=utf-8"),
            ("Text/HTML; Charset=\"UTF-8\"", "text/html;charset=UTF-8"),
            (
                "text/plain ; charset=utf-8 ; format=flowed",
                "text/plain;charset=utf-8;format=flowed",
            ),
            (
                "multipart/form-data; boundary=\"a b\"",
                "multipart/form-data;boundary=\"a b\"",
            ),
            ("text/plain; note=\"\"", "text/plain;note=\"\""),
            (
                "text/plain; note=\"say \\\"hi\\\"\"",
                "text/plain;note=\"say \\\"hi\\\"\"",
            ),
            ("text/plain; note=\"a\\b\"", "text/plain;note=ab"),
            (
                "text/plain; note=\"a;b\"; x=1",
                "text/plain;note=\"a;b\";x=1",
            ),
        ];
        for (input, canonical) in cases {
            assert_eq!(
                FileMediaType.parse(registry, input).as_deref(),
                Ok(canonical),
                "parse({input:?})"
            );
            assert_eq!(
                FileMediaType.normalize(registry, input).as_deref(),
                Ok(canonical),
                "normalize({input:?})"
            );
            assert_eq!(
                FileMediaType.parse(registry, canonical).as_deref(),
                Ok(canonical),
                "parse({canonical:?})"
            );
            assert!(canonical.len() <= input.len(), "{input:?} grew");
        }
    }

    #[test]
    fn rejects_by_kind() {
        assert_eq!(kind(""), Some(ErrorKind::Empty));
        assert_eq!(kind(" \t"), Some(ErrorKind::Empty));
        assert_eq!(
            kind(&format!("text/plain;note={}", "a".repeat(250))),
            Some(ErrorKind::Length)
        );
        let long_name = "a".repeat(128);
        for input in [
            "image",
            "image/",
            "/png",
            "image/*",
            "*/*",
            "image/png/extra",
            "image /png",
            "image/ png",
            "image/png jpeg",
            "-image/png",
            "image/.png",
            "imagé/png",
            "image/png;",
            "image/png;;charset=utf-8",
            "image/png; charset",
            "image/png; =utf-8",
            "image/png; charset=",
            "image/png; charset=utf 8",
            "image/png; charset=\"utf-8",
            "image/png; charset=\"utf-8\"x",
            "image/png; char set=utf-8",
            "image/png; charset=utf-8; Charset=latin1",
            "image/png, image/jpeg",
            &format!("{long_name}/png"),
            &format!("image/{long_name}"),
        ] {
            assert_eq!(kind(input), Some(ErrorKind::Parse), "{input}");
        }
    }

    #[test]
    fn normalize_returns_a_refused_value_as_written() {
        let registry = Registry::builtin();
        assert_eq!(
            FileMediaType.normalize(registry, " Image/* ").as_deref(),
            Ok(" Image/* ")
        );
    }
}
