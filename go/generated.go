// @generated; do not edit

package superscalar

import (
	"encoding/json"
	"fmt"
	"reflect"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// ScalarMetadata describes a single scalar type's identity and validation surface.
type ScalarMetadata struct {
	CanonicalName      string
	Symbol             string
	Primitive          string
	Description        string
	TypeScriptType     string
	PythonType         string
	RustType           string
	GoType             string
	SQLType            string
	JSONSchemaType     string
	Format             string
	MaxLength          int
	MinLength          int
	Maximum            *int64
	Minimum            *int64
	Pattern            string
	HasCustomNormalize bool
	HasCustomParse     bool
	HasCustomValidate  bool
	HasValidator       bool
	Examples           []string
	// Comparability equivalence class, empty when the scalar has none.
	// Field equality is NOT the comparability relation: absent is encoded
	// here as the empty string, which is also the zero value a failed map
	// lookup returns, so comparing two fields answers true for every pair
	// of unclassed scalars and for every pair of names that do not exist.
	// The relation is ComparableWith below, mirroring the Rust core's
	// Registry::comparable_with.
	ComparabilityClass string
	IsSortable         bool
}

func scalarInt64Ptr(value int64) *int64 {
	return &value
}

type ScalarPattern string

func (p ScalarPattern) String() string {
	return string(p)
}

func (p ScalarPattern) MatchString(value string) bool {
	matched, err := regexp.MatchString(string(p), value)
	return err == nil && matched
}

// Canonical scalar names, one constant per registry entry. A scalar's name is
// its identity: every call into the core passes it.
const (
	scalarNameAgentSkillName         = "AgentSkill.Name"
	scalarNameAuthJWT                = "Auth.JWT"
	scalarNameAuthPassword           = "Auth.Password"
	scalarNameContactEmail           = "Contact.Email"
	scalarNameContactPhoneNumber     = "Contact.PhoneNumber"
	scalarNameCryptoRSAPrivateKey    = "Crypto.RSAPrivateKey"
	scalarNameCryptoRSAPublicKey     = "Crypto.RSAPublicKey"
	scalarNameCryptoSHA256           = "Crypto.SHA256"
	scalarNameDesignColor            = "Design.Color"
	scalarNameEmbeddingVector        = "Embedding.Vector"
	scalarNameFileSizeBytes          = "File.SizeBytes"
	scalarNameFinanceMoney           = "Finance.Money"
	scalarNameGenericInt64           = "Generic.Int64"
	scalarNameGenericJSON            = "Generic.JSON"
	scalarNameGenericProbability     = "Generic.Probability"
	scalarNameGenericStringMap       = "Generic.StringMap"
	scalarNameGeoLocation            = "Geo.Location"
	scalarNameGitPathPattern         = "Git.PathPattern"
	scalarNameIdentityName           = "Identity.Name"
	scalarNameIdentitySlug           = "Identity.Slug"
	scalarNameIdentityUUID           = "Identity.UUID"
	scalarNameIdentityUserID         = "Identity.UserID"
	scalarNameLocalizationLocale     = "Localization.Locale"
	scalarNameNetworkDnsLabel        = "Network.DnsLabel"
	scalarNameNetworkDomainName      = "Network.DomainName"
	scalarNameNetworkIpAddress       = "Network.IpAddress"
	scalarNameNetworkUri             = "Network.Uri"
	scalarNameNetworkUrl             = "Network.Url"
	scalarNameOrderingRank           = "Ordering.Rank"
	scalarNameTemporalCronExpression = "Temporal.CronExpression"
	scalarNameTemporalDate           = "Temporal.Date"
	scalarNameTemporalDateTime       = "Temporal.DateTime"
	scalarNameTemporalDays           = "Temporal.Days"
	scalarNameTemporalDuration       = "Temporal.Duration"
	scalarNameTemporalHours          = "Temporal.Hours"
	scalarNameTemporalMilliseconds   = "Temporal.Milliseconds"
	scalarNameTemporalMinutes        = "Temporal.Minutes"
	scalarNameTemporalMonth          = "Temporal.Month"
	scalarNameTemporalQuarter        = "Temporal.Quarter"
	scalarNameTemporalQuarterYear    = "Temporal.QuarterYear"
	scalarNameTemporalRecurrenceRule = "Temporal.RecurrenceRule"
	scalarNameTemporalSeconds        = "Temporal.Seconds"
	scalarNameTemporalTime           = "Temporal.Time"
	scalarNameTemporalTimeZone       = "Temporal.TimeZone"
	scalarNameTemporalYear           = "Temporal.Year"
	scalarNameTextMarkdown           = "Text.Markdown"
	scalarNameTextSql                = "Text.Sql"
	scalarNameVersionSemVer          = "Version.SemVer"
)

// VALID_SCALARS lists every canonical scalar name in the registry, sorted.
var VALID_SCALARS = []string{
	string("AgentSkill.Name"),
	string("Auth.JWT"),
	string("Auth.Password"),
	string("Contact.Email"),
	string("Contact.PhoneNumber"),
	string("Crypto.RSAPrivateKey"),
	string("Crypto.RSAPublicKey"),
	string("Crypto.SHA256"),
	string("Design.Color"),
	string("Embedding.Vector"),
	string("File.SizeBytes"),
	string("Finance.Money"),
	string("Generic.Int64"),
	string("Generic.JSON"),
	string("Generic.Probability"),
	string("Generic.StringMap"),
	string("Geo.Location"),
	string("Git.PathPattern"),
	string("Identity.Name"),
	string("Identity.Slug"),
	string("Identity.UUID"),
	string("Identity.UserID"),
	string("Localization.Locale"),
	string("Network.DnsLabel"),
	string("Network.DomainName"),
	string("Network.IpAddress"),
	string("Network.Uri"),
	string("Network.Url"),
	string("Ordering.Rank"),
	string("Temporal.CronExpression"),
	string("Temporal.Date"),
	string("Temporal.DateTime"),
	string("Temporal.Days"),
	string("Temporal.Duration"),
	string("Temporal.Hours"),
	string("Temporal.Milliseconds"),
	string("Temporal.Minutes"),
	string("Temporal.Month"),
	string("Temporal.Quarter"),
	string("Temporal.QuarterYear"),
	string("Temporal.RecurrenceRule"),
	string("Temporal.Seconds"),
	string("Temporal.Time"),
	string("Temporal.TimeZone"),
	string("Temporal.Year"),
	string("Text.Markdown"),
	string("Text.Sql"),
	string("Version.SemVer"),
}

var validScalarsMap = func() map[string]struct{} {
	m := make(map[string]struct{}, len(VALID_SCALARS))
	for _, scalar := range VALID_SCALARS {
		m[scalar] = struct{}{}
	}
	return m
}()

// AgentSkill.Name - "Portable Agent Skills directory and frontmatter name"
type AgentSkillName string

// Auth.JWT - "JSON Web Token string"
type AuthJWT string

// Auth.Password - "User password (minimum 8 characters)"
type AuthPassword string

// Contact.Email - "An email address"
type ContactEmail string

// Contact.PhoneNumber - "Phone number in E.164 format"
type ContactPhoneNumber string

// Crypto.RSAPrivateKey - "RSA private key in PEM format"
type CryptoRSAPrivateKey string

// Crypto.RSAPublicKey - "RSA public key in PEM format"
type CryptoRSAPublicKey string

// Crypto.SHA256 - "Lowercase hexadecimal SHA-256 digest"
type CryptoSHA256 string

// Design.Color - "CSS color value normalized to 8-digit RGBA hex format (#RRGGBBAA)"
type DesignColor string

// Embedding.Vector - "Fixed-dimensional float32 vector"
type EmbeddingVector []float32

// File.SizeBytes - "File size in bytes (non-negative, BIGINT-backed)"
type FileSizeBytes int64

// Finance.Money - "Monetary amount in smallest currency unit (e.g., cents for USD)"
type FinanceMoney int64

// Generic.Int64 - "Signed 64-bit integer; range bounded by JavaScript's safe-integer ceiling."
type GenericInt64 int64

// Generic.JSON - "Any valid JSON value: object, array, primitive, or null"
type GenericJSON json.RawMessage

// Generic.Probability - "Probability value from 0.0 to 1.0 inclusive"
type GenericProbability float64

// Generic.StringMap - "A string-to-string map stored as JSON"
type GenericStringMap map[string]string

// Geo.Location - "Geographic location with latitude and longitude"
type GeoLocation struct {
	Lat float64 `json:"lat"`
	Lon float64 `json:"lon"`
}

// Git.PathPattern - "Repository-rooted, case-sensitive gitignore-style path pattern"
type GitPathPattern string

// Identity.Name - "An objects name"
type IdentityName string

// Identity.Slug - "A URL friendly version of a string"
type IdentitySlug string

// Identity.UUID - "UUID v4 with automatic base62 encoding for client-facing APIs"
type IdentityUUID = UUID

// Identity.UserID - "UUID v4 string as base62"
type IdentityUserID = UUID

// Localization.Locale - "BCP 47 language tag (e.g., en-US, fr-FR)"
type LocalizationLocale string

// Network.DnsLabel - "Single DNS label (RFC 1035): one hostname segment, no dots"
type NetworkDnsLabel string

// Network.DomainName - "Valid domain name (RFC 1035 compliant)"
type NetworkDomainName string

// Network.IpAddress - "IPv4 or IPv6 address"
type NetworkIpAddress string

// Network.Uri - "RFC 3986 URI for connection strings and non-HTTP resources"
type NetworkUri string

// Network.Url - "Valid HTTP/HTTPS URL"
type NetworkUrl string

// Ordering.Rank - "Positive JavaScript-safe ordering rank"
type OrderingRank int64

// Temporal.CronExpression - "Standard 5-field cron expression for scheduling"
type TemporalCronExpression string

// Temporal.Date - "Calendar date, normalized to ISO 'YYYY-MM-DD'. Accepts ISO ('2025-01-01'), slash-separated ('2025/01/15', '01/15/2025'), named-month ('January 15, 2025', 'Jan 15, 2025'), and full RFC3339 datetime (the time portion is dropped)."
type TemporalDate string

// Temporal.DateTime - "ISO8601 datetime string. Epoch wire values keep this scalar and declare x-temporal-format (unix, unix_millis, unix_micros, unix_nanos) on the property; the unit is never guessed from digit count."
type TemporalDateTime time.Time

// Temporal.Days - "Signed integer count of days: an amount of elapsed time, never a point in time or a calendar date."
type TemporalDays int64

// Temporal.Duration - "Duration for timeouts and intervals"
type TemporalDuration time.Duration

// Temporal.Hours - "Signed integer count of hours: an amount of elapsed time, never a point in time."
type TemporalHours int64

// Temporal.Milliseconds - "Signed integer count of milliseconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix_millis."
type TemporalMilliseconds int64

// Temporal.Minutes - "Signed integer count of minutes: an amount of elapsed time, never a point in time."
type TemporalMinutes int64

// Temporal.Month - "Calendar month, normalized to two-digit numeric (01-12). Accepts '2', '02', 'Feb', 'February' (case-insensitive)."
type TemporalMonth string

// Temporal.Quarter - "Calendar quarter (Q1-Q4)"
type TemporalQuarter string

// Temporal.QuarterYear - "Quarter and year, normalized to 'YYYY-Q#'. Accepts '2025-Q1', 'Q1/2025', 'Q1-2025', 'Q1-25' (2-digit years interpreted as 20XX)."
type TemporalQuarterYear string

// Temporal.RecurrenceRule - "RFC 5545 recurrence rule, without DTSTART"
type TemporalRecurrenceRule string

// Temporal.Seconds - "Signed integer count of seconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix."
type TemporalSeconds int64

// Temporal.Time - "Time of day. 24-hour 'HH:MM' or 'HH:MM:SS' (hours 00-23), or 12-hour 'H:MM'/'HH:MM' with optional ':SS' and required AM/PM suffix (hours 1-12). Seconds and the AM/PM separator space are optional."
type TemporalTime string

// Temporal.TimeZone - "IANA timezone identifier (e.g., America/New_York, UTC, Etc/UTC)"
type TemporalTimeZone string

// Temporal.Year - "Calendar year as a 4-digit string (e.g., 2025)"
type TemporalYear string

// Text.Markdown - "Markdown text"
type TextMarkdown string

// Text.Sql - "SQL text"
type TextSql string

// Version.SemVer - "Canonical Semantic Versioning 2.0.0 value"
type VersionSemVer string

var SCALAR_METADATA = []ScalarMetadata{
	{
		CanonicalName:      "AgentSkill.Name",
		Symbol:             "AgentSkillName",
		Primitive:          "String",
		Description:        "Portable Agent Skills directory and frontmatter name",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "CITEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          64,
		MinLength:          1,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-z0-9]+(?:-[a-z0-9]+)*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"data-analysis", "careful-refactors"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Auth.JWT",
		Symbol:             "AuthJWT",
		Primitive:          "String",
		Description:        "JSON Web Token string",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+\\.[A-Za-z0-9-_]+$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Auth.Password",
		Symbol:             "AuthPassword",
		Primitive:          "String",
		Description:        "User password (minimum 8 characters)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(128)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          128,
		MinLength:          8,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Contact.Email",
		Symbol:             "ContactEmail",
		Primitive:          "String",
		Description:        "An email address",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "CITEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          255,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
		HasCustomNormalize: true,
		HasCustomParse:     false,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"test@example.com"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Contact.PhoneNumber",
		Symbol:             "ContactPhoneNumber",
		Primitive:          "String",
		Description:        "Phone number in E.164 format",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(16)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          16,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^\\+[1-9]\\d{1,14}$",
		HasCustomNormalize: true,
		HasCustomParse:     false,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"+14155552671"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Crypto.RSAPrivateKey",
		Symbol:             "CryptoRSAPrivateKey",
		Primitive:          "String",
		Description:        "RSA private key in PEM format",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^-----BEGIN (RSA )?PRIVATE KEY-----[\\s\\S]*-----END (RSA )?PRIVATE KEY-----\\s*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"-----BEGIN RSA PRIVATE KEY----------END RSA PRIVATE KEY-----"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Crypto.RSAPublicKey",
		Symbol:             "CryptoRSAPublicKey",
		Primitive:          "String",
		Description:        "RSA public key in PEM format",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^-----BEGIN PUBLIC KEY-----[\\s\\S]*-----END PUBLIC KEY-----\\s*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"-----BEGIN PUBLIC KEY----------END PUBLIC KEY-----"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Crypto.SHA256",
		Symbol:             "CryptoSHA256",
		Primitive:          "String",
		Description:        "Lowercase hexadecimal SHA-256 digest",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(64)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          64,
		MinLength:          64,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[0-9a-f]{64}$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Design.Color",
		Symbol:             "DesignColor",
		Primitive:          "String",
		Description:        "CSS color value normalized to 8-digit RGBA hex format (#RRGGBBAA)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(9)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          9,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^#[0-9A-Fa-f]{8}$",
		HasCustomNormalize: true,
		HasCustomParse:     false,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"#FF5733FF"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Embedding.Vector",
		Symbol:             "EmbeddingVector",
		Primitive:          "String",
		Description:        "Fixed-dimensional float32 vector",
		TypeScriptType:     "number[]",
		PythonType:         "list[float]",
		RustType:           "Vec<f32>",
		GoType:             "[]float32",
		SQLType:            "",
		JSONSchemaType:     "array",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{},
		ComparabilityClass: "",
		IsSortable:         false,
	},
	{
		CanonicalName:      "File.SizeBytes",
		Symbol:             "FileSizeBytes",
		Primitive:          "Int",
		Description:        "File size in bytes (non-negative, BIGINT-backed)",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(0),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"204800"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Finance.Money",
		Symbol:             "FinanceMoney",
		Primitive:          "Int",
		Description:        "Monetary amount in smallest currency unit (e.g., cents for USD)",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(0),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"1000"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Generic.Int64",
		Symbol:             "GenericInt64",
		Primitive:          "Int",
		Description:        "Signed 64-bit integer; range bounded by JavaScript's safe-integer ceiling.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"1000"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Generic.JSON",
		Symbol:             "GenericJSON",
		Primitive:          "String",
		Description:        "Any valid JSON value: object, array, primitive, or null",
		TypeScriptType:     "JSONValue",
		PythonType:         "Any",
		RustType:           "serde_json::Value",
		GoType:             "json.RawMessage",
		SQLType:            "JSONB",
		JSONSchemaType:     "any",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"{\"k\":1}", "[1,2]", "\"text\"", "42", "true", "null"},
		ComparabilityClass: "",
		IsSortable:         false,
	},
	{
		CanonicalName:      "Generic.Probability",
		Symbol:             "GenericProbability",
		Primitive:          "Float",
		Description:        "Probability value from 0.0 to 1.0 inclusive",
		TypeScriptType:     "number",
		PythonType:         "float",
		RustType:           "f64",
		GoType:             "float64",
		SQLType:            "DOUBLE PRECISION",
		JSONSchemaType:     "number",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(1),
		Minimum:            scalarInt64Ptr(0),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"0.75"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Generic.StringMap",
		Symbol:             "GenericStringMap",
		Primitive:          "String",
		Description:        "A string-to-string map stored as JSON",
		TypeScriptType:     "Record<string, string>",
		PythonType:         "Dict[str, str]",
		RustType:           "std::collections::HashMap<String, String>",
		GoType:             "map[string]string",
		SQLType:            "JSONB",
		JSONSchemaType:     "object",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{},
		ComparabilityClass: "",
		IsSortable:         false,
	},
	{
		CanonicalName:      "Geo.Location",
		Symbol:             "GeoLocation",
		Primitive:          "String",
		Description:        "Geographic location with latitude and longitude",
		TypeScriptType:     "{ lat: number; lon: number }",
		PythonType:         "superscalar.GeoLocation",
		RustType:           "superscalar::metadata::geo_location::Location",
		GoType:             "struct{ Lat float64 `json:\"lat\"`; Lon float64 `json:\"lon\"` }",
		SQLType:            "POINT",
		JSONSchemaType:     "object",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"{\"lat\":37.7749,\"lon\":-122.4194}"},
		ComparabilityClass: "",
		IsSortable:         false,
	},
	{
		CanonicalName:      "Git.PathPattern",
		Symbol:             "GitPathPattern",
		Primitive:          "String",
		Description:        "Repository-rooted, case-sensitive gitignore-style path pattern",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(1024)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          1024,
		MinLength:          2,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^!?/[^\\x00\\r\\n]+$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"/skills/**", "!/skills/shared/**", "/assets/"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Identity.Name",
		Symbol:             "IdentityName",
		Primitive:          "String",
		Description:        "An objects name",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(80)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          80,
		MinLength:          2,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"A Name"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Identity.Slug",
		Symbol:             "IdentitySlug",
		Primitive:          "String",
		Description:        "A URL friendly version of a string",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "CITEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          255,
		MinLength:          1,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-z0-9]+(?:[-_][a-z0-9]+)*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"acme-corp"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Identity.UUID",
		Symbol:             "IdentityUUID",
		Primitive:          "String",
		Description:        "UUID v4 with automatic base62 encoding for client-facing APIs",
		TypeScriptType:     "string",
		PythonType:         "uuid.UUID",
		RustType:           "uuid::Uuid",
		GoType:             "UUID",
		SQLType:            "UUID",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^([0-9A-Za-z]{1,22}|[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"YQJpYwUwvbaLOwTUr4thA"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Identity.UserID",
		Symbol:             "IdentityUserID",
		Primitive:          "String",
		Description:        "UUID v4 string as base62",
		TypeScriptType:     "string",
		PythonType:         "uuid.UUID",
		RustType:           "uuid::Uuid",
		GoType:             "UUID",
		SQLType:            "UUID",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[0-9A-Za-z]{1,22}$",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"YQJpYwUwvbaLOwTUr4thA"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Localization.Locale",
		Symbol:             "LocalizationLocale",
		Primitive:          "String",
		Description:        "BCP 47 language tag (e.g., en-US, fr-FR)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(35)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          35,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-z]{2,3}(-[A-Z][a-z]{3})?(-[A-Z]{2})?$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"en-US"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Network.DnsLabel",
		Symbol:             "NetworkDnsLabel",
		Primitive:          "String",
		Description:        "Single DNS label (RFC 1035): one hostname segment, no dots",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "CITEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          63,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"mycompany"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Network.DomainName",
		Symbol:             "NetworkDomainName",
		Primitive:          "String",
		Description:        "Valid domain name (RFC 1035 compliant)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "CITEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          255,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?(\\.[a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?)*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"example.com"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Network.IpAddress",
		Symbol:             "NetworkIpAddress",
		Primitive:          "String",
		Description:        "IPv4 or IPv6 address",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "INET",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          45,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^(((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)|([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"192.168.1.1"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Network.Uri",
		Symbol:             "NetworkUri",
		Primitive:          "String",
		Description:        "RFC 3986 URI for connection strings and non-HTTP resources",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          4096,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[a-zA-Z][a-zA-Z0-9+.-]*://[^\\s]+$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"postgres://user:pass@localhost:5432/dbname"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Network.Url",
		Symbol:             "NetworkUrl",
		Primitive:          "String",
		Description:        "Valid HTTP/HTTPS URL",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "varchar(4096)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          2048,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^https?://[\\w\\-\\{\\}]+(\\.[\\w\\-\\{\\}]+)+([:/?#][\\w\\-\\._~:/?#\\[\\]@!\\$&'\\(\\)\\*\\+,;=\\{\\}%]*)?$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"https://www.example.com/example/path"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Ordering.Rank",
		Symbol:             "OrderingRank",
		Primitive:          "Int",
		Description:        "Positive JavaScript-safe ordering rank",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(1),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"1", "1000"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.CronExpression",
		Symbol:             "TemporalCronExpression",
		Primitive:          "String",
		Description:        "Standard 5-field cron expression for scheduling",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(100)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          100,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[0-9*,\\-\\/]+\\s+[0-9*,\\-\\/]+\\s+[0-9*,\\-\\/?]+\\s+[0-9*,\\-\\/A-Za-z]+\\s+[0-9*,\\-\\/A-Za-z]+$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"0 9 * * MON-FRI"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Date",
		Symbol:             "TemporalDate",
		Primitive:          "String",
		Description:        "Calendar date, normalized to ISO 'YYYY-MM-DD'. Accepts ISO ('2025-01-01'), slash-separated ('2025/01/15', '01/15/2025'), named-month ('January 15, 2025', 'Jan 15, 2025'), and full RFC3339 datetime (the time portion is dropped).",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "DATE",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          40,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"2025-01-01"},
		ComparabilityClass: "temporal_instant",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.DateTime",
		Symbol:             "TemporalDateTime",
		Primitive:          "String",
		Description:        "ISO8601 datetime string. Epoch wire values keep this scalar and declare x-temporal-format (unix, unix_millis, unix_micros, unix_nanos) on the property; the unit is never guessed from digit count.",
		TypeScriptType:     "JSDate",
		PythonType:         "datetime.datetime",
		RustType:           "chrono::DateTime<chrono::Utc>",
		GoType:             "time.Time",
		SQLType:            "TIMESTAMPTZ",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"2025-01-01T12:00:00Z"},
		ComparabilityClass: "temporal_instant",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Days",
		Symbol:             "TemporalDays",
		Primitive:          "Int",
		Description:        "Signed integer count of days: an amount of elapsed time, never a point in time or a calendar date.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"7"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Duration",
		Symbol:             "TemporalDuration",
		Primitive:          "String",
		Description:        "Duration for timeouts and intervals",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "time.Duration",
		SQLType:            "INTERVAL",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          32,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^(\\d+(\\.\\d+)?(ns|us|µs|ms|s|m|h))+$",
		HasCustomNormalize: true,
		HasCustomParse:     true,
		HasCustomValidate:  true,
		HasValidator:       true,
		Examples:           []string{"30s"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Hours",
		Symbol:             "TemporalHours",
		Primitive:          "Int",
		Description:        "Signed integer count of hours: an amount of elapsed time, never a point in time.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"2"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Milliseconds",
		Symbol:             "TemporalMilliseconds",
		Primitive:          "Int",
		Description:        "Signed integer count of milliseconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix_millis.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"1000"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Minutes",
		Symbol:             "TemporalMinutes",
		Primitive:          "Int",
		Description:        "Signed integer count of minutes: an amount of elapsed time, never a point in time.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"5"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Month",
		Symbol:             "TemporalMonth",
		Primitive:          "String",
		Description:        "Calendar month, normalized to two-digit numeric (01-12). Accepts '2', '02', 'Feb', 'February' (case-insensitive).",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(2)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          9,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"02"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Quarter",
		Symbol:             "TemporalQuarter",
		Primitive:          "String",
		Description:        "Calendar quarter (Q1-Q4)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(2)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          2,
		MinLength:          2,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[Qq][1-4]$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"Q1"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.QuarterYear",
		Symbol:             "TemporalQuarterYear",
		Primitive:          "String",
		Description:        "Quarter and year, normalized to 'YYYY-Q#'. Accepts '2025-Q1', 'Q1/2025', 'Q1-2025', 'Q1-25' (2-digit years interpreted as 20XX).",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(7)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          8,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"2025-Q1"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.RecurrenceRule",
		Symbol:             "TemporalRecurrenceRule",
		Primitive:          "String",
		Description:        "RFC 5545 recurrence rule, without DTSTART",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(512)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          512,
		MinLength:          6,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"FREQ=WEEKLY;BYMINUTE=0;BYHOUR=9;BYDAY=MO,WE,FR"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Seconds",
		Symbol:             "TemporalSeconds",
		Primitive:          "Int",
		Description:        "Signed integer count of seconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix.",
		TypeScriptType:     "number",
		PythonType:         "int",
		RustType:           "i64",
		GoType:             "int64",
		SQLType:            "BIGINT",
		JSONSchemaType:     "integer",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            scalarInt64Ptr(9007199254740991),
		Minimum:            scalarInt64Ptr(-9007199254740991),
		Pattern:            "",
		HasCustomNormalize: false,
		HasCustomParse:     true,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"60"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Time",
		Symbol:             "TemporalTime",
		Primitive:          "String",
		Description:        "Time of day. 24-hour 'HH:MM' or 'HH:MM:SS' (hours 00-23), or 12-hour 'H:MM'/'HH:MM' with optional ':SS' and required AM/PM suffix (hours 1-12). Seconds and the AM/PM separator space are optional.",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TIME",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^(?:(?:[01][0-9]|2[0-3]):[0-5][0-9](?::[0-5][0-9])?|(?:0?[1-9]|1[0-2]):[0-5][0-9](?::[0-5][0-9])?\\s?[AaPp][Mm])$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"12:25"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.TimeZone",
		Symbol:             "TemporalTimeZone",
		Primitive:          "String",
		Description:        "IANA timezone identifier (e.g., America/New_York, UTC, Etc/UTC)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(100)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          100,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^(?:UTC|[A-Za-z]+/[A-Za-z_/]+)$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"America/New_York", "UTC", "Etc/UTC"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Temporal.Year",
		Symbol:             "TemporalYear",
		Primitive:          "String",
		Description:        "Calendar year as a 4-digit string (e.g., 2025)",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(4)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          4,
		MinLength:          4,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[1-9]\\d{3}$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"2025"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Text.Markdown",
		Symbol:             "TextMarkdown",
		Primitive:          "String",
		Description:        "Markdown text",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          1,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[\\s\\S]*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"## Hello, World!"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Text.Sql",
		Symbol:             "TextSql",
		Primitive:          "String",
		Description:        "SQL text",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "TEXT",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          0,
		MinLength:          1,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^[\\s\\S]*$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"SELECT 1"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Version.SemVer",
		Symbol:             "VersionSemVer",
		Primitive:          "String",
		Description:        "Canonical Semantic Versioning 2.0.0 value",
		TypeScriptType:     "string",
		PythonType:         "str",
		RustType:           "String",
		GoType:             "string",
		SQLType:            "VARCHAR(255)",
		JSONSchemaType:     "string",
		Format:             "",
		MaxLength:          255,
		MinLength:          5,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)(?:-((?:0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)(?:\\.(?:0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*))*))?(?:\\+([0-9A-Za-z-]+(?:\\.[0-9A-Za-z-]+)*))?$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       true,
		Examples:           []string{"1.0.0", "2.4.1-rc.1+build.9"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
}

var ScalarMetadataByCanonical = func() map[string]*ScalarMetadata {
	m := make(map[string]*ScalarMetadata, len(SCALAR_METADATA))
	for i := range SCALAR_METADATA {
		m[SCALAR_METADATA[i].CanonicalName] = &SCALAR_METADATA[i]
	}
	return m
}()

// PrimitiveKindNames is one PrimitiveKind of the Rust core and both of the
// spellings it is written in.
//
// The two differ, and the difference is not cosmetic. A primitive named as a
// `typeRef` inside a schema IR -- a derived output schema, for example -- reads
// `Boolean` and `JSON`; the same primitive in the schema DSL reads `Bool` and
// `Type`. Emitting the pair keeps a Go reader from restating the mapping and
// becoming a mirror of the core that drifts.
type PrimitiveKindNames struct {
	// Name is the Rust member name, carried so a failure can say which
	// primitive it was about.
	Name string
	// TypeRefName is the schema-IR `typeRef` spelling.
	TypeRefName string
	// DSLName is the schema DSL spelling, which is also what a schema field's
	// primitive carries on the wire.
	DSLName string
}

// PRIMITIVE_KINDS is every primitive the Rust core knows, in declaration order.
var PRIMITIVE_KINDS = []PrimitiveKindNames{
	{Name: "String", TypeRefName: "String", DSLName: "String"},
	{Name: "Int", TypeRefName: "Int", DSLName: "Int"},
	{Name: "Float", TypeRefName: "Float", DSLName: "Float"},
	{Name: "Bool", TypeRefName: "Boolean", DSLName: "Bool"},
	{Name: "Object", TypeRefName: "JSON", DSLName: "Type"},
}

// PrimitiveDSLNameByTypeRefName turns a schema-IR type-ref name into the DSL
// primitive name. Absent for a `scalars/{canonical}` reference, which names a
// scalar rather than a bare primitive and is resolved through
// ScalarMetadataByCanonical instead.
var PrimitiveDSLNameByTypeRefName = func() map[string]string {
	m := make(map[string]string, len(PRIMITIVE_KINDS))
	for _, primitive := range PRIMITIVE_KINDS {
		m[primitive.TypeRefName] = primitive.DSLName
	}
	return m
}()

// scalarAliasTargets maps an alias scalar's canonical name to its target's.
// `alias_of` means one implementation under two ids, so the TARGET owns the
// comparability class and the alias inherits it. ComparableWith resolves through
// this map before it reads a class; reading the alias row's own class instead
// breaks transitivity one hop out (the alias pair itself still answers true
// because identity fires first, so the inconsistency hides where it was made).
var scalarAliasTargets = map[string]string{
	"Identity.UserID": "Identity.UUID",
}

// ComparableWith reports whether comparing or joining values of the two named
// scalars is meaningful. It mirrors ScalarDef::comparable_with in the Rust core:
// aliases resolve first, a scalar is always comparable with itself, and two
// distinct scalars are comparable only when both declare the same named class.
//
// Do NOT reimplement this as ComparabilityClass equality. Almost every scalar
// carries the empty class, so field equality makes
// the entire catalog mutually comparable, Contact.Email against
// Contact.PhoneNumber included, which is the exact comparison the relation
// exists to reject. The empty string is also what a ScalarMetadataByCanonical
// miss yields, so field equality cannot tell "no class" from "no such scalar".
//
// Fails closed on a name with no metadata row: an unknown scalar, and a
// scalar whose def sets metadata_omit and so has no comparability answer in
// this binding, both report false even against themselves. That matches the
// is_sortable allowlist, which answers "no" to a shape nobody declared.
func ComparableWith(a, b string) bool {
	if target, ok := scalarAliasTargets[a]; ok {
		a = target
	}
	if target, ok := scalarAliasTargets[b]; ok {
		b = target
	}
	metaA, metaB := ScalarMetadataByCanonical[a], ScalarMetadataByCanonical[b]
	if metaA == nil || metaB == nil {
		return false
	}
	if a == b {
		return true
	}
	return metaA.ComparabilityClass != "" && metaA.ComparabilityClass == metaB.ComparabilityClass
}

func patternForCanonical(canonical string) string {
	meta := ScalarMetadataByCanonical[canonical]
	if meta == nil {
		return ""
	}
	return meta.Pattern
}

var AgentSkillNamePattern = ScalarPattern(patternForCanonical("AgentSkill.Name"))
var AuthJWTPattern = ScalarPattern(patternForCanonical("Auth.JWT"))
var ContactEmailPattern = ScalarPattern(patternForCanonical("Contact.Email"))
var ContactPhoneNumberPattern = ScalarPattern(patternForCanonical("Contact.PhoneNumber"))
var CryptoRSAPrivateKeyPattern = ScalarPattern(patternForCanonical("Crypto.RSAPrivateKey"))
var CryptoRSAPublicKeyPattern = ScalarPattern(patternForCanonical("Crypto.RSAPublicKey"))
var CryptoSHA256Pattern = ScalarPattern(patternForCanonical("Crypto.SHA256"))
var DesignColorPattern = ScalarPattern(patternForCanonical("Design.Color"))
var GitPathPatternPattern = ScalarPattern(patternForCanonical("Git.PathPattern"))
var IdentitySlugPattern = ScalarPattern(patternForCanonical("Identity.Slug"))
var IdentityUUIDPattern = ScalarPattern(patternForCanonical("Identity.UUID"))
var IdentityUserIDPattern = ScalarPattern(patternForCanonical("Identity.UserID"))
var LocalizationLocalePattern = ScalarPattern(patternForCanonical("Localization.Locale"))
var NetworkDnsLabelPattern = ScalarPattern(patternForCanonical("Network.DnsLabel"))
var NetworkDomainNamePattern = ScalarPattern(patternForCanonical("Network.DomainName"))
var NetworkIpAddressPattern = ScalarPattern(patternForCanonical("Network.IpAddress"))
var NetworkUriPattern = ScalarPattern(patternForCanonical("Network.Uri"))
var NetworkUrlPattern = ScalarPattern(patternForCanonical("Network.Url"))
var TemporalCronExpressionPattern = ScalarPattern(patternForCanonical("Temporal.CronExpression"))
var TemporalDurationPattern = ScalarPattern(patternForCanonical("Temporal.Duration"))
var TemporalQuarterPattern = ScalarPattern(patternForCanonical("Temporal.Quarter"))
var TemporalTimePattern = ScalarPattern(patternForCanonical("Temporal.Time"))
var TemporalTimeZonePattern = ScalarPattern(patternForCanonical("Temporal.TimeZone"))
var TemporalYearPattern = ScalarPattern(patternForCanonical("Temporal.Year"))
var TextMarkdownPattern = ScalarPattern(patternForCanonical("Text.Markdown"))
var TextSqlPattern = ScalarPattern(patternForCanonical("Text.Sql"))
var VersionSemVerPattern = ScalarPattern(patternForCanonical("Version.SemVer"))

func validationErrorsFromError(err error) []ValidationError {
	if err == nil {
		return nil
	}
	message := err.Error()
	validator := "scalar"
	if strings.Contains(message, "reserved") {
		validator = "reservedWord"
	} else if strings.HasPrefix(message, "pattern:") {
		validator = "pattern"
	} else if strings.HasPrefix(message, "length:") {
		validator = "length"
	} else if strings.HasPrefix(message, "range:") {
		validator = "range"
	} else if strings.HasPrefix(message, "enum:") {
		validator = "enum"
	} else if strings.HasPrefix(message, "empty:") {
		validator = "required"
	} else if strings.HasPrefix(message, "custom:") {
		validator = "custom"
	} else if strings.HasPrefix(message, "parse:") {
		validator = "parse"
	}
	return []ValidationError{{Validator: validator, Message: message}}
}

func validateScalarValue(canonical string, value string) (bool, []ValidationError) {
	if err := callScalarValidate(canonical, value); err != nil {
		return false, validationErrorsFromError(err)
	}
	return true, nil
}

// scalarRequiredValueMissing reports whether a required scalar carries no value.
// Numeric and boolean kinds are exempt: 0 and false are legitimate values that
// Go cannot distinguish from an absent JSON key, so treating them as missing
// rejects valid payloads. The Python (`is None`) and TypeScript (`=== null`)
// bindings already accept them; this keeps Go in parity. GeoLocation is exempt
// for the same reason: its zero value is the point {"lat":0,"lon":0}.
func scalarRequiredValueMissing(value any) bool {
	if _, ok := value.(GeoLocation); ok {
		return false
	}
	rv := reflect.ValueOf(value)
	if !rv.IsValid() {
		return true
	}
	switch rv.Kind() {
	case reflect.Bool,
		reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64,
		reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64,
		reflect.Float32, reflect.Float64:
		return false
	}
	return rv.IsZero()
}

func scalarStringValue(value any) string {
	switch v := value.(type) {
	case EmbeddingVector:
		encoded, err := json.Marshal([]float32(v))
		if err == nil {
			return string(encoded)
		}
	case GenericJSON:
		return string(v)
	case GenericStringMap:
		encoded, err := json.Marshal(map[string]string(v))
		if err == nil {
			return string(encoded)
		}
	case GeoLocation:
		encoded, err := json.Marshal(v)
		if err == nil {
			return string(encoded)
		}
	case TemporalDateTime:
		return time.Time(v).Format(time.RFC3339Nano)
	case TemporalDuration:
		return time.Duration(v).String()
	}
	// Format via the underlying kind: fmt.Sprint on a scalar type whose
	// String() delegates back here would recurse infinitely.
	rv := reflect.ValueOf(value)
	switch rv.Kind() {
	case reflect.String:
		return rv.String()
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		return strconv.FormatInt(rv.Int(), 10)
	case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
		return strconv.FormatUint(rv.Uint(), 10)
	case reflect.Float32:
		return strconv.FormatFloat(rv.Float(), 'g', -1, 32)
	case reflect.Float64:
		return strconv.FormatFloat(rv.Float(), 'g', -1, 64)
	case reflect.Bool:
		return strconv.FormatBool(rv.Bool())
	}
	return fmt.Sprint(value)
}

func ValidatorFor(canonical string) func(string) error {
	switch canonical {
	case "AgentSkill.Name":
		return func(value string) error { return callScalarValidate(scalarNameAgentSkillName, value) }
	case "Auth.JWT":
		return func(value string) error { return callScalarValidate(scalarNameAuthJWT, value) }
	case "Auth.Password":
		return func(value string) error { return callScalarValidate(scalarNameAuthPassword, value) }
	case "Contact.Email":
		return func(value string) error { return callScalarValidate(scalarNameContactEmail, value) }
	case "Contact.PhoneNumber":
		return func(value string) error { return callScalarValidate(scalarNameContactPhoneNumber, value) }
	case "Crypto.RSAPrivateKey":
		return func(value string) error { return callScalarValidate(scalarNameCryptoRSAPrivateKey, value) }
	case "Crypto.RSAPublicKey":
		return func(value string) error { return callScalarValidate(scalarNameCryptoRSAPublicKey, value) }
	case "Crypto.SHA256":
		return func(value string) error { return callScalarValidate(scalarNameCryptoSHA256, value) }
	case "Design.Color":
		return func(value string) error { return callScalarValidate(scalarNameDesignColor, value) }
	case "Embedding.Vector":
		return func(value string) error { return callScalarValidate(scalarNameEmbeddingVector, value) }
	case "File.SizeBytes":
		return func(value string) error { return callScalarValidate(scalarNameFileSizeBytes, value) }
	case "Finance.Money":
		return func(value string) error { return callScalarValidate(scalarNameFinanceMoney, value) }
	case "Generic.Int64":
		return func(value string) error { return callScalarValidate(scalarNameGenericInt64, value) }
	case "Generic.JSON":
		return func(value string) error { return callScalarValidate(scalarNameGenericJSON, value) }
	case "Generic.Probability":
		return func(value string) error { return callScalarValidate(scalarNameGenericProbability, value) }
	case "Generic.StringMap":
		return func(value string) error { return callScalarValidate(scalarNameGenericStringMap, value) }
	case "Geo.Location":
		return func(value string) error { return callScalarValidate(scalarNameGeoLocation, value) }
	case "Git.PathPattern":
		return func(value string) error { return callScalarValidate(scalarNameGitPathPattern, value) }
	case "Identity.Name":
		return func(value string) error { return callScalarValidate(scalarNameIdentityName, value) }
	case "Identity.Slug":
		return func(value string) error { return callScalarValidate(scalarNameIdentitySlug, value) }
	case "Identity.UUID":
		return func(value string) error { return callScalarValidate(scalarNameIdentityUUID, value) }
	case "Identity.UserID":
		return func(value string) error { return callScalarValidate(scalarNameIdentityUserID, value) }
	case "Localization.Locale":
		return func(value string) error { return callScalarValidate(scalarNameLocalizationLocale, value) }
	case "Network.DnsLabel":
		return func(value string) error { return callScalarValidate(scalarNameNetworkDnsLabel, value) }
	case "Network.DomainName":
		return func(value string) error { return callScalarValidate(scalarNameNetworkDomainName, value) }
	case "Network.IpAddress":
		return func(value string) error { return callScalarValidate(scalarNameNetworkIpAddress, value) }
	case "Network.Uri":
		return func(value string) error { return callScalarValidate(scalarNameNetworkUri, value) }
	case "Network.Url":
		return func(value string) error { return callScalarValidate(scalarNameNetworkUrl, value) }
	case "Ordering.Rank":
		return func(value string) error { return callScalarValidate(scalarNameOrderingRank, value) }
	case "Temporal.CronExpression":
		return func(value string) error { return callScalarValidate(scalarNameTemporalCronExpression, value) }
	case "Temporal.Date":
		return func(value string) error { return callScalarValidate(scalarNameTemporalDate, value) }
	case "Temporal.DateTime":
		return func(value string) error { return callScalarValidate(scalarNameTemporalDateTime, value) }
	case "Temporal.Days":
		return func(value string) error { return callScalarValidate(scalarNameTemporalDays, value) }
	case "Temporal.Duration":
		return func(value string) error { return callScalarValidate(scalarNameTemporalDuration, value) }
	case "Temporal.Hours":
		return func(value string) error { return callScalarValidate(scalarNameTemporalHours, value) }
	case "Temporal.Milliseconds":
		return func(value string) error { return callScalarValidate(scalarNameTemporalMilliseconds, value) }
	case "Temporal.Minutes":
		return func(value string) error { return callScalarValidate(scalarNameTemporalMinutes, value) }
	case "Temporal.Month":
		return func(value string) error { return callScalarValidate(scalarNameTemporalMonth, value) }
	case "Temporal.Quarter":
		return func(value string) error { return callScalarValidate(scalarNameTemporalQuarter, value) }
	case "Temporal.QuarterYear":
		return func(value string) error { return callScalarValidate(scalarNameTemporalQuarterYear, value) }
	case "Temporal.RecurrenceRule":
		return func(value string) error { return callScalarValidate(scalarNameTemporalRecurrenceRule, value) }
	case "Temporal.Seconds":
		return func(value string) error { return callScalarValidate(scalarNameTemporalSeconds, value) }
	case "Temporal.Time":
		return func(value string) error { return callScalarValidate(scalarNameTemporalTime, value) }
	case "Temporal.TimeZone":
		return func(value string) error { return callScalarValidate(scalarNameTemporalTimeZone, value) }
	case "Temporal.Year":
		return func(value string) error { return callScalarValidate(scalarNameTemporalYear, value) }
	case "Text.Markdown":
		return func(value string) error { return callScalarValidate(scalarNameTextMarkdown, value) }
	case "Text.Sql":
		return func(value string) error { return callScalarValidate(scalarNameTextSql, value) }
	case "Version.SemVer":
		return func(value string) error { return callScalarValidate(scalarNameVersionSemVer, value) }
	default:
		return nil
	}
}

// ParseAgentSkillName validates and returns the canonical form of a AgentSkill.Name value.
func ParseAgentSkillName(value string) (string, error) {
	return callScalarParse(scalarNameAgentSkillName, value)
}

// NormalizeAgentSkillName returns the canonical form of a AgentSkill.Name value without enforcing shape.
func NormalizeAgentSkillName(value string) (string, error) {
	return callScalarNormalize(scalarNameAgentSkillName, value)
}

// ValidateAgentSkillName enforces the shape of a AgentSkill.Name value.
func ValidateAgentSkillName(value string) error {
	return callScalarValidate(scalarNameAgentSkillName, value)
}

func (v AgentSkillName) String() string {
	return string(v)
}

func (v AgentSkillName) ToLower() string {
	return strings.ToLower(string(v))
}

func (v AgentSkillName) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v AgentSkillName) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates AgentSkillName and returns whether validation passed along with any errors.
func (v AgentSkillName) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameAgentSkillName, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v AgentSkillName) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseAuthJWT validates and returns the canonical form of a Auth.JWT value.
func ParseAuthJWT(value string) (string, error) { return callScalarParse(scalarNameAuthJWT, value) }

// NormalizeAuthJWT returns the canonical form of a Auth.JWT value without enforcing shape.
func NormalizeAuthJWT(value string) (string, error) {
	return callScalarNormalize(scalarNameAuthJWT, value)
}

// ValidateAuthJWT enforces the shape of a Auth.JWT value.
func ValidateAuthJWT(value string) error { return callScalarValidate(scalarNameAuthJWT, value) }

func (v AuthJWT) String() string {
	return string(v)
}

func (v AuthJWT) ToLower() string {
	return strings.ToLower(string(v))
}

func (v AuthJWT) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v AuthJWT) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates AuthJWT and returns whether validation passed along with any errors.
func (v AuthJWT) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameAuthJWT, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v AuthJWT) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseAuthPassword validates and returns the canonical form of a Auth.Password value.
func ParseAuthPassword(value string) (string, error) {
	return callScalarParse(scalarNameAuthPassword, value)
}

// NormalizeAuthPassword returns the canonical form of a Auth.Password value without enforcing shape.
func NormalizeAuthPassword(value string) (string, error) {
	return callScalarNormalize(scalarNameAuthPassword, value)
}

// ValidateAuthPassword enforces the shape of a Auth.Password value.
func ValidateAuthPassword(value string) error {
	return callScalarValidate(scalarNameAuthPassword, value)
}

func (v AuthPassword) String() string {
	return string(v)
}

func (v AuthPassword) ToLower() string {
	return strings.ToLower(string(v))
}

func (v AuthPassword) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v AuthPassword) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates AuthPassword and returns whether validation passed along with any errors.
func (v AuthPassword) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameAuthPassword, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v AuthPassword) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseContactEmail validates and returns the canonical form of a Contact.Email value.
func ParseContactEmail(value string) (string, error) {
	return callScalarParse(scalarNameContactEmail, value)
}

// NormalizeContactEmail returns the canonical form of a Contact.Email value without enforcing shape.
func NormalizeContactEmail(value string) (string, error) {
	return callScalarNormalize(scalarNameContactEmail, value)
}

// ValidateContactEmail enforces the shape of a Contact.Email value.
func ValidateContactEmail(value string) error {
	return callScalarValidate(scalarNameContactEmail, value)
}

func (v ContactEmail) String() string {
	return string(v)
}

func (v ContactEmail) ToLower() string {
	return strings.ToLower(string(v))
}

func (v ContactEmail) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v ContactEmail) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates ContactEmail and returns whether validation passed along with any errors.
func (v ContactEmail) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameContactEmail, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v ContactEmail) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseContactPhoneNumber validates and returns the canonical form of a Contact.PhoneNumber value.
func ParseContactPhoneNumber(value string) (string, error) {
	return callScalarParse(scalarNameContactPhoneNumber, value)
}

// NormalizeContactPhoneNumber returns the canonical form of a Contact.PhoneNumber value without enforcing shape.
func NormalizeContactPhoneNumber(value string) (string, error) {
	return callScalarNormalize(scalarNameContactPhoneNumber, value)
}

// ValidateContactPhoneNumber enforces the shape of a Contact.PhoneNumber value.
func ValidateContactPhoneNumber(value string) error {
	return callScalarValidate(scalarNameContactPhoneNumber, value)
}

func (v ContactPhoneNumber) String() string {
	return string(v)
}

func (v ContactPhoneNumber) ToLower() string {
	return strings.ToLower(string(v))
}

func (v ContactPhoneNumber) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v ContactPhoneNumber) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates ContactPhoneNumber and returns whether validation passed along with any errors.
func (v ContactPhoneNumber) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameContactPhoneNumber, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v ContactPhoneNumber) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoRSAPrivateKey validates and returns the canonical form of a Crypto.RSAPrivateKey value.
func ParseCryptoRSAPrivateKey(value string) (string, error) {
	return callScalarParse(scalarNameCryptoRSAPrivateKey, value)
}

// NormalizeCryptoRSAPrivateKey returns the canonical form of a Crypto.RSAPrivateKey value without enforcing shape.
func NormalizeCryptoRSAPrivateKey(value string) (string, error) {
	return callScalarNormalize(scalarNameCryptoRSAPrivateKey, value)
}

// ValidateCryptoRSAPrivateKey enforces the shape of a Crypto.RSAPrivateKey value.
func ValidateCryptoRSAPrivateKey(value string) error {
	return callScalarValidate(scalarNameCryptoRSAPrivateKey, value)
}

func (v CryptoRSAPrivateKey) String() string {
	return string(v)
}

func (v CryptoRSAPrivateKey) ToLower() string {
	return strings.ToLower(string(v))
}

func (v CryptoRSAPrivateKey) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v CryptoRSAPrivateKey) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates CryptoRSAPrivateKey and returns whether validation passed along with any errors.
func (v CryptoRSAPrivateKey) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameCryptoRSAPrivateKey, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoRSAPrivateKey) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoRSAPublicKey validates and returns the canonical form of a Crypto.RSAPublicKey value.
func ParseCryptoRSAPublicKey(value string) (string, error) {
	return callScalarParse(scalarNameCryptoRSAPublicKey, value)
}

// NormalizeCryptoRSAPublicKey returns the canonical form of a Crypto.RSAPublicKey value without enforcing shape.
func NormalizeCryptoRSAPublicKey(value string) (string, error) {
	return callScalarNormalize(scalarNameCryptoRSAPublicKey, value)
}

// ValidateCryptoRSAPublicKey enforces the shape of a Crypto.RSAPublicKey value.
func ValidateCryptoRSAPublicKey(value string) error {
	return callScalarValidate(scalarNameCryptoRSAPublicKey, value)
}

func (v CryptoRSAPublicKey) String() string {
	return string(v)
}

func (v CryptoRSAPublicKey) ToLower() string {
	return strings.ToLower(string(v))
}

func (v CryptoRSAPublicKey) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v CryptoRSAPublicKey) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates CryptoRSAPublicKey and returns whether validation passed along with any errors.
func (v CryptoRSAPublicKey) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameCryptoRSAPublicKey, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoRSAPublicKey) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoSHA256 validates and returns the canonical form of a Crypto.SHA256 value.
func ParseCryptoSHA256(value string) (string, error) {
	return callScalarParse(scalarNameCryptoSHA256, value)
}

// NormalizeCryptoSHA256 returns the canonical form of a Crypto.SHA256 value without enforcing shape.
func NormalizeCryptoSHA256(value string) (string, error) {
	return callScalarNormalize(scalarNameCryptoSHA256, value)
}

// ValidateCryptoSHA256 enforces the shape of a Crypto.SHA256 value.
func ValidateCryptoSHA256(value string) error {
	return callScalarValidate(scalarNameCryptoSHA256, value)
}

func (v CryptoSHA256) String() string {
	return string(v)
}

func (v CryptoSHA256) ToLower() string {
	return strings.ToLower(string(v))
}

func (v CryptoSHA256) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v CryptoSHA256) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates CryptoSHA256 and returns whether validation passed along with any errors.
func (v CryptoSHA256) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameCryptoSHA256, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoSHA256) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseDesignColor validates and returns the canonical form of a Design.Color value.
func ParseDesignColor(value string) (string, error) {
	return callScalarParse(scalarNameDesignColor, value)
}

// NormalizeDesignColor returns the canonical form of a Design.Color value without enforcing shape.
func NormalizeDesignColor(value string) (string, error) {
	return callScalarNormalize(scalarNameDesignColor, value)
}

// ValidateDesignColor enforces the shape of a Design.Color value.
func ValidateDesignColor(value string) error { return callScalarValidate(scalarNameDesignColor, value) }

func (v DesignColor) String() string {
	return string(v)
}

func (v DesignColor) ToLower() string {
	return strings.ToLower(string(v))
}

func (v DesignColor) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v DesignColor) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates DesignColor and returns whether validation passed along with any errors.
func (v DesignColor) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameDesignColor, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v DesignColor) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseEmbeddingVector validates and returns the canonical form of a Embedding.Vector value.
func ParseEmbeddingVector(value string) (string, error) {
	return callScalarParse(scalarNameEmbeddingVector, value)
}

// NormalizeEmbeddingVector returns the canonical form of a Embedding.Vector value without enforcing shape.
func NormalizeEmbeddingVector(value string) (string, error) {
	return callScalarNormalize(scalarNameEmbeddingVector, value)
}

// ValidateEmbeddingVector enforces the shape of a Embedding.Vector value.
func ValidateEmbeddingVector(value string) error {
	return callScalarValidate(scalarNameEmbeddingVector, value)
}

func (v EmbeddingVector) String() string {
	return scalarStringValue(v)
}

// Validate validates EmbeddingVector and returns whether validation passed along with any errors.
func (v EmbeddingVector) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameEmbeddingVector, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v EmbeddingVector) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseFileSizeBytes validates and returns the canonical form of a File.SizeBytes value.
func ParseFileSizeBytes(value string) (string, error) {
	return callScalarParse(scalarNameFileSizeBytes, value)
}

// NormalizeFileSizeBytes returns the canonical form of a File.SizeBytes value without enforcing shape.
func NormalizeFileSizeBytes(value string) (string, error) {
	return callScalarNormalize(scalarNameFileSizeBytes, value)
}

// ValidateFileSizeBytes enforces the shape of a File.SizeBytes value.
func ValidateFileSizeBytes(value string) error {
	return callScalarValidate(scalarNameFileSizeBytes, value)
}

func (v FileSizeBytes) String() string {
	return scalarStringValue(v)
}

// Validate validates FileSizeBytes and returns whether validation passed along with any errors.
func (v FileSizeBytes) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameFileSizeBytes, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v FileSizeBytes) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseFinanceMoney validates and returns the canonical form of a Finance.Money value.
func ParseFinanceMoney(value string) (string, error) {
	return callScalarParse(scalarNameFinanceMoney, value)
}

// NormalizeFinanceMoney returns the canonical form of a Finance.Money value without enforcing shape.
func NormalizeFinanceMoney(value string) (string, error) {
	return callScalarNormalize(scalarNameFinanceMoney, value)
}

// ValidateFinanceMoney enforces the shape of a Finance.Money value.
func ValidateFinanceMoney(value string) error {
	return callScalarValidate(scalarNameFinanceMoney, value)
}

func (v FinanceMoney) String() string {
	return scalarStringValue(v)
}

// Validate validates FinanceMoney and returns whether validation passed along with any errors.
func (v FinanceMoney) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameFinanceMoney, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v FinanceMoney) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericInt64 validates and returns the canonical form of a Generic.Int64 value.
func ParseGenericInt64(value string) (string, error) {
	return callScalarParse(scalarNameGenericInt64, value)
}

// NormalizeGenericInt64 returns the canonical form of a Generic.Int64 value without enforcing shape.
func NormalizeGenericInt64(value string) (string, error) {
	return callScalarNormalize(scalarNameGenericInt64, value)
}

// ValidateGenericInt64 enforces the shape of a Generic.Int64 value.
func ValidateGenericInt64(value string) error {
	return callScalarValidate(scalarNameGenericInt64, value)
}

func (v GenericInt64) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericInt64 and returns whether validation passed along with any errors.
func (v GenericInt64) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGenericInt64, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericInt64) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericJSON validates and returns the canonical form of a Generic.JSON value.
func ParseGenericJSON(value string) (string, error) {
	return callScalarParse(scalarNameGenericJSON, value)
}

// NormalizeGenericJSON returns the canonical form of a Generic.JSON value without enforcing shape.
func NormalizeGenericJSON(value string) (string, error) {
	return callScalarNormalize(scalarNameGenericJSON, value)
}

// ValidateGenericJSON enforces the shape of a Generic.JSON value.
func ValidateGenericJSON(value string) error { return callScalarValidate(scalarNameGenericJSON, value) }

func (v GenericJSON) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericJSON and returns whether validation passed along with any errors.
func (v GenericJSON) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGenericJSON, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericJSON) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericProbability validates and returns the canonical form of a Generic.Probability value.
func ParseGenericProbability(value string) (string, error) {
	return callScalarParse(scalarNameGenericProbability, value)
}

// NormalizeGenericProbability returns the canonical form of a Generic.Probability value without enforcing shape.
func NormalizeGenericProbability(value string) (string, error) {
	return callScalarNormalize(scalarNameGenericProbability, value)
}

// ValidateGenericProbability enforces the shape of a Generic.Probability value.
func ValidateGenericProbability(value string) error {
	return callScalarValidate(scalarNameGenericProbability, value)
}

func (v GenericProbability) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericProbability and returns whether validation passed along with any errors.
func (v GenericProbability) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGenericProbability, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericProbability) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericStringMap validates and returns the canonical form of a Generic.StringMap value.
func ParseGenericStringMap(value string) (string, error) {
	return callScalarParse(scalarNameGenericStringMap, value)
}

// NormalizeGenericStringMap returns the canonical form of a Generic.StringMap value without enforcing shape.
func NormalizeGenericStringMap(value string) (string, error) {
	return callScalarNormalize(scalarNameGenericStringMap, value)
}

// ValidateGenericStringMap enforces the shape of a Generic.StringMap value.
func ValidateGenericStringMap(value string) error {
	return callScalarValidate(scalarNameGenericStringMap, value)
}

func (v GenericStringMap) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericStringMap and returns whether validation passed along with any errors.
func (v GenericStringMap) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGenericStringMap, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericStringMap) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGeoLocation validates and returns the canonical form of a Geo.Location value.
func ParseGeoLocation(value string) (string, error) {
	return callScalarParse(scalarNameGeoLocation, value)
}

// NormalizeGeoLocation returns the canonical form of a Geo.Location value without enforcing shape.
func NormalizeGeoLocation(value string) (string, error) {
	return callScalarNormalize(scalarNameGeoLocation, value)
}

// ValidateGeoLocation enforces the shape of a Geo.Location value.
func ValidateGeoLocation(value string) error { return callScalarValidate(scalarNameGeoLocation, value) }

func (v GeoLocation) String() string {
	return scalarStringValue(v)
}

// Validate validates GeoLocation and returns whether validation passed along with any errors.
func (v GeoLocation) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGeoLocation, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GeoLocation) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGitPathPattern validates and returns the canonical form of a Git.PathPattern value.
func ParseGitPathPattern(value string) (string, error) {
	return callScalarParse(scalarNameGitPathPattern, value)
}

// NormalizeGitPathPattern returns the canonical form of a Git.PathPattern value without enforcing shape.
func NormalizeGitPathPattern(value string) (string, error) {
	return callScalarNormalize(scalarNameGitPathPattern, value)
}

// ValidateGitPathPattern enforces the shape of a Git.PathPattern value.
func ValidateGitPathPattern(value string) error {
	return callScalarValidate(scalarNameGitPathPattern, value)
}

func (v GitPathPattern) String() string {
	return string(v)
}

func (v GitPathPattern) ToLower() string {
	return strings.ToLower(string(v))
}

func (v GitPathPattern) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v GitPathPattern) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates GitPathPattern and returns whether validation passed along with any errors.
func (v GitPathPattern) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameGitPathPattern, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GitPathPattern) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentityName validates and returns the canonical form of a Identity.Name value.
func ParseIdentityName(value string) (string, error) {
	return callScalarParse(scalarNameIdentityName, value)
}

// NormalizeIdentityName returns the canonical form of a Identity.Name value without enforcing shape.
func NormalizeIdentityName(value string) (string, error) {
	return callScalarNormalize(scalarNameIdentityName, value)
}

// ValidateIdentityName enforces the shape of a Identity.Name value.
func ValidateIdentityName(value string) error {
	return callScalarValidate(scalarNameIdentityName, value)
}

func (v IdentityName) String() string {
	return string(v)
}

func (v IdentityName) ToLower() string {
	return strings.ToLower(string(v))
}

func (v IdentityName) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v IdentityName) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates IdentityName and returns whether validation passed along with any errors.
func (v IdentityName) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameIdentityName, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v IdentityName) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentitySlug validates and returns the canonical form of a Identity.Slug value.
func ParseIdentitySlug(value string) (string, error) {
	return callScalarParse(scalarNameIdentitySlug, value)
}

// NormalizeIdentitySlug returns the canonical form of a Identity.Slug value without enforcing shape.
func NormalizeIdentitySlug(value string) (string, error) {
	return callScalarNormalize(scalarNameIdentitySlug, value)
}

// ValidateIdentitySlug enforces the shape of a Identity.Slug value.
func ValidateIdentitySlug(value string) error {
	return callScalarValidate(scalarNameIdentitySlug, value)
}

func (v IdentitySlug) String() string {
	return string(v)
}

func (v IdentitySlug) ToLower() string {
	return strings.ToLower(string(v))
}

func (v IdentitySlug) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v IdentitySlug) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates IdentitySlug and returns whether validation passed along with any errors.
func (v IdentitySlug) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameIdentitySlug, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v IdentitySlug) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentityUUID validates and returns the canonical form of a Identity.UUID value.
func ParseIdentityUUID(value string) (string, error) {
	return callScalarParse(scalarNameIdentityUUID, value)
}

// NormalizeIdentityUUID returns the canonical form of a Identity.UUID value without enforcing shape.
func NormalizeIdentityUUID(value string) (string, error) {
	return callScalarNormalize(scalarNameIdentityUUID, value)
}

// ValidateIdentityUUID enforces the shape of a Identity.UUID value.
func ValidateIdentityUUID(value string) error {
	return callScalarValidate(scalarNameIdentityUUID, value)
}

// ParseIdentityUserID validates and returns the canonical form of a Identity.UserID value.
func ParseIdentityUserID(value string) (string, error) {
	return callScalarParse(scalarNameIdentityUserID, value)
}

// NormalizeIdentityUserID returns the canonical form of a Identity.UserID value without enforcing shape.
func NormalizeIdentityUserID(value string) (string, error) {
	return callScalarNormalize(scalarNameIdentityUserID, value)
}

// ValidateIdentityUserID enforces the shape of a Identity.UserID value.
func ValidateIdentityUserID(value string) error {
	return callScalarValidate(scalarNameIdentityUserID, value)
}

// ParseLocalizationLocale validates and returns the canonical form of a Localization.Locale value.
func ParseLocalizationLocale(value string) (string, error) {
	return callScalarParse(scalarNameLocalizationLocale, value)
}

// NormalizeLocalizationLocale returns the canonical form of a Localization.Locale value without enforcing shape.
func NormalizeLocalizationLocale(value string) (string, error) {
	return callScalarNormalize(scalarNameLocalizationLocale, value)
}

// ValidateLocalizationLocale enforces the shape of a Localization.Locale value.
func ValidateLocalizationLocale(value string) error {
	return callScalarValidate(scalarNameLocalizationLocale, value)
}

func (v LocalizationLocale) String() string {
	return string(v)
}

func (v LocalizationLocale) ToLower() string {
	return strings.ToLower(string(v))
}

func (v LocalizationLocale) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v LocalizationLocale) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates LocalizationLocale and returns whether validation passed along with any errors.
func (v LocalizationLocale) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameLocalizationLocale, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v LocalizationLocale) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkDnsLabel validates and returns the canonical form of a Network.DnsLabel value.
func ParseNetworkDnsLabel(value string) (string, error) {
	return callScalarParse(scalarNameNetworkDnsLabel, value)
}

// NormalizeNetworkDnsLabel returns the canonical form of a Network.DnsLabel value without enforcing shape.
func NormalizeNetworkDnsLabel(value string) (string, error) {
	return callScalarNormalize(scalarNameNetworkDnsLabel, value)
}

// ValidateNetworkDnsLabel enforces the shape of a Network.DnsLabel value.
func ValidateNetworkDnsLabel(value string) error {
	return callScalarValidate(scalarNameNetworkDnsLabel, value)
}

func (v NetworkDnsLabel) String() string {
	return string(v)
}

func (v NetworkDnsLabel) ToLower() string {
	return strings.ToLower(string(v))
}

func (v NetworkDnsLabel) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v NetworkDnsLabel) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates NetworkDnsLabel and returns whether validation passed along with any errors.
func (v NetworkDnsLabel) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameNetworkDnsLabel, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkDnsLabel) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkDomainName validates and returns the canonical form of a Network.DomainName value.
func ParseNetworkDomainName(value string) (string, error) {
	return callScalarParse(scalarNameNetworkDomainName, value)
}

// NormalizeNetworkDomainName returns the canonical form of a Network.DomainName value without enforcing shape.
func NormalizeNetworkDomainName(value string) (string, error) {
	return callScalarNormalize(scalarNameNetworkDomainName, value)
}

// ValidateNetworkDomainName enforces the shape of a Network.DomainName value.
func ValidateNetworkDomainName(value string) error {
	return callScalarValidate(scalarNameNetworkDomainName, value)
}

func (v NetworkDomainName) String() string {
	return string(v)
}

func (v NetworkDomainName) ToLower() string {
	return strings.ToLower(string(v))
}

func (v NetworkDomainName) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v NetworkDomainName) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates NetworkDomainName and returns whether validation passed along with any errors.
func (v NetworkDomainName) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameNetworkDomainName, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkDomainName) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkIpAddress validates and returns the canonical form of a Network.IpAddress value.
func ParseNetworkIpAddress(value string) (string, error) {
	return callScalarParse(scalarNameNetworkIpAddress, value)
}

// NormalizeNetworkIpAddress returns the canonical form of a Network.IpAddress value without enforcing shape.
func NormalizeNetworkIpAddress(value string) (string, error) {
	return callScalarNormalize(scalarNameNetworkIpAddress, value)
}

// ValidateNetworkIpAddress enforces the shape of a Network.IpAddress value.
func ValidateNetworkIpAddress(value string) error {
	return callScalarValidate(scalarNameNetworkIpAddress, value)
}

func (v NetworkIpAddress) String() string {
	return string(v)
}

func (v NetworkIpAddress) ToLower() string {
	return strings.ToLower(string(v))
}

func (v NetworkIpAddress) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v NetworkIpAddress) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates NetworkIpAddress and returns whether validation passed along with any errors.
func (v NetworkIpAddress) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameNetworkIpAddress, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkIpAddress) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkUri validates and returns the canonical form of a Network.Uri value.
func ParseNetworkUri(value string) (string, error) {
	return callScalarParse(scalarNameNetworkUri, value)
}

// NormalizeNetworkUri returns the canonical form of a Network.Uri value without enforcing shape.
func NormalizeNetworkUri(value string) (string, error) {
	return callScalarNormalize(scalarNameNetworkUri, value)
}

// ValidateNetworkUri enforces the shape of a Network.Uri value.
func ValidateNetworkUri(value string) error { return callScalarValidate(scalarNameNetworkUri, value) }

func (v NetworkUri) String() string {
	return string(v)
}

func (v NetworkUri) ToLower() string {
	return strings.ToLower(string(v))
}

func (v NetworkUri) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v NetworkUri) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates NetworkUri and returns whether validation passed along with any errors.
func (v NetworkUri) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameNetworkUri, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkUri) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkUrl validates and returns the canonical form of a Network.Url value.
func ParseNetworkUrl(value string) (string, error) {
	return callScalarParse(scalarNameNetworkUrl, value)
}

// NormalizeNetworkUrl returns the canonical form of a Network.Url value without enforcing shape.
func NormalizeNetworkUrl(value string) (string, error) {
	return callScalarNormalize(scalarNameNetworkUrl, value)
}

// ValidateNetworkUrl enforces the shape of a Network.Url value.
func ValidateNetworkUrl(value string) error { return callScalarValidate(scalarNameNetworkUrl, value) }

func (v NetworkUrl) String() string {
	return string(v)
}

func (v NetworkUrl) ToLower() string {
	return strings.ToLower(string(v))
}

func (v NetworkUrl) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v NetworkUrl) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates NetworkUrl and returns whether validation passed along with any errors.
func (v NetworkUrl) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameNetworkUrl, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkUrl) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseOrderingRank validates and returns the canonical form of a Ordering.Rank value.
func ParseOrderingRank(value string) (string, error) {
	return callScalarParse(scalarNameOrderingRank, value)
}

// NormalizeOrderingRank returns the canonical form of a Ordering.Rank value without enforcing shape.
func NormalizeOrderingRank(value string) (string, error) {
	return callScalarNormalize(scalarNameOrderingRank, value)
}

// ValidateOrderingRank enforces the shape of a Ordering.Rank value.
func ValidateOrderingRank(value string) error {
	return callScalarValidate(scalarNameOrderingRank, value)
}

func (v OrderingRank) String() string {
	return scalarStringValue(v)
}

// Validate validates OrderingRank and returns whether validation passed along with any errors.
func (v OrderingRank) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameOrderingRank, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v OrderingRank) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalCronExpression validates and returns the canonical form of a Temporal.CronExpression value.
func ParseTemporalCronExpression(value string) (string, error) {
	return callScalarParse(scalarNameTemporalCronExpression, value)
}

// NormalizeTemporalCronExpression returns the canonical form of a Temporal.CronExpression value without enforcing shape.
func NormalizeTemporalCronExpression(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalCronExpression, value)
}

// ValidateTemporalCronExpression enforces the shape of a Temporal.CronExpression value.
func ValidateTemporalCronExpression(value string) error {
	return callScalarValidate(scalarNameTemporalCronExpression, value)
}

func (v TemporalCronExpression) String() string {
	return string(v)
}

func (v TemporalCronExpression) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalCronExpression) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalCronExpression) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalCronExpression and returns whether validation passed along with any errors.
func (v TemporalCronExpression) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalCronExpression, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalCronExpression) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDate validates and returns the canonical form of a Temporal.Date value.
func ParseTemporalDate(value string) (string, error) {
	return callScalarParse(scalarNameTemporalDate, value)
}

// NormalizeTemporalDate returns the canonical form of a Temporal.Date value without enforcing shape.
func NormalizeTemporalDate(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalDate, value)
}

// ValidateTemporalDate enforces the shape of a Temporal.Date value.
func ValidateTemporalDate(value string) error {
	return callScalarValidate(scalarNameTemporalDate, value)
}

func (v TemporalDate) String() string {
	return string(v)
}

func (v TemporalDate) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalDate) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalDate) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalDate and returns whether validation passed along with any errors.
func (v TemporalDate) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalDate, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDate) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDateTime validates and returns the canonical form of a Temporal.DateTime value.
func ParseTemporalDateTime(value string) (string, error) {
	return callScalarParse(scalarNameTemporalDateTime, value)
}

// NormalizeTemporalDateTime returns the canonical form of a Temporal.DateTime value without enforcing shape.
func NormalizeTemporalDateTime(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalDateTime, value)
}

// ValidateTemporalDateTime enforces the shape of a Temporal.DateTime value.
func ValidateTemporalDateTime(value string) error {
	return callScalarValidate(scalarNameTemporalDateTime, value)
}

func (v TemporalDateTime) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDateTime and returns whether validation passed along with any errors.
func (v TemporalDateTime) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalDateTime, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDateTime) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDays validates and returns the canonical form of a Temporal.Days value.
func ParseTemporalDays(value string) (string, error) {
	return callScalarParse(scalarNameTemporalDays, value)
}

// NormalizeTemporalDays returns the canonical form of a Temporal.Days value without enforcing shape.
func NormalizeTemporalDays(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalDays, value)
}

// ValidateTemporalDays enforces the shape of a Temporal.Days value.
func ValidateTemporalDays(value string) error {
	return callScalarValidate(scalarNameTemporalDays, value)
}

func (v TemporalDays) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDays and returns whether validation passed along with any errors.
func (v TemporalDays) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalDays, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDays) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDuration validates and returns the canonical form of a Temporal.Duration value.
func ParseTemporalDuration(value string) (string, error) {
	return callScalarParse(scalarNameTemporalDuration, value)
}

// NormalizeTemporalDuration returns the canonical form of a Temporal.Duration value without enforcing shape.
func NormalizeTemporalDuration(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalDuration, value)
}

// ValidateTemporalDuration enforces the shape of a Temporal.Duration value.
func ValidateTemporalDuration(value string) error {
	return callScalarValidate(scalarNameTemporalDuration, value)
}

func (v TemporalDuration) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDuration and returns whether validation passed along with any errors.
func (v TemporalDuration) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalDuration, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDuration) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalHours validates and returns the canonical form of a Temporal.Hours value.
func ParseTemporalHours(value string) (string, error) {
	return callScalarParse(scalarNameTemporalHours, value)
}

// NormalizeTemporalHours returns the canonical form of a Temporal.Hours value without enforcing shape.
func NormalizeTemporalHours(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalHours, value)
}

// ValidateTemporalHours enforces the shape of a Temporal.Hours value.
func ValidateTemporalHours(value string) error {
	return callScalarValidate(scalarNameTemporalHours, value)
}

func (v TemporalHours) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalHours and returns whether validation passed along with any errors.
func (v TemporalHours) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalHours, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalHours) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMilliseconds validates and returns the canonical form of a Temporal.Milliseconds value.
func ParseTemporalMilliseconds(value string) (string, error) {
	return callScalarParse(scalarNameTemporalMilliseconds, value)
}

// NormalizeTemporalMilliseconds returns the canonical form of a Temporal.Milliseconds value without enforcing shape.
func NormalizeTemporalMilliseconds(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalMilliseconds, value)
}

// ValidateTemporalMilliseconds enforces the shape of a Temporal.Milliseconds value.
func ValidateTemporalMilliseconds(value string) error {
	return callScalarValidate(scalarNameTemporalMilliseconds, value)
}

func (v TemporalMilliseconds) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalMilliseconds and returns whether validation passed along with any errors.
func (v TemporalMilliseconds) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalMilliseconds, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMilliseconds) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMinutes validates and returns the canonical form of a Temporal.Minutes value.
func ParseTemporalMinutes(value string) (string, error) {
	return callScalarParse(scalarNameTemporalMinutes, value)
}

// NormalizeTemporalMinutes returns the canonical form of a Temporal.Minutes value without enforcing shape.
func NormalizeTemporalMinutes(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalMinutes, value)
}

// ValidateTemporalMinutes enforces the shape of a Temporal.Minutes value.
func ValidateTemporalMinutes(value string) error {
	return callScalarValidate(scalarNameTemporalMinutes, value)
}

func (v TemporalMinutes) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalMinutes and returns whether validation passed along with any errors.
func (v TemporalMinutes) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalMinutes, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMinutes) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMonth validates and returns the canonical form of a Temporal.Month value.
func ParseTemporalMonth(value string) (string, error) {
	return callScalarParse(scalarNameTemporalMonth, value)
}

// NormalizeTemporalMonth returns the canonical form of a Temporal.Month value without enforcing shape.
func NormalizeTemporalMonth(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalMonth, value)
}

// ValidateTemporalMonth enforces the shape of a Temporal.Month value.
func ValidateTemporalMonth(value string) error {
	return callScalarValidate(scalarNameTemporalMonth, value)
}

func (v TemporalMonth) String() string {
	return string(v)
}

func (v TemporalMonth) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalMonth) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalMonth) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalMonth and returns whether validation passed along with any errors.
func (v TemporalMonth) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalMonth, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMonth) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalQuarter validates and returns the canonical form of a Temporal.Quarter value.
func ParseTemporalQuarter(value string) (string, error) {
	return callScalarParse(scalarNameTemporalQuarter, value)
}

// NormalizeTemporalQuarter returns the canonical form of a Temporal.Quarter value without enforcing shape.
func NormalizeTemporalQuarter(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalQuarter, value)
}

// ValidateTemporalQuarter enforces the shape of a Temporal.Quarter value.
func ValidateTemporalQuarter(value string) error {
	return callScalarValidate(scalarNameTemporalQuarter, value)
}

func (v TemporalQuarter) String() string {
	return string(v)
}

func (v TemporalQuarter) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalQuarter) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalQuarter) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalQuarter and returns whether validation passed along with any errors.
func (v TemporalQuarter) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalQuarter, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalQuarter) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalQuarterYear validates and returns the canonical form of a Temporal.QuarterYear value.
func ParseTemporalQuarterYear(value string) (string, error) {
	return callScalarParse(scalarNameTemporalQuarterYear, value)
}

// NormalizeTemporalQuarterYear returns the canonical form of a Temporal.QuarterYear value without enforcing shape.
func NormalizeTemporalQuarterYear(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalQuarterYear, value)
}

// ValidateTemporalQuarterYear enforces the shape of a Temporal.QuarterYear value.
func ValidateTemporalQuarterYear(value string) error {
	return callScalarValidate(scalarNameTemporalQuarterYear, value)
}

func (v TemporalQuarterYear) String() string {
	return string(v)
}

func (v TemporalQuarterYear) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalQuarterYear) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalQuarterYear) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalQuarterYear and returns whether validation passed along with any errors.
func (v TemporalQuarterYear) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalQuarterYear, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalQuarterYear) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalRecurrenceRule validates and returns the canonical form of a Temporal.RecurrenceRule value.
func ParseTemporalRecurrenceRule(value string) (string, error) {
	return callScalarParse(scalarNameTemporalRecurrenceRule, value)
}

// NormalizeTemporalRecurrenceRule returns the canonical form of a Temporal.RecurrenceRule value without enforcing shape.
func NormalizeTemporalRecurrenceRule(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalRecurrenceRule, value)
}

// ValidateTemporalRecurrenceRule enforces the shape of a Temporal.RecurrenceRule value.
func ValidateTemporalRecurrenceRule(value string) error {
	return callScalarValidate(scalarNameTemporalRecurrenceRule, value)
}

func (v TemporalRecurrenceRule) String() string {
	return string(v)
}

func (v TemporalRecurrenceRule) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalRecurrenceRule) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalRecurrenceRule) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalRecurrenceRule and returns whether validation passed along with any errors.
func (v TemporalRecurrenceRule) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalRecurrenceRule, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalRecurrenceRule) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalSeconds validates and returns the canonical form of a Temporal.Seconds value.
func ParseTemporalSeconds(value string) (string, error) {
	return callScalarParse(scalarNameTemporalSeconds, value)
}

// NormalizeTemporalSeconds returns the canonical form of a Temporal.Seconds value without enforcing shape.
func NormalizeTemporalSeconds(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalSeconds, value)
}

// ValidateTemporalSeconds enforces the shape of a Temporal.Seconds value.
func ValidateTemporalSeconds(value string) error {
	return callScalarValidate(scalarNameTemporalSeconds, value)
}

func (v TemporalSeconds) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalSeconds and returns whether validation passed along with any errors.
func (v TemporalSeconds) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalSeconds, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalSeconds) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalTime validates and returns the canonical form of a Temporal.Time value.
func ParseTemporalTime(value string) (string, error) {
	return callScalarParse(scalarNameTemporalTime, value)
}

// NormalizeTemporalTime returns the canonical form of a Temporal.Time value without enforcing shape.
func NormalizeTemporalTime(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalTime, value)
}

// ValidateTemporalTime enforces the shape of a Temporal.Time value.
func ValidateTemporalTime(value string) error {
	return callScalarValidate(scalarNameTemporalTime, value)
}

func (v TemporalTime) String() string {
	return string(v)
}

func (v TemporalTime) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalTime) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalTime) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalTime and returns whether validation passed along with any errors.
func (v TemporalTime) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalTime, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalTime) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalTimeZone validates and returns the canonical form of a Temporal.TimeZone value.
func ParseTemporalTimeZone(value string) (string, error) {
	return callScalarParse(scalarNameTemporalTimeZone, value)
}

// NormalizeTemporalTimeZone returns the canonical form of a Temporal.TimeZone value without enforcing shape.
func NormalizeTemporalTimeZone(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalTimeZone, value)
}

// ValidateTemporalTimeZone enforces the shape of a Temporal.TimeZone value.
func ValidateTemporalTimeZone(value string) error {
	return callScalarValidate(scalarNameTemporalTimeZone, value)
}

func (v TemporalTimeZone) String() string {
	return string(v)
}

func (v TemporalTimeZone) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalTimeZone) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalTimeZone) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalTimeZone and returns whether validation passed along with any errors.
func (v TemporalTimeZone) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalTimeZone, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalTimeZone) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalYear validates and returns the canonical form of a Temporal.Year value.
func ParseTemporalYear(value string) (string, error) {
	return callScalarParse(scalarNameTemporalYear, value)
}

// NormalizeTemporalYear returns the canonical form of a Temporal.Year value without enforcing shape.
func NormalizeTemporalYear(value string) (string, error) {
	return callScalarNormalize(scalarNameTemporalYear, value)
}

// ValidateTemporalYear enforces the shape of a Temporal.Year value.
func ValidateTemporalYear(value string) error {
	return callScalarValidate(scalarNameTemporalYear, value)
}

func (v TemporalYear) String() string {
	return string(v)
}

func (v TemporalYear) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TemporalYear) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TemporalYear) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TemporalYear and returns whether validation passed along with any errors.
func (v TemporalYear) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTemporalYear, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalYear) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTextMarkdown validates and returns the canonical form of a Text.Markdown value.
func ParseTextMarkdown(value string) (string, error) {
	return callScalarParse(scalarNameTextMarkdown, value)
}

// NormalizeTextMarkdown returns the canonical form of a Text.Markdown value without enforcing shape.
func NormalizeTextMarkdown(value string) (string, error) {
	return callScalarNormalize(scalarNameTextMarkdown, value)
}

// ValidateTextMarkdown enforces the shape of a Text.Markdown value.
func ValidateTextMarkdown(value string) error {
	return callScalarValidate(scalarNameTextMarkdown, value)
}

func (v TextMarkdown) String() string {
	return string(v)
}

func (v TextMarkdown) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TextMarkdown) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TextMarkdown) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TextMarkdown and returns whether validation passed along with any errors.
func (v TextMarkdown) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTextMarkdown, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TextMarkdown) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTextSql validates and returns the canonical form of a Text.Sql value.
func ParseTextSql(value string) (string, error) { return callScalarParse(scalarNameTextSql, value) }

// NormalizeTextSql returns the canonical form of a Text.Sql value without enforcing shape.
func NormalizeTextSql(value string) (string, error) {
	return callScalarNormalize(scalarNameTextSql, value)
}

// ValidateTextSql enforces the shape of a Text.Sql value.
func ValidateTextSql(value string) error { return callScalarValidate(scalarNameTextSql, value) }

func (v TextSql) String() string {
	return string(v)
}

func (v TextSql) ToLower() string {
	return strings.ToLower(string(v))
}

func (v TextSql) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v TextSql) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates TextSql and returns whether validation passed along with any errors.
func (v TextSql) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameTextSql, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TextSql) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseVersionSemVer validates and returns the canonical form of a Version.SemVer value.
func ParseVersionSemVer(value string) (string, error) {
	return callScalarParse(scalarNameVersionSemVer, value)
}

// NormalizeVersionSemVer returns the canonical form of a Version.SemVer value without enforcing shape.
func NormalizeVersionSemVer(value string) (string, error) {
	return callScalarNormalize(scalarNameVersionSemVer, value)
}

// ValidateVersionSemVer enforces the shape of a Version.SemVer value.
func ValidateVersionSemVer(value string) error {
	return callScalarValidate(scalarNameVersionSemVer, value)
}

func (v VersionSemVer) String() string {
	return string(v)
}

func (v VersionSemVer) ToLower() string {
	return strings.ToLower(string(v))
}

func (v VersionSemVer) ToUpper() string {
	return strings.ToUpper(string(v))
}

func (v VersionSemVer) TrimSpace() string {
	return strings.TrimSpace(string(v))
}

// Validate validates VersionSemVer and returns whether validation passed along with any errors.
func (v VersionSemVer) Validate() (bool, []ValidationError) {
	return validateScalarValue(scalarNameVersionSemVer, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v VersionSemVer) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}
