package superscalar

import (
	"testing"
)

func TestNormalizeEmail(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"lowercase simple email", "TEST@EXAMPLE.COM", "test@example.com"},
		{"trim whitespace", "  user@example.com  ", "user@example.com"},
		{"mixed case with whitespace", " User@Example.COM ", "user@example.com"},
		{"already normalized", "user@example.com", "user@example.com"},
		{"empty input", "", ""},
		{"whitespace only", "   ", ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := NormalizeEmail(tt.input)
			if result != tt.expected {
				t.Errorf("NormalizeEmail(%q) = %q, want %q", tt.input, result, tt.expected)
			}
			// Idempotence.
			if again := NormalizeEmail(result); again != result {
				t.Errorf("NormalizeEmail not idempotent: %q -> %q -> %q", tt.input, result, again)
			}
		})
	}
}

func TestContactEmail_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    ContactEmail
		wantErr bool
	}{
		{"valid email", `"User@Example.COM"`, ContactEmail("user@example.com"), false},
		{"with whitespace", `"  user@test.com  "`, ContactEmail("user@test.com"), false},
		{"invalid json", `42`, ContactEmail(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var e ContactEmail
			err := e.UnmarshalJSON([]byte(tt.json))
			if (err != nil) != tt.wantErr {
				t.Fatalf("UnmarshalJSON() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr && e != tt.want {
				t.Fatalf("UnmarshalJSON() = %q, want %q", e, tt.want)
			}
		})
	}
}

func TestValidateEmail(t *testing.T) {
	// ValidateEmail is the runtime extension hook and is intentionally a no-op:
	// the @scalarPattern directive on Contact_Email enforces the regex earlier
	// in the validation pipeline. These cases pin that no-op contract so we
	// notice if behavior drifts back into duplicating directive checks.
	tests := []struct {
		name     string
		input    string
		wantErrs bool
	}{
		{
			name:     "valid email",
			input:    "user@example.com",
			wantErrs: false,
		},
		{
			name:     "valid email with subdomain",
			input:    "user@mail.example.com",
			wantErrs: false,
		},
		{
			name:     "malformed email is no-op (directive enforces regex)",
			input:    "not-an-email",
			wantErrs: false,
		},
		{
			name:     "empty input is no-op",
			input:    "",
			wantErrs: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			errs := ValidateEmail(tt.input)
			hasErrs := len(errs) > 0
			if hasErrs != tt.wantErrs {
				t.Errorf("ValidateEmail() errors = %v, wantErrs %v", errs, tt.wantErrs)
			}
		})
	}
}

func TestNormalizeAndValidateEmail(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    string
		wantErr bool
	}{
		// accepted
		{"lowercase simple email", "TEST@EXAMPLE.COM", "test@example.com", false},
		{"trim whitespace", "  user@example.com  ", "user@example.com", false},
		{"mixed case with whitespace", " User@Example.COM ", "user@example.com", false},
		{"already normalized", "user@example.com", "user@example.com", false},
		{"with subdomain", "user@mail.example.com", "user@mail.example.com", false},
		{"with plus addressing", "user+tag@example.com", "user+tag@example.com", false},
		{"with dots in local part", "first.last@example.com", "first.last@example.com", false},

		// rejected
		{"empty input", "", "", true},
		{"whitespace only", "   ", "", true},
		{"non-email string", "not-an-email", "", true},
		{"missing @", "userexample.com", "", true},
		{"missing TLD", "user@example", "", true},
		{"TLD too short", "user@example.c", "", true},
		{"local part with whitespace", "us er@example.com", "", true},
		{"domain part with whitespace", "user@exa mple.com", "", true},
		{"multiple @", "user@@example.com", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := NormalizeAndValidateEmail(tt.input)
			if (err != nil) != tt.wantErr {
				t.Fatalf("NormalizeAndValidateEmail(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if !tt.wantErr && got != tt.want {
				t.Errorf("NormalizeAndValidateEmail(%q) = %q, want %q", tt.input, got, tt.want)
			}
		})
	}
}
