package superscalar

import (
	"encoding/json"
	"testing"
)

func TestNormalizeTemporalDate(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		wantErr  bool
	}{
		// ISO
		{"iso canonical", "2025-01-01", "2025-01-01", false},
		// slash
		{"iso slash", "2025/01/15", "2025-01-15", false},
		{"us slash", "01/15/2025", "2025-01-15", false},
		// named
		{"full month", "January 15, 2025", "2025-01-15", false},
		{"abbrev month", "Jan 15, 2025", "2025-01-15", false},
		{"day-month-year", "15 January 2025", "2025-01-15", false},
		// datetime extract
		{"rfc3339", "2025-01-15T12:00:00Z", "2025-01-15", false},
		{"datetime offset", "2025-01-15T07:00:00-05:00", "2025-01-15", false},
		// whitespace
		{"whitespace trim", "  2025-01-01  ", "2025-01-01", false},
		// rejected
		{"empty", "", "", true},
		{"whitespace only", "   ", "", true},
		{"garbage", "not-a-date", "", true},
		{"invalid month", "2025-13-01", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := normalizeTemporalDate(tt.input)
			if (err != nil) != tt.wantErr {
				t.Fatalf("normalizeTemporalDate(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if got != tt.expected {
				t.Errorf("normalizeTemporalDate(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

func TestNormalizeAndParseTemporalDate(t *testing.T) {
	got, err := NormalizeAndParseTemporalDate("January 15, 2025")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != TemporalDate("2025-01-15") {
		t.Errorf("NormalizeAndParseTemporalDate(January 15, 2025) = %q, want %q", got, TemporalDate("2025-01-15"))
	}

	if _, err := NormalizeAndParseTemporalDate("nope"); err == nil {
		t.Error("NormalizeAndParseTemporalDate(nope) expected error, got nil")
	}
}

func TestTemporalDate_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    TemporalDate
		wantErr bool
	}{
		{"canonical", `"2025-01-01"`, TemporalDate("2025-01-01"), false},
		{"slash iso", `"2025/01/15"`, TemporalDate("2025-01-15"), false},
		{"us slash", `"01/15/2025"`, TemporalDate("2025-01-15"), false},
		{"named", `"January 15, 2025"`, TemporalDate("2025-01-15"), false},
		{"datetime", `"2025-01-15T12:00:00Z"`, TemporalDate("2025-01-15"), false},
		{"whitespace", `"  2025-01-01  "`, TemporalDate("2025-01-01"), false},
		// rejected
		{"empty", `""`, TemporalDate(""), true},
		{"garbage", `"not-a-date"`, TemporalDate(""), true},
		{"non-string JSON", `42`, TemporalDate(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v TemporalDate
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

func TestTemporalDate_UnmarshalJSON_RoundTrip(t *testing.T) {
	type wrapper struct {
		Date TemporalDate `json:"date"`
	}
	var w wrapper
	if err := json.Unmarshal([]byte(`{"date":"01/15/2025"}`), &w); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if w.Date != TemporalDate("2025-01-15") {
		t.Errorf("got %q, want %q", w.Date, TemporalDate("2025-01-15"))
	}
}

func TestNormalizeTemporalDate_RoundTrip(t *testing.T) {
	cases := []string{"2025-01-01", "01/15/2025", "January 15, 2025", "2025-01-15T12:00:00Z"}
	for _, input := range cases {
		first, err := normalizeTemporalDate(input)
		if err != nil {
			t.Fatalf("first normalize(%q) error: %v", input, err)
		}
		second, err := normalizeTemporalDate(first)
		if err != nil {
			t.Fatalf("second normalize(%q) error: %v", first, err)
		}
		if first != second {
			t.Errorf("round trip mismatch for %q: %q -> %q", input, first, second)
		}
	}
}
