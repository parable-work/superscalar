package superscalar

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// GeoLocation marshals as the scalar's wire form, {"lat":...,"lon":...}, and
// its String() is the core's canonical text for the same value.
func TestGeoLocation_MarshalsAsTheCanonicalObject(t *testing.T) {
	for _, tt := range []struct {
		value GeoLocation
		want  string
	}{
		{GeoLocation{Lat: 37.7749, Lon: -122.4194}, `{"lat":37.7749,"lon":-122.4194}`},
		{GeoLocation{Lat: 90, Lon: -180}, `{"lat":90,"lon":-180}`},
		{GeoLocation{Lat: 1e-7, Lon: 0.000001}, `{"lat":1e-7,"lon":0.000001}`},
	} {
		encoded, err := json.Marshal(tt.value)
		require.NoError(t, err)
		assert.Equal(t, tt.want, string(encoded))
		assert.Equal(t, tt.want, tt.value.String())

		canonical, err := ParseGeoLocation(string(encoded))
		require.NoError(t, err)
		assert.Equal(t, tt.want, canonical, "core canonical form of %s", encoded)

		var decoded GeoLocation
		require.NoError(t, json.Unmarshal([]byte(canonical), &decoded))
		assert.Equal(t, tt.value, decoded)
	}
}

func TestGeoLocation_Validate(t *testing.T) {
	valid, errs := GeoLocation{Lat: 37.7749, Lon: -122.4194}.Validate()
	assert.True(t, valid)
	assert.Empty(t, errs)

	valid, errs = GeoLocation{Lat: 91, Lon: 0}.Validate()
	require.False(t, valid)
	require.Len(t, errs, 1)
	assert.Equal(t, "range", errs[0].Validator)

	require.NotNil(t, ValidatorFor("Geo.Location"))
	assert.Error(t, ValidatorFor("Geo.Location")("37.7749,-122.4194"))
}

// The zero value is the point {"lat":0,"lon":0}, a real location, so a
// required GeoLocation never reads as missing (as with numeric zero).
func TestGeoLocation_ZeroIsAValue(t *testing.T) {
	valid, errs := GeoLocation{}.ValidateRequired()
	assert.True(t, valid)
	assert.Empty(t, errs)
}
