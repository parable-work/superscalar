package superscalar

import "encoding/json"

// MarshalJSON preserves raw JSON payloads for the JSON scalar.
func (v GenericJSON) MarshalJSON() ([]byte, error) {
	if len(v) == 0 {
		return []byte("null"), nil
	}

	return []byte(v), nil
}

// UnmarshalJSON accepts any valid JSON token (object, array, string, etc.).
func (v *GenericJSON) UnmarshalJSON(data []byte) error {
	if v == nil {
		return nil
	}

	var raw json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}

	*v = GenericJSON(append([]byte(nil), raw...))
	return nil
}
