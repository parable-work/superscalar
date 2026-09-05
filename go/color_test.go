package superscalar

import (
	"testing"
)

func TestNormalizeColor(t *testing.T) {
	// Accepted inputs: NormalizeColor returns the canonical #RRGGBBAA form.
	accepted := []struct {
		name  string
		input string
		want  string
	}{
		// Hex colors
		{"3-digit hex", "#F00", "#FF0000FF"},
		{"6-digit hex", "#FF5733", "#FF5733FF"},
		{"8-digit hex", "#FF573380", "#FF573380"},
		{"hex without hash", "FF5733", "#FF5733FF"},
		{"lowercase hex", "#ff5733", "#FF5733FF"},

		// Named colors
		{"named color red", "red", "#FF0000FF"},
		{"named color blue", "blue", "#0000FFFF"},
		{"named color transparent", "transparent", "#00000000"},
		{"named color case insensitive", "RED", "#FF0000FF"},
		{"named color rebeccapurple", "rebeccapurple", "#663399FF"},

		// RGB/RGBA
		{"rgb", "rgb(255, 0, 0)", "#FF0000FF"},
		{"rgba with alpha", "rgba(255, 0, 0, 0.5)", "#FF000080"},
		{"rgba with spaces", "rgb( 255 , 0 , 0 )", "#FF0000FF"},
		{"case insensitive rgb", "RGB(255, 0, 0)", "#FF0000FF"},

		// HSL/HSLA
		{"hsl red", "hsl(0, 100%, 50%)", "#FF0000FF"},
		{"hsl green", "hsl(120, 100%, 50%)", "#00FF00FF"},
		{"hsl blue", "hsl(240, 100%, 50%)", "#0000FFFF"},
		{"hsla with alpha", "hsla(0, 100%, 50%, 0.5)", "#FF000080"},
		{"hsl grayscale", "hsl(0, 0%, 50%)", "#808080FF"},

		// Edge cases
		{"with whitespace", "  #FF5733  ", "#FF5733FF"},
		{"rgba with zero alpha", "rgba(255, 0, 0, 0)", "#FF000000"},
		{"rgba with full alpha", "rgba(255, 0, 0, 1)", "#FF0000FF"},
	}
	for _, tt := range accepted {
		t.Run(tt.name, func(t *testing.T) {
			got := NormalizeColor(tt.input)
			if got != tt.want {
				t.Errorf("NormalizeColor(%q) = %q, want %q", tt.input, got, tt.want)
			}
			// Idempotence on a successfully-normalized result.
			if again := NormalizeColor(got); again != got {
				t.Errorf("NormalizeColor not idempotent: %q -> %q -> %q", tt.input, got, again)
			}
		})
	}

	// Rejected inputs: NormalizeColor returns the trimmed input unchanged and
	// ValidateColor reports the failure.
	rejected := []struct {
		name  string
		input string
	}{
		{"empty string", ""},
		{"invalid named color", "notacolor"},
		{"invalid hex", "#GGG"},
		{"invalid rgb values", "rgb(256, 0, 0)"},
		{"malformed rgb", "rgb(255, 0)"},
		{"malformed hsl", "hsl(0, 100)"},
	}
	for _, tt := range rejected {
		t.Run(tt.name, func(t *testing.T) {
			got := NormalizeColor(tt.input)
			if errs := ValidateColor(got); len(errs) == 0 {
				t.Errorf("NormalizeColor(%q) = %q; ValidateColor expected to fail but passed", tt.input, got)
			}
		})
	}
}

func TestDesignColor_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    DesignColor
		wantErr bool
	}{
		{"hex color", `"#FF5733"`, DesignColor("#FF5733FF"), false},
		{"named color", `"red"`, DesignColor("#FF0000FF"), false},
		{"rgb color", `"rgb(0, 128, 255)"`, DesignColor("#0080FFFF"), false},
		{"invalid json", `42`, DesignColor(""), true},
		{"invalid color", `"notacolor"`, DesignColor(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var c DesignColor
			err := c.UnmarshalJSON([]byte(tt.json))
			if (err != nil) != tt.wantErr {
				t.Fatalf("UnmarshalJSON() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr && c != tt.want {
				t.Fatalf("UnmarshalJSON() = %q, want %q", c, tt.want)
			}
		})
	}
}

func TestValidateColor(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		wantErrs bool
	}{
		{"valid RGBA hex", "#FF5733FF", false},
		{"valid black with alpha", "#000000FF", false},
		{"valid white with alpha", "#FFFFFFFF", false},
		{"valid transparent", "#00000000", false},

		{"invalid 6-digit hex", "#FF5733", true},
		{"invalid 3-digit hex", "#F00", true},
		{"invalid named color", "red", true},
		{"invalid rgb", "rgb(255, 0, 0)", true},
		{"invalid lowercase", "#ff5733ff", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			errs := ValidateColor(tt.input)
			hasErrs := len(errs) > 0
			if hasErrs != tt.wantErrs {
				t.Errorf("ValidateColor() errors = %v, wantErrs %v", errs, tt.wantErrs)
			}
		})
	}
}
