package superscalar

import (
	"database/sql/driver"
	"fmt"
	"strconv"
)

// Value sends the canonical text, which a NUMERIC column reads exactly. The
// zero value is SQL NULL, since NUMERIC has no empty text. A value the scalar
// refuses is an error here rather than a write: Postgres would store NaN or
// 1e3, which Generic.Decimal does not accept.
func (v GenericDecimal) Value() (driver.Value, error) {
	if v == "" {
		return nil, nil
	}
	canonical, err := ParseGenericDecimal(string(v))
	if err != nil {
		return nil, fmt.Errorf("Generic.Decimal: %w", err)
	}
	return canonical, nil
}

// Scan reads a NUMERIC column into the canonical form. Postgres writes a
// NUMERIC(p,s) value with s fraction digits ("1.50"); the scan gives "1.5", the
// value every other source of it parses to. SQL NULL scans as the zero value.
// A float64 is refused, as it may already have lost digits: scan NUMERIC as
// text.
func (v *GenericDecimal) Scan(src any) error {
	switch t := src.(type) {
	case nil:
		*v = ""
		return nil
	case int64:
		*v = GenericDecimal(strconv.FormatInt(t, 10))
		return nil
	case []byte:
		return v.scanText(string(t))
	case string:
		return v.scanText(t)
	default:
		return fmt.Errorf("unsupported scan type for Generic.Decimal: %T", src)
	}
}

func (v *GenericDecimal) scanText(text string) error {
	canonical, err := ParseGenericDecimal(text)
	if err != nil {
		return fmt.Errorf("cannot scan %q into Generic.Decimal: %w", text, err)
	}
	*v = GenericDecimal(canonical)
	return nil
}
