package superscalar

import (
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestDateTime_Value(t *testing.T) {
	now := time.Date(2026, 4, 24, 12, 0, 0, 0, time.UTC)
	dt := TemporalDateTime(now)
	val, err := dt.Value()
	require.NoError(t, err)
	assert.Equal(t, now, val)
}

func TestDateTime_Scan_Nil(t *testing.T) {
	var dt TemporalDateTime
	require.NoError(t, dt.Scan(nil))
	assert.True(t, time.Time(dt).IsZero())
}

func TestDateTime_Scan_Time(t *testing.T) {
	now := time.Date(2026, 4, 24, 12, 0, 0, 0, time.UTC)
	var dt TemporalDateTime
	require.NoError(t, dt.Scan(now))
	assert.Equal(t, TemporalDateTime(now), dt)
}

func TestDateTime_Scan_Bytes(t *testing.T) {
	var dt TemporalDateTime
	require.NoError(t, dt.Scan([]byte("2026-04-24T12:00:00Z")))
	assert.Equal(t, 2026, time.Time(dt).Year())
}

func TestDateTime_Scan_String(t *testing.T) {
	var dt TemporalDateTime
	require.NoError(t, dt.Scan("2026-04-24T12:00:00Z"))
	assert.Equal(t, 2026, time.Time(dt).Year())
}

func TestDateTime_Scan_UnsupportedType(t *testing.T) {
	var dt TemporalDateTime
	err := dt.Scan(42)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unsupported type")
}

func TestDateTime_Scan_InvalidBytes(t *testing.T) {
	var dt TemporalDateTime
	err := dt.Scan([]byte("not-a-date"))
	assert.Error(t, err)
}

func TestDateTime_Scan_InvalidString(t *testing.T) {
	var dt TemporalDateTime
	err := dt.Scan("not-a-date")
	assert.Error(t, err)
}
