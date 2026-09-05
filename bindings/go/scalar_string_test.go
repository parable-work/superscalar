package superscalar

import "testing"

// Regression: String() on scalar types with non-string underlying kinds used to
// fall through to fmt.Sprint, which re-invokes the Stringer and recurses until
// stack overflow. scalarStringValue must format via the underlying kind.
func TestScalarString_NumericUnderlyingKinds(t *testing.T) {
	tests := []struct {
		name string
		got  string
		want string
	}{
		{"GenericInt64", GenericInt64(42).String(), "42"},
		{"GenericInt64 negative", GenericInt64(-7).String(), "-7"},
		{"FileSizeBytes", FileSizeBytes(1024).String(), "1024"},
		{"FinanceMoney", FinanceMoney(199).String(), "199"},
		{"GenericProbability", GenericProbability(0.5).String(), "0.5"},
		{"TemporalMilliseconds", TemporalMilliseconds(1500).String(), "1500"},
		{"TemporalSeconds", TemporalSeconds(-60).String(), "-60"},
		{"TemporalMinutes", TemporalMinutes(-5).String(), "-5"},
		{"TemporalHours", TemporalHours(-2).String(), "-2"},
		{"TemporalDays", TemporalDays(-7).String(), "-7"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.got != tt.want {
				t.Errorf("String() = %q, want %q", tt.got, tt.want)
			}
		})
	}
}

func TestTemporalUnitAliases_ParseSignedValues(t *testing.T) {
	for name, parse := range map[string]func(string) (string, error){
		"Milliseconds": ParseTemporalMilliseconds,
		"Seconds":      ParseTemporalSeconds,
		"Minutes":      ParseTemporalMinutes,
		"Hours":        ParseTemporalHours,
		"Days":         ParseTemporalDays,
	} {
		t.Run(name, func(t *testing.T) {
			value, err := parse("-1")
			if err != nil {
				t.Fatalf("parse signed value: %v", err)
			}
			if value != "-1" {
				t.Fatalf("parse signed value = %q, want -1", value)
			}
		})
	}
}

func TestScalarString_ValidateNumericDoesNotRecurse(t *testing.T) {
	ok, errs := GenericInt64(42).Validate()
	if !ok {
		t.Fatalf("GenericInt64(42).Validate() failed: %v", errs)
	}
	if len(errs) != 0 {
		t.Fatalf("GenericInt64(42).Validate() unexpected errors: %v", errs)
	}
}
