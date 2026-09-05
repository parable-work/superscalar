package superscalar

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// A required numeric scalar carrying 0 is a real value, not an absent field.
// Go cannot tell `{"datesWithData":0}` from `{}` once decoded, so the required
// check must not infer absence from the zero value. Regression for the
// web-admin-api report-ingestion-coverage 400s: a tap with no covered days in
// the audit window sends datesWithData=0 and every request was rejected with
// "required field".
func TestValidateRequiredAcceptsNumericZero(t *testing.T) {
	valid, errs := GenericInt64(0).ValidateRequired()

	assert.True(t, valid, "GenericInt64(0) must pass the required check")
	assert.Empty(t, errs)
}

func TestValidateRequiredAcceptsFloatZero(t *testing.T) {
	valid, errs := GenericProbability(0).ValidateRequired()

	assert.True(t, valid, "GenericProbability(0) must pass the required check")
	assert.Empty(t, errs)
}

func TestValidateRequiredStillAcceptsNonZeroNumeric(t *testing.T) {
	valid, errs := GenericInt64(7).ValidateRequired()

	assert.True(t, valid)
	assert.Empty(t, errs)
}

// String scalars keep the old behaviour on purpose: an empty string carries no
// value, so it stays a required-field failure.
func TestValidateRequiredRejectsEmptyString(t *testing.T) {
	valid, errs := ContactEmail("").ValidateRequired()

	require.False(t, valid, "empty required string must still fail")
	require.NotEmpty(t, errs)
	assert.Equal(t, "required", errs[0].Validator)
}

func TestScalarRequiredValueMissing(t *testing.T) {
	tests := []struct {
		name    string
		value   any
		missing bool
	}{
		{"int64 zero", GenericInt64(0), false},
		{"int64 non-zero", GenericInt64(3), false},
		{"float zero", GenericProbability(0), false},
		{"empty string", ContactEmail(""), true},
		{"non-empty string", ContactEmail("a@b.com"), false},
		{"nil", nil, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equal(t, tt.missing, scalarRequiredValueMissing(tt.value))
		})
	}
}
