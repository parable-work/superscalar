package superscalar

import "testing"

func TestIsPEMScalar(t *testing.T) {
	if !IsPEMScalar(ScalarCryptoRSAPrivateKey) {
		t.Fatalf("expected Crypto.RSAPrivateKey to be PEM scalar")
	}
	if !IsPEMScalar(ScalarRSAPrivateKey) {
		t.Fatalf("expected RSAPrivateKey to be PEM scalar")
	}
	if IsPEMScalar("String") {
		t.Fatalf("String must not be PEM scalar")
	}
}

func TestNormalizePemNewlines(t *testing.T) {
	if got := NormalizePemNewlines(`line1\nline2`); got != "line1\nline2" {
		t.Fatalf("got %q", got)
	}
	unchanged := "already\nreal"
	if got := NormalizePemNewlines(unchanged); got != unchanged {
		t.Fatalf("got %q", got)
	}
}
