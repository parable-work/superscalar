package superscalar

import (
	"encoding/json"
	"fmt"
	"strings"
)

func normalizeTemporalMonth(input string) (string, error) {
	if strings.TrimSpace(input) == "" {
		return "", fmt.Errorf("month must not be empty")
	}
	return callScalarNormalize(scalarIDTemporalMonth, input)
}

// The generator-emitted ParseTemporalMonth is a plain string conversion; use this when normalization is required.
func NormalizeAndParseTemporalMonth(input string) (TemporalMonth, error) {
	normalized, err := normalizeTemporalMonth(input)
	if err != nil {
		return TemporalMonth(""), err
	}
	return TemporalMonth(normalized), nil
}

// UnmarshalJSON normalizes the input so payloads become canonical before downstream validation runs.
func (v *TemporalMonth) UnmarshalJSON(data []byte) error {
	var raw string
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	normalized, err := normalizeTemporalMonth(raw)
	if err != nil {
		return fmt.Errorf("failed to parse Temporal.Month: %w", err)
	}
	*v = TemporalMonth(normalized)
	return nil
}
