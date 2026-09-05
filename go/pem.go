package superscalar

import "strings"

const (
	ScalarRSAPrivateKey       = "RSAPrivateKey"
	ScalarCryptoRSAPrivateKey = "Crypto.RSAPrivateKey"
)

// True for RSA private key PEM scalars, whose literal \n escapes must be normalized before parse.
func IsPEMScalar(typeName string) bool {
	return typeName == ScalarRSAPrivateKey || typeName == ScalarCryptoRSAPrivateKey
}

func NormalizePemNewlines(input string) string {
	if !strings.Contains(input, `\n`) {
		return input
	}
	return strings.ReplaceAll(input, `\n`, "\n")
}
