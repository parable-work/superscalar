package superscalar

import (
	"database/sql/driver"
	"encoding/json"
	"fmt"
	"math/big"

	uuid "github.com/google/uuid"
)

// UUID is a UUID v4 that marshals to compact base62 for clients and stores as standard hyphenated UUID in the DB.
const base62Alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"

var maxUUIDValue = func() *big.Int {
	max := new(big.Int).Lsh(big.NewInt(1), 128)
	return max.Sub(max, big.NewInt(1))
}()

type UUID uuid.UUID

func NewUUID() UUID {
	return UUID(uuid.New())
}

func FromUUID(u uuid.UUID) UUID {
	return UUID(u)
}

// ParseUUID accepts either a 36-char standard UUID or a 1-22 char base62 string, disambiguating by length.
func ParseUUID(s string) (UUID, error) {
	if len(s) == 36 {
		stdUUID, err := uuid.Parse(s)
		if err == nil {
			return FromUUID(stdUUID), nil
		}
	}

	if len(s) >= 1 && len(s) <= 22 {
		u, err := decodeBase62(s)
		if err == nil {
			return u, nil
		}
		return UUID{}, fmt.Errorf("invalid base62 UUID format: %s, %w", s, err)
	}

	stdUUID, err := uuid.Parse(s)
	if err != nil {
		return UUID{}, fmt.Errorf("invalid UUID format: %s, %w", s, err)
	}

	return FromUUID(stdUUID), nil
}

func (u UUID) ToUUID() uuid.UUID {
	return uuid.UUID(u)
}

func (u UUID) String() string {
	return encodeBase62(u)
}

func (u UUID) MarshalJSON() ([]byte, error) {
	return json.Marshal(u.String())
}

func (u *UUID) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	parsed, err := ParseUUID(s)
	if err != nil {
		return err
	}

	*u = parsed
	return nil
}

func (u UUID) IsZero() bool {
	return u.ToUUID() == uuid.Nil
}

func (u UUID) Value() (driver.Value, error) {
	return u.ToUUID().String(), nil
}

func (u *UUID) Scan(src interface{}) error {
	if src == nil {
		*u = UUID{}
		return nil
	}

	switch v := src.(type) {
	case []byte:
		// 16 bytes is PostgreSQL binary format; anything else is text.
		if len(v) == 16 {
			var stdUUID uuid.UUID
			copy(stdUUID[:], v)
			*u = FromUUID(stdUUID)
			return nil
		}
		stdUUID, err := uuid.Parse(string(v))
		if err != nil {
			return fmt.Errorf("failed to parse UUID from bytes: %w", err)
		}
		*u = FromUUID(stdUUID)
		return nil
	case string:
		stdUUID, err := uuid.Parse(v)
		if err != nil {
			return fmt.Errorf("failed to parse UUID from string: %w", err)
		}
		*u = FromUUID(stdUUID)
		return nil
	default:
		return fmt.Errorf("unsupported type for UUID: %T", src)
	}
}

func encodeBase62(u UUID) string {
	stdUUID := u.ToUUID()
	bytes := stdUUID[:]

	num := new(big.Int).SetBytes(bytes)

	if num.Sign() == 0 {
		return "0"
	}

	var result []byte
	base := big.NewInt(62)
	zero := big.NewInt(0)

	for num.Cmp(zero) > 0 {
		mod := new(big.Int)
		num.DivMod(num, base, mod)
		result = append(result, base62Alphabet[mod.Int64()])
	}

	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}

	return string(result)
}

func decodeBase62(s string) (UUID, error) {
	num := new(big.Int)
	base := big.NewInt(62)

	for _, char := range s {
		idx := -1
		for i, c := range base62Alphabet {
			if c == char {
				idx = i
				break
			}
		}

		if idx == -1 {
			return UUID{}, fmt.Errorf("invalid base62 character: %c", char)
		}

		num.Mul(num, base)
		num.Add(num, big.NewInt(int64(idx)))
	}

	if num.Cmp(maxUUIDValue) > 0 {
		return UUID{}, fmt.Errorf("base62 value exceeds UUID max value")
	}

	bytes := num.Bytes()

	if len(bytes) < 16 {
		padded := make([]byte, 16)
		copy(padded[16-len(bytes):], bytes)
		bytes = padded
	}

	if len(bytes) != 16 {
		return UUID{}, fmt.Errorf("invalid UUID bytes length: %d", len(bytes))
	}

	var u uuid.UUID
	copy(u[:], bytes)

	return FromUUID(u), nil
}

func (u UUID) Validate() (bool, []ValidationError) {
	return true, nil
}

func (u UUID) ValidateRequired() (bool, []ValidationError) {
	if u.IsZero() {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return true, nil
}
