const PEM_NORMALIZABLE_SCALARS = new Set(['RSAPrivateKey', 'Crypto.RSAPrivateKey']);

/** Whether a schema scalar type name stores RSA private key PEM that may need newline normalization. */
export function isPemNormalizableScalar(typeName: string): boolean {
  return PEM_NORMALIZABLE_SCALARS.has(typeName);
}

/** Replaces literal backslash-n sequences with real newlines. No-op when none are present. */
export function normalizePemNewlines(input: string): string {
  if (!input.includes('\\n')) {
    return input;
  }
  return input.split('\\n').join('\n');
}
