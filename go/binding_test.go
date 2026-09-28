package superscalar

import (
	"strings"
	"testing"
)


func TestCallScalarCoerceLenientTrimsMoneyString(t *testing.T) {
	// Finance.Money is an Int scalar: a padded numeric JSON string coerces to
	// the bare integer, serialized as the JSON document "12345".
	got, err := callScalarCoerceLenient(scalarNameFinanceMoney, `" 12345 "`)
	if err != nil {
		t.Fatalf("coerce_lenient money: unexpected error: %v", err)
	}
	if got != "12345" {
		t.Fatalf("coerce_lenient money: got %q, want %q", got, "12345")
	}
}

func TestCallScalarCoerceLenientNullPassthrough(t *testing.T) {
	// A JSON null passes through cleanly as the JSON document "null".
	got, err := callScalarCoerceLenient(scalarNameFinanceMoney, "null")
	if err != nil {
		t.Fatalf("coerce_lenient null: unexpected error: %v", err)
	}
	if got != "null" {
		t.Fatalf("coerce_lenient null: got %q, want %q", got, "null")
	}
}

func TestCallScalarCoerceLenientBadDateIsError(t *testing.T) {
	// Temporal.Date is a String scalar: an unparseable date surfaces the
	// captured coercion failure as an error at the C ABI boundary.
	_, err := callScalarCoerceLenient(scalarNameTemporalDate, `"not-a-date"`)
	if err == nil {
		t.Fatal("coerce_lenient bad date: expected an error, got nil")
	}
}

func TestCallScalarCoerceLenientInvalidJSONIsError(t *testing.T) {
	_, err := callScalarCoerceLenient(scalarNameFinanceMoney, "{not json")
	if err == nil {
		t.Fatal("coerce_lenient invalid json: expected an error, got nil")
	}
	if !strings.Contains(err.Error(), "invalid json") {
		t.Fatalf("coerce_lenient invalid json: got %q, want it to mention invalid json", err.Error())
	}
}

func TestCallScalarCoerceLenientUnknownNameIsError(t *testing.T) {
	_, err := callScalarCoerceLenient("No.Such", "123")
	if err == nil {
		t.Fatal("coerce_lenient unknown name: expected an error, got nil")
	}
	if !strings.Contains(err.Error(), `unknown scalar "No.Such"`) {
		t.Fatalf("coerce_lenient unknown name: got %q", err.Error())
	}
}

func TestCallScalarParseUnknownNameIsError(t *testing.T) {
	// Names are exact and case-sensitive.
	_, err := callScalarParse("contact.email", "a@b.com")
	if err == nil || !strings.Contains(err.Error(), `unknown scalar "contact.email"`) {
		t.Fatalf("parse under a lowercased name: got %v", err)
	}
}
