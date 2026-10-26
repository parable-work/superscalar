package superscalar

import (
	"database/sql"
	"database/sql/driver"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var (
	_ sql.Scanner   = (*GenericDecimal)(nil)
	_ driver.Valuer = GenericDecimal("")
)

func TestGenericDecimal_ScanCanonicalizes(t *testing.T) {
	for _, tc := range []struct {
		name string
		src  any
		want GenericDecimal
	}{
		{"numeric(5,2) text", "1.50", "1.5"},
		{"bytes", []byte("-0.0425000"), "-0.0425"},
		{"zero with scale", "0.00", "0"},
		{"integer", int64(-1250), "-1250"},
		{"null", nil, ""},
		{"past float64", "9007199254740993.10", "9007199254740993.1"},
	} {
		t.Run(tc.name, func(t *testing.T) {
			var v GenericDecimal
			require.NoError(t, v.Scan(tc.src))
			assert.Equal(t, tc.want, v)
		})
	}
}

func TestGenericDecimal_ScanRefuses(t *testing.T) {
	for _, src := range []any{"NaN", "Infinity", []byte("1e3"), 1.5, true} {
		var v GenericDecimal
		assert.Error(t, v.Scan(src), "%#v", src)
	}
}

func TestGenericDecimal_Value(t *testing.T) {
	got, err := GenericDecimal("007.50").Value()
	require.NoError(t, err)
	assert.Equal(t, "7.5", got)

	got, err = GenericDecimal("").Value()
	require.NoError(t, err)
	assert.Nil(t, got, "the zero value is SQL NULL")

	_, err = GenericDecimal("1e3").Value()
	assert.Error(t, err, "Postgres would store 1000; the scalar refuses the form")
}
