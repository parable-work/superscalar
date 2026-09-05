package superscalar

import (
	"encoding/json"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDuration_MarshalJSON(t *testing.T) {
	d := TemporalDuration(30 * time.Second)
	data, err := json.Marshal(d)
	require.NoError(t, err)
	assert.Equal(t, `"30s"`, string(data))
}

func TestDuration_MarshalJSON_Complex(t *testing.T) {
	d := TemporalDuration(90*time.Minute + 30*time.Second)
	data, err := json.Marshal(d)
	require.NoError(t, err)
	assert.Equal(t, `"1h30m30s"`, string(data))
}

func TestDuration_UnmarshalJSON(t *testing.T) {
	var d TemporalDuration
	require.NoError(t, json.Unmarshal([]byte(`"1h30m"`), &d))
	assert.Equal(t, TemporalDuration(90*time.Minute), d)
}

func TestDuration_UnmarshalJSON_Invalid(t *testing.T) {
	var d TemporalDuration
	err := json.Unmarshal([]byte(`"not-a-duration"`), &d)
	assert.Error(t, err)
}

func TestDuration_UnmarshalJSON_BadType(t *testing.T) {
	var d TemporalDuration
	err := json.Unmarshal([]byte(`42`), &d)
	assert.Error(t, err)
}

func TestDuration_RoundTrip(t *testing.T) {
	original := TemporalDuration(5*time.Minute + 30*time.Second)
	data, err := json.Marshal(original)
	require.NoError(t, err)

	var restored TemporalDuration
	require.NoError(t, json.Unmarshal(data, &restored))
	assert.Equal(t, original, restored)
}

func TestDuration_Value(t *testing.T) {
	d := TemporalDuration(30 * time.Second)
	val, err := d.Value()
	require.NoError(t, err)
	assert.Equal(t, "30s", val)
}

func TestDuration_Scan_Nil(t *testing.T) {
	var d TemporalDuration
	require.NoError(t, d.Scan(nil))
	assert.Equal(t, TemporalDuration(0), d)
}

func TestDuration_Scan_Bytes(t *testing.T) {
	var d TemporalDuration
	require.NoError(t, d.Scan([]byte("1h")))
	assert.Equal(t, TemporalDuration(time.Hour), d)
}

func TestDuration_Scan_String(t *testing.T) {
	var d TemporalDuration
	require.NoError(t, d.Scan("5m"))
	assert.Equal(t, TemporalDuration(5*time.Minute), d)
}

func TestDuration_Scan_Int64(t *testing.T) {
	var d TemporalDuration
	require.NoError(t, d.Scan(int64(time.Second)))
	assert.Equal(t, TemporalDuration(time.Second), d)
}

func TestDuration_Scan_UnsupportedType(t *testing.T) {
	var d TemporalDuration
	err := d.Scan(3.14)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unsupported type")
}

func TestDuration_Scan_InvalidString(t *testing.T) {
	var d TemporalDuration
	err := d.Scan("not-valid")
	assert.Error(t, err)
}

func TestNormalizeDuration(t *testing.T) {
	accepted := []struct {
		name  string
		input string
		want  string
	}{
		{"already canonical", "1h30m", "1h30m0s"},
		{"trim whitespace", "  1h30m  ", "1h30m0s"},
		{"composite reduces", "60s", "1m0s"},
		{"sub-second microseconds", "1500us", "1.5ms"},
		{"sub-second mixed", "1500ms", "1.5s"},
		{"hours minutes seconds", "1h2m3s", "1h2m3s"},
		{"zero", "0s", "0s"},
	}
	for _, tt := range accepted {
		t.Run(tt.name, func(t *testing.T) {
			got := NormalizeDuration(tt.input)
			assert.Equal(t, tt.want, got, "NormalizeDuration(%q)", tt.input)
			// Idempotence.
			assert.Equal(t, got, NormalizeDuration(got), "NormalizeDuration not idempotent for %q", tt.input)
		})
	}

	rejected := []string{"", "not-a-duration", "abc", "12"}
	for _, input := range rejected {
		t.Run("rejected/"+input, func(t *testing.T) {
			got := NormalizeDuration(input)
			errs := ValidateDuration(got)
			assert.NotEmpty(t, errs, "NormalizeDuration(%q) = %q; ValidateDuration expected to fail", input, got)
		})
	}
}

func TestValidateDuration(t *testing.T) {
	assert.Empty(t, ValidateDuration("1h30m"))
	assert.Empty(t, ValidateDuration("500ms"))
	assert.NotEmpty(t, ValidateDuration(""))
	assert.NotEmpty(t, ValidateDuration("not-a-duration"))
}
