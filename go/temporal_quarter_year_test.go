package superscalar

import (
	"encoding/json"
	"testing"
)

func TestNormalizeTemporalQuarterYear(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		wantErr  bool
	}{
		// canonical form
		{"canonical", "2025-Q1", "2025-Q1", false},
		{"canonical Q4", "2030-Q4", "2030-Q4", false},
		{"lowercase q", "2025-q1", "2025-Q1", false},
		// alternate shapes
		{"slash", "Q1/2025", "2025-Q1", false},
		{"slash lowercase", "q3/2025", "2025-Q3", false},
		{"dash full year", "Q1-2025", "2025-Q1", false},
		{"dash full year lowercase", "q2-2030", "2030-Q2", false},
		{"two digit year", "Q1-25", "2025-Q1", false},
		{"two digit year zero", "Q4-00", "2000-Q4", false},
		// whitespace
		{"whitespace trim", "  2025-Q1  ", "2025-Q1", false},
		// rejected
		{"empty", "", "", true},
		{"whitespace only", "   ", "", true},
		{"missing q", "2025-1", "", true},
		{"quarter zero", "2025-Q0", "", true},
		{"quarter five", "2025-Q5", "", true},
		{"three digit year", "Q1-202", "", true},
		{"five digit year", "Q1-20255", "", true},
		{"extra suffix", "2025-Q1x", "", true},
		{"swapped order", "Q1 2025", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := normalizeTemporalQuarterYear(tt.input)
			if (err != nil) != tt.wantErr {
				t.Fatalf("normalizeTemporalQuarterYear(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
			}
			if got != tt.expected {
				t.Errorf("normalizeTemporalQuarterYear(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}

func TestNormalizeAndParseTemporalQuarterYear(t *testing.T) {
	got, err := NormalizeAndParseTemporalQuarterYear("Q1/2025")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != TemporalQuarterYear("2025-Q1") {
		t.Errorf("NormalizeAndParseTemporalQuarterYear(Q1/2025) = %q, want %q", got, TemporalQuarterYear("2025-Q1"))
	}

	if _, err := NormalizeAndParseTemporalQuarterYear("nope"); err == nil {
		t.Error("NormalizeAndParseTemporalQuarterYear(nope) expected error, got nil")
	}
}

func TestTemporalQuarterYear_UnmarshalJSON(t *testing.T) {
	tests := []struct {
		name    string
		json    string
		want    TemporalQuarterYear
		wantErr bool
	}{
		{"canonical", `"2025-Q1"`, TemporalQuarterYear("2025-Q1"), false},
		{"lowercase q", `"2025-q1"`, TemporalQuarterYear("2025-Q1"), false},
		{"slash", `"Q1/2025"`, TemporalQuarterYear("2025-Q1"), false},
		{"dash full year", `"Q1-2025"`, TemporalQuarterYear("2025-Q1"), false},
		{"two digit year", `"Q1-25"`, TemporalQuarterYear("2025-Q1"), false},
		{"whitespace", `"  2025-Q1  "`, TemporalQuarterYear("2025-Q1"), false},
		// rejected
		{"empty", `""`, TemporalQuarterYear(""), true},
		{"garbage", `"garbage"`, TemporalQuarterYear(""), true},
		{"out of range quarter", `"2025-Q5"`, TemporalQuarterYear(""), true},
		{"non-string JSON", `42`, TemporalQuarterYear(""), true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var v TemporalQuarterYear
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

func TestTemporalQuarterYear_UnmarshalJSON_RoundTrip(t *testing.T) {
	type wrapper struct {
		QY TemporalQuarterYear `json:"qy"`
	}
	var w wrapper
	if err := json.Unmarshal([]byte(`{"qy":"Q1-25"}`), &w); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if w.QY != TemporalQuarterYear("2025-Q1") {
		t.Errorf("got %q, want %q", w.QY, TemporalQuarterYear("2025-Q1"))
	}
}

func TestNormalizeTemporalQuarterYear_RoundTrip(t *testing.T) {
	cases := []string{"2025-Q1", "Q1/2025", "Q1-2025", "Q1-25", "q4/2030"}
	for _, input := range cases {
		first, err := normalizeTemporalQuarterYear(input)
		if err != nil {
			t.Fatalf("first normalize(%q) error: %v", input, err)
		}
		second, err := normalizeTemporalQuarterYear(first)
		if err != nil {
			t.Fatalf("second normalize(%q) error: %v", first, err)
		}
		if first != second {
			t.Errorf("round trip mismatch for %q: %q -> %q", input, first, second)
		}
	}
}
