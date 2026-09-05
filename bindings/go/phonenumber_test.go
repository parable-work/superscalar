package superscalar

import (
	"testing"
)

func TestNormalizePhoneNumber(t *testing.T) {
	accepted := []struct {
		name  string
		input string
		want  string
	}{
		{"US number with country code and formatting", "+1 (415) 555-2671", "+14155552671"},
		{"US number E164 format", "+14155552671", "+14155552671"},
		{"US number with dashes", "+1-415-555-2671", "+14155552671"},
		{"UK number E164", "+442071838750", "+442071838750"},
		{"UK number with spaces", "+44 20 7183 8750", "+442071838750"},
		{"France number", "+33 1 23 45 67 89", "+33123456789"},
		{"Germany number", "+49 30 12345678", "+493012345678"},
		{"Australia number", "+61 2 1234 5678", "+61212345678"},
		{"Japan number", "+81 3 1234 5678", "+81312345678"},
	}
	for _, tt := range accepted {
		t.Run(tt.name, func(t *testing.T) {
			got := NormalizePhoneNumber(tt.input)
			if got != tt.want {
				t.Errorf("NormalizePhoneNumber(%q) = %q, want %q", tt.input, got, tt.want)
			}
			if again := NormalizePhoneNumber(got); again != got {
				t.Errorf("NormalizePhoneNumber not idempotent: %q -> %q -> %q", tt.input, got, again)
			}
		})
	}

	rejected := []struct {
		name  string
		input string
	}{
		{"number without country code", "4155552671"},
		{"invalid number", "123"},
		{"empty string", ""},
		{"missing country code prefix", "14155552671"},
	}
	for _, tt := range rejected {
		t.Run(tt.name, func(t *testing.T) {
			got := NormalizePhoneNumber(tt.input)
			if errs := ValidatePhoneNumber(got); len(errs) == 0 {
				t.Errorf("NormalizePhoneNumber(%q) = %q; ValidatePhoneNumber expected to fail but passed", tt.input, got)
			}
		})
	}
}

func TestContactPhoneNumber_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    ContactPhoneNumber
		wantErr bool
	}{
		{"valid US number", `"+14155552671"`, ContactPhoneNumber("+14155552671"), false},
		{"valid UK number", `"+442071838750"`, ContactPhoneNumber("+442071838750"), false},
		{"invalid json type", `42`, ContactPhoneNumber(""), true},
		{"invalid phone number", `"notaphone"`, ContactPhoneNumber(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var p ContactPhoneNumber
			err := p.UnmarshalJSON([]byte(tt.json))
			if (err != nil) != tt.wantErr {
				t.Fatalf("UnmarshalJSON() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr && p != tt.want {
				t.Fatalf("UnmarshalJSON() = %q, want %q", p, tt.want)
			}
		})
	}
}

func TestValidatePhoneNumber(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		wantErrs bool
	}{
		{
			name:     "valid US E164",
			input:    "+14155552671",
			wantErrs: false,
		},
		{
			name:     "valid US formatted",
			input:    "+1 (415) 555-2671",
			wantErrs: false,
		},
		{
			name:     "valid UK",
			input:    "+442071838750",
			wantErrs: false,
		},
		{
			name:     "valid France",
			input:    "+33123456789",
			wantErrs: false,
		},
		{
			name:     "valid Germany",
			input:    "+493012345678",
			wantErrs: false,
		},
		{
			name:     "invalid - no country code",
			input:    "4155552671",
			wantErrs: true,
		},
		{
			name:     "invalid - too short",
			input:    "123",
			wantErrs: true,
		},
		{
			name:     "invalid - missing + prefix",
			input:    "14155552671",
			wantErrs: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			errs := ValidatePhoneNumber(tt.input)
			hasErrs := len(errs) > 0
			if hasErrs != tt.wantErrs {
				t.Errorf("ValidatePhoneNumber() errors = %v, wantErrs %v", errs, tt.wantErrs)
			}
		})
	}
}
