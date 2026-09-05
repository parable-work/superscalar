//! The built-in contribution to every assembly: the hand-written impls behind
//! the catalog defs. Defs with no entry in `impls` are served by
//! `DirectiveScalar`; assembly checks that against each def's tag. The
//! built-ins declare no legacy aliases; an extension that needs flat
//! pre-namespace names supplies them through `Extension::aliases`.

use crate::catalog::ScalarId;
use crate::metadata;
use crate::registry::Scalar;
use crate::scalars;

/// Owner name the built-ins report in errors and the dump. No extension may
/// take it.
pub(crate) const NAME: &str = "builtin";

pub(crate) fn impls() -> Vec<(ScalarId, Box<dyn Scalar>)> {
    vec![
        (
            ScalarId::GEO_LOCATION,
            Box::new(metadata::geo_location::GeoLocation),
        ),
        (ScalarId::CONTACT_EMAIL, Box::new(scalars::email::Email)),
        (
            ScalarId::CONTACT_PHONE_NUMBER,
            Box::new(scalars::phone_number::PhoneNumberScalar),
        ),
        (ScalarId::DESIGN_COLOR, Box::new(scalars::color::Color)),
        (
            ScalarId::EMBEDDING_VECTOR,
            Box::new(scalars::embedding_vector::EmbeddingVector),
        ),
        (ScalarId::GENERIC_JSON, Box::new(scalars::json_scalar::Json)),
        (
            ScalarId::GENERIC_STRING_MAP,
            Box::new(scalars::generic_string_map::GenericStringMap),
        ),
        (
            ScalarId::NETWORK_DNS_LABEL,
            Box::new(scalars::network_dns_label::NetworkDnsLabelScalar),
        ),
        (
            ScalarId::NETWORK_URL,
            Box::new(scalars::network_url::NetworkUrlScalar),
        ),
        (
            ScalarId::IDENTITY_UUID,
            Box::new(scalars::uuid_scalar::IdentityUuid),
        ),
        (
            ScalarId::TEMPORAL_RECURRENCE_RULE,
            Box::new(scalars::recurrence_rule::TemporalRecurrenceRule),
        ),
        (
            ScalarId::TEMPORAL_DATE,
            Box::new(scalars::temporal_date::TemporalDate),
        ),
        (
            ScalarId::TEMPORAL_DATE_TIME,
            Box::new(scalars::datetime::DateTimeScalar),
        ),
        (
            ScalarId::TEMPORAL_DURATION,
            Box::new(scalars::duration::Duration),
        ),
        (
            ScalarId::TEMPORAL_MONTH,
            Box::new(scalars::temporal_month::TemporalMonth),
        ),
        (
            ScalarId::TEMPORAL_QUARTER_YEAR,
            Box::new(scalars::temporal_quarter_year::TemporalQuarterYear),
        ),
    ]
}
