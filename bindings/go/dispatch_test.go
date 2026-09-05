package superscalar

import (
	"errors"
	"testing"
)

func TestKnownScalar(t *testing.T) {
	cases := []struct {
		canonical string
		want      bool
	}{
		{"Contact.Email", true},
		{"Embedding.Vector", true},
		{"Identity.Slug", true},
		{"Generic.StringMap", true},
		{"Text.Markdown", true},
		{"Text.Sql", true},
		{"Network.DnsLabel", true},
		{"Not.AScalar", false},
		{"", false},
	}
	for _, c := range cases {
		if got := KnownScalar(c.canonical); got != c.want {
			t.Errorf("KnownScalar(%q) = %v, want %v", c.canonical, got, c.want)
		}
	}
}

func TestValidate(t *testing.T) {
	cases := []struct {
		name      string
		canonical string
		value     string
		wantErr   bool
	}{
		{"email invalid", "Contact.Email", "nope", true},
		{"email valid", "Contact.Email", "a@b.com", false},
		{"vector invalid", "Embedding.Vector", "notavector", true},
		{"vector valid", "Embedding.Vector", "[1.0, 2.0, 3.0]", false},
		{"stringmap invalid", "Generic.StringMap", "not json", true},
		{"stringmap valid", "Generic.StringMap", `{"a":"b"}`, false},
		{"slug empty", "Identity.Slug", "", true},
		{"slug valid", "Identity.Slug", "my-slug", false},
		{"markdown empty", "Text.Markdown", "", true},
		{"markdown valid", "Text.Markdown", "# hi", false},
		{"sql empty", "Text.Sql", "", true},
		{"sql valid", "Text.Sql", "SELECT 1", false},
		{"dnslabel hostname invalid", "Network.DnsLabel", "acme.okta.com", true},
		{"dnslabel valid", "Network.DnsLabel", "acme-corp", false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			err := Validate(c.canonical, c.value)
			if c.wantErr && err == nil {
				t.Fatalf("Validate(%q, %q) = nil, want error", c.canonical, c.value)
			}
			if !c.wantErr && err != nil {
				t.Fatalf("Validate(%q, %q) = %v, want nil", c.canonical, c.value, err)
			}
		})
	}
}

func TestValidateUnknownScalar(t *testing.T) {
	err := Validate("Not.AScalar", "anything")
	if err == nil {
		t.Fatal("Validate on unknown scalar = nil, want ErrUnknownScalar")
	}
	if !errors.Is(err, ErrUnknownScalar) {
		t.Fatalf("Validate on unknown scalar = %v, want errors.Is ErrUnknownScalar", err)
	}
}

func TestParse(t *testing.T) {
	got, err := Parse("Contact.Email", "  A@B.COM  ")
	if err != nil {
		t.Fatalf("Parse(Contact.Email) returned error: %v", err)
	}
	if got == "" {
		t.Fatal("Parse(Contact.Email) returned empty canonical form")
	}

	if _, err := Parse("Embedding.Vector", "notavector"); err == nil {
		t.Fatal("Parse(Embedding.Vector, notavector) = nil, want error")
	}

	if _, err := Parse("Not.AScalar", "x"); !errors.Is(err, ErrUnknownScalar) {
		t.Fatalf("Parse on unknown scalar = %v, want ErrUnknownScalar", err)
	}
}

func TestNormalize(t *testing.T) {
	got, err := Normalize("Contact.Email", "A@B.COM")
	if err != nil {
		t.Fatalf("Normalize(Contact.Email) returned error: %v", err)
	}
	if got == "" {
		t.Fatal("Normalize(Contact.Email) returned empty form")
	}

	if _, err := Normalize("Not.AScalar", "x"); !errors.Is(err, ErrUnknownScalar) {
		t.Fatalf("Normalize on unknown scalar = %v, want ErrUnknownScalar", err)
	}
}

func TestCoerceJSON(t *testing.T) {
	got, err := CoerceJSON("Generic.Int64", `"42"`)
	if err != nil {
		t.Fatalf("CoerceJSON(Generic.Int64): %v", err)
	}
	if got != "42" {
		t.Fatalf("CoerceJSON(Generic.Int64) = %q, want 42", got)
	}

	got, err = CoerceJSON("Generic.JSON", `{"b":2,"a":1}`)
	if err != nil {
		t.Fatalf("CoerceJSON(Generic.JSON): %v", err)
	}
	if got != `{"b":2,"a":1}` {
		t.Fatalf("CoerceJSON(Generic.JSON) = %q", got)
	}

	if _, err := CoerceJSON("Generic.Int64", `"not-a-number"`); err == nil {
		t.Fatal("CoerceJSON accepted an invalid Generic.Int64")
	}
	if _, err := CoerceJSON("Not.AScalar", `1`); !errors.Is(err, ErrUnknownScalar) {
		t.Fatalf("CoerceJSON unknown scalar = %v, want ErrUnknownScalar", err)
	}
}
