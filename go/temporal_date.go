package superscalar

import (
	"encoding/json"
	"fmt"
	"strings"
)

// For datetime inputs the time portion is dropped; inputs without timezone are interpreted as UTC.
func normalizeTemporalDate(input string) (string, error) {
	if strings.TrimSpace(input) == "" {
		return "", fmt.Errorf("date must not be empty")
	}
	return callScalarNormalize(scalarIDTemporalDate, input)
}

// The generator-emitted ParseTemporalDate is a plain string conversion; use this when normalization is required.
func NormalizeAndParseTemporalDate(input string) (TemporalDate, error) {
	normalized, err := normalizeTemporalDate(input)
	if err != nil {
		return TemporalDate(""), err
	}
	return TemporalDate(normalized), nil
}

// UnmarshalJSON normalizes the input so payloads become canonical before downstream validation runs.
func (v *TemporalDate) UnmarshalJSON(data []byte) error {
	var raw string
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	normalized, err := normalizeTemporalDate(raw)
	if err != nil {
		return fmt.Errorf("failed to parse Temporal.Date: %w", err)
	}
	*v = TemporalDate(normalized)
	return nil
}
