package superscalar

import (
	"bytes"
	"database/sql/driver"
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

// Returns (zero, false) on failure rather than an error; inputs without timezone information are interpreted as UTC.
func NormalizeDateTime(input string) (TemporalDateTime, bool) {
	if strings.TrimSpace(input) == "" {
		return TemporalDateTime{}, false
	}
	canonical, err := callScalarNormalize(scalarIDTemporalDateTime, input)
	if err != nil {
		return TemporalDateTime{}, false
	}
	t, err := time.Parse(time.RFC3339, canonical)
	if err != nil {
		return TemporalDateTime{}, false
	}
	return TemporalDateTime(t), true
}

// Zero-value times serialize as JSON null, matching nullable DateTime GraphQL fields.
// RFC3339Nano (not RFC3339) so sub-seconds survive the JSON round-trip: Go's
// RFC3339Nano keeps the fraction, trims trailing zeros, and omits it entirely
// when zero -- byte-identical to the scalar-lib core canonical (Temporal.DateTime
// in core/src/scalars/datetime.rs). Pinned by TestMarshalJSONMatchesCoreCanonical.
func (v TemporalDateTime) MarshalJSON() ([]byte, error) {
	if time.Time(v).IsZero() {
		return []byte("null"), nil
	}
	return json.Marshal(time.Time(v).Format(time.RFC3339Nano))
}

func (v *TemporalDateTime) UnmarshalJSON(data []byte) error {
	trimmed := bytes.TrimSpace(data)
	if bytes.Equal(trimmed, []byte("null")) {
		*v = TemporalDateTime(time.Time{})
		return nil
	}

	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}
	if strings.TrimSpace(s) == "" {
		*v = TemporalDateTime(time.Time{})
		return nil
	}

	dt, ok := NormalizeDateTime(s)
	if !ok {
		return fmt.Errorf("failed to parse DateTime: %q", s)
	}

	*v = dt
	return nil
}

func ParseDateTime(s string) (TemporalDateTime, error) {
	dt, ok := NormalizeDateTime(s)
	if !ok {
		return TemporalDateTime{}, fmt.Errorf("failed to parse DateTime: %q", s)
	}
	return dt, nil
}

func (v TemporalDateTime) Value() (driver.Value, error) {
	return time.Time(v), nil
}

func (v *TemporalDateTime) Scan(src interface{}) error {
	if src == nil {
		*v = TemporalDateTime(time.Time{})
		return nil
	}

	switch t := src.(type) {
	case time.Time:
		*v = TemporalDateTime(t)
		return nil
	case []byte:
		dt, ok := NormalizeDateTime(string(t))
		if !ok {
			return fmt.Errorf("failed to parse DateTime from bytes: %q", string(t))
		}
		*v = dt
		return nil
	case string:
		dt, ok := NormalizeDateTime(t)
		if !ok {
			return fmt.Errorf("failed to parse DateTime from string: %q", t)
		}
		*v = dt
		return nil
	default:
		return fmt.Errorf("unsupported type for DateTime: %T", src)
	}
}
