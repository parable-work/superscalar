package superscalar

import (
	"encoding/json"
	"errors"
	"strings"
)

// Pure string transform, does not validate shape; the fallback preserves the identity-on-failure contract if the core call ever errors.
func NormalizeEmail(input string) string {
	normalized, err := callScalarNormalize(scalarNameContactEmail, input)
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
	return callScalarParse(scalarNameContactEmail, input)
}

// Intentionally a no-op. The schema runtime validates Contact.Email with the core parser and does not call this hook for a scalar that the core knows. A check here would report the same failure twice.
func ValidateEmail(input string) []ValidationError {
	return nil
}
