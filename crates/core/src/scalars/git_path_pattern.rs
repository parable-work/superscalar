//! Canonical repository-rooted gitignore-style path-pattern validation.
//!
//! This scalar validates portable syntax only. Matching remains a consumer
//! concern because it requires a repository tree and its case-sensitive paths.

use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};

pub struct GitPathPatternScalar;

fn pattern_error(input: &str, reason: &str) -> ScalarError {
    ScalarError::new(
        ErrorKind::Pattern,
        format!("invalid Git.PathPattern {input:?}: {reason}"),
    )
}

fn validate_segment(input: &str, segment: &str) -> Result<(), ScalarError> {
    if segment == "." || segment == ".." {
        return Err(pattern_error(input, "dot path segments are not canonical"));
    }

    let chars: Vec<char> = segment.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        match chars[index] {
            '\\' => {
                index += 1;
                if index == chars.len() {
                    return Err(pattern_error(input, "escape cannot terminate a pattern"));
                }
                if chars[index] == '/' {
                    return Err(pattern_error(input, "path separators cannot be escaped"));
                }
            }
            '[' => {
                let opened_at = index;
                index += 1;
                if index < chars.len() && matches!(chars[index], '!' | '^') {
                    index += 1;
                }
                let content_start = index;
                let mut escaped = false;
                while index < chars.len() {
                    let current = chars[index];
                    if escaped {
                        escaped = false;
                    } else if current == '\\' {
                        escaped = true;
                    } else if current == ']' {
                        break;
                    }
                    index += 1;
                }
                if index == chars.len() {
                    return Err(pattern_error(input, "character class is not closed"));
                }
                if index == content_start {
                    return Err(pattern_error(input, "character class is empty"));
                }
                if opened_at > 0 && chars[opened_at - 1] == '\\' {
                    return Err(pattern_error(input, "escaped '[' cannot open a class"));
                }
            }
            '*' => {
                let start = index;
                while index + 1 < chars.len() && chars[index + 1] == '*' {
                    index += 1;
                }
                let run = index - start + 1;
                if run > 1 && (run != 2 || chars.len() != 2) {
                    return Err(pattern_error(
                        input,
                        "'**' is canonical only as a complete path segment",
                    ));
                }
            }
            c if c.is_control() => {
                return Err(pattern_error(input, "control characters are forbidden"));
            }
            _ => {}
        }
        index += 1;
    }
    Ok(())
}

impl Scalar for GitPathPatternScalar {
    fn id(&self) -> ScalarId {
        ScalarId::GIT_PATH_PATTERN
    }

    fn parse(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.validate(registry, input)?;
        Ok(input.to_string())
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        Ok(input.to_string())
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        let len = input.chars().count();
        if !(2..=1024).contains(&len) {
            return Err(ScalarError::new(
                ErrorKind::Length,
                format!("length {len} outside 2..=1024"),
            ));
        }
        if input
            .chars()
            .any(|character| matches!(character, '\0' | '\r' | '\n'))
        {
            return Err(pattern_error(input, "NUL and line breaks are forbidden"));
        }
        if input.ends_with(' ') && !input.ends_with("\\ ") {
            return Err(pattern_error(input, "trailing spaces must be escaped"));
        }

        let rooted = input.strip_prefix('!').unwrap_or(input);
        if !rooted.starts_with('/') {
            return Err(pattern_error(input, "pattern must begin '/' or '!/'"));
        }
        if rooted == "/" {
            return Err(pattern_error(
                input,
                "repository root alone is not a selector",
            ));
        }
        if rooted.contains("//") {
            return Err(pattern_error(
                input,
                "empty path segments are not canonical",
            ));
        }

        let without_root = &rooted[1..];
        let without_directory_suffix = without_root.strip_suffix('/').unwrap_or(without_root);
        if without_directory_suffix.is_empty() {
            return Err(pattern_error(input, "pattern must select a path"));
        }
        for segment in without_directory_suffix.split('/') {
            validate_segment(input, segment)?;
        }
        Ok(())
    }
}
