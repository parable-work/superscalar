package superscalar

import (
	"errors"
	"fmt"
)

// ErrUnknownScalar is returned by [Validate], [Parse], and [Normalize] when the
// canonical scalar name is not present in [VALID_SCALARS]. Callers that want
// to branch on "is this scalar handled by the core" should prefer [KnownScalar]
// over inspecting this error, but it is exported so the unknown case can be told
// apart from a genuine validation/parse failure with errors.Is.
var ErrUnknownScalar = errors.New("scalars: unknown canonical scalar")

// KnownScalar reports whether the given canonical scalar name (e.g. "Contact.Email")
// is in the core's scalar table. It is the guard consumers use
// before delegating value validation/normalization to the core: an unknown name means
// the consumer should fall back to its own generic handling rather than calling the
// dispatch functions, which would return [ErrUnknownScalar].
func KnownScalar(canonical string) bool {
	_, ok := validScalarsMap[canonical]
	return ok
}

// Validate runs the core validator for the named canonical scalar over value. It
// returns nil when value is a valid instance of the scalar, a validation error
// (the core's message, e.g. "pattern: invalid email format") when it is not, and
// an error wrapping [ErrUnknownScalar] when canonical is not a known scalar.
//
// This is the generic, name-keyed entry point onto the same FFI surface the
// generated per-scalar Validate<Name> wrappers use, so a consumer holding only a
// canonical string (such as the schema-runtime validator) can delegate without a
// hand-maintained name->func mirror.
func Validate(canonical string, value string) error {
	if !KnownScalar(canonical) {
		return fmt.Errorf("%w: %q", ErrUnknownScalar, canonical)
	}
	return callScalarValidate(canonical, value)
}

// Parse validates value and returns its canonical form for the named scalar. It
// returns an error wrapping [ErrUnknownScalar] when canonical is not a known
// scalar, and the core's parse error otherwise.
func Parse(canonical string, value string) (string, error) {
	if !KnownScalar(canonical) {
		return "", fmt.Errorf("%w: %q", ErrUnknownScalar, canonical)
	}
	return callScalarParse(canonical, value)
}

// Normalize returns the canonical normalized form of value for the named scalar
// without validating beyond what normalization requires. It returns an error
// wrapping [ErrUnknownScalar] when canonical is not a known scalar, and the core's
// normalize error otherwise.
func Normalize(canonical string, value string) (string, error) {
	if !KnownScalar(canonical) {
		return "", fmt.Errorf("%w: %q", ErrUnknownScalar, canonical)
	}
	return callScalarNormalize(canonical, value)
}

// CoerceJSON validates one JSON value against the named scalar and returns the
// scalar's canonical JSON encoding. It is the name-keyed form of the core
// coercion used when a schema stores a scalar-selected value as Generic.JSON.
func CoerceJSON(canonical string, value string) (string, error) {
	if !KnownScalar(canonical) {
		return "", fmt.Errorf("%w: %q", ErrUnknownScalar, canonical)
	}
	if canonical == "Generic.JSON" {
		return callScalarParse(canonical, value)
	}
	return callScalarCoerceLenient(canonical, value)
}
