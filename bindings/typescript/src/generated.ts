// @generated; do not edit
import { backend } from "./backend";
import type { ScalarValidationResult, ValidationError } from "./validation";
export type JSDate = globalThis.Date;

export interface ScalarMetadata {
  canonicalName: string;
  symbol: string;
  primitive: string;
  tsType: string;
  format: string;
  maxLength: number;
  minLength: number;
  pattern: string;
  hasValidator: boolean;
  examples: string[];
  /**
   * Comparability equivalence class, null when the scalar has none.
   * Field equality is NOT the comparability relation: two nulls compare
   * equal, so a direct comparison answers true for every pair of
   * unclassed scalars. The relation is `comparableWith` below, mirroring the
   * Rust core's `Registry::comparable_with`.
   */
  comparabilityClass: string | null;
  isSortable: boolean;
}

/**
 * Flat (single-identifier) form of a canonical scalar name, e.g.
 * "Contact.Email" -> "Contact_Email". Used where dotted names are not valid
 * identifiers (runtime schema payload keys, scalar registry keys). Canonical
 * segments are PascalCase and never contain underscores, so the mapping is
 * bijective.
 */
export function flatScalarName(canonicalName: string): string {
  return canonicalName.replace(/\./g, "_");
}

export const scalarIdByCanonical: Record<string, number> = {
  "Auth.JWT": 5,
  "Auth.Password": 6,
  "Contact.Email": 8,
  "Contact.PhoneNumber": 9,
  "Crypto.RSAPrivateKey": 10,
  "Crypto.RSAPublicKey": 11,
  "Design.Color": 12,
  "Embedding.Vector": 13,
  "File.SizeBytes": 14,
  "Finance.Money": 15,
  "Generic.Int64": 16,
  "Generic.JSON": 17,
  "Generic.Probability": 18,
  "Generic.StringMap": 19,
  "Geo.Location": 20,
  "Identity.Name": 21,
  "Identity.Slug": 22,
  "Identity.UUID": 23,
  "Identity.UserID": 24,
  "Localization.Locale": 25,
  "Network.DomainName": 26,
  "Network.IpAddress": 27,
  "Network.Uri": 28,
  "Network.Url": 29,
  "Temporal.CronExpression": 39,
  "Temporal.Date": 40,
  "Temporal.DateTime": 41,
  "Temporal.Duration": 42,
  "Temporal.Milliseconds": 43,
  "Temporal.Month": 44,
  "Temporal.Quarter": 45,
  "Temporal.QuarterYear": 46,
  "Temporal.Time": 47,
  "Temporal.TimeZone": 48,
  "Temporal.Year": 49,
  "Text.Markdown": 50,
  "Temporal.Seconds": 52,
  "Temporal.Minutes": 53,
  "Temporal.Hours": 54,
  "Temporal.Days": 55,
  "Text.Sql": 56,
  "Crypto.SHA256": 58,
  "Network.DnsLabel": 59,
  "Temporal.RecurrenceRule": 60,
};

interface LenientScalarError {
  kind?: string;
  message: string;
}

interface LenientCoerceResult {
  value: unknown | null;
  error: LenientScalarError | null;
}

function scalarErrorFromUnknown(error: unknown): LenientScalarError {
  if (error instanceof Error) {
    return { kind: "scalar", message: error.message };
  }
  return { kind: "scalar", message: String(error) };
}

function validationErrorFromLenient(error: LenientScalarError): ValidationError {
  return { validator: error.kind || "scalar", message: error.message };
}

function coerceLenient(id: number, value: unknown): LenientCoerceResult {
  try {
    const encoded = JSON.stringify(value);
    const jsonIn = encoded === undefined ? "null" : encoded;
    const decoded = JSON.parse(backend.coerceLenient(id, jsonIn)) as Partial<LenientCoerceResult>;
    return {
      value: decoded.value === undefined ? null : decoded.value,
      error: decoded.error === undefined ? null : decoded.error,
    };
  } catch (error) {
    return { value: null, error: scalarErrorFromUnknown(error) };
  }
}

function coerceValue(id: number, value: unknown): unknown | null {
  const result = coerceLenient(id, value);
  if (result.error) {
    return null;
  }
  return result.value;
}

function parseJsonObject(value: string): unknown | null {
  try {
    return JSON.parse(value);
  } catch {
    return null;
  }
}

function validateWithBackend(id: number, value: unknown | null | undefined): ScalarValidationResult {
  if (value === null || value === undefined || value === "") {
    return [true, null];
  }
  const result = coerceLenient(id, value);
  if (!result.error) {
    return [true, null];
  }
  return [false, [validationErrorFromLenient(result.error)]];
}

export type AuthJWT = string & { readonly __brand: "Auth.JWT" };
function convertAuthJWTValue(value: unknown | null): AuthJWT | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as AuthJWT;
}
export function parseAuthJWT(value: unknown): AuthJWT | null {
  return convertAuthJWTValue(coerceValue(5, value));
}
export function normalizeAuthJWT(value: unknown): AuthJWT | null {
  return convertAuthJWTValue(coerceValue(5, value));
}
export function parseAuthJWTStrict(value: string): AuthJWT {
  const parsed = convertAuthJWTValue(backend.parse(5, value));
  if (parsed === null) {
    throw new Error("invalid Auth.JWT");
  }
  return parsed;
}
export function normalizeAuthJWTStrict(value: string): AuthJWT {
  const normalized = convertAuthJWTValue(backend.normalize(5, value));
  if (normalized === null) {
    throw new Error("invalid Auth.JWT");
  }
  return normalized;
}
export function validateAuthJWT(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(5, value);
}

export type AuthPassword = string & { readonly __brand: "Auth.Password" };
function convertAuthPasswordValue(value: unknown | null): AuthPassword | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as AuthPassword;
}
export function parseAuthPassword(value: unknown): AuthPassword | null {
  return convertAuthPasswordValue(coerceValue(6, value));
}
export function normalizeAuthPassword(value: unknown): AuthPassword | null {
  return convertAuthPasswordValue(coerceValue(6, value));
}
export function parseAuthPasswordStrict(value: string): AuthPassword {
  const parsed = convertAuthPasswordValue(backend.parse(6, value));
  if (parsed === null) {
    throw new Error("invalid Auth.Password");
  }
  return parsed;
}
export function normalizeAuthPasswordStrict(value: string): AuthPassword {
  const normalized = convertAuthPasswordValue(backend.normalize(6, value));
  if (normalized === null) {
    throw new Error("invalid Auth.Password");
  }
  return normalized;
}
export function validateAuthPassword(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(6, value);
}

export type ContactEmail = string & { readonly __brand: "Contact.Email" };
function convertContactEmailValue(value: unknown | null): ContactEmail | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as ContactEmail;
}
export function parseContactEmail(value: unknown): ContactEmail | null {
  return convertContactEmailValue(coerceValue(8, value));
}
export function normalizeContactEmail(value: unknown): ContactEmail | null {
  return convertContactEmailValue(coerceValue(8, value));
}
export function parseContactEmailStrict(value: string): ContactEmail {
  const parsed = convertContactEmailValue(backend.parse(8, value));
  if (parsed === null) {
    throw new Error("invalid Contact.Email");
  }
  return parsed;
}
export function normalizeContactEmailStrict(value: string): ContactEmail {
  const normalized = convertContactEmailValue(backend.normalize(8, value));
  if (normalized === null) {
    throw new Error("invalid Contact.Email");
  }
  return normalized;
}
export function validateContactEmail(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(8, value);
}

export type ContactPhoneNumber = string & { readonly __brand: "Contact.PhoneNumber" };
function convertContactPhoneNumberValue(value: unknown | null): ContactPhoneNumber | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as ContactPhoneNumber;
}
export function parseContactPhoneNumber(value: unknown): ContactPhoneNumber | null {
  return convertContactPhoneNumberValue(coerceValue(9, value));
}
export function normalizeContactPhoneNumber(value: unknown): ContactPhoneNumber | null {
  return convertContactPhoneNumberValue(coerceValue(9, value));
}
export function parseContactPhoneNumberStrict(value: string): ContactPhoneNumber {
  const parsed = convertContactPhoneNumberValue(backend.parse(9, value));
  if (parsed === null) {
    throw new Error("invalid Contact.PhoneNumber");
  }
  return parsed;
}
export function normalizeContactPhoneNumberStrict(value: string): ContactPhoneNumber {
  const normalized = convertContactPhoneNumberValue(backend.normalize(9, value));
  if (normalized === null) {
    throw new Error("invalid Contact.PhoneNumber");
  }
  return normalized;
}
export function validateContactPhoneNumber(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(9, value);
}

export type CryptoRSAPrivateKey = string & { readonly __brand: "Crypto.RSAPrivateKey" };
function convertCryptoRSAPrivateKeyValue(value: unknown | null): CryptoRSAPrivateKey | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as CryptoRSAPrivateKey;
}
export function parseCryptoRSAPrivateKey(value: unknown): CryptoRSAPrivateKey | null {
  return convertCryptoRSAPrivateKeyValue(coerceValue(10, value));
}
export function normalizeCryptoRSAPrivateKey(value: unknown): CryptoRSAPrivateKey | null {
  return convertCryptoRSAPrivateKeyValue(coerceValue(10, value));
}
export function parseCryptoRSAPrivateKeyStrict(value: string): CryptoRSAPrivateKey {
  const parsed = convertCryptoRSAPrivateKeyValue(backend.parse(10, value));
  if (parsed === null) {
    throw new Error("invalid Crypto.RSAPrivateKey");
  }
  return parsed;
}
export function normalizeCryptoRSAPrivateKeyStrict(value: string): CryptoRSAPrivateKey {
  const normalized = convertCryptoRSAPrivateKeyValue(backend.normalize(10, value));
  if (normalized === null) {
    throw new Error("invalid Crypto.RSAPrivateKey");
  }
  return normalized;
}
export function validateCryptoRSAPrivateKey(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(10, value);
}

export type CryptoRSAPublicKey = string & { readonly __brand: "Crypto.RSAPublicKey" };
function convertCryptoRSAPublicKeyValue(value: unknown | null): CryptoRSAPublicKey | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as CryptoRSAPublicKey;
}
export function parseCryptoRSAPublicKey(value: unknown): CryptoRSAPublicKey | null {
  return convertCryptoRSAPublicKeyValue(coerceValue(11, value));
}
export function normalizeCryptoRSAPublicKey(value: unknown): CryptoRSAPublicKey | null {
  return convertCryptoRSAPublicKeyValue(coerceValue(11, value));
}
export function parseCryptoRSAPublicKeyStrict(value: string): CryptoRSAPublicKey {
  const parsed = convertCryptoRSAPublicKeyValue(backend.parse(11, value));
  if (parsed === null) {
    throw new Error("invalid Crypto.RSAPublicKey");
  }
  return parsed;
}
export function normalizeCryptoRSAPublicKeyStrict(value: string): CryptoRSAPublicKey {
  const normalized = convertCryptoRSAPublicKeyValue(backend.normalize(11, value));
  if (normalized === null) {
    throw new Error("invalid Crypto.RSAPublicKey");
  }
  return normalized;
}
export function validateCryptoRSAPublicKey(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(11, value);
}

export type DesignColor = string & { readonly __brand: "Design.Color" };
function convertDesignColorValue(value: unknown | null): DesignColor | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as DesignColor;
}
export function parseDesignColor(value: unknown): DesignColor | null {
  return convertDesignColorValue(coerceValue(12, value));
}
export function normalizeDesignColor(value: unknown): DesignColor | null {
  return convertDesignColorValue(coerceValue(12, value));
}
export function parseDesignColorStrict(value: string): DesignColor {
  const parsed = convertDesignColorValue(backend.parse(12, value));
  if (parsed === null) {
    throw new Error("invalid Design.Color");
  }
  return parsed;
}
export function normalizeDesignColorStrict(value: string): DesignColor {
  const normalized = convertDesignColorValue(backend.normalize(12, value));
  if (normalized === null) {
    throw new Error("invalid Design.Color");
  }
  return normalized;
}
export function validateDesignColor(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(12, value);
}

export type EmbeddingVector = number[];
function convertEmbeddingVectorValue(value: unknown | null): EmbeddingVector | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as EmbeddingVector;
}
export function parseEmbeddingVector(value: unknown): EmbeddingVector | null {
  return convertEmbeddingVectorValue(coerceValue(13, value));
}
export function normalizeEmbeddingVector(value: unknown): EmbeddingVector | null {
  return convertEmbeddingVectorValue(coerceValue(13, value));
}
export function parseEmbeddingVectorStrict(value: string): EmbeddingVector {
  const parsed = convertEmbeddingVectorValue(backend.parse(13, value));
  if (parsed === null) {
    throw new Error("invalid Embedding.Vector");
  }
  return parsed;
}
export function normalizeEmbeddingVectorStrict(value: string): EmbeddingVector {
  const normalized = convertEmbeddingVectorValue(backend.normalize(13, value));
  if (normalized === null) {
    throw new Error("invalid Embedding.Vector");
  }
  return normalized;
}
export function validateEmbeddingVector(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(13, value);
}

export type FileSizeBytes = number;
function convertFileSizeBytesValue(value: unknown | null): FileSizeBytes | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as FileSizeBytes;
}
export function parseFileSizeBytes(value: unknown): FileSizeBytes | null {
  return convertFileSizeBytesValue(coerceValue(14, value));
}
export function normalizeFileSizeBytes(value: unknown): FileSizeBytes | null {
  return convertFileSizeBytesValue(coerceValue(14, value));
}
export function parseFileSizeBytesStrict(value: string): FileSizeBytes {
  const parsed = convertFileSizeBytesValue(backend.parse(14, value));
  if (parsed === null) {
    throw new Error("invalid File.SizeBytes");
  }
  return parsed;
}
export function normalizeFileSizeBytesStrict(value: string): FileSizeBytes {
  const normalized = convertFileSizeBytesValue(backend.normalize(14, value));
  if (normalized === null) {
    throw new Error("invalid File.SizeBytes");
  }
  return normalized;
}
export function validateFileSizeBytes(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(14, value);
}

export type FinanceMoney = number;
function convertFinanceMoneyValue(value: unknown | null): FinanceMoney | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as FinanceMoney;
}
export function parseFinanceMoney(value: unknown): FinanceMoney | null {
  return convertFinanceMoneyValue(coerceValue(15, value));
}
export function normalizeFinanceMoney(value: unknown): FinanceMoney | null {
  return convertFinanceMoneyValue(coerceValue(15, value));
}
export function parseFinanceMoneyStrict(value: string): FinanceMoney {
  const parsed = convertFinanceMoneyValue(backend.parse(15, value));
  if (parsed === null) {
    throw new Error("invalid Finance.Money");
  }
  return parsed;
}
export function normalizeFinanceMoneyStrict(value: string): FinanceMoney {
  const normalized = convertFinanceMoneyValue(backend.normalize(15, value));
  if (normalized === null) {
    throw new Error("invalid Finance.Money");
  }
  return normalized;
}
export function validateFinanceMoney(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(15, value);
}

export type GenericInt64 = number;
function convertGenericInt64Value(value: unknown | null): GenericInt64 | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as GenericInt64;
}
export function parseGenericInt64(value: unknown): GenericInt64 | null {
  return convertGenericInt64Value(coerceValue(16, value));
}
export function normalizeGenericInt64(value: unknown): GenericInt64 | null {
  return convertGenericInt64Value(coerceValue(16, value));
}
export function parseGenericInt64Strict(value: string): GenericInt64 {
  const parsed = convertGenericInt64Value(backend.parse(16, value));
  if (parsed === null) {
    throw new Error("invalid Generic.Int64");
  }
  return parsed;
}
export function normalizeGenericInt64Strict(value: string): GenericInt64 {
  const normalized = convertGenericInt64Value(backend.normalize(16, value));
  if (normalized === null) {
    throw new Error("invalid Generic.Int64");
  }
  return normalized;
}
export function validateGenericInt64(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(16, value);
}

export type GenericJSON = Record<string, any>;
function convertGenericJSONValue(value: unknown | null): GenericJSON | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as GenericJSON;
}
export function parseGenericJSON(value: unknown): GenericJSON | null {
  return convertGenericJSONValue(coerceValue(17, value));
}
export function normalizeGenericJSON(value: unknown): GenericJSON | null {
  return convertGenericJSONValue(coerceValue(17, value));
}
export function parseGenericJSONStrict(value: string): GenericJSON {
  const parsed = convertGenericJSONValue(backend.parse(17, value));
  if (parsed === null) {
    throw new Error("invalid Generic.JSON");
  }
  return parsed;
}
export function normalizeGenericJSONStrict(value: string): GenericJSON {
  const normalized = convertGenericJSONValue(backend.normalize(17, value));
  if (normalized === null) {
    throw new Error("invalid Generic.JSON");
  }
  return normalized;
}
export function validateGenericJSON(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(17, value);
}

export type GenericProbability = number;
function convertGenericProbabilityValue(value: unknown | null): GenericProbability | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as GenericProbability;
}
export function parseGenericProbability(value: unknown): GenericProbability | null {
  return convertGenericProbabilityValue(coerceValue(18, value));
}
export function normalizeGenericProbability(value: unknown): GenericProbability | null {
  return convertGenericProbabilityValue(coerceValue(18, value));
}
export function parseGenericProbabilityStrict(value: string): GenericProbability {
  const parsed = convertGenericProbabilityValue(backend.parse(18, value));
  if (parsed === null) {
    throw new Error("invalid Generic.Probability");
  }
  return parsed;
}
export function normalizeGenericProbabilityStrict(value: string): GenericProbability {
  const normalized = convertGenericProbabilityValue(backend.normalize(18, value));
  if (normalized === null) {
    throw new Error("invalid Generic.Probability");
  }
  return normalized;
}
export function validateGenericProbability(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(18, value);
}

export type GenericStringMap = Record<string, string>;
function convertGenericStringMapValue(value: unknown | null): GenericStringMap | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as GenericStringMap;
}
export function parseGenericStringMap(value: unknown): GenericStringMap | null {
  return convertGenericStringMapValue(coerceValue(19, value));
}
export function normalizeGenericStringMap(value: unknown): GenericStringMap | null {
  return convertGenericStringMapValue(coerceValue(19, value));
}
export function parseGenericStringMapStrict(value: string): GenericStringMap {
  const parsed = convertGenericStringMapValue(backend.parse(19, value));
  if (parsed === null) {
    throw new Error("invalid Generic.StringMap");
  }
  return parsed;
}
export function normalizeGenericStringMapStrict(value: string): GenericStringMap {
  const normalized = convertGenericStringMapValue(backend.normalize(19, value));
  if (normalized === null) {
    throw new Error("invalid Generic.StringMap");
  }
  return normalized;
}
export function validateGenericStringMap(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(19, value);
}

export type GeoLocation = { lat: number; lon: number };
function convertGeoLocationValue(value: unknown | null): GeoLocation | null {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value === "string") {
    const parsed = parseJsonObject(value);
    if (parsed === null) {
      return null;
    }
    return parsed as GeoLocation;
  }
  return value as GeoLocation;
}
export function parseGeoLocation(value: unknown): GeoLocation | null {
  return convertGeoLocationValue(coerceValue(20, value));
}
export function normalizeGeoLocation(value: unknown): GeoLocation | null {
  return convertGeoLocationValue(coerceValue(20, value));
}
export function parseGeoLocationStrict(value: string): GeoLocation {
  const parsed = convertGeoLocationValue(backend.parse(20, value));
  if (parsed === null) {
    throw new Error("invalid Geo.Location");
  }
  return parsed;
}
export function normalizeGeoLocationStrict(value: string): GeoLocation {
  const normalized = convertGeoLocationValue(backend.normalize(20, value));
  if (normalized === null) {
    throw new Error("invalid Geo.Location");
  }
  return normalized;
}
export function validateGeoLocation(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(20, value);
}

export type IdentityName = string & { readonly __brand: "Identity.Name" };
function convertIdentityNameValue(value: unknown | null): IdentityName | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as IdentityName;
}
export function parseIdentityName(value: unknown): IdentityName | null {
  return convertIdentityNameValue(coerceValue(21, value));
}
export function normalizeIdentityName(value: unknown): IdentityName | null {
  return convertIdentityNameValue(coerceValue(21, value));
}
export function parseIdentityNameStrict(value: string): IdentityName {
  const parsed = convertIdentityNameValue(backend.parse(21, value));
  if (parsed === null) {
    throw new Error("invalid Identity.Name");
  }
  return parsed;
}
export function normalizeIdentityNameStrict(value: string): IdentityName {
  const normalized = convertIdentityNameValue(backend.normalize(21, value));
  if (normalized === null) {
    throw new Error("invalid Identity.Name");
  }
  return normalized;
}
export function validateIdentityName(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(21, value);
}

export type IdentitySlug = string & { readonly __brand: "Identity.Slug" };
function convertIdentitySlugValue(value: unknown | null): IdentitySlug | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as IdentitySlug;
}
export function parseIdentitySlug(value: unknown): IdentitySlug | null {
  return convertIdentitySlugValue(coerceValue(22, value));
}
export function normalizeIdentitySlug(value: unknown): IdentitySlug | null {
  return convertIdentitySlugValue(coerceValue(22, value));
}
export function parseIdentitySlugStrict(value: string): IdentitySlug {
  const parsed = convertIdentitySlugValue(backend.parse(22, value));
  if (parsed === null) {
    throw new Error("invalid Identity.Slug");
  }
  return parsed;
}
export function normalizeIdentitySlugStrict(value: string): IdentitySlug {
  const normalized = convertIdentitySlugValue(backend.normalize(22, value));
  if (normalized === null) {
    throw new Error("invalid Identity.Slug");
  }
  return normalized;
}
export function validateIdentitySlug(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(22, value);
}

export type IdentityUUID = string & { readonly __brand: "Identity.UUID" };
function convertIdentityUUIDValue(value: unknown | null): IdentityUUID | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as IdentityUUID;
}
export function parseIdentityUUID(value: unknown): IdentityUUID | null {
  return convertIdentityUUIDValue(coerceValue(23, value));
}
export function normalizeIdentityUUID(value: unknown): IdentityUUID | null {
  return convertIdentityUUIDValue(coerceValue(23, value));
}
export function parseIdentityUUIDStrict(value: string): IdentityUUID {
  const parsed = convertIdentityUUIDValue(backend.parse(23, value));
  if (parsed === null) {
    throw new Error("invalid Identity.UUID");
  }
  return parsed;
}
export function normalizeIdentityUUIDStrict(value: string): IdentityUUID {
  const normalized = convertIdentityUUIDValue(backend.normalize(23, value));
  if (normalized === null) {
    throw new Error("invalid Identity.UUID");
  }
  return normalized;
}
export function validateIdentityUUID(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(23, value);
}

export type IdentityUserID = string & { readonly __brand: "Identity.UserID" };
function convertIdentityUserIDValue(value: unknown | null): IdentityUserID | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as IdentityUserID;
}
export function parseIdentityUserID(value: unknown): IdentityUserID | null {
  return convertIdentityUserIDValue(coerceValue(24, value));
}
export function normalizeIdentityUserID(value: unknown): IdentityUserID | null {
  return convertIdentityUserIDValue(coerceValue(24, value));
}
export function parseIdentityUserIDStrict(value: string): IdentityUserID {
  const parsed = convertIdentityUserIDValue(backend.parse(24, value));
  if (parsed === null) {
    throw new Error("invalid Identity.UserID");
  }
  return parsed;
}
export function normalizeIdentityUserIDStrict(value: string): IdentityUserID {
  const normalized = convertIdentityUserIDValue(backend.normalize(24, value));
  if (normalized === null) {
    throw new Error("invalid Identity.UserID");
  }
  return normalized;
}
export function validateIdentityUserID(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(24, value);
}

export type LocalizationLocale = string & { readonly __brand: "Localization.Locale" };
function convertLocalizationLocaleValue(value: unknown | null): LocalizationLocale | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as LocalizationLocale;
}
export function parseLocalizationLocale(value: unknown): LocalizationLocale | null {
  return convertLocalizationLocaleValue(coerceValue(25, value));
}
export function normalizeLocalizationLocale(value: unknown): LocalizationLocale | null {
  return convertLocalizationLocaleValue(coerceValue(25, value));
}
export function parseLocalizationLocaleStrict(value: string): LocalizationLocale {
  const parsed = convertLocalizationLocaleValue(backend.parse(25, value));
  if (parsed === null) {
    throw new Error("invalid Localization.Locale");
  }
  return parsed;
}
export function normalizeLocalizationLocaleStrict(value: string): LocalizationLocale {
  const normalized = convertLocalizationLocaleValue(backend.normalize(25, value));
  if (normalized === null) {
    throw new Error("invalid Localization.Locale");
  }
  return normalized;
}
export function validateLocalizationLocale(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(25, value);
}

export type NetworkDomainName = string & { readonly __brand: "Network.DomainName" };
function convertNetworkDomainNameValue(value: unknown | null): NetworkDomainName | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as NetworkDomainName;
}
export function parseNetworkDomainName(value: unknown): NetworkDomainName | null {
  return convertNetworkDomainNameValue(coerceValue(26, value));
}
export function normalizeNetworkDomainName(value: unknown): NetworkDomainName | null {
  return convertNetworkDomainNameValue(coerceValue(26, value));
}
export function parseNetworkDomainNameStrict(value: string): NetworkDomainName {
  const parsed = convertNetworkDomainNameValue(backend.parse(26, value));
  if (parsed === null) {
    throw new Error("invalid Network.DomainName");
  }
  return parsed;
}
export function normalizeNetworkDomainNameStrict(value: string): NetworkDomainName {
  const normalized = convertNetworkDomainNameValue(backend.normalize(26, value));
  if (normalized === null) {
    throw new Error("invalid Network.DomainName");
  }
  return normalized;
}
export function validateNetworkDomainName(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(26, value);
}

export type NetworkIpAddress = string & { readonly __brand: "Network.IpAddress" };
function convertNetworkIpAddressValue(value: unknown | null): NetworkIpAddress | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as NetworkIpAddress;
}
export function parseNetworkIpAddress(value: unknown): NetworkIpAddress | null {
  return convertNetworkIpAddressValue(coerceValue(27, value));
}
export function normalizeNetworkIpAddress(value: unknown): NetworkIpAddress | null {
  return convertNetworkIpAddressValue(coerceValue(27, value));
}
export function parseNetworkIpAddressStrict(value: string): NetworkIpAddress {
  const parsed = convertNetworkIpAddressValue(backend.parse(27, value));
  if (parsed === null) {
    throw new Error("invalid Network.IpAddress");
  }
  return parsed;
}
export function normalizeNetworkIpAddressStrict(value: string): NetworkIpAddress {
  const normalized = convertNetworkIpAddressValue(backend.normalize(27, value));
  if (normalized === null) {
    throw new Error("invalid Network.IpAddress");
  }
  return normalized;
}
export function validateNetworkIpAddress(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(27, value);
}

export type NetworkUri = string & { readonly __brand: "Network.Uri" };
function convertNetworkUriValue(value: unknown | null): NetworkUri | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as NetworkUri;
}
export function parseNetworkUri(value: unknown): NetworkUri | null {
  return convertNetworkUriValue(coerceValue(28, value));
}
export function normalizeNetworkUri(value: unknown): NetworkUri | null {
  return convertNetworkUriValue(coerceValue(28, value));
}
export function parseNetworkUriStrict(value: string): NetworkUri {
  const parsed = convertNetworkUriValue(backend.parse(28, value));
  if (parsed === null) {
    throw new Error("invalid Network.Uri");
  }
  return parsed;
}
export function normalizeNetworkUriStrict(value: string): NetworkUri {
  const normalized = convertNetworkUriValue(backend.normalize(28, value));
  if (normalized === null) {
    throw new Error("invalid Network.Uri");
  }
  return normalized;
}
export function validateNetworkUri(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(28, value);
}

export type NetworkUrl = string & { readonly __brand: "Network.Url" };
function convertNetworkUrlValue(value: unknown | null): NetworkUrl | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as NetworkUrl;
}
export function parseNetworkUrl(value: unknown): NetworkUrl | null {
  return convertNetworkUrlValue(coerceValue(29, value));
}
export function normalizeNetworkUrl(value: unknown): NetworkUrl | null {
  return convertNetworkUrlValue(coerceValue(29, value));
}
export function parseNetworkUrlStrict(value: string): NetworkUrl {
  const parsed = convertNetworkUrlValue(backend.parse(29, value));
  if (parsed === null) {
    throw new Error("invalid Network.Url");
  }
  return parsed;
}
export function normalizeNetworkUrlStrict(value: string): NetworkUrl {
  const normalized = convertNetworkUrlValue(backend.normalize(29, value));
  if (normalized === null) {
    throw new Error("invalid Network.Url");
  }
  return normalized;
}
export function validateNetworkUrl(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(29, value);
}

export type TemporalCronExpression = string & { readonly __brand: "Temporal.CronExpression" };
function convertTemporalCronExpressionValue(value: unknown | null): TemporalCronExpression | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalCronExpression;
}
export function parseTemporalCronExpression(value: unknown): TemporalCronExpression | null {
  return convertTemporalCronExpressionValue(coerceValue(39, value));
}
export function normalizeTemporalCronExpression(value: unknown): TemporalCronExpression | null {
  return convertTemporalCronExpressionValue(coerceValue(39, value));
}
export function parseTemporalCronExpressionStrict(value: string): TemporalCronExpression {
  const parsed = convertTemporalCronExpressionValue(backend.parse(39, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.CronExpression");
  }
  return parsed;
}
export function normalizeTemporalCronExpressionStrict(value: string): TemporalCronExpression {
  const normalized = convertTemporalCronExpressionValue(backend.normalize(39, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.CronExpression");
  }
  return normalized;
}
export function validateTemporalCronExpression(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(39, value);
}

export type TemporalDate = string & { readonly __brand: "Temporal.Date" };
function convertTemporalDateValue(value: unknown | null): TemporalDate | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalDate;
}
export function parseTemporalDate(value: unknown): TemporalDate | null {
  return convertTemporalDateValue(coerceValue(40, value));
}
export function normalizeTemporalDate(value: unknown): TemporalDate | null {
  return convertTemporalDateValue(coerceValue(40, value));
}
export function parseTemporalDateStrict(value: string): TemporalDate {
  const parsed = convertTemporalDateValue(backend.parse(40, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Date");
  }
  return parsed;
}
export function normalizeTemporalDateStrict(value: string): TemporalDate {
  const normalized = convertTemporalDateValue(backend.normalize(40, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Date");
  }
  return normalized;
}
export function validateTemporalDate(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(40, value);
}

export type TemporalDateTime = JSDate;
function convertTemporalDateTimeValue(value: unknown | null): TemporalDateTime | null {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value !== "string") {
    return null;
  }
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return null;
  }
  return date as unknown as TemporalDateTime;
}
export function parseTemporalDateTime(value: unknown): TemporalDateTime | null {
  return convertTemporalDateTimeValue(coerceValue(41, value));
}
export function normalizeTemporalDateTime(value: unknown): TemporalDateTime | null {
  return convertTemporalDateTimeValue(coerceValue(41, value));
}
export function parseTemporalDateTimeStrict(value: string): TemporalDateTime {
  const parsed = convertTemporalDateTimeValue(backend.parse(41, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.DateTime");
  }
  return parsed;
}
export function normalizeTemporalDateTimeStrict(value: string): TemporalDateTime {
  const normalized = convertTemporalDateTimeValue(backend.normalize(41, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.DateTime");
  }
  return normalized;
}
export function validateTemporalDateTime(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(41, value);
}

export type TemporalDuration = string & { readonly __brand: "Temporal.Duration" };
function convertTemporalDurationValue(value: unknown | null): TemporalDuration | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalDuration;
}
export function parseTemporalDuration(value: unknown): TemporalDuration | null {
  return convertTemporalDurationValue(coerceValue(42, value));
}
export function normalizeTemporalDuration(value: unknown): TemporalDuration | null {
  return convertTemporalDurationValue(coerceValue(42, value));
}
export function parseTemporalDurationStrict(value: string): TemporalDuration {
  const parsed = convertTemporalDurationValue(backend.parse(42, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Duration");
  }
  return parsed;
}
export function normalizeTemporalDurationStrict(value: string): TemporalDuration {
  const normalized = convertTemporalDurationValue(backend.normalize(42, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Duration");
  }
  return normalized;
}
export function validateTemporalDuration(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(42, value);
}

export type TemporalMilliseconds = number;
function convertTemporalMillisecondsValue(value: unknown | null): TemporalMilliseconds | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalMilliseconds;
}
export function parseTemporalMilliseconds(value: unknown): TemporalMilliseconds | null {
  return convertTemporalMillisecondsValue(coerceValue(43, value));
}
export function normalizeTemporalMilliseconds(value: unknown): TemporalMilliseconds | null {
  return convertTemporalMillisecondsValue(coerceValue(43, value));
}
export function parseTemporalMillisecondsStrict(value: string): TemporalMilliseconds {
  const parsed = convertTemporalMillisecondsValue(backend.parse(43, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Milliseconds");
  }
  return parsed;
}
export function normalizeTemporalMillisecondsStrict(value: string): TemporalMilliseconds {
  const normalized = convertTemporalMillisecondsValue(backend.normalize(43, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Milliseconds");
  }
  return normalized;
}
export function validateTemporalMilliseconds(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(43, value);
}

export type TemporalMonth = string & { readonly __brand: "Temporal.Month" };
function convertTemporalMonthValue(value: unknown | null): TemporalMonth | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalMonth;
}
export function parseTemporalMonth(value: unknown): TemporalMonth | null {
  return convertTemporalMonthValue(coerceValue(44, value));
}
export function normalizeTemporalMonth(value: unknown): TemporalMonth | null {
  return convertTemporalMonthValue(coerceValue(44, value));
}
export function parseTemporalMonthStrict(value: string): TemporalMonth {
  const parsed = convertTemporalMonthValue(backend.parse(44, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Month");
  }
  return parsed;
}
export function normalizeTemporalMonthStrict(value: string): TemporalMonth {
  const normalized = convertTemporalMonthValue(backend.normalize(44, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Month");
  }
  return normalized;
}
export function validateTemporalMonth(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(44, value);
}

export type TemporalQuarter = string & { readonly __brand: "Temporal.Quarter" };
function convertTemporalQuarterValue(value: unknown | null): TemporalQuarter | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalQuarter;
}
export function parseTemporalQuarter(value: unknown): TemporalQuarter | null {
  return convertTemporalQuarterValue(coerceValue(45, value));
}
export function normalizeTemporalQuarter(value: unknown): TemporalQuarter | null {
  return convertTemporalQuarterValue(coerceValue(45, value));
}
export function parseTemporalQuarterStrict(value: string): TemporalQuarter {
  const parsed = convertTemporalQuarterValue(backend.parse(45, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Quarter");
  }
  return parsed;
}
export function normalizeTemporalQuarterStrict(value: string): TemporalQuarter {
  const normalized = convertTemporalQuarterValue(backend.normalize(45, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Quarter");
  }
  return normalized;
}
export function validateTemporalQuarter(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(45, value);
}

export type TemporalQuarterYear = string & { readonly __brand: "Temporal.QuarterYear" };
function convertTemporalQuarterYearValue(value: unknown | null): TemporalQuarterYear | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalQuarterYear;
}
export function parseTemporalQuarterYear(value: unknown): TemporalQuarterYear | null {
  return convertTemporalQuarterYearValue(coerceValue(46, value));
}
export function normalizeTemporalQuarterYear(value: unknown): TemporalQuarterYear | null {
  return convertTemporalQuarterYearValue(coerceValue(46, value));
}
export function parseTemporalQuarterYearStrict(value: string): TemporalQuarterYear {
  const parsed = convertTemporalQuarterYearValue(backend.parse(46, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.QuarterYear");
  }
  return parsed;
}
export function normalizeTemporalQuarterYearStrict(value: string): TemporalQuarterYear {
  const normalized = convertTemporalQuarterYearValue(backend.normalize(46, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.QuarterYear");
  }
  return normalized;
}
export function validateTemporalQuarterYear(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(46, value);
}

export type TemporalTime = string & { readonly __brand: "Temporal.Time" };
function convertTemporalTimeValue(value: unknown | null): TemporalTime | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalTime;
}
export function parseTemporalTime(value: unknown): TemporalTime | null {
  return convertTemporalTimeValue(coerceValue(47, value));
}
export function normalizeTemporalTime(value: unknown): TemporalTime | null {
  return convertTemporalTimeValue(coerceValue(47, value));
}
export function parseTemporalTimeStrict(value: string): TemporalTime {
  const parsed = convertTemporalTimeValue(backend.parse(47, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Time");
  }
  return parsed;
}
export function normalizeTemporalTimeStrict(value: string): TemporalTime {
  const normalized = convertTemporalTimeValue(backend.normalize(47, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Time");
  }
  return normalized;
}
export function validateTemporalTime(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(47, value);
}

export type TemporalTimeZone = string & { readonly __brand: "Temporal.TimeZone" };
function convertTemporalTimeZoneValue(value: unknown | null): TemporalTimeZone | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalTimeZone;
}
export function parseTemporalTimeZone(value: unknown): TemporalTimeZone | null {
  return convertTemporalTimeZoneValue(coerceValue(48, value));
}
export function normalizeTemporalTimeZone(value: unknown): TemporalTimeZone | null {
  return convertTemporalTimeZoneValue(coerceValue(48, value));
}
export function parseTemporalTimeZoneStrict(value: string): TemporalTimeZone {
  const parsed = convertTemporalTimeZoneValue(backend.parse(48, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.TimeZone");
  }
  return parsed;
}
export function normalizeTemporalTimeZoneStrict(value: string): TemporalTimeZone {
  const normalized = convertTemporalTimeZoneValue(backend.normalize(48, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.TimeZone");
  }
  return normalized;
}
export function validateTemporalTimeZone(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(48, value);
}

export type TemporalYear = string & { readonly __brand: "Temporal.Year" };
function convertTemporalYearValue(value: unknown | null): TemporalYear | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalYear;
}
export function parseTemporalYear(value: unknown): TemporalYear | null {
  return convertTemporalYearValue(coerceValue(49, value));
}
export function normalizeTemporalYear(value: unknown): TemporalYear | null {
  return convertTemporalYearValue(coerceValue(49, value));
}
export function parseTemporalYearStrict(value: string): TemporalYear {
  const parsed = convertTemporalYearValue(backend.parse(49, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Year");
  }
  return parsed;
}
export function normalizeTemporalYearStrict(value: string): TemporalYear {
  const normalized = convertTemporalYearValue(backend.normalize(49, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Year");
  }
  return normalized;
}
export function validateTemporalYear(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(49, value);
}

export type TextMarkdown = string & { readonly __brand: "Text.Markdown" };
function convertTextMarkdownValue(value: unknown | null): TextMarkdown | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TextMarkdown;
}
export function parseTextMarkdown(value: unknown): TextMarkdown | null {
  return convertTextMarkdownValue(coerceValue(50, value));
}
export function normalizeTextMarkdown(value: unknown): TextMarkdown | null {
  return convertTextMarkdownValue(coerceValue(50, value));
}
export function parseTextMarkdownStrict(value: string): TextMarkdown {
  const parsed = convertTextMarkdownValue(backend.parse(50, value));
  if (parsed === null) {
    throw new Error("invalid Text.Markdown");
  }
  return parsed;
}
export function normalizeTextMarkdownStrict(value: string): TextMarkdown {
  const normalized = convertTextMarkdownValue(backend.normalize(50, value));
  if (normalized === null) {
    throw new Error("invalid Text.Markdown");
  }
  return normalized;
}
export function validateTextMarkdown(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(50, value);
}

export type TemporalSeconds = number;
function convertTemporalSecondsValue(value: unknown | null): TemporalSeconds | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalSeconds;
}
export function parseTemporalSeconds(value: unknown): TemporalSeconds | null {
  return convertTemporalSecondsValue(coerceValue(52, value));
}
export function normalizeTemporalSeconds(value: unknown): TemporalSeconds | null {
  return convertTemporalSecondsValue(coerceValue(52, value));
}
export function parseTemporalSecondsStrict(value: string): TemporalSeconds {
  const parsed = convertTemporalSecondsValue(backend.parse(52, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Seconds");
  }
  return parsed;
}
export function normalizeTemporalSecondsStrict(value: string): TemporalSeconds {
  const normalized = convertTemporalSecondsValue(backend.normalize(52, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Seconds");
  }
  return normalized;
}
export function validateTemporalSeconds(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(52, value);
}

export type TemporalMinutes = number;
function convertTemporalMinutesValue(value: unknown | null): TemporalMinutes | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalMinutes;
}
export function parseTemporalMinutes(value: unknown): TemporalMinutes | null {
  return convertTemporalMinutesValue(coerceValue(53, value));
}
export function normalizeTemporalMinutes(value: unknown): TemporalMinutes | null {
  return convertTemporalMinutesValue(coerceValue(53, value));
}
export function parseTemporalMinutesStrict(value: string): TemporalMinutes {
  const parsed = convertTemporalMinutesValue(backend.parse(53, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Minutes");
  }
  return parsed;
}
export function normalizeTemporalMinutesStrict(value: string): TemporalMinutes {
  const normalized = convertTemporalMinutesValue(backend.normalize(53, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Minutes");
  }
  return normalized;
}
export function validateTemporalMinutes(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(53, value);
}

export type TemporalHours = number;
function convertTemporalHoursValue(value: unknown | null): TemporalHours | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalHours;
}
export function parseTemporalHours(value: unknown): TemporalHours | null {
  return convertTemporalHoursValue(coerceValue(54, value));
}
export function normalizeTemporalHours(value: unknown): TemporalHours | null {
  return convertTemporalHoursValue(coerceValue(54, value));
}
export function parseTemporalHoursStrict(value: string): TemporalHours {
  const parsed = convertTemporalHoursValue(backend.parse(54, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Hours");
  }
  return parsed;
}
export function normalizeTemporalHoursStrict(value: string): TemporalHours {
  const normalized = convertTemporalHoursValue(backend.normalize(54, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Hours");
  }
  return normalized;
}
export function validateTemporalHours(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(54, value);
}

export type TemporalDays = number;
function convertTemporalDaysValue(value: unknown | null): TemporalDays | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalDays;
}
export function parseTemporalDays(value: unknown): TemporalDays | null {
  return convertTemporalDaysValue(coerceValue(55, value));
}
export function normalizeTemporalDays(value: unknown): TemporalDays | null {
  return convertTemporalDaysValue(coerceValue(55, value));
}
export function parseTemporalDaysStrict(value: string): TemporalDays {
  const parsed = convertTemporalDaysValue(backend.parse(55, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Days");
  }
  return parsed;
}
export function normalizeTemporalDaysStrict(value: string): TemporalDays {
  const normalized = convertTemporalDaysValue(backend.normalize(55, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Days");
  }
  return normalized;
}
export function validateTemporalDays(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(55, value);
}

export type TextSql = string & { readonly __brand: "Text.Sql" };
function convertTextSqlValue(value: unknown | null): TextSql | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TextSql;
}
export function parseTextSql(value: unknown): TextSql | null {
  return convertTextSqlValue(coerceValue(56, value));
}
export function normalizeTextSql(value: unknown): TextSql | null {
  return convertTextSqlValue(coerceValue(56, value));
}
export function parseTextSqlStrict(value: string): TextSql {
  const parsed = convertTextSqlValue(backend.parse(56, value));
  if (parsed === null) {
    throw new Error("invalid Text.Sql");
  }
  return parsed;
}
export function normalizeTextSqlStrict(value: string): TextSql {
  const normalized = convertTextSqlValue(backend.normalize(56, value));
  if (normalized === null) {
    throw new Error("invalid Text.Sql");
  }
  return normalized;
}
export function validateTextSql(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(56, value);
}

export type CryptoSHA256 = string & { readonly __brand: "Crypto.SHA256" };
function convertCryptoSHA256Value(value: unknown | null): CryptoSHA256 | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as CryptoSHA256;
}
export function parseCryptoSHA256(value: unknown): CryptoSHA256 | null {
  return convertCryptoSHA256Value(coerceValue(58, value));
}
export function normalizeCryptoSHA256(value: unknown): CryptoSHA256 | null {
  return convertCryptoSHA256Value(coerceValue(58, value));
}
export function parseCryptoSHA256Strict(value: string): CryptoSHA256 {
  const parsed = convertCryptoSHA256Value(backend.parse(58, value));
  if (parsed === null) {
    throw new Error("invalid Crypto.SHA256");
  }
  return parsed;
}
export function normalizeCryptoSHA256Strict(value: string): CryptoSHA256 {
  const normalized = convertCryptoSHA256Value(backend.normalize(58, value));
  if (normalized === null) {
    throw new Error("invalid Crypto.SHA256");
  }
  return normalized;
}
export function validateCryptoSHA256(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(58, value);
}

export type NetworkDnsLabel = string & { readonly __brand: "Network.DnsLabel" };
function convertNetworkDnsLabelValue(value: unknown | null): NetworkDnsLabel | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as NetworkDnsLabel;
}
export function parseNetworkDnsLabel(value: unknown): NetworkDnsLabel | null {
  return convertNetworkDnsLabelValue(coerceValue(59, value));
}
export function normalizeNetworkDnsLabel(value: unknown): NetworkDnsLabel | null {
  return convertNetworkDnsLabelValue(coerceValue(59, value));
}
export function parseNetworkDnsLabelStrict(value: string): NetworkDnsLabel {
  const parsed = convertNetworkDnsLabelValue(backend.parse(59, value));
  if (parsed === null) {
    throw new Error("invalid Network.DnsLabel");
  }
  return parsed;
}
export function normalizeNetworkDnsLabelStrict(value: string): NetworkDnsLabel {
  const normalized = convertNetworkDnsLabelValue(backend.normalize(59, value));
  if (normalized === null) {
    throw new Error("invalid Network.DnsLabel");
  }
  return normalized;
}
export function validateNetworkDnsLabel(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(59, value);
}

export type TemporalRecurrenceRule = string & { readonly __brand: "Temporal.RecurrenceRule" };
function convertTemporalRecurrenceRuleValue(value: unknown | null): TemporalRecurrenceRule | null {
  if (value === null || value === undefined) {
    return null;
  }
  return value as TemporalRecurrenceRule;
}
export function parseTemporalRecurrenceRule(value: unknown): TemporalRecurrenceRule | null {
  return convertTemporalRecurrenceRuleValue(coerceValue(60, value));
}
export function normalizeTemporalRecurrenceRule(value: unknown): TemporalRecurrenceRule | null {
  return convertTemporalRecurrenceRuleValue(coerceValue(60, value));
}
export function parseTemporalRecurrenceRuleStrict(value: string): TemporalRecurrenceRule {
  const parsed = convertTemporalRecurrenceRuleValue(backend.parse(60, value));
  if (parsed === null) {
    throw new Error("invalid Temporal.RecurrenceRule");
  }
  return parsed;
}
export function normalizeTemporalRecurrenceRuleStrict(value: string): TemporalRecurrenceRule {
  const normalized = convertTemporalRecurrenceRuleValue(backend.normalize(60, value));
  if (normalized === null) {
    throw new Error("invalid Temporal.RecurrenceRule");
  }
  return normalized;
}
export function validateTemporalRecurrenceRule(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend(60, value);
}



export const SCALAR_METADATA: ScalarMetadata[] = [
  {
    canonicalName: "Auth.JWT",
    symbol: "AuthJWT",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^[A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+$",
    hasValidator: true,
    examples: [],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Auth.Password",
    symbol: "AuthPassword",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 128,
    minLength: 8,
    pattern: "",
    hasValidator: true,
    examples: [],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Contact.Email",
    symbol: "ContactEmail",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 255,
    minLength: 0,
    pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
    hasValidator: true,
    examples: ["test@example.com"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Contact.PhoneNumber",
    symbol: "ContactPhoneNumber",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 16,
    minLength: 0,
    pattern: "^\\+[1-9]\\d{1,14}$",
    hasValidator: true,
    examples: ["+1234567890"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Crypto.RSAPrivateKey",
    symbol: "CryptoRSAPrivateKey",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^-----BEGIN (RSA )?PRIVATE KEY-----[\\s\\S]*-----END (RSA )?PRIVATE KEY-----\\s*$",
    hasValidator: true,
    examples: ["-----BEGIN RSA PRIVATE KEY----------END RSA PRIVATE KEY-----"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Crypto.RSAPublicKey",
    symbol: "CryptoRSAPublicKey",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^-----BEGIN PUBLIC KEY-----[\\s\\S]*-----END PUBLIC KEY-----\\s*$",
    hasValidator: true,
    examples: ["-----BEGIN PUBLIC KEY----------END PUBLIC KEY-----"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Design.Color",
    symbol: "DesignColor",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 9,
    minLength: 0,
    pattern: "^#[0-9A-Fa-f]{8}$",
    hasValidator: true,
    examples: ["#FF5733FF"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Embedding.Vector",
    symbol: "EmbeddingVector",
    primitive: "String",
    tsType: "number[]",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: [],
    comparabilityClass: null,
    isSortable: false,
  },
  {
    canonicalName: "File.SizeBytes",
    symbol: "FileSizeBytes",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["204800"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Finance.Money",
    symbol: "FinanceMoney",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["1000"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Generic.Int64",
    symbol: "GenericInt64",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["1000"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Generic.JSON",
    symbol: "GenericJSON",
    primitive: "String",
    tsType: "Record<string, any>",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: [],
    comparabilityClass: null,
    isSortable: false,
  },
  {
    canonicalName: "Generic.Probability",
    symbol: "GenericProbability",
    primitive: "Float",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["0.75"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Generic.StringMap",
    symbol: "GenericStringMap",
    primitive: "String",
    tsType: "Record<string, string>",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: [],
    comparabilityClass: null,
    isSortable: false,
  },
  {
    canonicalName: "Geo.Location",
    symbol: "GeoLocation",
    primitive: "String",
    tsType: "{ lat: number; lon: number }",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^-?\\d+(\\.\\d+)?,-?\\d+(\\.\\d+)?$",
    hasValidator: false,
    examples: ["37.7749,-122.4194"],
    comparabilityClass: null,
    isSortable: false,
  },
  {
    canonicalName: "Identity.Name",
    symbol: "IdentityName",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 80,
    minLength: 2,
    pattern: "",
    hasValidator: true,
    examples: ["A Name"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Identity.Slug",
    symbol: "IdentitySlug",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 255,
    minLength: 1,
    pattern: "^[a-z0-9]+(?:[-_][a-z0-9]+)*$",
    hasValidator: true,
    examples: ["acme-corp"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Identity.UUID",
    symbol: "IdentityUUID",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^([0-9A-Za-z]{1,22}|[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$",
    hasValidator: true,
    examples: ["YQJpYwUwvbaLOwTUr4thA"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Identity.UserID",
    symbol: "IdentityUserID",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^[0-9A-Za-z]{1,22}$",
    hasValidator: true,
    examples: ["YQJpYwUwvbaLOwTUr4thA"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Localization.Locale",
    symbol: "LocalizationLocale",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 35,
    minLength: 0,
    pattern: "^[a-z]{2,3}(-[A-Z][a-z]{3})?(-[A-Z]{2})?$",
    hasValidator: true,
    examples: ["en-US"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Network.DomainName",
    symbol: "NetworkDomainName",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 255,
    minLength: 0,
    pattern: "^[a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?(\\.[a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?)*$",
    hasValidator: true,
    examples: ["example.com"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Network.IpAddress",
    symbol: "NetworkIpAddress",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 45,
    minLength: 0,
    pattern: "^(((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)|([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))$",
    hasValidator: true,
    examples: ["192.168.1.1"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Network.Uri",
    symbol: "NetworkUri",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 4096,
    minLength: 0,
    pattern: "^[a-zA-Z][a-zA-Z0-9+.-]*://[^\\s]+$",
    hasValidator: true,
    examples: ["postgres://user:pass@localhost:5432/dbname"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Network.Url",
    symbol: "NetworkUrl",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 2048,
    minLength: 0,
    pattern: "^https?://[\\w\\-\\{\\}]+(\\.[\\w\\-\\{\\}]+)+([:/?#][\\w\\-\\._~:/?#\\[\\]@!\\$&'\\(\\)\\*\\+,;=\\{\\}%]*)?$",
    hasValidator: true,
    examples: ["https://www.example.com/example/path"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.CronExpression",
    symbol: "TemporalCronExpression",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 100,
    minLength: 0,
    pattern: "^[0-9*,\\-\\/]+\\s+[0-9*,\\-\\/]+\\s+[0-9*,\\-\\/?]+\\s+[0-9*,\\-\\/A-Za-z]+\\s+[0-9*,\\-\\/A-Za-z]+$",
    hasValidator: true,
    examples: ["0 9 * * MON-FRI"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Date",
    symbol: "TemporalDate",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 40,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["2025-01-01"],
    comparabilityClass: "temporal_instant",
    isSortable: true,
  },
  {
    canonicalName: "Temporal.DateTime",
    symbol: "TemporalDateTime",
    primitive: "String",
    tsType: "JSDate",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["2025-01-01T12:00:00Z"],
    comparabilityClass: "temporal_instant",
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Duration",
    symbol: "TemporalDuration",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 32,
    minLength: 0,
    pattern: "^(\\d+(\\.\\d+)?(ns|us|µs|ms|s|m|h))+$",
    hasValidator: true,
    examples: ["30s"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Milliseconds",
    symbol: "TemporalMilliseconds",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["1000"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Month",
    symbol: "TemporalMonth",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 9,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["02"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Quarter",
    symbol: "TemporalQuarter",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 2,
    minLength: 2,
    pattern: "^[Qq][1-4]$",
    hasValidator: true,
    examples: ["Q1"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.QuarterYear",
    symbol: "TemporalQuarterYear",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 8,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["2025-Q1"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Time",
    symbol: "TemporalTime",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "^(?:(?:[01][0-9]|2[0-3]):[0-5][0-9](?::[0-5][0-9])?|(?:0?[1-9]|1[0-2]):[0-5][0-9](?::[0-5][0-9])?\\s?[AaPp][Mm])$",
    hasValidator: true,
    examples: ["12:25"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.TimeZone",
    symbol: "TemporalTimeZone",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 100,
    minLength: 0,
    pattern: "^(?:UTC|[A-Za-z]+/[A-Za-z_/]+)$",
    hasValidator: true,
    examples: ["America/New_York", "UTC", "Etc/UTC"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Year",
    symbol: "TemporalYear",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 4,
    minLength: 4,
    pattern: "^[1-9]\\d{3}$",
    hasValidator: true,
    examples: ["2025"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Text.Markdown",
    symbol: "TextMarkdown",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 1,
    pattern: "^[\\s\\S]*$",
    hasValidator: true,
    examples: ["## Hello, World!"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Seconds",
    symbol: "TemporalSeconds",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["60"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Minutes",
    symbol: "TemporalMinutes",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["5"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Hours",
    symbol: "TemporalHours",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["2"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.Days",
    symbol: "TemporalDays",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["7"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Text.Sql",
    symbol: "TextSql",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 0,
    minLength: 1,
    pattern: "^[\\s\\S]*$",
    hasValidator: true,
    examples: ["SELECT 1"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Crypto.SHA256",
    symbol: "CryptoSHA256",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 64,
    minLength: 64,
    pattern: "^[0-9a-f]{64}$",
    hasValidator: true,
    examples: ["0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Network.DnsLabel",
    symbol: "NetworkDnsLabel",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 63,
    minLength: 0,
    pattern: "^[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?$",
    hasValidator: true,
    examples: ["mycompany"],
    comparabilityClass: null,
    isSortable: true,
  },
  {
    canonicalName: "Temporal.RecurrenceRule",
    symbol: "TemporalRecurrenceRule",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 512,
    minLength: 6,
    pattern: "",
    hasValidator: true,
    examples: ["FREQ=WEEKLY;BYMINUTE=0;BYHOUR=9;BYDAY=MO,WE,FR"],
    comparabilityClass: null,
    isSortable: true,
  },
];

export const SCALAR_METADATA_BY_CANONICAL: Record<string, ScalarMetadata> =
  SCALAR_METADATA.reduce<Record<string, ScalarMetadata>>((acc, meta) => {
    acc[meta.canonicalName] = meta;
    return acc;
  }, {});

// Maps an alias scalar's canonical name to its target's. `alias_of` means one
// implementation under two ids, so the TARGET owns the comparability class and
// the alias inherits it. comparableWith resolves through this before it reads a
// class; reading the alias row's own class instead breaks transitivity one hop
// out (the alias pair itself still answers true because identity fires first,
// so the inconsistency hides where it was made). Module-local on purpose: the
// predicate is the contract, the table is its implementation.
const SCALAR_ALIAS_TARGETS: Record<string, string> = {
  "Identity.UserID": "Identity.UUID",
};

/**
 * Reports whether comparing or joining values of the two named scalars is
 * meaningful. Mirrors `ScalarDef::comparable_with` in the Rust core: aliases
 * resolve first, a scalar is always comparable with itself, and two distinct
 * scalars are comparable only when both declare the same named class.
 *
 * Do NOT reimplement this as `comparabilityClass` equality. Almost every
 * scalar carries `null`, so field equality makes
 * the entire catalog mutually comparable, `Contact.Email` against
 * `Contact.PhoneNumber` included, which is the exact comparison the relation
 * exists to reject.
 *
 * Fails closed on a name with no metadata row: an unknown scalar, and a
 * scalar whose def sets `metadata_omit` and so has no comparability answer in
 * this binding, both report false even against themselves. That matches the
 * `isSortable` allowlist, which answers "no" to a shape nobody declared.
 */
export function comparableWith(a: string, b: string): boolean {
  // Own-property lookups only. Both maps are plain objects, so a bare index
  // on an inherited name ("constructor", "toString", "__proto__") returns an
  // Object.prototype member instead of undefined, and the fail-closed
  // contract breaks: "constructor" would compare equal to itself, and two
  // distinct inherited names would compare equal through an undefined class.
  let left = a;
  if (Object.prototype.hasOwnProperty.call(SCALAR_ALIAS_TARGETS, a)) {
    left = SCALAR_ALIAS_TARGETS[a];
  }
  let right = b;
  if (Object.prototype.hasOwnProperty.call(SCALAR_ALIAS_TARGETS, b)) {
    right = SCALAR_ALIAS_TARGETS[b];
  }
  if (
    !Object.prototype.hasOwnProperty.call(SCALAR_METADATA_BY_CANONICAL, left) ||
    !Object.prototype.hasOwnProperty.call(SCALAR_METADATA_BY_CANONICAL, right)
  ) {
    return false;
  }
  const metaLeft = SCALAR_METADATA_BY_CANONICAL[left];
  const metaRight = SCALAR_METADATA_BY_CANONICAL[right];
  if (left === right) {
    return true;
  }
  return (
    metaLeft.comparabilityClass !== null &&
    metaLeft.comparabilityClass === metaRight.comparabilityClass
  );
}
