package superscalar

import (
	"database/sql/driver"
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

// Value stays Go's time.Duration so the parsed, marshaled, and normalized forms all agree on that one representation.
func ParseDuration(s string) (TemporalDuration, error) {
	if err := callScalarValidate(scalarIDTemporalDuration, s); err != nil {
		return TemporalDuration(0), fmt.Errorf("failed to parse Duration: %w", err)
	}
	d, err := time.ParseDuration(s)
	if err != nil {
		return TemporalDuration(0), fmt.Errorf("failed to parse Duration: %w", err)
	}
	return TemporalDuration(d), nil
}

// The microsecond unit is spelled ASCII "us" (not Go's Unicode micro sign) because that is the cross-language canonical form the core emits.
func NormalizeDuration(input string) string {
	trimmed := strings.TrimSpace(input)
	if err := callScalarValidate(scalarIDTemporalDuration, trimmed); err != nil {
		return trimmed
	}
	d, err := time.ParseDuration(trimmed)
	if err != nil {
		return trimmed
	}
	return strings.ReplaceAll(d.String(), "µs", "us")
}

func ValidateDuration(input string) []ValidationError {
	if err := callScalarValidate(scalarIDTemporalDuration, input); err != nil {
		return []ValidationError{{
			Validator: "custom",
			Message:   fmt.Sprintf("must be a valid duration: %v", err),
		}}
	}
	return nil
}

func (v TemporalDuration) MarshalJSON() ([]byte, error) {
	return json.Marshal(time.Duration(v).String())
}

func (v *TemporalDuration) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	d, err := time.ParseDuration(s)
	if err != nil {
		return fmt.Errorf("failed to parse Duration: %w", err)
	}

	*v = TemporalDuration(d)
	return nil
}

func (v TemporalDuration) Value() (driver.Value, error) {
	// Store as PostgreSQL INTERVAL string.
	return time.Duration(v).String(), nil
}

func (v *TemporalDuration) Scan(src interface{}) error {
	if src == nil {
		*v = TemporalDuration(0)
		return nil
	}

	switch t := src.(type) {
	case []byte:
		d, err := time.ParseDuration(string(t))
		if err != nil {
			return fmt.Errorf("failed to parse Duration from bytes: %w", err)
		}
		*v = TemporalDuration(d)
		return nil
	case string:
		d, err := time.ParseDuration(t)
		if err != nil {
			return fmt.Errorf("failed to parse Duration from string: %w", err)
		}
		*v = TemporalDuration(d)
		return nil
	case int64:
		// Interpret as nanoseconds
		*v = TemporalDuration(time.Duration(t))
		return nil
	default:
		return fmt.Errorf("unsupported type for Duration: %T", src)
	}
}
