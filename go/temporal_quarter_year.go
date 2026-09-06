package superscalar

import (
	"encoding/json"
	"fmt"
	"strings"
)

// A 2-digit year is always interpreted as 20XX.
func normalizeTemporalQuarterYear(input string) (string, error) {
	if strings.TrimSpace(input) == "" {
		return "", fmt.Errorf("quarter-year must not be empty")
	}
	return callScalarNormalize(scalarIDTemporalQuarterYear, input)
}

// The generator-emitted ParseTemporalQuarterYear is a plain string conversion; use this when normalization is required.
func NormalizeAndParseTemporalQuarterYear(input string) (TemporalQuarterYear, error) {
	normalized, err := normalizeTemporalQuarterYear(input)
	if err != nil {
		return TemporalQuarterYear(""), err
	}
	return TemporalQuarterYear(normalized), nil
}

// UnmarshalJSON normalizes the input so payloads become canonical before downstream validation runs.
func (v *TemporalQuarterYear) UnmarshalJSON(data []byte) error {
	var raw string
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	normalized, err := normalizeTemporalQuarterYear(raw)
	if err != nil {
		return fmt.Errorf("failed to parse Temporal.QuarterYear: %w", err)
	}
	*v = TemporalQuarterYear(normalized)
	return nil
}
