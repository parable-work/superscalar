# @generated; do not edit
from . import _native

SCALAR_ID_BY_CANONICAL = {
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
}

# Semantic facts, one row per scalar with a metadata row.
# `comparability_class` names an equivalence class, or is None for a scalar
# that has none, meaning self-comparable only. Field equality is NOT the
# comparability relation: two Nones compare equal, so a direct comparison
# answers true for every pair of unclassed scalars. The relation is
# `comparable_with` below, mirroring the Rust core. `is_sortable` is False for the
# structural, JSON and vector scalars. A scalar with no row here has NO
# sortability answer in this binding: `SCALAR_METADATA[name]` raises KeyError,
# which is the intended behaviour, not a gap.
#
# Deliberately ONE dict of plain dicts and not a NamedTuple and not one dict per
# field. Not a NamedTuple: this module is imported by six services and
# `pyproject.toml` declares requires-python = ">=3.9", and every dict form here
# is annotation-free, so nothing is evaluated at class-creation time on the
# floor (A33). Not one dict per field: the next metadata field would grow a
# third module-level name in a package that star-imports this module (A26,
# F11). The row shape matches the parity corpus row shape exactly.
SCALAR_METADATA = {
    "Auth.JWT": {"comparability_class": None, "is_sortable": True},
    "Auth.Password": {"comparability_class": None, "is_sortable": True},
    "Contact.Email": {"comparability_class": None, "is_sortable": True},
    "Contact.PhoneNumber": {"comparability_class": None, "is_sortable": True},
    "Crypto.RSAPrivateKey": {"comparability_class": None, "is_sortable": True},
    "Crypto.RSAPublicKey": {"comparability_class": None, "is_sortable": True},
    "Design.Color": {"comparability_class": None, "is_sortable": True},
    "Embedding.Vector": {"comparability_class": None, "is_sortable": False},
    "File.SizeBytes": {"comparability_class": None, "is_sortable": True},
    "Finance.Money": {"comparability_class": None, "is_sortable": True},
    "Generic.Int64": {"comparability_class": None, "is_sortable": True},
    "Generic.JSON": {"comparability_class": None, "is_sortable": False},
    "Generic.Probability": {"comparability_class": None, "is_sortable": True},
    "Generic.StringMap": {"comparability_class": None, "is_sortable": False},
    "Geo.Location": {"comparability_class": None, "is_sortable": False},
    "Identity.Name": {"comparability_class": None, "is_sortable": True},
    "Identity.Slug": {"comparability_class": None, "is_sortable": True},
    "Identity.UUID": {"comparability_class": None, "is_sortable": True},
    "Identity.UserID": {"comparability_class": None, "is_sortable": True},
    "Localization.Locale": {"comparability_class": None, "is_sortable": True},
    "Network.DomainName": {"comparability_class": None, "is_sortable": True},
    "Network.IpAddress": {"comparability_class": None, "is_sortable": True},
    "Network.Uri": {"comparability_class": None, "is_sortable": True},
    "Network.Url": {"comparability_class": None, "is_sortable": True},
    "Temporal.CronExpression": {"comparability_class": None, "is_sortable": True},
    "Temporal.Date": {"comparability_class": "temporal_instant", "is_sortable": True},
    "Temporal.DateTime": {"comparability_class": "temporal_instant", "is_sortable": True},
    "Temporal.Duration": {"comparability_class": None, "is_sortable": True},
    "Temporal.Milliseconds": {"comparability_class": None, "is_sortable": True},
    "Temporal.Month": {"comparability_class": None, "is_sortable": True},
    "Temporal.Quarter": {"comparability_class": None, "is_sortable": True},
    "Temporal.QuarterYear": {"comparability_class": None, "is_sortable": True},
    "Temporal.Time": {"comparability_class": None, "is_sortable": True},
    "Temporal.TimeZone": {"comparability_class": None, "is_sortable": True},
    "Temporal.Year": {"comparability_class": None, "is_sortable": True},
    "Text.Markdown": {"comparability_class": None, "is_sortable": True},
    "Temporal.Seconds": {"comparability_class": None, "is_sortable": True},
    "Temporal.Minutes": {"comparability_class": None, "is_sortable": True},
    "Temporal.Hours": {"comparability_class": None, "is_sortable": True},
    "Temporal.Days": {"comparability_class": None, "is_sortable": True},
    "Text.Sql": {"comparability_class": None, "is_sortable": True},
    "Crypto.SHA256": {"comparability_class": None, "is_sortable": True},
    "Network.DnsLabel": {"comparability_class": None, "is_sortable": True},
    "Temporal.RecurrenceRule": {"comparability_class": None, "is_sortable": True},
}

# Maps an alias scalar's canonical name to its target's. `alias_of` means one
# implementation under two ids, so the TARGET owns the comparability class and
# the alias inherits it. `comparable_with` resolves through this before it reads
# a class; reading the alias row's own class instead breaks transitivity one hop
# out (the alias pair itself still answers True because identity fires first, so
# the inconsistency hides where it was made). Underscore-prefixed so
# `from ._generated import *` does not grow a module-level name for it (A26,
# F11) -- the predicate is the contract, this table is its implementation.
_SCALAR_ALIAS_TARGETS = {
    "Identity.UserID": "Identity.UUID",
}


def comparable_with(a: str, b: str) -> bool:
    """Report whether comparing or joining values of two named scalars is meaningful.

    Mirrors ``ScalarDef::comparable_with`` in the Rust core: aliases resolve
    first, a scalar is always comparable with itself, and two distinct scalars
    are comparable only when both declare the same named class.

    Do NOT reimplement this as ``comparability_class`` equality. Almost every
    scalar carries ``None``, so field equality makes
    the entire catalog mutually comparable, ``Contact.Email`` against
    ``Contact.PhoneNumber`` included, which is the exact comparison the relation
    exists to reject.

    Fails closed on a name with no metadata row: an unknown scalar, and a
    scalar whose def sets ``metadata_omit`` and so has no comparability answer
    in this binding, both report False even against themselves. That matches
    the ``is_sortable`` allowlist, which answers "no" to a shape nobody declared.
    Note this differs from ``SCALAR_METADATA[name]``, which raises ``KeyError``
    -- a predicate has a fail-closed answer, a row lookup does not.

    The ``str``/``bool`` annotations are builtins, so they evaluate on the 3.9
    floor this module is checked against (``python/scripts/py39_floor_check.py``).
    """
    a = _SCALAR_ALIAS_TARGETS.get(a, a)
    b = _SCALAR_ALIAS_TARGETS.get(b, b)
    meta_a = SCALAR_METADATA.get(a)
    meta_b = SCALAR_METADATA.get(b)
    if meta_a is None or meta_b is None:
        return False
    if a == b:
        return True
    class_a = meta_a["comparability_class"]
    return class_a is not None and class_a == meta_b["comparability_class"]

def parse_auth_jwt(value: str) -> str:
    return _native.parse(5, value)

def normalize_auth_jwt(value: str) -> str:
    return _native.normalize(5, value)

def validate_auth_jwt(value: str) -> None:
    _native.validate(5, value)

def parse_auth_password(value: str) -> str:
    return _native.parse(6, value)

def normalize_auth_password(value: str) -> str:
    return _native.normalize(6, value)

def validate_auth_password(value: str) -> None:
    _native.validate(6, value)

def parse_contact_email(value: str) -> str:
    return _native.parse(8, value)

def normalize_contact_email(value: str) -> str:
    return _native.normalize(8, value)

def validate_contact_email(value: str) -> None:
    _native.validate(8, value)

def parse_contact_phone_number(value: str) -> str:
    return _native.parse(9, value)

def normalize_contact_phone_number(value: str) -> str:
    return _native.normalize(9, value)

def validate_contact_phone_number(value: str) -> None:
    _native.validate(9, value)

def parse_crypto_rsa_private_key(value: str) -> str:
    return _native.parse(10, value)

def normalize_crypto_rsa_private_key(value: str) -> str:
    return _native.normalize(10, value)

def validate_crypto_rsa_private_key(value: str) -> None:
    _native.validate(10, value)

def parse_crypto_rsa_public_key(value: str) -> str:
    return _native.parse(11, value)

def normalize_crypto_rsa_public_key(value: str) -> str:
    return _native.normalize(11, value)

def validate_crypto_rsa_public_key(value: str) -> None:
    _native.validate(11, value)

def parse_design_color(value: str) -> str:
    return _native.parse(12, value)

def normalize_design_color(value: str) -> str:
    return _native.normalize(12, value)

def validate_design_color(value: str) -> None:
    _native.validate(12, value)

def parse_embedding_vector(value: str) -> str:
    return _native.parse(13, value)

def normalize_embedding_vector(value: str) -> str:
    return _native.normalize(13, value)

def validate_embedding_vector(value: str) -> None:
    _native.validate(13, value)

def parse_file_size_bytes(value: str) -> str:
    return _native.parse(14, value)

def normalize_file_size_bytes(value: str) -> str:
    return _native.normalize(14, value)

def validate_file_size_bytes(value: str) -> None:
    _native.validate(14, value)

def parse_finance_money(value: str) -> str:
    return _native.parse(15, value)

def normalize_finance_money(value: str) -> str:
    return _native.normalize(15, value)

def validate_finance_money(value: str) -> None:
    _native.validate(15, value)

def parse_generic_int64(value: str) -> str:
    return _native.parse(16, value)

def normalize_generic_int64(value: str) -> str:
    return _native.normalize(16, value)

def validate_generic_int64(value: str) -> None:
    _native.validate(16, value)

def parse_generic_json(value: str) -> str:
    return _native.parse(17, value)

def normalize_generic_json(value: str) -> str:
    return _native.normalize(17, value)

def validate_generic_json(value: str) -> None:
    _native.validate(17, value)

def parse_generic_probability(value: str) -> str:
    return _native.parse(18, value)

def normalize_generic_probability(value: str) -> str:
    return _native.normalize(18, value)

def validate_generic_probability(value: str) -> None:
    _native.validate(18, value)

def parse_generic_string_map(value: str) -> str:
    return _native.parse(19, value)

def normalize_generic_string_map(value: str) -> str:
    return _native.normalize(19, value)

def validate_generic_string_map(value: str) -> None:
    _native.validate(19, value)

def parse_geo_location(value: str) -> str:
    return _native.parse(20, value)

def normalize_geo_location(value: str) -> str:
    return _native.normalize(20, value)

def validate_geo_location(value: str) -> None:
    _native.validate(20, value)

def parse_identity_name(value: str) -> str:
    return _native.parse(21, value)

def normalize_identity_name(value: str) -> str:
    return _native.normalize(21, value)

def validate_identity_name(value: str) -> None:
    _native.validate(21, value)

def parse_identity_slug(value: str) -> str:
    return _native.parse(22, value)

def normalize_identity_slug(value: str) -> str:
    return _native.normalize(22, value)

def validate_identity_slug(value: str) -> None:
    _native.validate(22, value)

def parse_identity_uuid(value: str) -> str:
    return _native.parse(23, value)

def normalize_identity_uuid(value: str) -> str:
    return _native.normalize(23, value)

def validate_identity_uuid(value: str) -> None:
    _native.validate(23, value)

def parse_identity_user_id(value: str) -> str:
    return _native.parse(24, value)

def normalize_identity_user_id(value: str) -> str:
    return _native.normalize(24, value)

def validate_identity_user_id(value: str) -> None:
    _native.validate(24, value)

def parse_localization_locale(value: str) -> str:
    return _native.parse(25, value)

def normalize_localization_locale(value: str) -> str:
    return _native.normalize(25, value)

def validate_localization_locale(value: str) -> None:
    _native.validate(25, value)

def parse_network_domain_name(value: str) -> str:
    return _native.parse(26, value)

def normalize_network_domain_name(value: str) -> str:
    return _native.normalize(26, value)

def validate_network_domain_name(value: str) -> None:
    _native.validate(26, value)

def parse_network_ip_address(value: str) -> str:
    return _native.parse(27, value)

def normalize_network_ip_address(value: str) -> str:
    return _native.normalize(27, value)

def validate_network_ip_address(value: str) -> None:
    _native.validate(27, value)

def parse_network_uri(value: str) -> str:
    return _native.parse(28, value)

def normalize_network_uri(value: str) -> str:
    return _native.normalize(28, value)

def validate_network_uri(value: str) -> None:
    _native.validate(28, value)

def parse_network_url(value: str) -> str:
    return _native.parse(29, value)

def normalize_network_url(value: str) -> str:
    return _native.normalize(29, value)

def validate_network_url(value: str) -> None:
    _native.validate(29, value)

def parse_temporal_cron_expression(value: str) -> str:
    return _native.parse(39, value)

def normalize_temporal_cron_expression(value: str) -> str:
    return _native.normalize(39, value)

def validate_temporal_cron_expression(value: str) -> None:
    _native.validate(39, value)

def parse_temporal_date(value: str) -> str:
    return _native.parse(40, value)

def normalize_temporal_date(value: str) -> str:
    return _native.normalize(40, value)

def validate_temporal_date(value: str) -> None:
    _native.validate(40, value)

def parse_temporal_date_time(value: str) -> str:
    return _native.parse(41, value)

def normalize_temporal_date_time(value: str) -> str:
    return _native.normalize(41, value)

def validate_temporal_date_time(value: str) -> None:
    _native.validate(41, value)

def parse_temporal_duration(value: str) -> str:
    return _native.parse(42, value)

def normalize_temporal_duration(value: str) -> str:
    return _native.normalize(42, value)

def validate_temporal_duration(value: str) -> None:
    _native.validate(42, value)

def parse_temporal_milliseconds(value: str) -> str:
    return _native.parse(43, value)

def normalize_temporal_milliseconds(value: str) -> str:
    return _native.normalize(43, value)

def validate_temporal_milliseconds(value: str) -> None:
    _native.validate(43, value)

def parse_temporal_month(value: str) -> str:
    return _native.parse(44, value)

def normalize_temporal_month(value: str) -> str:
    return _native.normalize(44, value)

def validate_temporal_month(value: str) -> None:
    _native.validate(44, value)

def parse_temporal_quarter(value: str) -> str:
    return _native.parse(45, value)

def normalize_temporal_quarter(value: str) -> str:
    return _native.normalize(45, value)

def validate_temporal_quarter(value: str) -> None:
    _native.validate(45, value)

def parse_temporal_quarter_year(value: str) -> str:
    return _native.parse(46, value)

def normalize_temporal_quarter_year(value: str) -> str:
    return _native.normalize(46, value)

def validate_temporal_quarter_year(value: str) -> None:
    _native.validate(46, value)

def parse_temporal_time(value: str) -> str:
    return _native.parse(47, value)

def normalize_temporal_time(value: str) -> str:
    return _native.normalize(47, value)

def validate_temporal_time(value: str) -> None:
    _native.validate(47, value)

def parse_temporal_time_zone(value: str) -> str:
    return _native.parse(48, value)

def normalize_temporal_time_zone(value: str) -> str:
    return _native.normalize(48, value)

def validate_temporal_time_zone(value: str) -> None:
    _native.validate(48, value)

def parse_temporal_year(value: str) -> str:
    return _native.parse(49, value)

def normalize_temporal_year(value: str) -> str:
    return _native.normalize(49, value)

def validate_temporal_year(value: str) -> None:
    _native.validate(49, value)

def parse_text_markdown(value: str) -> str:
    return _native.parse(50, value)

def normalize_text_markdown(value: str) -> str:
    return _native.normalize(50, value)

def validate_text_markdown(value: str) -> None:
    _native.validate(50, value)

def parse_temporal_seconds(value: str) -> str:
    return _native.parse(52, value)

def normalize_temporal_seconds(value: str) -> str:
    return _native.normalize(52, value)

def validate_temporal_seconds(value: str) -> None:
    _native.validate(52, value)

def parse_temporal_minutes(value: str) -> str:
    return _native.parse(53, value)

def normalize_temporal_minutes(value: str) -> str:
    return _native.normalize(53, value)

def validate_temporal_minutes(value: str) -> None:
    _native.validate(53, value)

def parse_temporal_hours(value: str) -> str:
    return _native.parse(54, value)

def normalize_temporal_hours(value: str) -> str:
    return _native.normalize(54, value)

def validate_temporal_hours(value: str) -> None:
    _native.validate(54, value)

def parse_temporal_days(value: str) -> str:
    return _native.parse(55, value)

def normalize_temporal_days(value: str) -> str:
    return _native.normalize(55, value)

def validate_temporal_days(value: str) -> None:
    _native.validate(55, value)

def parse_text_sql(value: str) -> str:
    return _native.parse(56, value)

def normalize_text_sql(value: str) -> str:
    return _native.normalize(56, value)

def validate_text_sql(value: str) -> None:
    _native.validate(56, value)

def parse_crypto_sha256(value: str) -> str:
    return _native.parse(58, value)

def normalize_crypto_sha256(value: str) -> str:
    return _native.normalize(58, value)

def validate_crypto_sha256(value: str) -> None:
    _native.validate(58, value)

def parse_network_dns_label(value: str) -> str:
    return _native.parse(59, value)

def normalize_network_dns_label(value: str) -> str:
    return _native.normalize(59, value)

def validate_network_dns_label(value: str) -> None:
    _native.validate(59, value)

def parse_temporal_recurrence_rule(value: str) -> str:
    return _native.parse(60, value)

def normalize_temporal_recurrence_rule(value: str) -> str:
    return _native.normalize(60, value)

def validate_temporal_recurrence_rule(value: str) -> None:
    _native.validate(60, value)
