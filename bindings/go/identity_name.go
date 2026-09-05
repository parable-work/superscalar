package superscalar

import "strings"

// IdentityNameMaxLength mirrors the @scalarMaxLength directive on the
// Identity_Name scalar (stored as VARCHAR(80)). Lives here so the limit has a
// single canonical home in scalar-lib rather than being mirrored as a literal
// across the services that derive a display name server-side. Mirrors the
// generated IdentityName.Validate() bound in generated.go.
const IdentityNameMaxLength = 80

// ClampIdentityName fits a candidate display name to the Identity_Name scalar's
// max length. Rune-safe: it truncates on a rune boundary so a multibyte name is
// never split mid-codepoint -- a byte slice could emit invalid UTF-8 and reject
// the VARCHAR(80) write. Trailing spaces left by truncation are trimmed. The DB
// column counts characters, so a result of <=IdentityNameMaxLength runes always
// fits.
//
// This is a coercion for SERVER-DERIVED names (email local-parts, OAuth/SSO
// claims) that never pass through the input-boundary IdentityName.Validate();
// user-supplied Name input fields are still validated by the generated scalar.
func ClampIdentityName(s string) string {
	runes := []rune(s)
	if len(runes) <= IdentityNameMaxLength {
		return s
	}
	return strings.TrimRight(string(runes[:IdentityNameMaxLength]), " ")
}
