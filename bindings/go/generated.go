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

// Frozen scalar ids: the stable u32 C ABI contract, one constant per registry entry.
const (
	scalarIDAuthJWT                uint32 = 5
	scalarIDAuthPassword           uint32 = 6
	scalarIDContactEmail           uint32 = 8
	scalarIDContactPhoneNumber     uint32 = 9
	scalarIDCryptoRSAPrivateKey    uint32 = 10
	scalarIDCryptoRSAPublicKey     uint32 = 11
	scalarIDDesignColor            uint32 = 12
	scalarIDEmbeddingVector        uint32 = 13
	scalarIDFileSizeBytes          uint32 = 14
	scalarIDFinanceMoney           uint32 = 15
	scalarIDGenericInt64           uint32 = 16
	scalarIDGenericJSON            uint32 = 17
	scalarIDGenericProbability     uint32 = 18
	scalarIDGenericStringMap       uint32 = 19
	scalarIDGeoLocation            uint32 = 20
	scalarIDIdentityName           uint32 = 21
	scalarIDIdentitySlug           uint32 = 22
	scalarIDIdentityUUID           uint32 = 23
	scalarIDIdentityUserID         uint32 = 24
	scalarIDLocalizationLocale     uint32 = 25
	scalarIDNetworkDomainName      uint32 = 26
	scalarIDNetworkIpAddress       uint32 = 27
	scalarIDNetworkUri             uint32 = 28
	scalarIDNetworkUrl             uint32 = 29
	scalarIDTemporalCronExpression uint32 = 39
	scalarIDTemporalDate           uint32 = 40
	scalarIDTemporalDateTime       uint32 = 41
	scalarIDTemporalDuration       uint32 = 42
	scalarIDTemporalMilliseconds   uint32 = 43
	scalarIDTemporalMonth          uint32 = 44
	scalarIDTemporalQuarter        uint32 = 45
	scalarIDTemporalQuarterYear    uint32 = 46
	scalarIDTemporalTime           uint32 = 47
	scalarIDTemporalTimeZone       uint32 = 48
	scalarIDTemporalYear           uint32 = 49
	scalarIDTextMarkdown           uint32 = 50
	scalarIDTemporalSeconds        uint32 = 52
	scalarIDTemporalMinutes        uint32 = 53
	scalarIDTemporalHours          uint32 = 54
	scalarIDTemporalDays           uint32 = 55
	scalarIDTextSql                uint32 = 56
	scalarIDCryptoSHA256           uint32 = 58
	scalarIDNetworkDnsLabel        uint32 = 59
	scalarIDTemporalRecurrenceRule uint32 = 60
)

// ScalarIDByCanonical maps a canonical scalar string to its frozen u32 id.
var ScalarIDByCanonical = map[string]uint32{
	"Auth.JWT":                5,
	"Auth.Password":           6,
	"Contact.Email":           8,
	"Contact.PhoneNumber":     9,
	"Crypto.RSAPrivateKey":    10,
	"Crypto.RSAPublicKey":     11,
	"Design.Color":            12,
	"Embedding.Vector":        13,
	"File.SizeBytes":          14,
	"Finance.Money":           15,
	"Generic.Int64":           16,
	"Generic.JSON":            17,
	"Generic.Probability":     18,
	"Generic.StringMap":       19,
	"Geo.Location":            20,
	"Identity.Name":           21,
	"Identity.Slug":           22,
	"Identity.UUID":           23,
	"Identity.UserID":         24,
	"Localization.Locale":     25,
	"Network.DomainName":      26,
	"Network.IpAddress":       27,
	"Network.Uri":             28,
	"Network.Url":             29,
	"Temporal.CronExpression": 39,
	"Temporal.Date":           40,
	"Temporal.DateTime":       41,
	"Temporal.Duration":       42,
	"Temporal.Milliseconds":   43,
	"Temporal.Month":          44,
	"Temporal.Quarter":        45,
	"Temporal.QuarterYear":    46,
	"Temporal.Time":           47,
	"Temporal.TimeZone":       48,
	"Temporal.Year":           49,
	"Text.Markdown":           50,
	"Temporal.Seconds":        52,
	"Temporal.Minutes":        53,
	"Temporal.Hours":          54,
	"Temporal.Days":           55,
	"Text.Sql":                56,
	"Crypto.SHA256":           58,
	"Network.DnsLabel":        59,
	"Temporal.RecurrenceRule": 60,
}

// VALID_SCALARS lists every canonical scalar name known to scalar-lib.
var VALID_SCALARS = []string{
	string("Auth.JWT"),
	string("Auth.Password"),
	string("Contact.Email"),
	string("Contact.PhoneNumber"),
	string("Crypto.RSAPrivateKey"),
	string("Crypto.RSAPublicKey"),
	string("Design.Color"),
	string("Embedding.Vector"),
	string("File.SizeBytes"),
	string("Finance.Money"),
	string("Generic.Int64"),
	string("Generic.JSON"),
	string("Generic.Probability"),
	string("Generic.StringMap"),
	string("Geo.Location"),
	string("Identity.Name"),
	string("Identity.Slug"),
	string("Identity.UUID"),
	string("Identity.UserID"),
	string("Localization.Locale"),
	string("Network.DomainName"),
	string("Network.IpAddress"),
	string("Network.Uri"),
	string("Network.Url"),
	string("Temporal.CronExpression"),
	string("Temporal.Date"),
	string("Temporal.DateTime"),
	string("Temporal.Duration"),
	string("Temporal.Milliseconds"),
	string("Temporal.Month"),
	string("Temporal.Quarter"),
	string("Temporal.QuarterYear"),
	string("Temporal.Time"),
	string("Temporal.TimeZone"),
	string("Temporal.Year"),
	string("Text.Markdown"),
	string("Temporal.Seconds"),
	string("Temporal.Minutes"),
	string("Temporal.Hours"),
	string("Temporal.Days"),
	string("Text.Sql"),
	string("Crypto.SHA256"),
	string("Network.DnsLabel"),
	string("Temporal.RecurrenceRule"),
}

var validScalarsMap = func() map[string]struct{} {
	m := make(map[string]struct{}, len(VALID_SCALARS))
	for _, scalar := range VALID_SCALARS {
		m[scalar] = struct{}{}
	}
	return m
}()

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

// Generic.JSON - "A JSON object represented as a string"
type GenericJSON json.RawMessage

// Generic.Probability - "Probability value from 0.0 to 1.0 inclusive"
type GenericProbability float64

// Generic.StringMap - "A string-to-string map stored as JSON"
type GenericStringMap map[string]string

// Geo.Location - "Geographic location with latitude and longitude"
type GeoLocation struct {
	Lat float64
	Lon float64
}

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

// Network.DomainName - "Valid domain name (RFC 1035 compliant)"
type NetworkDomainName string

// Network.IpAddress - "IPv4 or IPv6 address"
type NetworkIpAddress string

// Network.Uri - "RFC 3986 URI for connection strings and non-HTTP resources"
type NetworkUri string

// Network.Url - "Valid HTTP/HTTPS URL"
type NetworkUrl string

// Temporal.CronExpression - "Standard 5-field cron expression for scheduling"
type TemporalCronExpression string

// Temporal.Date - "Calendar date, normalized to ISO 'YYYY-MM-DD'. Accepts ISO ('2025-01-01'), slash-separated ('2025/01/15', '01/15/2025'), named-month ('January 15, 2025', 'Jan 15, 2025'), and full RFC3339 datetime (the time portion is dropped)."
type TemporalDate string

// Temporal.DateTime - "ISO8601 datetime string. Epoch wire values keep this scalar and declare x-temporal-format (unix, unix_millis, unix_micros, unix_nanos) on the property; the unit is never guessed from digit count."
type TemporalDateTime time.Time

// Temporal.Duration - "Duration for timeouts and intervals"
type TemporalDuration time.Duration

// Temporal.Milliseconds - "Signed integer count of milliseconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix_millis."
type TemporalMilliseconds int64

// Temporal.Month - "Calendar month, normalized to two-digit numeric (01-12). Accepts '2', '02', 'Feb', 'February' (case-insensitive)."
type TemporalMonth string

// Temporal.Quarter - "Calendar quarter (Q1-Q4)"
type TemporalQuarter string

// Temporal.QuarterYear - "Quarter and year, normalized to 'YYYY-Q#'. Accepts '2025-Q1', 'Q1/2025', 'Q1-2025', 'Q1-25' (2-digit years interpreted as 20XX)."
type TemporalQuarterYear string

// Temporal.Time - "Time of day. 24-hour 'HH:MM' or 'HH:MM:SS' (hours 00-23), or 12-hour 'H:MM'/'HH:MM' with optional ':SS' and required AM/PM suffix (hours 1-12). Seconds and the AM/PM separator space are optional."
type TemporalTime string

// Temporal.TimeZone - "IANA timezone identifier (e.g., America/New_York, UTC, Etc/UTC)"
type TemporalTimeZone string

// Temporal.Year - "Calendar year as a 4-digit string (e.g., 2025)"
type TemporalYear string

// Text.Markdown - "Markdown text"
type TextMarkdown string

// Temporal.Seconds - "Signed integer count of seconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix."
type TemporalSeconds int64

// Temporal.Minutes - "Signed integer count of minutes: an amount of elapsed time, never a point in time."
type TemporalMinutes int64

// Temporal.Hours - "Signed integer count of hours: an amount of elapsed time, never a point in time."
type TemporalHours int64

// Temporal.Days - "Signed integer count of days: an amount of elapsed time, never a point in time or a calendar date."
type TemporalDays int64

// Text.Sql - "SQL text"
type TextSql string

// Crypto.SHA256 - "Lowercase hexadecimal SHA-256 digest"
type CryptoSHA256 string

// Network.DnsLabel - "Single DNS label (RFC 1035): one hostname segment, no dots"
type NetworkDnsLabel string

// Temporal.RecurrenceRule - "RFC 5545 recurrence rule, without DTSTART"
type TemporalRecurrenceRule string

var SCALAR_METADATA = []ScalarMetadata{
	{
		CanonicalName:      "Auth.JWT",
		Symbol:             "AuthJWT",
		Primitive:          "String",
		Description:        "JSON Web Token string",
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
		Examples:           []string{"+1234567890"},
		ComparabilityClass: "",
		IsSortable:         true,
	},
	{
		CanonicalName:      "Crypto.RSAPrivateKey",
		Symbol:             "CryptoRSAPrivateKey",
		Primitive:          "String",
		Description:        "RSA private key in PEM format",
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
		CanonicalName:      "Design.Color",
		Symbol:             "DesignColor",
		Primitive:          "String",
		Description:        "CSS color value normalized to 8-digit RGBA hex format (#RRGGBBAA)",
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
		Description:        "A JSON object represented as a string",
		GoType:             "json.RawMessage",
		SQLType:            "JSONB",
		JSONSchemaType:     "object",
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
		CanonicalName:      "Generic.Probability",
		Symbol:             "GenericProbability",
		Primitive:          "Float",
		Description:        "Probability value from 0.0 to 1.0 inclusive",
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
		HasCustomParse:     false,
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
		GoType:             "struct{ Lat float64; Lon float64 }",
		SQLType:            "POINT",
		JSONSchemaType:     "object",
		Format:             "",
		MaxLength:          0,
		MinLength:          0,
		Maximum:            nil,
		Minimum:            nil,
		Pattern:            "^-?\\d+(\\.\\d+)?,-?\\d+(\\.\\d+)?$",
		HasCustomNormalize: false,
		HasCustomParse:     false,
		HasCustomValidate:  false,
		HasValidator:       false,
		Examples:           []string{"37.7749,-122.4194"},
		ComparabilityClass: "",
		IsSortable:         false,
	},
	{
		CanonicalName:      "Identity.Name",
		Symbol:             "IdentityName",
		Primitive:          "String",
		Description:        "An objects name",
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
		CanonicalName:      "Network.DomainName",
		Symbol:             "NetworkDomainName",
		Primitive:          "String",
		Description:        "Valid domain name (RFC 1035 compliant)",
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
		CanonicalName:      "Temporal.CronExpression",
		Symbol:             "TemporalCronExpression",
		Primitive:          "String",
		Description:        "Standard 5-field cron expression for scheduling",
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
		CanonicalName:      "Temporal.Duration",
		Symbol:             "TemporalDuration",
		Primitive:          "String",
		Description:        "Duration for timeouts and intervals",
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
		CanonicalName:      "Temporal.Milliseconds",
		Symbol:             "TemporalMilliseconds",
		Primitive:          "Int",
		Description:        "Signed integer count of milliseconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix_millis.",
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
		CanonicalName:      "Temporal.Month",
		Symbol:             "TemporalMonth",
		Primitive:          "String",
		Description:        "Calendar month, normalized to two-digit numeric (01-12). Accepts '2', '02', 'Feb', 'February' (case-insensitive).",
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
		CanonicalName:      "Temporal.Time",
		Symbol:             "TemporalTime",
		Primitive:          "String",
		Description:        "Time of day. 24-hour 'HH:MM' or 'HH:MM:SS' (hours 00-23), or 12-hour 'H:MM'/'HH:MM' with optional ':SS' and required AM/PM suffix (hours 1-12). Seconds and the AM/PM separator space are optional.",
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
		CanonicalName:      "Temporal.Seconds",
		Symbol:             "TemporalSeconds",
		Primitive:          "Int",
		Description:        "Signed integer count of seconds: an amount of elapsed time, never a point in time. An epoch timestamp is Temporal.DateTime with x-temporal-format: unix.",
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
		CanonicalName:      "Temporal.Minutes",
		Symbol:             "TemporalMinutes",
		Primitive:          "Int",
		Description:        "Signed integer count of minutes: an amount of elapsed time, never a point in time.",
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
		CanonicalName:      "Temporal.Hours",
		Symbol:             "TemporalHours",
		Primitive:          "Int",
		Description:        "Signed integer count of hours: an amount of elapsed time, never a point in time.",
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
		CanonicalName:      "Temporal.Days",
		Symbol:             "TemporalDays",
		Primitive:          "Int",
		Description:        "Signed integer count of days: an amount of elapsed time, never a point in time or a calendar date.",
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
		CanonicalName:      "Text.Sql",
		Symbol:             "TextSql",
		Primitive:          "String",
		Description:        "SQL text",
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
		CanonicalName:      "Crypto.SHA256",
		Symbol:             "CryptoSHA256",
		Primitive:          "String",
		Description:        "Lowercase hexadecimal SHA-256 digest",
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
		CanonicalName:      "Network.DnsLabel",
		Symbol:             "NetworkDnsLabel",
		Primitive:          "String",
		Description:        "Single DNS label (RFC 1035): one hostname segment, no dots",
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
		CanonicalName:      "Temporal.RecurrenceRule",
		Symbol:             "TemporalRecurrenceRule",
		Primitive:          "String",
		Description:        "RFC 5545 recurrence rule, without DTSTART",
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
}

var ScalarMetadataByCanonical = func() map[string]*ScalarMetadata {
	m := make(map[string]*ScalarMetadata, len(SCALAR_METADATA))
	for i := range SCALAR_METADATA {
		m[SCALAR_METADATA[i].CanonicalName] = &SCALAR_METADATA[i]
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

func patternForCanonical(canonicalId string) string {
	meta := ScalarMetadataByCanonical[canonicalId]
	if meta == nil {
		return ""
	}
	return meta.Pattern
}

var AuthJWTPattern = ScalarPattern(patternForCanonical("Auth.JWT"))
var ContactEmailPattern = ScalarPattern(patternForCanonical("Contact.Email"))
var ContactPhoneNumberPattern = ScalarPattern(patternForCanonical("Contact.PhoneNumber"))
var CryptoRSAPrivateKeyPattern = ScalarPattern(patternForCanonical("Crypto.RSAPrivateKey"))
var CryptoRSAPublicKeyPattern = ScalarPattern(patternForCanonical("Crypto.RSAPublicKey"))
var DesignColorPattern = ScalarPattern(patternForCanonical("Design.Color"))
var GeoLocationPattern = ScalarPattern(patternForCanonical("Geo.Location"))
var IdentitySlugPattern = ScalarPattern(patternForCanonical("Identity.Slug"))
var IdentityUUIDPattern = ScalarPattern(patternForCanonical("Identity.UUID"))
var IdentityUserIDPattern = ScalarPattern(patternForCanonical("Identity.UserID"))
var LocalizationLocalePattern = ScalarPattern(patternForCanonical("Localization.Locale"))
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
var CryptoSHA256Pattern = ScalarPattern(patternForCanonical("Crypto.SHA256"))
var NetworkDnsLabelPattern = ScalarPattern(patternForCanonical("Network.DnsLabel"))

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

func validateScalarValue(id uint32, value string) (bool, []ValidationError) {
	if err := callScalarValidate(id, value); err != nil {
		return false, validationErrorsFromError(err)
	}
	return true, nil
}

// scalarRequiredValueMissing reports whether a required scalar carries no value.
// Numeric and boolean kinds are exempt: 0 and false are legitimate values that
// Go cannot distinguish from an absent JSON key, so treating them as missing
// rejects valid payloads. The Python (`is None`) and TypeScript (`=== null`)
// bindings already accept them; this keeps Go in parity.
func scalarRequiredValueMissing(value any) bool {
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

func ValidatorFor(canonicalId string) func(string) error {
	switch canonicalId {
	case "Auth.JWT":
		return func(value string) error { return callScalarValidate(5, value) }
	case "Auth.Password":
		return func(value string) error { return callScalarValidate(6, value) }
	case "Contact.Email":
		return func(value string) error { return callScalarValidate(8, value) }
	case "Contact.PhoneNumber":
		return func(value string) error { return callScalarValidate(9, value) }
	case "Crypto.RSAPrivateKey":
		return func(value string) error { return callScalarValidate(10, value) }
	case "Crypto.RSAPublicKey":
		return func(value string) error { return callScalarValidate(11, value) }
	case "Design.Color":
		return func(value string) error { return callScalarValidate(12, value) }
	case "Embedding.Vector":
		return func(value string) error { return callScalarValidate(13, value) }
	case "File.SizeBytes":
		return func(value string) error { return callScalarValidate(14, value) }
	case "Finance.Money":
		return func(value string) error { return callScalarValidate(15, value) }
	case "Generic.Int64":
		return func(value string) error { return callScalarValidate(16, value) }
	case "Generic.JSON":
		return func(value string) error { return callScalarValidate(17, value) }
	case "Generic.Probability":
		return func(value string) error { return callScalarValidate(18, value) }
	case "Generic.StringMap":
		return func(value string) error { return callScalarValidate(19, value) }
	case "Identity.Name":
		return func(value string) error { return callScalarValidate(21, value) }
	case "Identity.Slug":
		return func(value string) error { return callScalarValidate(22, value) }
	case "Identity.UUID":
		return func(value string) error { return callScalarValidate(23, value) }
	case "Identity.UserID":
		return func(value string) error { return callScalarValidate(24, value) }
	case "Localization.Locale":
		return func(value string) error { return callScalarValidate(25, value) }
	case "Network.DomainName":
		return func(value string) error { return callScalarValidate(26, value) }
	case "Network.IpAddress":
		return func(value string) error { return callScalarValidate(27, value) }
	case "Network.Uri":
		return func(value string) error { return callScalarValidate(28, value) }
	case "Network.Url":
		return func(value string) error { return callScalarValidate(29, value) }
	case "Temporal.CronExpression":
		return func(value string) error { return callScalarValidate(39, value) }
	case "Temporal.Date":
		return func(value string) error { return callScalarValidate(40, value) }
	case "Temporal.DateTime":
		return func(value string) error { return callScalarValidate(41, value) }
	case "Temporal.Duration":
		return func(value string) error { return callScalarValidate(42, value) }
	case "Temporal.Milliseconds":
		return func(value string) error { return callScalarValidate(43, value) }
	case "Temporal.Month":
		return func(value string) error { return callScalarValidate(44, value) }
	case "Temporal.Quarter":
		return func(value string) error { return callScalarValidate(45, value) }
	case "Temporal.QuarterYear":
		return func(value string) error { return callScalarValidate(46, value) }
	case "Temporal.Time":
		return func(value string) error { return callScalarValidate(47, value) }
	case "Temporal.TimeZone":
		return func(value string) error { return callScalarValidate(48, value) }
	case "Temporal.Year":
		return func(value string) error { return callScalarValidate(49, value) }
	case "Text.Markdown":
		return func(value string) error { return callScalarValidate(50, value) }
	case "Temporal.Seconds":
		return func(value string) error { return callScalarValidate(52, value) }
	case "Temporal.Minutes":
		return func(value string) error { return callScalarValidate(53, value) }
	case "Temporal.Hours":
		return func(value string) error { return callScalarValidate(54, value) }
	case "Temporal.Days":
		return func(value string) error { return callScalarValidate(55, value) }
	case "Text.Sql":
		return func(value string) error { return callScalarValidate(56, value) }
	case "Crypto.SHA256":
		return func(value string) error { return callScalarValidate(58, value) }
	case "Network.DnsLabel":
		return func(value string) error { return callScalarValidate(59, value) }
	case "Temporal.RecurrenceRule":
		return func(value string) error { return callScalarValidate(60, value) }
	default:
		return nil
	}
}

// ParseAuthJWT validates and returns the canonical form of a Auth.JWT value.
func ParseAuthJWT(value string) (string, error) { return callScalarParse(5, value) }

// NormalizeAuthJWT returns the canonical form of a Auth.JWT value without enforcing shape.
func NormalizeAuthJWT(value string) (string, error) { return callScalarNormalize(5, value) }

// ValidateAuthJWT enforces the shape of a Auth.JWT value.
func ValidateAuthJWT(value string) error { return callScalarValidate(5, value) }

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
	return validateScalarValue(5, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v AuthJWT) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseAuthPassword validates and returns the canonical form of a Auth.Password value.
func ParseAuthPassword(value string) (string, error) { return callScalarParse(6, value) }

// NormalizeAuthPassword returns the canonical form of a Auth.Password value without enforcing shape.
func NormalizeAuthPassword(value string) (string, error) { return callScalarNormalize(6, value) }

// ValidateAuthPassword enforces the shape of a Auth.Password value.
func ValidateAuthPassword(value string) error { return callScalarValidate(6, value) }

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
	return validateScalarValue(6, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v AuthPassword) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseContactEmail validates and returns the canonical form of a Contact.Email value.
func ParseContactEmail(value string) (string, error) { return callScalarParse(8, value) }

// NormalizeContactEmail returns the canonical form of a Contact.Email value without enforcing shape.
func NormalizeContactEmail(value string) (string, error) { return callScalarNormalize(8, value) }

// ValidateContactEmail enforces the shape of a Contact.Email value.
func ValidateContactEmail(value string) error { return callScalarValidate(8, value) }

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
	return validateScalarValue(8, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v ContactEmail) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseContactPhoneNumber validates and returns the canonical form of a Contact.PhoneNumber value.
func ParseContactPhoneNumber(value string) (string, error) { return callScalarParse(9, value) }

// NormalizeContactPhoneNumber returns the canonical form of a Contact.PhoneNumber value without enforcing shape.
func NormalizeContactPhoneNumber(value string) (string, error) { return callScalarNormalize(9, value) }

// ValidateContactPhoneNumber enforces the shape of a Contact.PhoneNumber value.
func ValidateContactPhoneNumber(value string) error { return callScalarValidate(9, value) }

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
	return validateScalarValue(9, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v ContactPhoneNumber) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoRSAPrivateKey validates and returns the canonical form of a Crypto.RSAPrivateKey value.
func ParseCryptoRSAPrivateKey(value string) (string, error) { return callScalarParse(10, value) }

// NormalizeCryptoRSAPrivateKey returns the canonical form of a Crypto.RSAPrivateKey value without enforcing shape.
func NormalizeCryptoRSAPrivateKey(value string) (string, error) {
	return callScalarNormalize(10, value)
}

// ValidateCryptoRSAPrivateKey enforces the shape of a Crypto.RSAPrivateKey value.
func ValidateCryptoRSAPrivateKey(value string) error { return callScalarValidate(10, value) }

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
	return validateScalarValue(10, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoRSAPrivateKey) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoRSAPublicKey validates and returns the canonical form of a Crypto.RSAPublicKey value.
func ParseCryptoRSAPublicKey(value string) (string, error) { return callScalarParse(11, value) }

// NormalizeCryptoRSAPublicKey returns the canonical form of a Crypto.RSAPublicKey value without enforcing shape.
func NormalizeCryptoRSAPublicKey(value string) (string, error) { return callScalarNormalize(11, value) }

// ValidateCryptoRSAPublicKey enforces the shape of a Crypto.RSAPublicKey value.
func ValidateCryptoRSAPublicKey(value string) error { return callScalarValidate(11, value) }

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
	return validateScalarValue(11, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoRSAPublicKey) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseDesignColor validates and returns the canonical form of a Design.Color value.
func ParseDesignColor(value string) (string, error) { return callScalarParse(12, value) }

// NormalizeDesignColor returns the canonical form of a Design.Color value without enforcing shape.
func NormalizeDesignColor(value string) (string, error) { return callScalarNormalize(12, value) }

// ValidateDesignColor enforces the shape of a Design.Color value.
func ValidateDesignColor(value string) error { return callScalarValidate(12, value) }

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
	return validateScalarValue(12, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v DesignColor) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseEmbeddingVector validates and returns the canonical form of a Embedding.Vector value.
func ParseEmbeddingVector(value string) (string, error) { return callScalarParse(13, value) }

// NormalizeEmbeddingVector returns the canonical form of a Embedding.Vector value without enforcing shape.
func NormalizeEmbeddingVector(value string) (string, error) { return callScalarNormalize(13, value) }

// ValidateEmbeddingVector enforces the shape of a Embedding.Vector value.
func ValidateEmbeddingVector(value string) error { return callScalarValidate(13, value) }

func (v EmbeddingVector) String() string {
	return scalarStringValue(v)
}

// Validate validates EmbeddingVector and returns whether validation passed along with any errors.
func (v EmbeddingVector) Validate() (bool, []ValidationError) {
	return validateScalarValue(13, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v EmbeddingVector) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseFileSizeBytes validates and returns the canonical form of a File.SizeBytes value.
func ParseFileSizeBytes(value string) (string, error) { return callScalarParse(14, value) }

// NormalizeFileSizeBytes returns the canonical form of a File.SizeBytes value without enforcing shape.
func NormalizeFileSizeBytes(value string) (string, error) { return callScalarNormalize(14, value) }

// ValidateFileSizeBytes enforces the shape of a File.SizeBytes value.
func ValidateFileSizeBytes(value string) error { return callScalarValidate(14, value) }

func (v FileSizeBytes) String() string {
	return scalarStringValue(v)
}

// Validate validates FileSizeBytes and returns whether validation passed along with any errors.
func (v FileSizeBytes) Validate() (bool, []ValidationError) {
	return validateScalarValue(14, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v FileSizeBytes) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseFinanceMoney validates and returns the canonical form of a Finance.Money value.
func ParseFinanceMoney(value string) (string, error) { return callScalarParse(15, value) }

// NormalizeFinanceMoney returns the canonical form of a Finance.Money value without enforcing shape.
func NormalizeFinanceMoney(value string) (string, error) { return callScalarNormalize(15, value) }

// ValidateFinanceMoney enforces the shape of a Finance.Money value.
func ValidateFinanceMoney(value string) error { return callScalarValidate(15, value) }

func (v FinanceMoney) String() string {
	return scalarStringValue(v)
}

// Validate validates FinanceMoney and returns whether validation passed along with any errors.
func (v FinanceMoney) Validate() (bool, []ValidationError) {
	return validateScalarValue(15, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v FinanceMoney) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericInt64 validates and returns the canonical form of a Generic.Int64 value.
func ParseGenericInt64(value string) (string, error) { return callScalarParse(16, value) }

// NormalizeGenericInt64 returns the canonical form of a Generic.Int64 value without enforcing shape.
func NormalizeGenericInt64(value string) (string, error) { return callScalarNormalize(16, value) }

// ValidateGenericInt64 enforces the shape of a Generic.Int64 value.
func ValidateGenericInt64(value string) error { return callScalarValidate(16, value) }

func (v GenericInt64) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericInt64 and returns whether validation passed along with any errors.
func (v GenericInt64) Validate() (bool, []ValidationError) {
	return validateScalarValue(16, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericInt64) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericJSON validates and returns the canonical form of a Generic.JSON value.
func ParseGenericJSON(value string) (string, error) { return callScalarParse(17, value) }

// NormalizeGenericJSON returns the canonical form of a Generic.JSON value without enforcing shape.
func NormalizeGenericJSON(value string) (string, error) { return callScalarNormalize(17, value) }

// ValidateGenericJSON enforces the shape of a Generic.JSON value.
func ValidateGenericJSON(value string) error { return callScalarValidate(17, value) }

func (v GenericJSON) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericJSON and returns whether validation passed along with any errors.
func (v GenericJSON) Validate() (bool, []ValidationError) {
	return validateScalarValue(17, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericJSON) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericProbability validates and returns the canonical form of a Generic.Probability value.
func ParseGenericProbability(value string) (string, error) { return callScalarParse(18, value) }

// NormalizeGenericProbability returns the canonical form of a Generic.Probability value without enforcing shape.
func NormalizeGenericProbability(value string) (string, error) { return callScalarNormalize(18, value) }

// ValidateGenericProbability enforces the shape of a Generic.Probability value.
func ValidateGenericProbability(value string) error { return callScalarValidate(18, value) }

func (v GenericProbability) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericProbability and returns whether validation passed along with any errors.
func (v GenericProbability) Validate() (bool, []ValidationError) {
	return validateScalarValue(18, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericProbability) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseGenericStringMap validates and returns the canonical form of a Generic.StringMap value.
func ParseGenericStringMap(value string) (string, error) { return callScalarParse(19, value) }

// NormalizeGenericStringMap returns the canonical form of a Generic.StringMap value without enforcing shape.
func NormalizeGenericStringMap(value string) (string, error) { return callScalarNormalize(19, value) }

// ValidateGenericStringMap enforces the shape of a Generic.StringMap value.
func ValidateGenericStringMap(value string) error { return callScalarValidate(19, value) }

func (v GenericStringMap) String() string {
	return scalarStringValue(v)
}

// Validate validates GenericStringMap and returns whether validation passed along with any errors.
func (v GenericStringMap) Validate() (bool, []ValidationError) {
	return validateScalarValue(19, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v GenericStringMap) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentityName validates and returns the canonical form of a Identity.Name value.
func ParseIdentityName(value string) (string, error) { return callScalarParse(21, value) }

// NormalizeIdentityName returns the canonical form of a Identity.Name value without enforcing shape.
func NormalizeIdentityName(value string) (string, error) { return callScalarNormalize(21, value) }

// ValidateIdentityName enforces the shape of a Identity.Name value.
func ValidateIdentityName(value string) error { return callScalarValidate(21, value) }

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
	return validateScalarValue(21, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v IdentityName) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentitySlug validates and returns the canonical form of a Identity.Slug value.
func ParseIdentitySlug(value string) (string, error) { return callScalarParse(22, value) }

// NormalizeIdentitySlug returns the canonical form of a Identity.Slug value without enforcing shape.
func NormalizeIdentitySlug(value string) (string, error) { return callScalarNormalize(22, value) }

// ValidateIdentitySlug enforces the shape of a Identity.Slug value.
func ValidateIdentitySlug(value string) error { return callScalarValidate(22, value) }

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
	return validateScalarValue(22, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v IdentitySlug) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseIdentityUUID validates and returns the canonical form of a Identity.UUID value.
func ParseIdentityUUID(value string) (string, error) { return callScalarParse(23, value) }

// NormalizeIdentityUUID returns the canonical form of a Identity.UUID value without enforcing shape.
func NormalizeIdentityUUID(value string) (string, error) { return callScalarNormalize(23, value) }

// ValidateIdentityUUID enforces the shape of a Identity.UUID value.
func ValidateIdentityUUID(value string) error { return callScalarValidate(23, value) }

// ParseIdentityUserID validates and returns the canonical form of a Identity.UserID value.
func ParseIdentityUserID(value string) (string, error) { return callScalarParse(24, value) }

// NormalizeIdentityUserID returns the canonical form of a Identity.UserID value without enforcing shape.
func NormalizeIdentityUserID(value string) (string, error) { return callScalarNormalize(24, value) }

// ValidateIdentityUserID enforces the shape of a Identity.UserID value.
func ValidateIdentityUserID(value string) error { return callScalarValidate(24, value) }

// ParseLocalizationLocale validates and returns the canonical form of a Localization.Locale value.
func ParseLocalizationLocale(value string) (string, error) { return callScalarParse(25, value) }

// NormalizeLocalizationLocale returns the canonical form of a Localization.Locale value without enforcing shape.
func NormalizeLocalizationLocale(value string) (string, error) { return callScalarNormalize(25, value) }

// ValidateLocalizationLocale enforces the shape of a Localization.Locale value.
func ValidateLocalizationLocale(value string) error { return callScalarValidate(25, value) }

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
	return validateScalarValue(25, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v LocalizationLocale) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkDomainName validates and returns the canonical form of a Network.DomainName value.
func ParseNetworkDomainName(value string) (string, error) { return callScalarParse(26, value) }

// NormalizeNetworkDomainName returns the canonical form of a Network.DomainName value without enforcing shape.
func NormalizeNetworkDomainName(value string) (string, error) { return callScalarNormalize(26, value) }

// ValidateNetworkDomainName enforces the shape of a Network.DomainName value.
func ValidateNetworkDomainName(value string) error { return callScalarValidate(26, value) }

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
	return validateScalarValue(26, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkDomainName) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkIpAddress validates and returns the canonical form of a Network.IpAddress value.
func ParseNetworkIpAddress(value string) (string, error) { return callScalarParse(27, value) }

// NormalizeNetworkIpAddress returns the canonical form of a Network.IpAddress value without enforcing shape.
func NormalizeNetworkIpAddress(value string) (string, error) { return callScalarNormalize(27, value) }

// ValidateNetworkIpAddress enforces the shape of a Network.IpAddress value.
func ValidateNetworkIpAddress(value string) error { return callScalarValidate(27, value) }

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
	return validateScalarValue(27, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkIpAddress) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkUri validates and returns the canonical form of a Network.Uri value.
func ParseNetworkUri(value string) (string, error) { return callScalarParse(28, value) }

// NormalizeNetworkUri returns the canonical form of a Network.Uri value without enforcing shape.
func NormalizeNetworkUri(value string) (string, error) { return callScalarNormalize(28, value) }

// ValidateNetworkUri enforces the shape of a Network.Uri value.
func ValidateNetworkUri(value string) error { return callScalarValidate(28, value) }

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
	return validateScalarValue(28, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkUri) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkUrl validates and returns the canonical form of a Network.Url value.
func ParseNetworkUrl(value string) (string, error) { return callScalarParse(29, value) }

// NormalizeNetworkUrl returns the canonical form of a Network.Url value without enforcing shape.
func NormalizeNetworkUrl(value string) (string, error) { return callScalarNormalize(29, value) }

// ValidateNetworkUrl enforces the shape of a Network.Url value.
func ValidateNetworkUrl(value string) error { return callScalarValidate(29, value) }

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
	return validateScalarValue(29, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkUrl) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalCronExpression validates and returns the canonical form of a Temporal.CronExpression value.
func ParseTemporalCronExpression(value string) (string, error) { return callScalarParse(39, value) }

// NormalizeTemporalCronExpression returns the canonical form of a Temporal.CronExpression value without enforcing shape.
func NormalizeTemporalCronExpression(value string) (string, error) {
	return callScalarNormalize(39, value)
}

// ValidateTemporalCronExpression enforces the shape of a Temporal.CronExpression value.
func ValidateTemporalCronExpression(value string) error { return callScalarValidate(39, value) }

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
	return validateScalarValue(39, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalCronExpression) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDate validates and returns the canonical form of a Temporal.Date value.
func ParseTemporalDate(value string) (string, error) { return callScalarParse(40, value) }

// NormalizeTemporalDate returns the canonical form of a Temporal.Date value without enforcing shape.
func NormalizeTemporalDate(value string) (string, error) { return callScalarNormalize(40, value) }

// ValidateTemporalDate enforces the shape of a Temporal.Date value.
func ValidateTemporalDate(value string) error { return callScalarValidate(40, value) }

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
	return validateScalarValue(40, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDate) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDateTime validates and returns the canonical form of a Temporal.DateTime value.
func ParseTemporalDateTime(value string) (string, error) { return callScalarParse(41, value) }

// NormalizeTemporalDateTime returns the canonical form of a Temporal.DateTime value without enforcing shape.
func NormalizeTemporalDateTime(value string) (string, error) { return callScalarNormalize(41, value) }

// ValidateTemporalDateTime enforces the shape of a Temporal.DateTime value.
func ValidateTemporalDateTime(value string) error { return callScalarValidate(41, value) }

func (v TemporalDateTime) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDateTime and returns whether validation passed along with any errors.
func (v TemporalDateTime) Validate() (bool, []ValidationError) {
	return validateScalarValue(41, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDateTime) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDuration validates and returns the canonical form of a Temporal.Duration value.
func ParseTemporalDuration(value string) (string, error) { return callScalarParse(42, value) }

// NormalizeTemporalDuration returns the canonical form of a Temporal.Duration value without enforcing shape.
func NormalizeTemporalDuration(value string) (string, error) { return callScalarNormalize(42, value) }

// ValidateTemporalDuration enforces the shape of a Temporal.Duration value.
func ValidateTemporalDuration(value string) error { return callScalarValidate(42, value) }

func (v TemporalDuration) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDuration and returns whether validation passed along with any errors.
func (v TemporalDuration) Validate() (bool, []ValidationError) {
	return validateScalarValue(42, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDuration) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMilliseconds validates and returns the canonical form of a Temporal.Milliseconds value.
func ParseTemporalMilliseconds(value string) (string, error) { return callScalarParse(43, value) }

// NormalizeTemporalMilliseconds returns the canonical form of a Temporal.Milliseconds value without enforcing shape.
func NormalizeTemporalMilliseconds(value string) (string, error) {
	return callScalarNormalize(43, value)
}

// ValidateTemporalMilliseconds enforces the shape of a Temporal.Milliseconds value.
func ValidateTemporalMilliseconds(value string) error { return callScalarValidate(43, value) }

func (v TemporalMilliseconds) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalMilliseconds and returns whether validation passed along with any errors.
func (v TemporalMilliseconds) Validate() (bool, []ValidationError) {
	return validateScalarValue(43, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMilliseconds) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMonth validates and returns the canonical form of a Temporal.Month value.
func ParseTemporalMonth(value string) (string, error) { return callScalarParse(44, value) }

// NormalizeTemporalMonth returns the canonical form of a Temporal.Month value without enforcing shape.
func NormalizeTemporalMonth(value string) (string, error) { return callScalarNormalize(44, value) }

// ValidateTemporalMonth enforces the shape of a Temporal.Month value.
func ValidateTemporalMonth(value string) error { return callScalarValidate(44, value) }

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
	return validateScalarValue(44, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMonth) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalQuarter validates and returns the canonical form of a Temporal.Quarter value.
func ParseTemporalQuarter(value string) (string, error) { return callScalarParse(45, value) }

// NormalizeTemporalQuarter returns the canonical form of a Temporal.Quarter value without enforcing shape.
func NormalizeTemporalQuarter(value string) (string, error) { return callScalarNormalize(45, value) }

// ValidateTemporalQuarter enforces the shape of a Temporal.Quarter value.
func ValidateTemporalQuarter(value string) error { return callScalarValidate(45, value) }

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
	return validateScalarValue(45, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalQuarter) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalQuarterYear validates and returns the canonical form of a Temporal.QuarterYear value.
func ParseTemporalQuarterYear(value string) (string, error) { return callScalarParse(46, value) }

// NormalizeTemporalQuarterYear returns the canonical form of a Temporal.QuarterYear value without enforcing shape.
func NormalizeTemporalQuarterYear(value string) (string, error) {
	return callScalarNormalize(46, value)
}

// ValidateTemporalQuarterYear enforces the shape of a Temporal.QuarterYear value.
func ValidateTemporalQuarterYear(value string) error { return callScalarValidate(46, value) }

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
	return validateScalarValue(46, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalQuarterYear) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalTime validates and returns the canonical form of a Temporal.Time value.
func ParseTemporalTime(value string) (string, error) { return callScalarParse(47, value) }

// NormalizeTemporalTime returns the canonical form of a Temporal.Time value without enforcing shape.
func NormalizeTemporalTime(value string) (string, error) { return callScalarNormalize(47, value) }

// ValidateTemporalTime enforces the shape of a Temporal.Time value.
func ValidateTemporalTime(value string) error { return callScalarValidate(47, value) }

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
	return validateScalarValue(47, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalTime) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalTimeZone validates and returns the canonical form of a Temporal.TimeZone value.
func ParseTemporalTimeZone(value string) (string, error) { return callScalarParse(48, value) }

// NormalizeTemporalTimeZone returns the canonical form of a Temporal.TimeZone value without enforcing shape.
func NormalizeTemporalTimeZone(value string) (string, error) { return callScalarNormalize(48, value) }

// ValidateTemporalTimeZone enforces the shape of a Temporal.TimeZone value.
func ValidateTemporalTimeZone(value string) error { return callScalarValidate(48, value) }

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
	return validateScalarValue(48, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalTimeZone) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalYear validates and returns the canonical form of a Temporal.Year value.
func ParseTemporalYear(value string) (string, error) { return callScalarParse(49, value) }

// NormalizeTemporalYear returns the canonical form of a Temporal.Year value without enforcing shape.
func NormalizeTemporalYear(value string) (string, error) { return callScalarNormalize(49, value) }

// ValidateTemporalYear enforces the shape of a Temporal.Year value.
func ValidateTemporalYear(value string) error { return callScalarValidate(49, value) }

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
	return validateScalarValue(49, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalYear) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTextMarkdown validates and returns the canonical form of a Text.Markdown value.
func ParseTextMarkdown(value string) (string, error) { return callScalarParse(50, value) }

// NormalizeTextMarkdown returns the canonical form of a Text.Markdown value without enforcing shape.
func NormalizeTextMarkdown(value string) (string, error) { return callScalarNormalize(50, value) }

// ValidateTextMarkdown enforces the shape of a Text.Markdown value.
func ValidateTextMarkdown(value string) error { return callScalarValidate(50, value) }

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
	return validateScalarValue(50, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TextMarkdown) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalSeconds validates and returns the canonical form of a Temporal.Seconds value.
func ParseTemporalSeconds(value string) (string, error) { return callScalarParse(52, value) }

// NormalizeTemporalSeconds returns the canonical form of a Temporal.Seconds value without enforcing shape.
func NormalizeTemporalSeconds(value string) (string, error) { return callScalarNormalize(52, value) }

// ValidateTemporalSeconds enforces the shape of a Temporal.Seconds value.
func ValidateTemporalSeconds(value string) error { return callScalarValidate(52, value) }

func (v TemporalSeconds) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalSeconds and returns whether validation passed along with any errors.
func (v TemporalSeconds) Validate() (bool, []ValidationError) {
	return validateScalarValue(52, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalSeconds) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalMinutes validates and returns the canonical form of a Temporal.Minutes value.
func ParseTemporalMinutes(value string) (string, error) { return callScalarParse(53, value) }

// NormalizeTemporalMinutes returns the canonical form of a Temporal.Minutes value without enforcing shape.
func NormalizeTemporalMinutes(value string) (string, error) { return callScalarNormalize(53, value) }

// ValidateTemporalMinutes enforces the shape of a Temporal.Minutes value.
func ValidateTemporalMinutes(value string) error { return callScalarValidate(53, value) }

func (v TemporalMinutes) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalMinutes and returns whether validation passed along with any errors.
func (v TemporalMinutes) Validate() (bool, []ValidationError) {
	return validateScalarValue(53, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalMinutes) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalHours validates and returns the canonical form of a Temporal.Hours value.
func ParseTemporalHours(value string) (string, error) { return callScalarParse(54, value) }

// NormalizeTemporalHours returns the canonical form of a Temporal.Hours value without enforcing shape.
func NormalizeTemporalHours(value string) (string, error) { return callScalarNormalize(54, value) }

// ValidateTemporalHours enforces the shape of a Temporal.Hours value.
func ValidateTemporalHours(value string) error { return callScalarValidate(54, value) }

func (v TemporalHours) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalHours and returns whether validation passed along with any errors.
func (v TemporalHours) Validate() (bool, []ValidationError) {
	return validateScalarValue(54, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalHours) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalDays validates and returns the canonical form of a Temporal.Days value.
func ParseTemporalDays(value string) (string, error) { return callScalarParse(55, value) }

// NormalizeTemporalDays returns the canonical form of a Temporal.Days value without enforcing shape.
func NormalizeTemporalDays(value string) (string, error) { return callScalarNormalize(55, value) }

// ValidateTemporalDays enforces the shape of a Temporal.Days value.
func ValidateTemporalDays(value string) error { return callScalarValidate(55, value) }

func (v TemporalDays) String() string {
	return scalarStringValue(v)
}

// Validate validates TemporalDays and returns whether validation passed along with any errors.
func (v TemporalDays) Validate() (bool, []ValidationError) {
	return validateScalarValue(55, scalarStringValue(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalDays) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTextSql validates and returns the canonical form of a Text.Sql value.
func ParseTextSql(value string) (string, error) { return callScalarParse(56, value) }

// NormalizeTextSql returns the canonical form of a Text.Sql value without enforcing shape.
func NormalizeTextSql(value string) (string, error) { return callScalarNormalize(56, value) }

// ValidateTextSql enforces the shape of a Text.Sql value.
func ValidateTextSql(value string) error { return callScalarValidate(56, value) }

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
	return validateScalarValue(56, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TextSql) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseCryptoSHA256 validates and returns the canonical form of a Crypto.SHA256 value.
func ParseCryptoSHA256(value string) (string, error) { return callScalarParse(58, value) }

// NormalizeCryptoSHA256 returns the canonical form of a Crypto.SHA256 value without enforcing shape.
func NormalizeCryptoSHA256(value string) (string, error) { return callScalarNormalize(58, value) }

// ValidateCryptoSHA256 enforces the shape of a Crypto.SHA256 value.
func ValidateCryptoSHA256(value string) error { return callScalarValidate(58, value) }

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
	return validateScalarValue(58, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v CryptoSHA256) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseNetworkDnsLabel validates and returns the canonical form of a Network.DnsLabel value.
func ParseNetworkDnsLabel(value string) (string, error) { return callScalarParse(59, value) }

// NormalizeNetworkDnsLabel returns the canonical form of a Network.DnsLabel value without enforcing shape.
func NormalizeNetworkDnsLabel(value string) (string, error) { return callScalarNormalize(59, value) }

// ValidateNetworkDnsLabel enforces the shape of a Network.DnsLabel value.
func ValidateNetworkDnsLabel(value string) error { return callScalarValidate(59, value) }

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
	return validateScalarValue(59, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v NetworkDnsLabel) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}

// ParseTemporalRecurrenceRule validates and returns the canonical form of a Temporal.RecurrenceRule value.
func ParseTemporalRecurrenceRule(value string) (string, error) { return callScalarParse(60, value) }

// NormalizeTemporalRecurrenceRule returns the canonical form of a Temporal.RecurrenceRule value without enforcing shape.
func NormalizeTemporalRecurrenceRule(value string) (string, error) {
	return callScalarNormalize(60, value)
}

// ValidateTemporalRecurrenceRule enforces the shape of a Temporal.RecurrenceRule value.
func ValidateTemporalRecurrenceRule(value string) error { return callScalarValidate(60, value) }

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
	return validateScalarValue(60, string(v))
}

// ValidateRequired validates with required check and returns whether validation passed along with any errors.
func (v TemporalRecurrenceRule) ValidateRequired() (bool, []ValidationError) {
	if scalarRequiredValueMissing(v) {
		return false, []ValidationError{{Validator: "required", Message: "required field"}}
	}
	return v.Validate()
}
