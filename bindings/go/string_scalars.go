package superscalar

import (
	"database/sql/driver"
	"fmt"
)

// Slug needs its own Scan so text/citext/varchar columns do not route through UUID.Scan.
func (v *IdentitySlug) Scan(src interface{}) error {
	if src == nil {
		*v = ""
		return nil
	}
	switch t := src.(type) {
	case []byte:
		*v = IdentitySlug(string(t))
		return nil
	case string:
		*v = IdentitySlug(t)
		return nil
	default:
		return fmt.Errorf("unsupported scan type for Slug: %T", src)
	}
}

func (v IdentitySlug) Value() (driver.Value, error) {
	return string(v), nil
}

// Name needs its own Scan so text/varchar columns do not route through UUID.Scan.
func (v *IdentityName) Scan(src interface{}) error {
	if src == nil {
		*v = ""
		return nil
	}
	switch t := src.(type) {
	case []byte:
		*v = IdentityName(string(t))
		return nil
	case string:
		*v = IdentityName(t)
		return nil
	default:
		return fmt.Errorf("unsupported scan type for Name: %T", src)
	}
}

func (v IdentityName) Value() (driver.Value, error) {
	return string(v), nil
}
