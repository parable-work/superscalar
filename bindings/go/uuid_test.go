package superscalar

import (
	"encoding/json"
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewUUID(t *testing.T) {
	u := NewUUID()
	assert.False(t, u.IsZero(), "NewUUID should not return zero UUID")
}

func TestFromUUID(t *testing.T) {
	stdUUID := uuid.New()
	u := FromUUID(stdUUID)
	assert.Equal(t, stdUUID, u.ToUUID())
}

func TestParseUUID(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		wantErr bool
	}{
		{
			name:    "standard UUID format",
			input:   "123e4567-e89b-12d3-a456-426614174000",
			wantErr: false,
		},
		{
			name:    "base62 format",
			input:   "1q3ftxYKI0brlcFFxCGBu",
			wantErr: false,
		},
		{
			name:    "invalid format",
			input:   "invalid-uuid",
			wantErr: true,
		},
		{
			name:    "empty string",
			input:   "",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := ParseUUID(tt.input)
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
			}
		})
	}
}

func TestUUID_String(t *testing.T) {
	u := NewUUID()
	s := u.String()

	assert.True(t, len(s) >= 20 && len(s) <= 22, "String should return base62 format")
}

func TestUUID_MarshalJSON(t *testing.T) {
	u := NewUUID()
	data, err := json.Marshal(u)
	require.NoError(t, err)

	assert.True(t, data[0] == '"' && data[len(data)-1] == '"', "JSON should be a quoted string")
}

func TestUUID_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		wantErr bool
	}{
		{
			name:    "standard UUID format",
			json:    `"123e4567-e89b-12d3-a456-426614174000"`,
			wantErr: false,
		},
		{
			name:    "base62 format",
			json:    `"1q3ftxYKI0brlcFFxCGBu"`,
			wantErr: false,
		},
		{
			name:    "invalid format",
			json:    `"invalid!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"`,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var u UUID
			err := json.Unmarshal([]byte(tt.json), &u)
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.False(t, u.IsZero())
			}
		})
	}
}

func TestUUID_RoundTrip(t *testing.T) {
	original := NewUUID()

	data, err := json.Marshal(original)
	require.NoError(t, err)

	var parsed UUID
	err = json.Unmarshal(data, &parsed)
	require.NoError(t, err)

	assert.Equal(t, original.ToUUID(), parsed.ToUUID())
}

func TestUUID_IsZero(t *testing.T) {
	var zero UUID
	assert.True(t, zero.IsZero())

	nonZero := NewUUID()
	assert.False(t, nonZero.IsZero())
}

func TestUUID_Value(t *testing.T) {
	u := NewUUID()
	val, err := u.Value()
	require.NoError(t, err)

	str, ok := val.(string)
	assert.True(t, ok)
	assert.Equal(t, 36, len(str), "Database value should be standard 36-char UUID")
}

func TestUUID_Scan(t *testing.T) {
	tests := []struct {
		name    string
		input   interface{}
		wantErr bool
	}{
		{
			name:    "nil",
			input:   nil,
			wantErr: false,
		},
		{
			name:    "string",
			input:   "123e4567-e89b-12d3-a456-426614174000",
			wantErr: false,
		},
		{
			name:    "bytes text format",
			input:   []byte("123e4567-e89b-12d3-a456-426614174000"),
			wantErr: false,
		},
		{
			name:    "invalid string",
			input:   "invalid",
			wantErr: true,
		},
		{
			name:    "unsupported type",
			input:   123,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var u UUID
			err := u.Scan(tt.input)
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
			}
		})
	}
}

func TestUUID_Validate(t *testing.T) {
	u := NewUUID()
	valid, errs := u.Validate()
	assert.True(t, valid)
	assert.Empty(t, errs)
}

func TestUUID_ValidateRequired(t *testing.T) {
	t.Run("non-zero UUID", func(t *testing.T) {
		u := NewUUID()
		valid, errs := u.ValidateRequired()
		assert.True(t, valid)
		assert.Empty(t, errs)
	})

	t.Run("zero UUID", func(t *testing.T) {
		var u UUID
		valid, errs := u.ValidateRequired()
		assert.False(t, valid)
		assert.Len(t, errs, 1)
		assert.Equal(t, "required", errs[0].Validator)
	})
}

func TestBase62Encoding(t *testing.T) {
	for i := 0; i < 100; i++ {
		original := NewUUID()
		encoded := original.String()
		decoded, err := ParseUUID(encoded)
		require.NoError(t, err)
		assert.Equal(t, original.ToUUID(), decoded.ToUUID())
	}
}

func TestDecodeBase62_RangeValidation(t *testing.T) {
	t.Run("max UUID value", func(t *testing.T) {
		decoded, err := decodeBase62("7n42DGM5Tflk9n8mt7Fhc7")
		require.NoError(t, err)
		assert.Equal(t, "ffffffff-ffff-ffff-ffff-ffffffffffff", decoded.ToUUID().String())
	})

	t.Run("value above max UUID", func(t *testing.T) {
		_, err := decodeBase62("8AMTIJy4UWqRr3zMiOATlN")
		require.Error(t, err)
		assert.ErrorContains(t, err, "exceeds UUID max value")
	})
}
