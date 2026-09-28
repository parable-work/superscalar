# @generated; do not edit
from . import _native

# Every canonical scalar name in the registry, sorted. The name is a scalar's
# identity: every call into the core passes it.
VALID_SCALARS = (
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
)

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
    "AgentSkill.Name": {"comparability_class": None, "is_sortable": True},
    "Auth.JWT": {"comparability_class": None, "is_sortable": True},
    "Auth.Password": {"comparability_class": None, "is_sortable": True},
    "Contact.Email": {"comparability_class": None, "is_sortable": True},
    "Contact.PhoneNumber": {"comparability_class": None, "is_sortable": True},
    "Crypto.RSAPrivateKey": {"comparability_class": None, "is_sortable": True},
    "Crypto.RSAPublicKey": {"comparability_class": None, "is_sortable": True},
    "Crypto.SHA256": {"comparability_class": None, "is_sortable": True},
    "Design.Color": {"comparability_class": None, "is_sortable": True},
    "Embedding.Vector": {"comparability_class": None, "is_sortable": False},
    "File.SizeBytes": {"comparability_class": None, "is_sortable": True},
    "Finance.Money": {"comparability_class": None, "is_sortable": True},
    "Generic.Int64": {"comparability_class": None, "is_sortable": True},
    "Generic.JSON": {"comparability_class": None, "is_sortable": False},
    "Generic.Probability": {"comparability_class": None, "is_sortable": True},
    "Generic.StringMap": {"comparability_class": None, "is_sortable": False},
    "Geo.Location": {"comparability_class": None, "is_sortable": False},
    "Git.PathPattern": {"comparability_class": None, "is_sortable": True},
    "Identity.Name": {"comparability_class": None, "is_sortable": True},
    "Identity.Slug": {"comparability_class": None, "is_sortable": True},
    "Identity.UUID": {"comparability_class": None, "is_sortable": True},
    "Identity.UserID": {"comparability_class": None, "is_sortable": True},
    "Localization.Locale": {"comparability_class": None, "is_sortable": True},
    "Network.DnsLabel": {"comparability_class": None, "is_sortable": True},
    "Network.DomainName": {"comparability_class": None, "is_sortable": True},
    "Network.IpAddress": {"comparability_class": None, "is_sortable": True},
    "Network.Uri": {"comparability_class": None, "is_sortable": True},
    "Network.Url": {"comparability_class": None, "is_sortable": True},
    "Ordering.Rank": {"comparability_class": None, "is_sortable": True},
    "Temporal.CronExpression": {"comparability_class": None, "is_sortable": True},
    "Temporal.Date": {"comparability_class": "temporal_instant", "is_sortable": True},
    "Temporal.DateTime": {"comparability_class": "temporal_instant", "is_sortable": True},
    "Temporal.Days": {"comparability_class": None, "is_sortable": True},
    "Temporal.Duration": {"comparability_class": None, "is_sortable": True},
    "Temporal.Hours": {"comparability_class": None, "is_sortable": True},
    "Temporal.Milliseconds": {"comparability_class": None, "is_sortable": True},
    "Temporal.Minutes": {"comparability_class": None, "is_sortable": True},
    "Temporal.Month": {"comparability_class": None, "is_sortable": True},
    "Temporal.Quarter": {"comparability_class": None, "is_sortable": True},
    "Temporal.QuarterYear": {"comparability_class": None, "is_sortable": True},
    "Temporal.RecurrenceRule": {"comparability_class": None, "is_sortable": True},
    "Temporal.Seconds": {"comparability_class": None, "is_sortable": True},
    "Temporal.Time": {"comparability_class": None, "is_sortable": True},
    "Temporal.TimeZone": {"comparability_class": None, "is_sortable": True},
    "Temporal.Year": {"comparability_class": None, "is_sortable": True},
    "Text.Markdown": {"comparability_class": None, "is_sortable": True},
    "Text.Sql": {"comparability_class": None, "is_sortable": True},
    "Version.SemVer": {"comparability_class": None, "is_sortable": True},
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

def parse_agent_skill_name(value: str) -> str:
    return _native.parse("AgentSkill.Name", value)

def normalize_agent_skill_name(value: str) -> str:
    return _native.normalize("AgentSkill.Name", value)

def validate_agent_skill_name(value: str) -> None:
    _native.validate("AgentSkill.Name", value)

def parse_auth_jwt(value: str) -> str:
    return _native.parse("Auth.JWT", value)

def normalize_auth_jwt(value: str) -> str:
    return _native.normalize("Auth.JWT", value)

def validate_auth_jwt(value: str) -> None:
    _native.validate("Auth.JWT", value)

def parse_auth_password(value: str) -> str:
    return _native.parse("Auth.Password", value)

def normalize_auth_password(value: str) -> str:
    return _native.normalize("Auth.Password", value)

def validate_auth_password(value: str) -> None:
    _native.validate("Auth.Password", value)

def parse_contact_email(value: str) -> str:
    return _native.parse("Contact.Email", value)

def normalize_contact_email(value: str) -> str:
    return _native.normalize("Contact.Email", value)

def validate_contact_email(value: str) -> None:
    _native.validate("Contact.Email", value)

def parse_contact_phone_number(value: str) -> str:
    return _native.parse("Contact.PhoneNumber", value)

def normalize_contact_phone_number(value: str) -> str:
    return _native.normalize("Contact.PhoneNumber", value)

def validate_contact_phone_number(value: str) -> None:
    _native.validate("Contact.PhoneNumber", value)

def parse_crypto_rsa_private_key(value: str) -> str:
    return _native.parse("Crypto.RSAPrivateKey", value)

def normalize_crypto_rsa_private_key(value: str) -> str:
    return _native.normalize("Crypto.RSAPrivateKey", value)

def validate_crypto_rsa_private_key(value: str) -> None:
    _native.validate("Crypto.RSAPrivateKey", value)

def parse_crypto_rsa_public_key(value: str) -> str:
    return _native.parse("Crypto.RSAPublicKey", value)

def normalize_crypto_rsa_public_key(value: str) -> str:
    return _native.normalize("Crypto.RSAPublicKey", value)

def validate_crypto_rsa_public_key(value: str) -> None:
    _native.validate("Crypto.RSAPublicKey", value)

def parse_crypto_sha256(value: str) -> str:
    return _native.parse("Crypto.SHA256", value)

def normalize_crypto_sha256(value: str) -> str:
    return _native.normalize("Crypto.SHA256", value)

def validate_crypto_sha256(value: str) -> None:
    _native.validate("Crypto.SHA256", value)

def parse_design_color(value: str) -> str:
    return _native.parse("Design.Color", value)

def normalize_design_color(value: str) -> str:
    return _native.normalize("Design.Color", value)

def validate_design_color(value: str) -> None:
    _native.validate("Design.Color", value)

def parse_embedding_vector(value: str) -> str:
    return _native.parse("Embedding.Vector", value)

def normalize_embedding_vector(value: str) -> str:
    return _native.normalize("Embedding.Vector", value)

def validate_embedding_vector(value: str) -> None:
    _native.validate("Embedding.Vector", value)

def parse_file_size_bytes(value: str) -> str:
    return _native.parse("File.SizeBytes", value)

def normalize_file_size_bytes(value: str) -> str:
    return _native.normalize("File.SizeBytes", value)

def validate_file_size_bytes(value: str) -> None:
    _native.validate("File.SizeBytes", value)

def parse_finance_money(value: str) -> str:
    return _native.parse("Finance.Money", value)

def normalize_finance_money(value: str) -> str:
    return _native.normalize("Finance.Money", value)

def validate_finance_money(value: str) -> None:
    _native.validate("Finance.Money", value)

def parse_generic_int64(value: str) -> str:
    return _native.parse("Generic.Int64", value)

def normalize_generic_int64(value: str) -> str:
    return _native.normalize("Generic.Int64", value)

def validate_generic_int64(value: str) -> None:
    _native.validate("Generic.Int64", value)

def parse_generic_json(value: str) -> str:
    return _native.parse("Generic.JSON", value)

def normalize_generic_json(value: str) -> str:
    return _native.normalize("Generic.JSON", value)

def validate_generic_json(value: str) -> None:
    _native.validate("Generic.JSON", value)

def parse_generic_probability(value: str) -> str:
    return _native.parse("Generic.Probability", value)

def normalize_generic_probability(value: str) -> str:
    return _native.normalize("Generic.Probability", value)

def validate_generic_probability(value: str) -> None:
    _native.validate("Generic.Probability", value)

def parse_generic_string_map(value: str) -> str:
    return _native.parse("Generic.StringMap", value)

def normalize_generic_string_map(value: str) -> str:
    return _native.normalize("Generic.StringMap", value)

def validate_generic_string_map(value: str) -> None:
    _native.validate("Generic.StringMap", value)

def parse_geo_location(value: str) -> str:
    return _native.parse("Geo.Location", value)

def normalize_geo_location(value: str) -> str:
    return _native.normalize("Geo.Location", value)

def validate_geo_location(value: str) -> None:
    _native.validate("Geo.Location", value)

def parse_git_path_pattern(value: str) -> str:
    return _native.parse("Git.PathPattern", value)

def normalize_git_path_pattern(value: str) -> str:
    return _native.normalize("Git.PathPattern", value)

def validate_git_path_pattern(value: str) -> None:
    _native.validate("Git.PathPattern", value)

def parse_identity_name(value: str) -> str:
    return _native.parse("Identity.Name", value)

def normalize_identity_name(value: str) -> str:
    return _native.normalize("Identity.Name", value)

def validate_identity_name(value: str) -> None:
    _native.validate("Identity.Name", value)

def parse_identity_slug(value: str) -> str:
    return _native.parse("Identity.Slug", value)

def normalize_identity_slug(value: str) -> str:
    return _native.normalize("Identity.Slug", value)

def validate_identity_slug(value: str) -> None:
    _native.validate("Identity.Slug", value)

def parse_identity_uuid(value: str) -> str:
    return _native.parse("Identity.UUID", value)

def normalize_identity_uuid(value: str) -> str:
    return _native.normalize("Identity.UUID", value)

def validate_identity_uuid(value: str) -> None:
    _native.validate("Identity.UUID", value)

def parse_identity_user_id(value: str) -> str:
    return _native.parse("Identity.UserID", value)

def normalize_identity_user_id(value: str) -> str:
    return _native.normalize("Identity.UserID", value)

def validate_identity_user_id(value: str) -> None:
    _native.validate("Identity.UserID", value)

def parse_localization_locale(value: str) -> str:
    return _native.parse("Localization.Locale", value)

def normalize_localization_locale(value: str) -> str:
    return _native.normalize("Localization.Locale", value)

def validate_localization_locale(value: str) -> None:
    _native.validate("Localization.Locale", value)

def parse_network_dns_label(value: str) -> str:
    return _native.parse("Network.DnsLabel", value)

def normalize_network_dns_label(value: str) -> str:
    return _native.normalize("Network.DnsLabel", value)

def validate_network_dns_label(value: str) -> None:
    _native.validate("Network.DnsLabel", value)

def parse_network_domain_name(value: str) -> str:
    return _native.parse("Network.DomainName", value)

def normalize_network_domain_name(value: str) -> str:
    return _native.normalize("Network.DomainName", value)

def validate_network_domain_name(value: str) -> None:
    _native.validate("Network.DomainName", value)

def parse_network_ip_address(value: str) -> str:
    return _native.parse("Network.IpAddress", value)

def normalize_network_ip_address(value: str) -> str:
    return _native.normalize("Network.IpAddress", value)

def validate_network_ip_address(value: str) -> None:
    _native.validate("Network.IpAddress", value)

def parse_network_uri(value: str) -> str:
    return _native.parse("Network.Uri", value)

def normalize_network_uri(value: str) -> str:
    return _native.normalize("Network.Uri", value)

def validate_network_uri(value: str) -> None:
    _native.validate("Network.Uri", value)

def parse_network_url(value: str) -> str:
    return _native.parse("Network.Url", value)

def normalize_network_url(value: str) -> str:
    return _native.normalize("Network.Url", value)

def validate_network_url(value: str) -> None:
    _native.validate("Network.Url", value)

def parse_ordering_rank(value: str) -> str:
    return _native.parse("Ordering.Rank", value)

def normalize_ordering_rank(value: str) -> str:
    return _native.normalize("Ordering.Rank", value)

def validate_ordering_rank(value: str) -> None:
    _native.validate("Ordering.Rank", value)

def parse_temporal_cron_expression(value: str) -> str:
    return _native.parse("Temporal.CronExpression", value)

def normalize_temporal_cron_expression(value: str) -> str:
    return _native.normalize("Temporal.CronExpression", value)

def validate_temporal_cron_expression(value: str) -> None:
    _native.validate("Temporal.CronExpression", value)

def parse_temporal_date(value: str) -> str:
    return _native.parse("Temporal.Date", value)

def normalize_temporal_date(value: str) -> str:
    return _native.normalize("Temporal.Date", value)

def validate_temporal_date(value: str) -> None:
    _native.validate("Temporal.Date", value)

def parse_temporal_date_time(value: str) -> str:
    return _native.parse("Temporal.DateTime", value)

def normalize_temporal_date_time(value: str) -> str:
    return _native.normalize("Temporal.DateTime", value)

def validate_temporal_date_time(value: str) -> None:
    _native.validate("Temporal.DateTime", value)

def parse_temporal_days(value: str) -> str:
    return _native.parse("Temporal.Days", value)

def normalize_temporal_days(value: str) -> str:
    return _native.normalize("Temporal.Days", value)

def validate_temporal_days(value: str) -> None:
    _native.validate("Temporal.Days", value)

def parse_temporal_duration(value: str) -> str:
    return _native.parse("Temporal.Duration", value)

def normalize_temporal_duration(value: str) -> str:
    return _native.normalize("Temporal.Duration", value)

def validate_temporal_duration(value: str) -> None:
    _native.validate("Temporal.Duration", value)

def parse_temporal_hours(value: str) -> str:
    return _native.parse("Temporal.Hours", value)

def normalize_temporal_hours(value: str) -> str:
    return _native.normalize("Temporal.Hours", value)

def validate_temporal_hours(value: str) -> None:
    _native.validate("Temporal.Hours", value)

def parse_temporal_milliseconds(value: str) -> str:
    return _native.parse("Temporal.Milliseconds", value)

def normalize_temporal_milliseconds(value: str) -> str:
    return _native.normalize("Temporal.Milliseconds", value)

def validate_temporal_milliseconds(value: str) -> None:
    _native.validate("Temporal.Milliseconds", value)

def parse_temporal_minutes(value: str) -> str:
    return _native.parse("Temporal.Minutes", value)

def normalize_temporal_minutes(value: str) -> str:
    return _native.normalize("Temporal.Minutes", value)

def validate_temporal_minutes(value: str) -> None:
    _native.validate("Temporal.Minutes", value)

def parse_temporal_month(value: str) -> str:
    return _native.parse("Temporal.Month", value)

def normalize_temporal_month(value: str) -> str:
    return _native.normalize("Temporal.Month", value)

def validate_temporal_month(value: str) -> None:
    _native.validate("Temporal.Month", value)

def parse_temporal_quarter(value: str) -> str:
    return _native.parse("Temporal.Quarter", value)

def normalize_temporal_quarter(value: str) -> str:
    return _native.normalize("Temporal.Quarter", value)

def validate_temporal_quarter(value: str) -> None:
    _native.validate("Temporal.Quarter", value)

def parse_temporal_quarter_year(value: str) -> str:
    return _native.parse("Temporal.QuarterYear", value)

def normalize_temporal_quarter_year(value: str) -> str:
    return _native.normalize("Temporal.QuarterYear", value)

def validate_temporal_quarter_year(value: str) -> None:
    _native.validate("Temporal.QuarterYear", value)

def parse_temporal_recurrence_rule(value: str) -> str:
    return _native.parse("Temporal.RecurrenceRule", value)

def normalize_temporal_recurrence_rule(value: str) -> str:
    return _native.normalize("Temporal.RecurrenceRule", value)

def validate_temporal_recurrence_rule(value: str) -> None:
    _native.validate("Temporal.RecurrenceRule", value)

def parse_temporal_seconds(value: str) -> str:
    return _native.parse("Temporal.Seconds", value)

def normalize_temporal_seconds(value: str) -> str:
    return _native.normalize("Temporal.Seconds", value)

def validate_temporal_seconds(value: str) -> None:
    _native.validate("Temporal.Seconds", value)

def parse_temporal_time(value: str) -> str:
    return _native.parse("Temporal.Time", value)

def normalize_temporal_time(value: str) -> str:
    return _native.normalize("Temporal.Time", value)

def validate_temporal_time(value: str) -> None:
    _native.validate("Temporal.Time", value)

def parse_temporal_time_zone(value: str) -> str:
    return _native.parse("Temporal.TimeZone", value)

def normalize_temporal_time_zone(value: str) -> str:
    return _native.normalize("Temporal.TimeZone", value)

def validate_temporal_time_zone(value: str) -> None:
    _native.validate("Temporal.TimeZone", value)

def parse_temporal_year(value: str) -> str:
    return _native.parse("Temporal.Year", value)

def normalize_temporal_year(value: str) -> str:
    return _native.normalize("Temporal.Year", value)

def validate_temporal_year(value: str) -> None:
    _native.validate("Temporal.Year", value)

def parse_text_markdown(value: str) -> str:
    return _native.parse("Text.Markdown", value)

def normalize_text_markdown(value: str) -> str:
    return _native.normalize("Text.Markdown", value)

def validate_text_markdown(value: str) -> None:
    _native.validate("Text.Markdown", value)

def parse_text_sql(value: str) -> str:
    return _native.parse("Text.Sql", value)

def normalize_text_sql(value: str) -> str:
    return _native.normalize("Text.Sql", value)

def validate_text_sql(value: str) -> None:
    _native.validate("Text.Sql", value)

def parse_version_sem_ver(value: str) -> str:
    return _native.parse("Version.SemVer", value)

def normalize_version_sem_ver(value: str) -> str:
    return _native.normalize("Version.SemVer", value)

def validate_version_sem_ver(value: str) -> None:
    _native.validate("Version.SemVer", value)
