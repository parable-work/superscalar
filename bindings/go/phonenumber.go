package superscalar

import (
	"encoding/json"
	"fmt"
	"strings"
)

// Best-effort: the Rust core normalize errors on unparseable input, so the fallback returns the trimmed input to preserve this package's never-reject contract. Use ValidatePhoneNumber to enforce rejection.
func NormalizePhoneNumber(input string) string {
	trimmed := strings.TrimSpace(input)
	normalized, err := callScalarNormalize(scalarIDContactPhoneNumber, input)
	if err != nil {
		return trimmed
	}
	return normalized
}

func (p *ContactPhoneNumber) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	normalized := NormalizePhoneNumber(s)
	if errs := ValidatePhoneNumber(normalized); len(errs) > 0 {
		return fmt.Errorf("invalid phone number: %s", errs[0].Message)
	}
	*p = ContactPhoneNumber(normalized)
	return nil
}

func ValidatePhoneNumber(input string) []ValidationError {
	if err := callScalarValidate(scalarIDContactPhoneNumber, input); err != nil {
		return []ValidationError{{
			Validator: "custom",
			Message:   err.Error(),
		}}
	}
	return nil
}
