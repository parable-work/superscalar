// @generated; do not edit
import { backend } from "./backend";
import { isJSONValue } from "./json-value";
import type { JSONValue } from "./json-value";
import type { ScalarValidationResult, ValidationError } from "./validation";
export { isJSONValue } from "./json-value";
export type { JSONValue } from "./json-value";
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
  /** The def's `case_insensitive` flag. */
  caseInsensitive: boolean;
  /** The values the def reserves, empty when it has none. */
  reservedWords: string[];
  /** The def's `reserved_words_case_insensitive` flag. */
  reservedWordsCaseInsensitive: boolean;
  /** The def's `reserved_words_match_partial` flag. */
  reservedWordsMatchPartial: boolean;
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

/**
 * Every canonical scalar name in the registry, sorted. The name is a scalar's
 * identity: every call into the core passes it.
 */
export const VALID_SCALARS: readonly string[] = [
  "AgentSkill.Name",
  "Auth.JWT",
  "Auth.Password",
  "Contact.Email",
  "Contact.PhoneNumber",
  "Crypto.RSAPrivateKey",
  "Crypto.RSAPublicKey",
  "Crypto.SHA256",
  "Design.Color",
  "Embedding.Vector",
  "File.SizeBytes",
  "Finance.Money",
  "Generic.Int64",
  "Generic.JSON",
  "Generic.Probability",
  "Generic.StringMap",
  "Geo.Location",
  "Git.PathPattern",
  "Identity.Name",
  "Identity.Slug",
  "Identity.UUID",
  "Identity.UserID",
  "Localization.Locale",
  "Network.DnsLabel",
  "Network.DomainName",
  "Network.IpAddress",
  "Network.Uri",
  "Network.Url",
  "Ordering.Rank",
  "Temporal.CronExpression",
  "Temporal.Date",
  "Temporal.DateTime",
  "Temporal.Days",
  "Temporal.Duration",
  "Temporal.Hours",
  "Temporal.Milliseconds",
  "Temporal.Minutes",
  "Temporal.Month",
  "Temporal.Quarter",
  "Temporal.QuarterYear",
  "Temporal.RecurrenceRule",
  "Temporal.Seconds",
  "Temporal.Time",
  "Temporal.TimeZone",
  "Temporal.Year",
  "Text.Markdown",
  "Text.Sql",
  "Version.SemVer",
];

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

function coerceLenient(scalar: string, value: unknown): LenientCoerceResult {
  try {
    const encoded = JSON.stringify(value);
    const jsonIn = encoded === undefined ? "null" : encoded;
    const decoded = JSON.parse(backend.coerceLenient(scalar, jsonIn)) as Partial<LenientCoerceResult>;
    return {
      value: decoded.value === undefined ? null : decoded.value,
      error: decoded.error === undefined ? null : decoded.error,
    };
  } catch (error) {
    return { value: null, error: scalarErrorFromUnknown(error) };
  }
}

function coerceValue(scalar: string, value: unknown): unknown | null {
  const result = coerceLenient(scalar, value);
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

function validateWithBackend(scalar: string, value: unknown | null | undefined): ScalarValidationResult {
  if (value === null || value === undefined || value === "") {
    return [true, null];
  }
  const result = coerceLenient(scalar, value);
  if (!result.error) {
    return [true, null];
  }
  return [false, [validationErrorFromLenient(result.error)]];
}

function canonicalizeJSONValue(
  scalar: string,
  value: unknown,
  operation: "parse" | "normalize",
): JSONValue | undefined {
  if (!isJSONValue(value)) {
    return undefined;
  }
  try {
    const encoded = JSON.stringify(value);
    if (encoded === undefined) {
      return undefined;
    }
    const canonical = operation === "parse"
      ? backend.parse(scalar, encoded)
      : backend.normalize(scalar, encoded);
    return JSON.parse(canonical) as JSONValue;
  } catch {
    return undefined;
  }
}

function validateJSONValueWithBackend(scalar: string, value: unknown): ScalarValidationResult {
  // Scalar validators treat undefined as an absent optional field. Required
  // generated field validators reject it before reaching this function.
  if (value === undefined) {
    return [true, null];
  }
  if (!isJSONValue(value)) {
    return [false, [{ validator: "type", message: "must be a valid JSON value" }]];
  }
  try {
    const encoded = JSON.stringify(value);
    if (encoded === undefined) {
      return [false, [{ validator: "type", message: "must be a valid JSON value" }]];
    }
    backend.validate(scalar, encoded);
    return [true, null];
  } catch (error) {
    return [false, [validationErrorFromLenient(scalarErrorFromUnknown(error))]];
  }
}

export type AgentSkillName = string & { readonly __brand: "AgentSkill.Name" };
function convertAgentSkillNameValue(value: unknown | null): AgentSkillName | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as AgentSkillName;
}
export function parseAgentSkillName(value: unknown): AgentSkillName | null {
  return convertAgentSkillNameValue(coerceValue("AgentSkill.Name", value));
}
export function normalizeAgentSkillName(value: unknown): AgentSkillName | null {
  return convertAgentSkillNameValue(coerceValue("AgentSkill.Name", value));
}
export function parseAgentSkillNameStrict(value: string): AgentSkillName {
  const parsed = convertAgentSkillNameValue(backend.parse("AgentSkill.Name", value));
  if (parsed === null) {
    throw new Error("invalid AgentSkill.Name");
  }
  return parsed;
}
export function normalizeAgentSkillNameStrict(value: string): AgentSkillName {
  const normalized = convertAgentSkillNameValue(backend.normalize("AgentSkill.Name", value));
  if (normalized === null) {
    throw new Error("invalid AgentSkill.Name");
  }
  return normalized;
}
export function validateAgentSkillName(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("AgentSkill.Name", value);
}

export type AuthJWT = string & { readonly __brand: "Auth.JWT" };
function convertAuthJWTValue(value: unknown | null): AuthJWT | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as AuthJWT;
}
export function parseAuthJWT(value: unknown): AuthJWT | null {
  return convertAuthJWTValue(coerceValue("Auth.JWT", value));
}
export function normalizeAuthJWT(value: unknown): AuthJWT | null {
  return convertAuthJWTValue(coerceValue("Auth.JWT", value));
}
export function parseAuthJWTStrict(value: string): AuthJWT {
  const parsed = convertAuthJWTValue(backend.parse("Auth.JWT", value));
  if (parsed === null) {
    throw new Error("invalid Auth.JWT");
  }
  return parsed;
}
export function normalizeAuthJWTStrict(value: string): AuthJWT {
  const normalized = convertAuthJWTValue(backend.normalize("Auth.JWT", value));
  if (normalized === null) {
    throw new Error("invalid Auth.JWT");
  }
  return normalized;
}
export function validateAuthJWT(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Auth.JWT", value);
}

export type AuthPassword = string & { readonly __brand: "Auth.Password" };
function convertAuthPasswordValue(value: unknown | null): AuthPassword | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as AuthPassword;
}
export function parseAuthPassword(value: unknown): AuthPassword | null {
  return convertAuthPasswordValue(coerceValue("Auth.Password", value));
}
export function normalizeAuthPassword(value: unknown): AuthPassword | null {
  return convertAuthPasswordValue(coerceValue("Auth.Password", value));
}
export function parseAuthPasswordStrict(value: string): AuthPassword {
  const parsed = convertAuthPasswordValue(backend.parse("Auth.Password", value));
  if (parsed === null) {
    throw new Error("invalid Auth.Password");
  }
  return parsed;
}
export function normalizeAuthPasswordStrict(value: string): AuthPassword {
  const normalized = convertAuthPasswordValue(backend.normalize("Auth.Password", value));
  if (normalized === null) {
    throw new Error("invalid Auth.Password");
  }
  return normalized;
}
export function validateAuthPassword(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Auth.Password", value);
}

export type ContactEmail = string & { readonly __brand: "Contact.Email" };
function convertContactEmailValue(value: unknown | null): ContactEmail | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as ContactEmail;
}
export function parseContactEmail(value: unknown): ContactEmail | null {
  return convertContactEmailValue(coerceValue("Contact.Email", value));
}
export function normalizeContactEmail(value: unknown): ContactEmail | null {
  return convertContactEmailValue(coerceValue("Contact.Email", value));
}
export function parseContactEmailStrict(value: string): ContactEmail {
  const parsed = convertContactEmailValue(backend.parse("Contact.Email", value));
  if (parsed === null) {
    throw new Error("invalid Contact.Email");
  }
  return parsed;
}
export function normalizeContactEmailStrict(value: string): ContactEmail {
  const normalized = convertContactEmailValue(backend.normalize("Contact.Email", value));
  if (normalized === null) {
    throw new Error("invalid Contact.Email");
  }
  return normalized;
}
export function validateContactEmail(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Contact.Email", value);
}

export type ContactPhoneNumber = string & { readonly __brand: "Contact.PhoneNumber" };
function convertContactPhoneNumberValue(value: unknown | null): ContactPhoneNumber | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as ContactPhoneNumber;
}
export function parseContactPhoneNumber(value: unknown): ContactPhoneNumber | null {
  return convertContactPhoneNumberValue(coerceValue("Contact.PhoneNumber", value));
}
export function normalizeContactPhoneNumber(value: unknown): ContactPhoneNumber | null {
  return convertContactPhoneNumberValue(coerceValue("Contact.PhoneNumber", value));
}
export function parseContactPhoneNumberStrict(value: string): ContactPhoneNumber {
  const parsed = convertContactPhoneNumberValue(backend.parse("Contact.PhoneNumber", value));
  if (parsed === null) {
    throw new Error("invalid Contact.PhoneNumber");
  }
  return parsed;
}
export function normalizeContactPhoneNumberStrict(value: string): ContactPhoneNumber {
  const normalized = convertContactPhoneNumberValue(backend.normalize("Contact.PhoneNumber", value));
  if (normalized === null) {
    throw new Error("invalid Contact.PhoneNumber");
  }
  return normalized;
}
export function validateContactPhoneNumber(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Contact.PhoneNumber", value);
}

export type CryptoRSAPrivateKey = string & { readonly __brand: "Crypto.RSAPrivateKey" };
function convertCryptoRSAPrivateKeyValue(value: unknown | null): CryptoRSAPrivateKey | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as CryptoRSAPrivateKey;
}
export function parseCryptoRSAPrivateKey(value: unknown): CryptoRSAPrivateKey | null {
  return convertCryptoRSAPrivateKeyValue(coerceValue("Crypto.RSAPrivateKey", value));
}
export function normalizeCryptoRSAPrivateKey(value: unknown): CryptoRSAPrivateKey | null {
  return convertCryptoRSAPrivateKeyValue(coerceValue("Crypto.RSAPrivateKey", value));
}
export function parseCryptoRSAPrivateKeyStrict(value: string): CryptoRSAPrivateKey {
  const parsed = convertCryptoRSAPrivateKeyValue(backend.parse("Crypto.RSAPrivateKey", value));
  if (parsed === null) {
    throw new Error("invalid Crypto.RSAPrivateKey");
  }
  return parsed;
}
export function normalizeCryptoRSAPrivateKeyStrict(value: string): CryptoRSAPrivateKey {
  const normalized = convertCryptoRSAPrivateKeyValue(backend.normalize("Crypto.RSAPrivateKey", value));
  if (normalized === null) {
    throw new Error("invalid Crypto.RSAPrivateKey");
  }
  return normalized;
}
export function validateCryptoRSAPrivateKey(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Crypto.RSAPrivateKey", value);
}

export type CryptoRSAPublicKey = string & { readonly __brand: "Crypto.RSAPublicKey" };
function convertCryptoRSAPublicKeyValue(value: unknown | null): CryptoRSAPublicKey | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as CryptoRSAPublicKey;
}
export function parseCryptoRSAPublicKey(value: unknown): CryptoRSAPublicKey | null {
  return convertCryptoRSAPublicKeyValue(coerceValue("Crypto.RSAPublicKey", value));
}
export function normalizeCryptoRSAPublicKey(value: unknown): CryptoRSAPublicKey | null {
  return convertCryptoRSAPublicKeyValue(coerceValue("Crypto.RSAPublicKey", value));
}
export function parseCryptoRSAPublicKeyStrict(value: string): CryptoRSAPublicKey {
  const parsed = convertCryptoRSAPublicKeyValue(backend.parse("Crypto.RSAPublicKey", value));
  if (parsed === null) {
    throw new Error("invalid Crypto.RSAPublicKey");
  }
  return parsed;
}
export function normalizeCryptoRSAPublicKeyStrict(value: string): CryptoRSAPublicKey {
  const normalized = convertCryptoRSAPublicKeyValue(backend.normalize("Crypto.RSAPublicKey", value));
  if (normalized === null) {
    throw new Error("invalid Crypto.RSAPublicKey");
  }
  return normalized;
}
export function validateCryptoRSAPublicKey(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Crypto.RSAPublicKey", value);
}

export type CryptoSHA256 = string & { readonly __brand: "Crypto.SHA256" };
function convertCryptoSHA256Value(value: unknown | null): CryptoSHA256 | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as CryptoSHA256;
}
export function parseCryptoSHA256(value: unknown): CryptoSHA256 | null {
  return convertCryptoSHA256Value(coerceValue("Crypto.SHA256", value));
}
export function normalizeCryptoSHA256(value: unknown): CryptoSHA256 | null {
  return convertCryptoSHA256Value(coerceValue("Crypto.SHA256", value));
}
export function parseCryptoSHA256Strict(value: string): CryptoSHA256 {
  const parsed = convertCryptoSHA256Value(backend.parse("Crypto.SHA256", value));
  if (parsed === null) {
    throw new Error("invalid Crypto.SHA256");
  }
  return parsed;
}
export function normalizeCryptoSHA256Strict(value: string): CryptoSHA256 {
  const normalized = convertCryptoSHA256Value(backend.normalize("Crypto.SHA256", value));
  if (normalized === null) {
    throw new Error("invalid Crypto.SHA256");
  }
  return normalized;
}
export function validateCryptoSHA256(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Crypto.SHA256", value);
}

export type DesignColor = string & { readonly __brand: "Design.Color" };
function convertDesignColorValue(value: unknown | null): DesignColor | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as DesignColor;
}
export function parseDesignColor(value: unknown): DesignColor | null {
  return convertDesignColorValue(coerceValue("Design.Color", value));
}
export function normalizeDesignColor(value: unknown): DesignColor | null {
  return convertDesignColorValue(coerceValue("Design.Color", value));
}
export function parseDesignColorStrict(value: string): DesignColor {
  const parsed = convertDesignColorValue(backend.parse("Design.Color", value));
  if (parsed === null) {
    throw new Error("invalid Design.Color");
  }
  return parsed;
}
export function normalizeDesignColorStrict(value: string): DesignColor {
  const normalized = convertDesignColorValue(backend.normalize("Design.Color", value));
  if (normalized === null) {
    throw new Error("invalid Design.Color");
  }
  return normalized;
}
export function validateDesignColor(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Design.Color", value);
}

export type EmbeddingVector = number[];
function convertEmbeddingVectorValue(value: unknown | null): EmbeddingVector | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as EmbeddingVector;
}
export function parseEmbeddingVector(value: unknown): EmbeddingVector | null {
  return convertEmbeddingVectorValue(coerceValue("Embedding.Vector", value));
}
export function normalizeEmbeddingVector(value: unknown): EmbeddingVector | null {
  return convertEmbeddingVectorValue(coerceValue("Embedding.Vector", value));
}
export function parseEmbeddingVectorStrict(value: string): EmbeddingVector {
  const parsed = convertEmbeddingVectorValue(backend.parse("Embedding.Vector", value));
  if (parsed === null) {
    throw new Error("invalid Embedding.Vector");
  }
  return parsed;
}
export function normalizeEmbeddingVectorStrict(value: string): EmbeddingVector {
  const normalized = convertEmbeddingVectorValue(backend.normalize("Embedding.Vector", value));
  if (normalized === null) {
    throw new Error("invalid Embedding.Vector");
  }
  return normalized;
}
export function validateEmbeddingVector(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Embedding.Vector", value);
}

export type FileSizeBytes = number;
function convertFileSizeBytesValue(value: unknown | null): FileSizeBytes | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as FileSizeBytes;
}
export function parseFileSizeBytes(value: unknown): FileSizeBytes | null {
  return convertFileSizeBytesValue(coerceValue("File.SizeBytes", value));
}
export function normalizeFileSizeBytes(value: unknown): FileSizeBytes | null {
  return convertFileSizeBytesValue(coerceValue("File.SizeBytes", value));
}
export function parseFileSizeBytesStrict(value: string): FileSizeBytes {
  const parsed = convertFileSizeBytesValue(backend.parse("File.SizeBytes", value));
  if (parsed === null) {
    throw new Error("invalid File.SizeBytes");
  }
  return parsed;
}
export function normalizeFileSizeBytesStrict(value: string): FileSizeBytes {
  const normalized = convertFileSizeBytesValue(backend.normalize("File.SizeBytes", value));
  if (normalized === null) {
    throw new Error("invalid File.SizeBytes");
  }
  return normalized;
}
export function validateFileSizeBytes(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("File.SizeBytes", value);
}

export type FinanceMoney = number;
function convertFinanceMoneyValue(value: unknown | null): FinanceMoney | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as FinanceMoney;
}
export function parseFinanceMoney(value: unknown): FinanceMoney | null {
  return convertFinanceMoneyValue(coerceValue("Finance.Money", value));
}
export function normalizeFinanceMoney(value: unknown): FinanceMoney | null {
  return convertFinanceMoneyValue(coerceValue("Finance.Money", value));
}
export function parseFinanceMoneyStrict(value: string): FinanceMoney {
  const parsed = convertFinanceMoneyValue(backend.parse("Finance.Money", value));
  if (parsed === null) {
    throw new Error("invalid Finance.Money");
  }
  return parsed;
}
export function normalizeFinanceMoneyStrict(value: string): FinanceMoney {
  const normalized = convertFinanceMoneyValue(backend.normalize("Finance.Money", value));
  if (normalized === null) {
    throw new Error("invalid Finance.Money");
  }
  return normalized;
}
export function validateFinanceMoney(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Finance.Money", value);
}

export type GenericInt64 = number;
function convertGenericInt64Value(value: unknown | null): GenericInt64 | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as GenericInt64;
}
export function parseGenericInt64(value: unknown): GenericInt64 | null {
  return convertGenericInt64Value(coerceValue("Generic.Int64", value));
}
export function normalizeGenericInt64(value: unknown): GenericInt64 | null {
  return convertGenericInt64Value(coerceValue("Generic.Int64", value));
}
export function parseGenericInt64Strict(value: string): GenericInt64 {
  const parsed = convertGenericInt64Value(backend.parse("Generic.Int64", value));
  if (parsed === null) {
    throw new Error("invalid Generic.Int64");
  }
  return parsed;
}
export function normalizeGenericInt64Strict(value: string): GenericInt64 {
  const normalized = convertGenericInt64Value(backend.normalize("Generic.Int64", value));
  if (normalized === null) {
    throw new Error("invalid Generic.Int64");
  }
  return normalized;
}
export function validateGenericInt64(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Generic.Int64", value);
}

export type GenericJSON = JSONValue;
function convertGenericJSONValue(value: unknown | null): GenericJSON | undefined {
  if (value === undefined) {
    return undefined;
  }

  if (typeof value === "string") {
    try {
      return JSON.parse(value) as GenericJSON;
    } catch {
      return undefined;
    }
  }
  return value as GenericJSON;
}
// The lenient API accepts an already-decoded host JSON value. Use the
// strict API when the input is serialized JSON text.
export function parseGenericJSON(value: unknown): GenericJSON | undefined {
  return canonicalizeJSONValue("Generic.JSON", value, "parse") as GenericJSON | undefined;
}
export function normalizeGenericJSON(value: unknown): GenericJSON | undefined {
  return canonicalizeJSONValue("Generic.JSON", value, "normalize") as GenericJSON | undefined;
}
export function parseGenericJSONStrict(value: string): GenericJSON {
  const parsed = convertGenericJSONValue(backend.parse("Generic.JSON", value));
  if (parsed === undefined) {
    throw new Error("invalid Generic.JSON");
  }
  return parsed;
}
export function normalizeGenericJSONStrict(value: string): GenericJSON {
  const normalized = convertGenericJSONValue(backend.normalize("Generic.JSON", value));
  if (normalized === undefined) {
    throw new Error("invalid Generic.JSON");
  }
  return normalized;
}
export function validateGenericJSON(value: unknown | null | undefined): ScalarValidationResult {
  return validateJSONValueWithBackend("Generic.JSON", value);
}

export type GenericProbability = number;
function convertGenericProbabilityValue(value: unknown | null): GenericProbability | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as GenericProbability;
}
export function parseGenericProbability(value: unknown): GenericProbability | null {
  return convertGenericProbabilityValue(coerceValue("Generic.Probability", value));
}
export function normalizeGenericProbability(value: unknown): GenericProbability | null {
  return convertGenericProbabilityValue(coerceValue("Generic.Probability", value));
}
export function parseGenericProbabilityStrict(value: string): GenericProbability {
  const parsed = convertGenericProbabilityValue(backend.parse("Generic.Probability", value));
  if (parsed === null) {
    throw new Error("invalid Generic.Probability");
  }
  return parsed;
}
export function normalizeGenericProbabilityStrict(value: string): GenericProbability {
  const normalized = convertGenericProbabilityValue(backend.normalize("Generic.Probability", value));
  if (normalized === null) {
    throw new Error("invalid Generic.Probability");
  }
  return normalized;
}
export function validateGenericProbability(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Generic.Probability", value);
}

export type GenericStringMap = Record<string, string>;
function convertGenericStringMapValue(value: unknown | null): GenericStringMap | null {
  if (value === null || value === undefined) {
    return null;
  }

  if (typeof value === "string") {
    const parsed = parseJsonObject(value);
    if (parsed === null) {
      return null;
    }
    return parsed as GenericStringMap;
  }
  return value as GenericStringMap;
}
export function parseGenericStringMap(value: unknown): GenericStringMap | null {
  return convertGenericStringMapValue(coerceValue("Generic.StringMap", value));
}
export function normalizeGenericStringMap(value: unknown): GenericStringMap | null {
  return convertGenericStringMapValue(coerceValue("Generic.StringMap", value));
}
export function parseGenericStringMapStrict(value: string): GenericStringMap {
  const parsed = convertGenericStringMapValue(backend.parse("Generic.StringMap", value));
  if (parsed === null) {
    throw new Error("invalid Generic.StringMap");
  }
  return parsed;
}
export function normalizeGenericStringMapStrict(value: string): GenericStringMap {
  const normalized = convertGenericStringMapValue(backend.normalize("Generic.StringMap", value));
  if (normalized === null) {
    throw new Error("invalid Generic.StringMap");
  }
  return normalized;
}
export function validateGenericStringMap(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Generic.StringMap", value);
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
  return convertGeoLocationValue(coerceValue("Geo.Location", value));
}
export function normalizeGeoLocation(value: unknown): GeoLocation | null {
  return convertGeoLocationValue(coerceValue("Geo.Location", value));
}
export function parseGeoLocationStrict(value: string): GeoLocation {
  const parsed = convertGeoLocationValue(backend.parse("Geo.Location", value));
  if (parsed === null) {
    throw new Error("invalid Geo.Location");
  }
  return parsed;
}
export function normalizeGeoLocationStrict(value: string): GeoLocation {
  const normalized = convertGeoLocationValue(backend.normalize("Geo.Location", value));
  if (normalized === null) {
    throw new Error("invalid Geo.Location");
  }
  return normalized;
}
export function validateGeoLocation(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Geo.Location", value);
}

export type GitPathPattern = string & { readonly __brand: "Git.PathPattern" };
function convertGitPathPatternValue(value: unknown | null): GitPathPattern | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as GitPathPattern;
}
export function parseGitPathPattern(value: unknown): GitPathPattern | null {
  return convertGitPathPatternValue(coerceValue("Git.PathPattern", value));
}
export function normalizeGitPathPattern(value: unknown): GitPathPattern | null {
  return convertGitPathPatternValue(coerceValue("Git.PathPattern", value));
}
export function parseGitPathPatternStrict(value: string): GitPathPattern {
  const parsed = convertGitPathPatternValue(backend.parse("Git.PathPattern", value));
  if (parsed === null) {
    throw new Error("invalid Git.PathPattern");
  }
  return parsed;
}
export function normalizeGitPathPatternStrict(value: string): GitPathPattern {
  const normalized = convertGitPathPatternValue(backend.normalize("Git.PathPattern", value));
  if (normalized === null) {
    throw new Error("invalid Git.PathPattern");
  }
  return normalized;
}
export function validateGitPathPattern(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Git.PathPattern", value);
}

export type IdentityName = string & { readonly __brand: "Identity.Name" };
function convertIdentityNameValue(value: unknown | null): IdentityName | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as IdentityName;
}
export function parseIdentityName(value: unknown): IdentityName | null {
  return convertIdentityNameValue(coerceValue("Identity.Name", value));
}
export function normalizeIdentityName(value: unknown): IdentityName | null {
  return convertIdentityNameValue(coerceValue("Identity.Name", value));
}
export function parseIdentityNameStrict(value: string): IdentityName {
  const parsed = convertIdentityNameValue(backend.parse("Identity.Name", value));
  if (parsed === null) {
    throw new Error("invalid Identity.Name");
  }
  return parsed;
}
export function normalizeIdentityNameStrict(value: string): IdentityName {
  const normalized = convertIdentityNameValue(backend.normalize("Identity.Name", value));
  if (normalized === null) {
    throw new Error("invalid Identity.Name");
  }
  return normalized;
}
export function validateIdentityName(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Identity.Name", value);
}

export type IdentitySlug = string & { readonly __brand: "Identity.Slug" };
function convertIdentitySlugValue(value: unknown | null): IdentitySlug | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as IdentitySlug;
}
export function parseIdentitySlug(value: unknown): IdentitySlug | null {
  return convertIdentitySlugValue(coerceValue("Identity.Slug", value));
}
export function normalizeIdentitySlug(value: unknown): IdentitySlug | null {
  return convertIdentitySlugValue(coerceValue("Identity.Slug", value));
}
export function parseIdentitySlugStrict(value: string): IdentitySlug {
  const parsed = convertIdentitySlugValue(backend.parse("Identity.Slug", value));
  if (parsed === null) {
    throw new Error("invalid Identity.Slug");
  }
  return parsed;
}
export function normalizeIdentitySlugStrict(value: string): IdentitySlug {
  const normalized = convertIdentitySlugValue(backend.normalize("Identity.Slug", value));
  if (normalized === null) {
    throw new Error("invalid Identity.Slug");
  }
  return normalized;
}
export function validateIdentitySlug(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Identity.Slug", value);
}

export type IdentityUUID = string & { readonly __brand: "Identity.UUID" };
function convertIdentityUUIDValue(value: unknown | null): IdentityUUID | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as IdentityUUID;
}
export function parseIdentityUUID(value: unknown): IdentityUUID | null {
  return convertIdentityUUIDValue(coerceValue("Identity.UUID", value));
}
export function normalizeIdentityUUID(value: unknown): IdentityUUID | null {
  return convertIdentityUUIDValue(coerceValue("Identity.UUID", value));
}
export function parseIdentityUUIDStrict(value: string): IdentityUUID {
  const parsed = convertIdentityUUIDValue(backend.parse("Identity.UUID", value));
  if (parsed === null) {
    throw new Error("invalid Identity.UUID");
  }
  return parsed;
}
export function normalizeIdentityUUIDStrict(value: string): IdentityUUID {
  const normalized = convertIdentityUUIDValue(backend.normalize("Identity.UUID", value));
  if (normalized === null) {
    throw new Error("invalid Identity.UUID");
  }
  return normalized;
}
export function validateIdentityUUID(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Identity.UUID", value);
}

export type IdentityUserID = string & { readonly __brand: "Identity.UserID" };
function convertIdentityUserIDValue(value: unknown | null): IdentityUserID | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as IdentityUserID;
}
export function parseIdentityUserID(value: unknown): IdentityUserID | null {
  return convertIdentityUserIDValue(coerceValue("Identity.UserID", value));
}
export function normalizeIdentityUserID(value: unknown): IdentityUserID | null {
  return convertIdentityUserIDValue(coerceValue("Identity.UserID", value));
}
export function parseIdentityUserIDStrict(value: string): IdentityUserID {
  const parsed = convertIdentityUserIDValue(backend.parse("Identity.UserID", value));
  if (parsed === null) {
    throw new Error("invalid Identity.UserID");
  }
  return parsed;
}
export function normalizeIdentityUserIDStrict(value: string): IdentityUserID {
  const normalized = convertIdentityUserIDValue(backend.normalize("Identity.UserID", value));
  if (normalized === null) {
    throw new Error("invalid Identity.UserID");
  }
  return normalized;
}
export function validateIdentityUserID(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Identity.UserID", value);
}

export type LocalizationLocale = string & { readonly __brand: "Localization.Locale" };
function convertLocalizationLocaleValue(value: unknown | null): LocalizationLocale | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as LocalizationLocale;
}
export function parseLocalizationLocale(value: unknown): LocalizationLocale | null {
  return convertLocalizationLocaleValue(coerceValue("Localization.Locale", value));
}
export function normalizeLocalizationLocale(value: unknown): LocalizationLocale | null {
  return convertLocalizationLocaleValue(coerceValue("Localization.Locale", value));
}
export function parseLocalizationLocaleStrict(value: string): LocalizationLocale {
  const parsed = convertLocalizationLocaleValue(backend.parse("Localization.Locale", value));
  if (parsed === null) {
    throw new Error("invalid Localization.Locale");
  }
  return parsed;
}
export function normalizeLocalizationLocaleStrict(value: string): LocalizationLocale {
  const normalized = convertLocalizationLocaleValue(backend.normalize("Localization.Locale", value));
  if (normalized === null) {
    throw new Error("invalid Localization.Locale");
  }
  return normalized;
}
export function validateLocalizationLocale(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Localization.Locale", value);
}

export type NetworkDnsLabel = string & { readonly __brand: "Network.DnsLabel" };
function convertNetworkDnsLabelValue(value: unknown | null): NetworkDnsLabel | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as NetworkDnsLabel;
}
export function parseNetworkDnsLabel(value: unknown): NetworkDnsLabel | null {
  return convertNetworkDnsLabelValue(coerceValue("Network.DnsLabel", value));
}
export function normalizeNetworkDnsLabel(value: unknown): NetworkDnsLabel | null {
  return convertNetworkDnsLabelValue(coerceValue("Network.DnsLabel", value));
}
export function parseNetworkDnsLabelStrict(value: string): NetworkDnsLabel {
  const parsed = convertNetworkDnsLabelValue(backend.parse("Network.DnsLabel", value));
  if (parsed === null) {
    throw new Error("invalid Network.DnsLabel");
  }
  return parsed;
}
export function normalizeNetworkDnsLabelStrict(value: string): NetworkDnsLabel {
  const normalized = convertNetworkDnsLabelValue(backend.normalize("Network.DnsLabel", value));
  if (normalized === null) {
    throw new Error("invalid Network.DnsLabel");
  }
  return normalized;
}
export function validateNetworkDnsLabel(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Network.DnsLabel", value);
}

export type NetworkDomainName = string & { readonly __brand: "Network.DomainName" };
function convertNetworkDomainNameValue(value: unknown | null): NetworkDomainName | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as NetworkDomainName;
}
export function parseNetworkDomainName(value: unknown): NetworkDomainName | null {
  return convertNetworkDomainNameValue(coerceValue("Network.DomainName", value));
}
export function normalizeNetworkDomainName(value: unknown): NetworkDomainName | null {
  return convertNetworkDomainNameValue(coerceValue("Network.DomainName", value));
}
export function parseNetworkDomainNameStrict(value: string): NetworkDomainName {
  const parsed = convertNetworkDomainNameValue(backend.parse("Network.DomainName", value));
  if (parsed === null) {
    throw new Error("invalid Network.DomainName");
  }
  return parsed;
}
export function normalizeNetworkDomainNameStrict(value: string): NetworkDomainName {
  const normalized = convertNetworkDomainNameValue(backend.normalize("Network.DomainName", value));
  if (normalized === null) {
    throw new Error("invalid Network.DomainName");
  }
  return normalized;
}
export function validateNetworkDomainName(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Network.DomainName", value);
}

export type NetworkIpAddress = string & { readonly __brand: "Network.IpAddress" };
function convertNetworkIpAddressValue(value: unknown | null): NetworkIpAddress | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as NetworkIpAddress;
}
export function parseNetworkIpAddress(value: unknown): NetworkIpAddress | null {
  return convertNetworkIpAddressValue(coerceValue("Network.IpAddress", value));
}
export function normalizeNetworkIpAddress(value: unknown): NetworkIpAddress | null {
  return convertNetworkIpAddressValue(coerceValue("Network.IpAddress", value));
}
export function parseNetworkIpAddressStrict(value: string): NetworkIpAddress {
  const parsed = convertNetworkIpAddressValue(backend.parse("Network.IpAddress", value));
  if (parsed === null) {
    throw new Error("invalid Network.IpAddress");
  }
  return parsed;
}
export function normalizeNetworkIpAddressStrict(value: string): NetworkIpAddress {
  const normalized = convertNetworkIpAddressValue(backend.normalize("Network.IpAddress", value));
  if (normalized === null) {
    throw new Error("invalid Network.IpAddress");
  }
  return normalized;
}
export function validateNetworkIpAddress(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Network.IpAddress", value);
}

export type NetworkUri = string & { readonly __brand: "Network.Uri" };
function convertNetworkUriValue(value: unknown | null): NetworkUri | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as NetworkUri;
}
export function parseNetworkUri(value: unknown): NetworkUri | null {
  return convertNetworkUriValue(coerceValue("Network.Uri", value));
}
export function normalizeNetworkUri(value: unknown): NetworkUri | null {
  return convertNetworkUriValue(coerceValue("Network.Uri", value));
}
export function parseNetworkUriStrict(value: string): NetworkUri {
  const parsed = convertNetworkUriValue(backend.parse("Network.Uri", value));
  if (parsed === null) {
    throw new Error("invalid Network.Uri");
  }
  return parsed;
}
export function normalizeNetworkUriStrict(value: string): NetworkUri {
  const normalized = convertNetworkUriValue(backend.normalize("Network.Uri", value));
  if (normalized === null) {
    throw new Error("invalid Network.Uri");
  }
  return normalized;
}
export function validateNetworkUri(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Network.Uri", value);
}

export type NetworkUrl = string & { readonly __brand: "Network.Url" };
function convertNetworkUrlValue(value: unknown | null): NetworkUrl | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as NetworkUrl;
}
export function parseNetworkUrl(value: unknown): NetworkUrl | null {
  return convertNetworkUrlValue(coerceValue("Network.Url", value));
}
export function normalizeNetworkUrl(value: unknown): NetworkUrl | null {
  return convertNetworkUrlValue(coerceValue("Network.Url", value));
}
export function parseNetworkUrlStrict(value: string): NetworkUrl {
  const parsed = convertNetworkUrlValue(backend.parse("Network.Url", value));
  if (parsed === null) {
    throw new Error("invalid Network.Url");
  }
  return parsed;
}
export function normalizeNetworkUrlStrict(value: string): NetworkUrl {
  const normalized = convertNetworkUrlValue(backend.normalize("Network.Url", value));
  if (normalized === null) {
    throw new Error("invalid Network.Url");
  }
  return normalized;
}
export function validateNetworkUrl(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Network.Url", value);
}

export type OrderingRank = number;
function convertOrderingRankValue(value: unknown | null): OrderingRank | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as OrderingRank;
}
export function parseOrderingRank(value: unknown): OrderingRank | null {
  return convertOrderingRankValue(coerceValue("Ordering.Rank", value));
}
export function normalizeOrderingRank(value: unknown): OrderingRank | null {
  return convertOrderingRankValue(coerceValue("Ordering.Rank", value));
}
export function parseOrderingRankStrict(value: string): OrderingRank {
  const parsed = convertOrderingRankValue(backend.parse("Ordering.Rank", value));
  if (parsed === null) {
    throw new Error("invalid Ordering.Rank");
  }
  return parsed;
}
export function normalizeOrderingRankStrict(value: string): OrderingRank {
  const normalized = convertOrderingRankValue(backend.normalize("Ordering.Rank", value));
  if (normalized === null) {
    throw new Error("invalid Ordering.Rank");
  }
  return normalized;
}
export function validateOrderingRank(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Ordering.Rank", value);
}

export type TemporalCronExpression = string & { readonly __brand: "Temporal.CronExpression" };
function convertTemporalCronExpressionValue(value: unknown | null): TemporalCronExpression | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalCronExpression;
}
export function parseTemporalCronExpression(value: unknown): TemporalCronExpression | null {
  return convertTemporalCronExpressionValue(coerceValue("Temporal.CronExpression", value));
}
export function normalizeTemporalCronExpression(value: unknown): TemporalCronExpression | null {
  return convertTemporalCronExpressionValue(coerceValue("Temporal.CronExpression", value));
}
export function parseTemporalCronExpressionStrict(value: string): TemporalCronExpression {
  const parsed = convertTemporalCronExpressionValue(backend.parse("Temporal.CronExpression", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.CronExpression");
  }
  return parsed;
}
export function normalizeTemporalCronExpressionStrict(value: string): TemporalCronExpression {
  const normalized = convertTemporalCronExpressionValue(backend.normalize("Temporal.CronExpression", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.CronExpression");
  }
  return normalized;
}
export function validateTemporalCronExpression(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.CronExpression", value);
}

export type TemporalDate = string & { readonly __brand: "Temporal.Date" };
function convertTemporalDateValue(value: unknown | null): TemporalDate | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalDate;
}
export function parseTemporalDate(value: unknown): TemporalDate | null {
  return convertTemporalDateValue(coerceValue("Temporal.Date", value));
}
export function normalizeTemporalDate(value: unknown): TemporalDate | null {
  return convertTemporalDateValue(coerceValue("Temporal.Date", value));
}
export function parseTemporalDateStrict(value: string): TemporalDate {
  const parsed = convertTemporalDateValue(backend.parse("Temporal.Date", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Date");
  }
  return parsed;
}
export function normalizeTemporalDateStrict(value: string): TemporalDate {
  const normalized = convertTemporalDateValue(backend.normalize("Temporal.Date", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Date");
  }
  return normalized;
}
export function validateTemporalDate(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Date", value);
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
  return convertTemporalDateTimeValue(coerceValue("Temporal.DateTime", value));
}
export function normalizeTemporalDateTime(value: unknown): TemporalDateTime | null {
  return convertTemporalDateTimeValue(coerceValue("Temporal.DateTime", value));
}
export function parseTemporalDateTimeStrict(value: string): TemporalDateTime {
  const parsed = convertTemporalDateTimeValue(backend.parse("Temporal.DateTime", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.DateTime");
  }
  return parsed;
}
export function normalizeTemporalDateTimeStrict(value: string): TemporalDateTime {
  const normalized = convertTemporalDateTimeValue(backend.normalize("Temporal.DateTime", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.DateTime");
  }
  return normalized;
}
export function validateTemporalDateTime(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.DateTime", value);
}

export type TemporalDays = number;
function convertTemporalDaysValue(value: unknown | null): TemporalDays | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalDays;
}
export function parseTemporalDays(value: unknown): TemporalDays | null {
  return convertTemporalDaysValue(coerceValue("Temporal.Days", value));
}
export function normalizeTemporalDays(value: unknown): TemporalDays | null {
  return convertTemporalDaysValue(coerceValue("Temporal.Days", value));
}
export function parseTemporalDaysStrict(value: string): TemporalDays {
  const parsed = convertTemporalDaysValue(backend.parse("Temporal.Days", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Days");
  }
  return parsed;
}
export function normalizeTemporalDaysStrict(value: string): TemporalDays {
  const normalized = convertTemporalDaysValue(backend.normalize("Temporal.Days", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Days");
  }
  return normalized;
}
export function validateTemporalDays(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Days", value);
}

export type TemporalDuration = string & { readonly __brand: "Temporal.Duration" };
function convertTemporalDurationValue(value: unknown | null): TemporalDuration | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalDuration;
}
export function parseTemporalDuration(value: unknown): TemporalDuration | null {
  return convertTemporalDurationValue(coerceValue("Temporal.Duration", value));
}
export function normalizeTemporalDuration(value: unknown): TemporalDuration | null {
  return convertTemporalDurationValue(coerceValue("Temporal.Duration", value));
}
export function parseTemporalDurationStrict(value: string): TemporalDuration {
  const parsed = convertTemporalDurationValue(backend.parse("Temporal.Duration", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Duration");
  }
  return parsed;
}
export function normalizeTemporalDurationStrict(value: string): TemporalDuration {
  const normalized = convertTemporalDurationValue(backend.normalize("Temporal.Duration", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Duration");
  }
  return normalized;
}
export function validateTemporalDuration(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Duration", value);
}

export type TemporalHours = number;
function convertTemporalHoursValue(value: unknown | null): TemporalHours | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalHours;
}
export function parseTemporalHours(value: unknown): TemporalHours | null {
  return convertTemporalHoursValue(coerceValue("Temporal.Hours", value));
}
export function normalizeTemporalHours(value: unknown): TemporalHours | null {
  return convertTemporalHoursValue(coerceValue("Temporal.Hours", value));
}
export function parseTemporalHoursStrict(value: string): TemporalHours {
  const parsed = convertTemporalHoursValue(backend.parse("Temporal.Hours", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Hours");
  }
  return parsed;
}
export function normalizeTemporalHoursStrict(value: string): TemporalHours {
  const normalized = convertTemporalHoursValue(backend.normalize("Temporal.Hours", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Hours");
  }
  return normalized;
}
export function validateTemporalHours(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Hours", value);
}

export type TemporalMilliseconds = number;
function convertTemporalMillisecondsValue(value: unknown | null): TemporalMilliseconds | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalMilliseconds;
}
export function parseTemporalMilliseconds(value: unknown): TemporalMilliseconds | null {
  return convertTemporalMillisecondsValue(coerceValue("Temporal.Milliseconds", value));
}
export function normalizeTemporalMilliseconds(value: unknown): TemporalMilliseconds | null {
  return convertTemporalMillisecondsValue(coerceValue("Temporal.Milliseconds", value));
}
export function parseTemporalMillisecondsStrict(value: string): TemporalMilliseconds {
  const parsed = convertTemporalMillisecondsValue(backend.parse("Temporal.Milliseconds", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Milliseconds");
  }
  return parsed;
}
export function normalizeTemporalMillisecondsStrict(value: string): TemporalMilliseconds {
  const normalized = convertTemporalMillisecondsValue(backend.normalize("Temporal.Milliseconds", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Milliseconds");
  }
  return normalized;
}
export function validateTemporalMilliseconds(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Milliseconds", value);
}

export type TemporalMinutes = number;
function convertTemporalMinutesValue(value: unknown | null): TemporalMinutes | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalMinutes;
}
export function parseTemporalMinutes(value: unknown): TemporalMinutes | null {
  return convertTemporalMinutesValue(coerceValue("Temporal.Minutes", value));
}
export function normalizeTemporalMinutes(value: unknown): TemporalMinutes | null {
  return convertTemporalMinutesValue(coerceValue("Temporal.Minutes", value));
}
export function parseTemporalMinutesStrict(value: string): TemporalMinutes {
  const parsed = convertTemporalMinutesValue(backend.parse("Temporal.Minutes", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Minutes");
  }
  return parsed;
}
export function normalizeTemporalMinutesStrict(value: string): TemporalMinutes {
  const normalized = convertTemporalMinutesValue(backend.normalize("Temporal.Minutes", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Minutes");
  }
  return normalized;
}
export function validateTemporalMinutes(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Minutes", value);
}

export type TemporalMonth = string & { readonly __brand: "Temporal.Month" };
function convertTemporalMonthValue(value: unknown | null): TemporalMonth | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalMonth;
}
export function parseTemporalMonth(value: unknown): TemporalMonth | null {
  return convertTemporalMonthValue(coerceValue("Temporal.Month", value));
}
export function normalizeTemporalMonth(value: unknown): TemporalMonth | null {
  return convertTemporalMonthValue(coerceValue("Temporal.Month", value));
}
export function parseTemporalMonthStrict(value: string): TemporalMonth {
  const parsed = convertTemporalMonthValue(backend.parse("Temporal.Month", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Month");
  }
  return parsed;
}
export function normalizeTemporalMonthStrict(value: string): TemporalMonth {
  const normalized = convertTemporalMonthValue(backend.normalize("Temporal.Month", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Month");
  }
  return normalized;
}
export function validateTemporalMonth(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Month", value);
}

export type TemporalQuarter = string & { readonly __brand: "Temporal.Quarter" };
function convertTemporalQuarterValue(value: unknown | null): TemporalQuarter | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalQuarter;
}
export function parseTemporalQuarter(value: unknown): TemporalQuarter | null {
  return convertTemporalQuarterValue(coerceValue("Temporal.Quarter", value));
}
export function normalizeTemporalQuarter(value: unknown): TemporalQuarter | null {
  return convertTemporalQuarterValue(coerceValue("Temporal.Quarter", value));
}
export function parseTemporalQuarterStrict(value: string): TemporalQuarter {
  const parsed = convertTemporalQuarterValue(backend.parse("Temporal.Quarter", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Quarter");
  }
  return parsed;
}
export function normalizeTemporalQuarterStrict(value: string): TemporalQuarter {
  const normalized = convertTemporalQuarterValue(backend.normalize("Temporal.Quarter", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Quarter");
  }
  return normalized;
}
export function validateTemporalQuarter(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Quarter", value);
}

export type TemporalQuarterYear = string & { readonly __brand: "Temporal.QuarterYear" };
function convertTemporalQuarterYearValue(value: unknown | null): TemporalQuarterYear | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalQuarterYear;
}
export function parseTemporalQuarterYear(value: unknown): TemporalQuarterYear | null {
  return convertTemporalQuarterYearValue(coerceValue("Temporal.QuarterYear", value));
}
export function normalizeTemporalQuarterYear(value: unknown): TemporalQuarterYear | null {
  return convertTemporalQuarterYearValue(coerceValue("Temporal.QuarterYear", value));
}
export function parseTemporalQuarterYearStrict(value: string): TemporalQuarterYear {
  const parsed = convertTemporalQuarterYearValue(backend.parse("Temporal.QuarterYear", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.QuarterYear");
  }
  return parsed;
}
export function normalizeTemporalQuarterYearStrict(value: string): TemporalQuarterYear {
  const normalized = convertTemporalQuarterYearValue(backend.normalize("Temporal.QuarterYear", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.QuarterYear");
  }
  return normalized;
}
export function validateTemporalQuarterYear(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.QuarterYear", value);
}

export type TemporalRecurrenceRule = string & { readonly __brand: "Temporal.RecurrenceRule" };
function convertTemporalRecurrenceRuleValue(value: unknown | null): TemporalRecurrenceRule | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalRecurrenceRule;
}
export function parseTemporalRecurrenceRule(value: unknown): TemporalRecurrenceRule | null {
  return convertTemporalRecurrenceRuleValue(coerceValue("Temporal.RecurrenceRule", value));
}
export function normalizeTemporalRecurrenceRule(value: unknown): TemporalRecurrenceRule | null {
  return convertTemporalRecurrenceRuleValue(coerceValue("Temporal.RecurrenceRule", value));
}
export function parseTemporalRecurrenceRuleStrict(value: string): TemporalRecurrenceRule {
  const parsed = convertTemporalRecurrenceRuleValue(backend.parse("Temporal.RecurrenceRule", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.RecurrenceRule");
  }
  return parsed;
}
export function normalizeTemporalRecurrenceRuleStrict(value: string): TemporalRecurrenceRule {
  const normalized = convertTemporalRecurrenceRuleValue(backend.normalize("Temporal.RecurrenceRule", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.RecurrenceRule");
  }
  return normalized;
}
export function validateTemporalRecurrenceRule(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.RecurrenceRule", value);
}

export type TemporalSeconds = number;
function convertTemporalSecondsValue(value: unknown | null): TemporalSeconds | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalSeconds;
}
export function parseTemporalSeconds(value: unknown): TemporalSeconds | null {
  return convertTemporalSecondsValue(coerceValue("Temporal.Seconds", value));
}
export function normalizeTemporalSeconds(value: unknown): TemporalSeconds | null {
  return convertTemporalSecondsValue(coerceValue("Temporal.Seconds", value));
}
export function parseTemporalSecondsStrict(value: string): TemporalSeconds {
  const parsed = convertTemporalSecondsValue(backend.parse("Temporal.Seconds", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Seconds");
  }
  return parsed;
}
export function normalizeTemporalSecondsStrict(value: string): TemporalSeconds {
  const normalized = convertTemporalSecondsValue(backend.normalize("Temporal.Seconds", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Seconds");
  }
  return normalized;
}
export function validateTemporalSeconds(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Seconds", value);
}

export type TemporalTime = string & { readonly __brand: "Temporal.Time" };
function convertTemporalTimeValue(value: unknown | null): TemporalTime | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalTime;
}
export function parseTemporalTime(value: unknown): TemporalTime | null {
  return convertTemporalTimeValue(coerceValue("Temporal.Time", value));
}
export function normalizeTemporalTime(value: unknown): TemporalTime | null {
  return convertTemporalTimeValue(coerceValue("Temporal.Time", value));
}
export function parseTemporalTimeStrict(value: string): TemporalTime {
  const parsed = convertTemporalTimeValue(backend.parse("Temporal.Time", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Time");
  }
  return parsed;
}
export function normalizeTemporalTimeStrict(value: string): TemporalTime {
  const normalized = convertTemporalTimeValue(backend.normalize("Temporal.Time", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Time");
  }
  return normalized;
}
export function validateTemporalTime(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Time", value);
}

export type TemporalTimeZone = string & { readonly __brand: "Temporal.TimeZone" };
function convertTemporalTimeZoneValue(value: unknown | null): TemporalTimeZone | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalTimeZone;
}
export function parseTemporalTimeZone(value: unknown): TemporalTimeZone | null {
  return convertTemporalTimeZoneValue(coerceValue("Temporal.TimeZone", value));
}
export function normalizeTemporalTimeZone(value: unknown): TemporalTimeZone | null {
  return convertTemporalTimeZoneValue(coerceValue("Temporal.TimeZone", value));
}
export function parseTemporalTimeZoneStrict(value: string): TemporalTimeZone {
  const parsed = convertTemporalTimeZoneValue(backend.parse("Temporal.TimeZone", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.TimeZone");
  }
  return parsed;
}
export function normalizeTemporalTimeZoneStrict(value: string): TemporalTimeZone {
  const normalized = convertTemporalTimeZoneValue(backend.normalize("Temporal.TimeZone", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.TimeZone");
  }
  return normalized;
}
export function validateTemporalTimeZone(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.TimeZone", value);
}

export type TemporalYear = string & { readonly __brand: "Temporal.Year" };
function convertTemporalYearValue(value: unknown | null): TemporalYear | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TemporalYear;
}
export function parseTemporalYear(value: unknown): TemporalYear | null {
  return convertTemporalYearValue(coerceValue("Temporal.Year", value));
}
export function normalizeTemporalYear(value: unknown): TemporalYear | null {
  return convertTemporalYearValue(coerceValue("Temporal.Year", value));
}
export function parseTemporalYearStrict(value: string): TemporalYear {
  const parsed = convertTemporalYearValue(backend.parse("Temporal.Year", value));
  if (parsed === null) {
    throw new Error("invalid Temporal.Year");
  }
  return parsed;
}
export function normalizeTemporalYearStrict(value: string): TemporalYear {
  const normalized = convertTemporalYearValue(backend.normalize("Temporal.Year", value));
  if (normalized === null) {
    throw new Error("invalid Temporal.Year");
  }
  return normalized;
}
export function validateTemporalYear(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Temporal.Year", value);
}

export type TextMarkdown = string & { readonly __brand: "Text.Markdown" };
function convertTextMarkdownValue(value: unknown | null): TextMarkdown | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TextMarkdown;
}
export function parseTextMarkdown(value: unknown): TextMarkdown | null {
  return convertTextMarkdownValue(coerceValue("Text.Markdown", value));
}
export function normalizeTextMarkdown(value: unknown): TextMarkdown | null {
  return convertTextMarkdownValue(coerceValue("Text.Markdown", value));
}
export function parseTextMarkdownStrict(value: string): TextMarkdown {
  const parsed = convertTextMarkdownValue(backend.parse("Text.Markdown", value));
  if (parsed === null) {
    throw new Error("invalid Text.Markdown");
  }
  return parsed;
}
export function normalizeTextMarkdownStrict(value: string): TextMarkdown {
  const normalized = convertTextMarkdownValue(backend.normalize("Text.Markdown", value));
  if (normalized === null) {
    throw new Error("invalid Text.Markdown");
  }
  return normalized;
}
export function validateTextMarkdown(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Text.Markdown", value);
}

export type TextSql = string & { readonly __brand: "Text.Sql" };
function convertTextSqlValue(value: unknown | null): TextSql | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as TextSql;
}
export function parseTextSql(value: unknown): TextSql | null {
  return convertTextSqlValue(coerceValue("Text.Sql", value));
}
export function normalizeTextSql(value: unknown): TextSql | null {
  return convertTextSqlValue(coerceValue("Text.Sql", value));
}
export function parseTextSqlStrict(value: string): TextSql {
  const parsed = convertTextSqlValue(backend.parse("Text.Sql", value));
  if (parsed === null) {
    throw new Error("invalid Text.Sql");
  }
  return parsed;
}
export function normalizeTextSqlStrict(value: string): TextSql {
  const normalized = convertTextSqlValue(backend.normalize("Text.Sql", value));
  if (normalized === null) {
    throw new Error("invalid Text.Sql");
  }
  return normalized;
}
export function validateTextSql(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Text.Sql", value);
}

export type VersionSemVer = string & { readonly __brand: "Version.SemVer" };
function convertVersionSemVerValue(value: unknown | null): VersionSemVer | null {
  if (value === null || value === undefined) {
    return null;
  }

  return value as VersionSemVer;
}
export function parseVersionSemVer(value: unknown): VersionSemVer | null {
  return convertVersionSemVerValue(coerceValue("Version.SemVer", value));
}
export function normalizeVersionSemVer(value: unknown): VersionSemVer | null {
  return convertVersionSemVerValue(coerceValue("Version.SemVer", value));
}
export function parseVersionSemVerStrict(value: string): VersionSemVer {
  const parsed = convertVersionSemVerValue(backend.parse("Version.SemVer", value));
  if (parsed === null) {
    throw new Error("invalid Version.SemVer");
  }
  return parsed;
}
export function normalizeVersionSemVerStrict(value: string): VersionSemVer {
  const normalized = convertVersionSemVerValue(backend.normalize("Version.SemVer", value));
  if (normalized === null) {
    throw new Error("invalid Version.SemVer");
  }
  return normalized;
}
export function validateVersionSemVer(value: unknown | null | undefined): ScalarValidationResult {
  return validateWithBackend("Version.SemVer", value);
}



export const SCALAR_METADATA: ScalarMetadata[] = [
  {
    canonicalName: "AgentSkill.Name",
    symbol: "AgentSkillName",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 64,
    minLength: 1,
    pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$",
    hasValidator: true,
    examples: ["data-analysis", "careful-refactors"],
    comparabilityClass: null,
    isSortable: true,
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: true,
    reservedWordsMatchPartial: false,
  },
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    examples: ["+14155552671"],
    comparabilityClass: null,
    isSortable: true,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
  },
  {
    canonicalName: "Generic.JSON",
    symbol: "GenericJSON",
    primitive: "String",
    tsType: "JSONValue",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["{\"k\":1}", "[1,2]", "\"text\"", "42", "true", "null"],
    comparabilityClass: null,
    isSortable: false,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
  },
  {
    canonicalName: "Geo.Location",
    symbol: "GeoLocation",
    primitive: "String",
    tsType: "{ lat: number; lon: number }",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["{\"lat\":37.7749,\"lon\":-122.4194}"],
    comparabilityClass: null,
    isSortable: false,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
  },
  {
    canonicalName: "Git.PathPattern",
    symbol: "GitPathPattern",
    primitive: "String",
    tsType: "string",
    format: "",
    maxLength: 1024,
    minLength: 2,
    pattern: "^!?/[^\\x00\\r\\n]+$",
    hasValidator: true,
    examples: ["/skills/**", "!/skills/shared/**", "/assets/"],
    comparabilityClass: null,
    isSortable: true,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
  },
  {
    canonicalName: "Ordering.Rank",
    symbol: "OrderingRank",
    primitive: "Int",
    tsType: "number",
    format: "",
    maxLength: 0,
    minLength: 0,
    pattern: "",
    hasValidator: true,
    examples: ["1", "1000"],
    comparabilityClass: null,
    isSortable: true,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: true,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
  },
  {
    canonicalName: "Version.SemVer",
    symbol: "VersionSemVer",
    primitive: "String",
    tsType: "string",
    format: "semver",
    maxLength: 255,
    minLength: 5,
    pattern: "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(?:-((?:0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)(?:\\.(?:0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*))*))?(?:\\+([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?$",
    hasValidator: true,
    examples: ["1.0.0", "2.4.1-rc.1+build.9"],
    comparabilityClass: null,
    isSortable: true,
    caseInsensitive: false,
    reservedWords: [],
    reservedWordsCaseInsensitive: false,
    reservedWordsMatchPartial: false,
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
