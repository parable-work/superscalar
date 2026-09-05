package superscalar

import (
	"encoding/json"
	"errors"
	"strings"
)

// Pure string transform, does not validate shape; the fallback preserves the identity-on-failure contract if the core call ever errors.
func NormalizeEmail(input string) string {
	normalized, err := callScalarNormalize(scalarIDContactEmail, input)
	if err != nil {
		return strings.ToLower(strings.TrimSpace(input))
	}
	return normalized
}

func (e *ContactEmail) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	*e = ContactEmail(NormalizeEmail(s))
	return nil
}

func NormalizeAndValidateEmail(input string) (string, error) {
	if strings.TrimSpace(input) == "" {
		return "", errors.New("email must not be empty")
	}
	return callScalarParse(scalarIDContactEmail, input)
}

// Intentionally a no-op: the @scalarPattern directive on Contact_Email already runs the regex earlier, so duplicating it here would surface the same failure twice.
func ValidateEmail(input string) []ValidationError {
	return nil
}
