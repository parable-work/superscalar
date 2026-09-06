package superscalar

import (
	"encoding/json"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDateTime_MarshalJSON_ZeroValue(t *testing.T) {
	dt := TemporalDateTime{}
	data, err := json.Marshal(dt)
	require.NoError(t, err)
	assert.Equal(t, "null", string(data))
}

func TestDateTime_MarshalJSON_NonZero(t *testing.T) {
	dt := TemporalDateTime(time.Date(2026, 1, 15, 12, 0, 0, 0, time.UTC))
	data, err := json.Marshal(dt)
	require.NoError(t, err)
	assert.Equal(t, `"2026-01-15T12:00:00Z"`, string(data))
}

func TestDateTime_MarshalJSON_RFC3339(t *testing.T) {
	dt := TemporalDateTime(time.Date(2026, 6, 15, 14, 30, 45, 0, time.UTC))
	data, err := json.Marshal(dt)
	require.NoError(t, err)

	var s string
	require.NoError(t, json.Unmarshal(data, &s))
	_, err = time.Parse(time.RFC3339, s)
	assert.NoError(t, err, "output should be valid RFC3339")
}

// Pins the core canonical: MarshalJSON must emit the exact byte form the
// superscalar core produces (sub-seconds preserved + trailing zeros trimmed,
// numeric offset preserved). RFC3339 (the legacy format) dropped sub-seconds;
// this guards against silently regressing back to it.
func TestDateTime_MarshalJSON_MatchesCoreCanonical(t *testing.T) {
	cases := []string{
		"2026-01-15T12:00:00.5Z",      // sub-second kept (RFC3339 would drop to ...00Z)
		"2026-06-15T14:30:45.123456Z", // micros kept
		"2026-01-15T12:00:00+05:00",   // numeric offset kept (not normalized to UTC)
		"2026-01-15T12:00:00Z",        // zero fraction omitted entirely
	}
	for _, in := range cases {
		t.Run(in, func(t *testing.T) {
			canonical, err := callScalarNormalize(scalarIDTemporalDateTime, in)
			require.NoError(t, err)

			dt, ok := NormalizeDateTime(in)
			require.True(t, ok)
			data, err := json.Marshal(dt)
			require.NoError(t, err)

			var got string
			require.NoError(t, json.Unmarshal(data, &got))
			assert.Equal(t, canonical, got, "MarshalJSON must equal scalar-lib core canonical")
		})
	}
}

func TestDateTime_UnmarshalJSON_Valid(t *testing.T) {
	var dt TemporalDateTime
	err := json.Unmarshal([]byte(`"2026-01-15T12:00:00Z"`), &dt)
	require.NoError(t, err)
	assert.Equal(t, 2026, time.Time(dt).Year())
	assert.Equal(t, time.January, time.Time(dt).Month())
	assert.Equal(t, 15, time.Time(dt).Day())
}

func TestDateTime_UnmarshalJSON_Invalid(t *testing.T) {
	var dt TemporalDateTime
	err := json.Unmarshal([]byte(`"not-a-date"`), &dt)
	assert.Error(t, err)
}

func TestDateTime_UnmarshalJSON_Null(t *testing.T) {
	var dt TemporalDateTime
	err := json.Unmarshal([]byte(`null`), &dt)
	require.NoError(t, err)
	assert.True(t, time.Time(dt).IsZero())
}

func TestDateTime_UnmarshalJSON_EmptyString(t *testing.T) {
	var dt TemporalDateTime
	err := json.Unmarshal([]byte(`""`), &dt)
	require.NoError(t, err)
	assert.True(t, time.Time(dt).IsZero())
}

func TestDateTime_RoundTrip(t *testing.T) {
	original := TemporalDateTime(time.Date(2026, 3, 10, 8, 15, 30, 0, time.UTC))

	data, err := json.Marshal(original)
	require.NoError(t, err)

	var restored TemporalDateTime
	require.NoError(t, json.Unmarshal(data, &restored))

	assert.True(t, time.Time(original).Equal(time.Time(restored)),
		"round-trip should preserve the value")
}

func TestDateTime_ZeroValue_InStruct_OmitEmpty(t *testing.T) {
	type wrapper struct {
		Required TemporalDateTime `json:"required"`
		Optional TemporalDateTime `json:"optional,omitempty"`
	}

	w := wrapper{
		Required: TemporalDateTime(time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC)),
	}

	data, err := json.Marshal(w)
	require.NoError(t, err)

	var m map[string]interface{}
	require.NoError(t, json.Unmarshal(data, &m))

	assert.Contains(t, m, "required")
	_, hasOptional := m["optional"]
	if hasOptional {
		assert.Nil(t, m["optional"], "zero DateTime in omitempty field should serialize as null")
	}
}

func TestNormalizeDateTime_RFC3339(t *testing.T) {
	dt, ok := NormalizeDateTime("2026-01-15T12:00:00Z")
	require.True(t, ok)
	assert.Equal(t, time.Date(2026, 1, 15, 12, 0, 0, 0, time.UTC), time.Time(dt))
}

func TestNormalizeDateTime_AlternateFormats(t *testing.T) {
	cases := []struct {
		input string
		want  time.Time
	}{
		{"2026-01-15T12:00:00", time.Date(2026, 1, 15, 12, 0, 0, 0, time.UTC)},
		{"2026-01-15 12:00:00", time.Date(2026, 1, 15, 12, 0, 0, 0, time.UTC)},
		{"2026-01-15", time.Date(2026, 1, 15, 0, 0, 0, 0, time.UTC)},
		{"2026/01/15 12:00:00", time.Date(2026, 1, 15, 12, 0, 0, 0, time.UTC)},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			dt, ok := NormalizeDateTime(tc.input)
			require.True(t, ok, "expected %q to normalize", tc.input)
			assert.True(t, time.Time(dt).Equal(tc.want),
				"got %v, want %v", time.Time(dt), tc.want)
		})
	}
}

func TestNormalizeDateTime_Rejects(t *testing.T) {
	for _, input := range []string{"", "   ", "not-a-date"} {
		_, ok := NormalizeDateTime(input)
		assert.False(t, ok, "expected %q to be rejected", input)
	}
}

func TestDateTime_NonZeroValue_InStruct(t *testing.T) {
	type wrapper struct {
		At TemporalDateTime `json:"at"`
	}

	w := wrapper{At: TemporalDateTime(time.Date(2026, 6, 1, 12, 0, 0, 0, time.UTC))}
	data, err := json.Marshal(w)
	require.NoError(t, err)

	var m map[string]interface{}
	require.NoError(t, json.Unmarshal(data, &m))

	assert.Equal(t, "2026-06-01T12:00:00Z", m["at"])
}
