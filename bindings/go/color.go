package superscalar

import (
	"encoding/json"
	"fmt"
	"regexp"
	"strings"
)

// Best-effort: the core normalize errors on unparseable input, so the fallback re-establishes the "return the trimmed input" contract this package guarantees.
func NormalizeColor(input string) string {
	trimmed := strings.TrimSpace(strings.ToLower(input))
	if trimmed == "" {
		return trimmed
	}
	normalized, err := callScalarNormalize(scalarIDDesignColor, input)
	if err != nil {
		return trimmed
	}
	return normalized
}

func (c *DesignColor) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	normalized := NormalizeColor(s)
	if errs := ValidateColor(normalized); len(errs) > 0 {
		return fmt.Errorf("invalid color: %s", errs[0].Message)
	}
	*c = DesignColor(normalized)
	return nil
}

// A Go-local assertion on NormalizeColor's output, not a color validity check (the core accepts un-normalized forms like "red" or "#F00").
var canonicalColorPattern = regexp.MustCompile(`^#[0-9A-F]{8}$`)

// Called after normalization: a value that did not normalize fails here.
func ValidateColor(input string) []ValidationError {
	if !canonicalColorPattern.MatchString(input) {
		return []ValidationError{{
			Validator: "custom",
			Message:   "color must be in #RRGGBBAA format after normalization",
		}}
	}

	return nil
}
