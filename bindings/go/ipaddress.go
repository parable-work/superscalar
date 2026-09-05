package superscalar

import (
	"database/sql/driver"
	"fmt"
)

func (v NetworkIpAddress) Value() (driver.Value, error) {
	return string(v), nil
}

func (v *NetworkIpAddress) Scan(src interface{}) error {
	if src == nil {
		*v = NetworkIpAddress("")
		return nil
	}

	switch t := src.(type) {
	case []byte:
		*v = NetworkIpAddress(string(t))
		return nil
	case string:
		*v = NetworkIpAddress(t)
		return nil
	default:
		return fmt.Errorf("unsupported type for IpAddress: %T", src)
	}
}
