package superscalar

import (
	"bytes"
	"database/sql/driver"
	"encoding/json"
	"fmt"
)

// MarshalJSON preserves raw JSON payloads for the JSON scalar.
func (v GenericJSON) MarshalJSON() ([]byte, error) {
	if len(v) == 0 {
		return []byte("null"), nil
	}

	return []byte(v), nil
}

// UnmarshalJSON accepts any valid JSON token (object, array, string, etc.)
// and keeps it compact, as Scan does. A value then holds the same bytes
// whether it came from a column or from a JSON document, such as a jsonb
// column that holds it, so a digest or a compare of its bytes cannot depend on
// the path that read it.
func (v *GenericJSON) UnmarshalJSON(data []byte) error {
	if v == nil {
		return nil
	}

	var compact bytes.Buffer
	if err := json.Compact(&compact, data); err != nil {
		return err
	}

	*v = GenericJSON(append([]byte(nil), compact.Bytes()...))
	return nil
}

// Value sends the exact JSON token to SQL drivers. A zero value represents SQL
// NULL; the four-byte token "null" represents an explicit JSON null root.
func (v GenericJSON) Value() (driver.Value, error) {
	if len(v) == 0 {
		return nil, nil
	}
	if !json.Valid(v) {
		return nil, fmt.Errorf("Generic.JSON contains invalid JSON")
	}
	return append([]byte(nil), v...), nil
}

// Scan preserves the distinction between SQL NULL and an explicit JSON null
// token returned by PostgreSQL JSON/JSONB codecs. Native pgx scans into **T
// intentionally collapse JSON null into nil; a nullable field that must keep
// the distinction scans into raw bytes first and then calls this method.
func (v *GenericJSON) Scan(src any) error {
	if v == nil {
		return fmt.Errorf("cannot scan Generic.JSON into a nil receiver")
	}
	if src == nil {
		*v = nil
		return nil
	}

	var raw []byte
	switch value := src.(type) {
	case []byte:
		raw = value
	case json.RawMessage:
		raw = value
	case string:
		raw = []byte(value)
	default:
		return fmt.Errorf("cannot scan Generic.JSON from %T", src)
	}
	if !json.Valid(raw) {
		return fmt.Errorf("cannot scan invalid JSON into Generic.JSON")
	}
	var compact bytes.Buffer
	if err := json.Compact(&compact, raw); err != nil {
		return fmt.Errorf("cannot compact Generic.JSON: %w", err)
	}
	*v = GenericJSON(append([]byte(nil), compact.Bytes()...))
	return nil
}
