package superscalar

import (
	"encoding/json"
	"testing"
)

func TestNormalizeTemporalMonth(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		wantErr  bool
	}{
		// numeric
		{"one digit", "2", "02", false},
		{"two digit padded", "02", "02", false},
		{"december numeric", "12", "12", false},
		{"december one digit", "9", "09", false},
		// full names
		{"full name lowercase", "february", "02", false},
		{"full name titlecase", "February", "02", false},
		{"full name uppercase", "FEBRUARY", "02", false},
		// abbreviations
		{"abbrev lowercase", "feb", "02", false},
		{"abbrev mixed case", "Feb", "02", false},
		{"sep abbrev", "Sep", "09", false},
		// whitespace
		{"whitespace trim", "  Feb  ", "02", false},
		// rejected
		{"empty", "", "", true},
		{"whitespace only", "   ", "", true},
		{"zero", "0", "", true},
		{"thirteen", "13", "", true},
		{"negative", "-1", "", true},
		{"three digits", "001", "", true},
		{"unknown name", "Smarch", "", true},
		{"partial name", "febr", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := normalizeTemporalMonth(tt.input)
			if (err != nil) != tt.wantErr {
				t.Fatalf("normalizeTemporalMonth(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if got != tt.expected {
				t.Errorf("normalizeTemporalMonth(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

func TestNormalizeAndParseTemporalMonth(t *testing.T) {
	got, err := NormalizeAndParseTemporalMonth("July")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != TemporalMonth("07") {
		t.Errorf("NormalizeAndParseTemporalMonth(July) = %q, want %q", got, TemporalMonth("07"))
	}

	if _, err := NormalizeAndParseTemporalMonth("nope"); err == nil {
		t.Error("NormalizeAndParseTemporalMonth(nope) expected error, got nil")
	}
}

func TestTemporalMonth_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    TemporalMonth
		wantErr bool
	}{
		{"canonical", `"02"`, TemporalMonth("02"), false},
		{"one-digit numeric", `"2"`, TemporalMonth("02"), false},
		{"abbrev", `"Feb"`, TemporalMonth("02"), false},
		{"full name", `"February"`, TemporalMonth("02"), false},
		{"uppercase", `"FEBRUARY"`, TemporalMonth("02"), false},
		{"whitespace", `"  Feb  "`, TemporalMonth("02"), false},
		// rejected
		{"unknown", `"Smarch"`, TemporalMonth(""), true},
		{"zero", `"0"`, TemporalMonth(""), true},
		{"empty", `""`, TemporalMonth(""), true},
		{"non-string JSON", `42`, TemporalMonth(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v TemporalMonth
			err := v.UnmarshalJSON([]byte(tt.json))
			if (err != nil) != tt.wantErr {
				t.Fatalf("UnmarshalJSON(%s) error = %v, wantErr %v", tt.json, err, tt.wantErr)
			}
			if !tt.wantErr && v != tt.want {
				t.Errorf("UnmarshalJSON(%s) = %q, want %q", tt.json, v, tt.want)
			}
		})
	}
}

func TestTemporalMonth_UnmarshalJSON_RoundTrip(t *testing.T) {
	type wrapper struct {
		Month TemporalMonth `json:"month"`
	}
	var w wrapper
	if err := json.Unmarshal([]byte(`{"month":"Feb"}`), &w); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if w.Month != TemporalMonth("02") {
		t.Errorf("got %q, want %q", w.Month, TemporalMonth("02"))
	}
}

func TestNormalizeTemporalMonth_RoundTrip(t *testing.T) {
	cases := []string{"1", "01", "Feb", "feb", "FEBRUARY", "December", "Dec"}
	for _, input := range cases {
		first, err := normalizeTemporalMonth(input)
		if err != nil {
			t.Fatalf("first normalize(%q) error: %v", input, err)
		}
		second, err := normalizeTemporalMonth(first)
		if err != nil {
			t.Fatalf("second normalize(%q) error: %v", first, err)
		}
		if first != second {
			t.Errorf("round trip mismatch for %q: %q -> %q", input, first, second)
		}
	}
}
