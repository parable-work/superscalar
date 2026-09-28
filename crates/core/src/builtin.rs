//! The built-in contribution to every assembly: the hand-written impls behind
//! the catalog defs. Defs with no entry in `impls` are served by
//! `DirectiveScalar`; assembly checks that against each def's tag. The
//! built-ins declare no legacy aliases; an extension that needs flat
//! pre-namespace names supplies them through `Extension::aliases`.

use crate::catalog::names;
use crate::metadata;
use crate::registry::Scalar;
use crate::scalars;

/// Owner name the built-ins report in errors and the dump. No extension may
/// take it. Shared with `Definitions` assembly, which reports the same owner.
pub(crate) const NAME: &str = crate::definitions::BUILTIN_OWNER;

pub(crate) fn impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
    vec![
        (
            names::GEO_LOCATION,
            Box::new(metadata::geo_location::GeoLocation),
        ),
        (names::CONTACT_EMAIL, Box::new(scalars::email::Email)),
        (
            names::CONTACT_PHONE_NUMBER,
            Box::new(scalars::phone_number::PhoneNumberScalar),
        ),
        (names::DESIGN_COLOR, Box::new(scalars::color::Color)),
        (
            names::EMBEDDING_VECTOR,
            Box::new(scalars::embedding_vector::EmbeddingVector),
        ),
        (names::GENERIC_JSON, Box::new(scalars::json_scalar::Json)),
        (
            names::GENERIC_STRING_MAP,
            Box::new(scalars::generic_string_map::GenericStringMap),
        ),
        (
            names::GIT_PATH_PATTERN,
            Box::new(scalars::git_path_pattern::GitPathPatternScalar),
        ),
        (
            names::NETWORK_DNS_LABEL,
            Box::new(scalars::network_dns_label::NetworkDnsLabelScalar),
        ),
        (
            names::NETWORK_URL,
            Box::new(scalars::network_url::NetworkUrlScalar),
        ),
        (
            names::IDENTITY_UUID,
            Box::new(scalars::uuid_scalar::IdentityUuid),
        ),
        (
            names::TEMPORAL_RECURRENCE_RULE,
            Box::new(scalars::recurrence_rule::TemporalRecurrenceRule),
        ),
        (
            names::TEMPORAL_DATE,
            Box::new(scalars::temporal_date::TemporalDate),
        ),
        (
            names::TEMPORAL_DATE_TIME,
            Box::new(scalars::datetime::DateTimeScalar),
        ),
        (
            names::TEMPORAL_DURATION,
            Box::new(scalars::duration::Duration),
        ),
        (
            names::TEMPORAL_MONTH,
            Box::new(scalars::temporal_month::TemporalMonth),
        ),
        (
            names::TEMPORAL_QUARTER_YEAR,
            Box::new(scalars::temporal_quarter_year::TemporalQuarterYear),
        ),
    ]
}
