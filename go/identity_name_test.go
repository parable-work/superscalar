package superscalar

import (
	"strings"
	"testing"
	"unicode/utf8"
)

func TestIdentityNameMaxLengthMatchesValidate(t *testing.T) {
	// The const must match the generated IdentityName.Validate() upper bound so
	// the clamp and the scalar validator agree on the limit.
	over := IdentityName(strings.Repeat("a", IdentityNameMaxLength+1))
	if ok, _ := over.Validate(); ok {
		t.Fatalf("IdentityName of %d chars should fail Validate()", IdentityNameMaxLength+1)
	}
	atMax := IdentityName(strings.Repeat("a", IdentityNameMaxLength))
	if ok, errs := atMax.Validate(); !ok {
		t.Fatalf("IdentityName of %d chars should pass Validate(): %v", IdentityNameMaxLength, errs)
	}
}

func TestClampIdentityName(t *testing.T) {
	cases := []struct {
		name string
		in   string
		want string
	}{
		{"short name untouched", "Ana Kowalski", "Ana Kowalski"},
		{"exactly max stays intact", strings.Repeat("a", IdentityNameMaxLength), strings.Repeat("a", IdentityNameMaxLength)},
		{"over max truncates and trims trailing space", strings.Repeat("a", IdentityNameMaxLength-1) + " bb", strings.Repeat("a", IdentityNameMaxLength-1)},
		// 3-byte runes: byte slicing would split a rune mid-sequence; rune-based
		// clamp cuts cleanly at IdentityNameMaxLength runes.
		{"multibyte truncates on rune boundary", strings.Repeat("世", IdentityNameMaxLength+10), strings.Repeat("世", IdentityNameMaxLength)},
		{"multibyte under max intact", strings.Repeat("世", 40), strings.Repeat("世", 40)},
		{"empty stays empty", "", ""},
	}
	for _, c := range cases {
		c := c
		t.Run(c.name, func(t *testing.T) {
			got := ClampIdentityName(c.in)
			if got != c.want {
				t.Fatalf("ClampIdentityName(%q) = %q, want %q", c.in, got, c.want)
			}
			if utf8.RuneCountInString(got) > IdentityNameMaxLength {
				t.Fatalf("ClampIdentityName returned %d runes (max %d)", utf8.RuneCountInString(got), IdentityNameMaxLength)
			}
			if !utf8.ValidString(got) {
				t.Fatalf("ClampIdentityName returned invalid UTF-8: %q", got)
			}
		})
	}
}
