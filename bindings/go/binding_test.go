package superscalar

import (
	"strings"
	"testing"
)

// Finance.Money (Int) and an out-of-range id, named so they do not collide with
// the generated scalarID constants in generated.go (which already define
// scalarIDFinanceMoney and scalarIDTemporalDate).
const (
	scalarIDFinanceMoneyForTest uint32 = 15
	scalarIDUnknownForTest      uint32 = 99999
)

func TestCallScalarCoerceLenientTrimsMoneyString(t *testing.T) {
	// Finance.Money is an Int scalar: a padded numeric JSON string coerces to
	// the bare integer, serialized as the JSON document "12345".
	got, err := callScalarCoerceLenient(scalarIDFinanceMoneyForTest, `" 12345 "`)
	if err != nil {
		t.Fatalf("coerce_lenient money: unexpected error: %v", err)
	}
	if got != "12345" {
		t.Fatalf("coerce_lenient money: got %q, want %q", got, "12345")
	}
}

func TestCallScalarCoerceLenientNullPassthrough(t *testing.T) {
	// A JSON null passes through cleanly as the JSON document "null".
	got, err := callScalarCoerceLenient(scalarIDFinanceMoneyForTest, "null")
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
	_, err := callScalarCoerceLenient(scalarIDTemporalDate, `"not-a-date"`)
	if err == nil {
		t.Fatal("coerce_lenient bad date: expected an error, got nil")
	}
}

func TestCallScalarCoerceLenientInvalidJSONIsError(t *testing.T) {
	_, err := callScalarCoerceLenient(scalarIDFinanceMoneyForTest, "{not json")
	if err == nil {
		t.Fatal("coerce_lenient invalid json: expected an error, got nil")
	}
	if !strings.Contains(err.Error(), "invalid json") {
		t.Fatalf("coerce_lenient invalid json: got %q, want it to mention invalid json", err.Error())
	}
}

func TestCallScalarCoerceLenientUnknownIDIsError(t *testing.T) {
	_, err := callScalarCoerceLenient(scalarIDUnknownForTest, "123")
	if err == nil {
		t.Fatal("coerce_lenient unknown id: expected an error, got nil")
	}
}
