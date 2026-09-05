"""superscalar: the scalar types library.

The per-scalar modules expose historical signatures some consumers depend on
(``validate_*`` returning ``list[ValidationError]``, ``Optional[str]``
parse/normalize for the temporal scalars). They are imported AFTER
``from ._generated import *`` so the richer-contract versions win for the
names they share.
"""

from dataclasses import dataclass

from ._generated import *  # noqa: F401,F403
from ._generated import SCALAR_ID_BY_CANONICAL  # noqa: F401
from ._generated import SCALAR_METADATA  # noqa: F401


@dataclass
class ValidationError:
    """Represents a single validation error.

    Attributes:
        validator: The name of the validator that failed (e.g., "format", "custom").
        message: A human-readable description of the validation failure.
    """

    validator: str
    message: str


from .contact_email import (  # noqa: E402,F401
    normalize_contact_email,
    parse_contact_email,
    validate_contact_email,
)
from .contact_phone_number import (  # noqa: E402,F401
    normalize_contact_phone_number,
    parse_contact_phone_number,
    validate_contact_phone_number,
)
from .design_color import (  # noqa: E402,F401
    normalize_design_color,
    parse_design_color,
    validate_design_color,
)
from .generic_json import (  # noqa: E402,F401
    normalize_generic_json,
    parse_generic_json,
    validate_generic_json,
)
from .identity_uuid import (  # noqa: E402,F401
    normalize_identity_uuid,
    parse_identity_uuid,
    parse_identity_uuid_to_uuid,
    validate_identity_uuid,
)
from .temporal_date import (  # noqa: E402,F401
    normalize_temporal_date,
    parse_temporal_date,
    validate_temporal_date,
)
from .temporal_date_time import (  # noqa: E402,F401
    normalize_temporal_date_time,
    parse_temporal_date_time,
    validate_temporal_date_time,
)
from .temporal_duration import (  # noqa: E402,F401
    normalize_temporal_duration,
    parse_temporal_duration,
    validate_temporal_duration,
)
from .temporal_month import (  # noqa: E402,F401
    normalize_temporal_month,
    parse_temporal_month,
    validate_temporal_month,
)
from .temporal_quarter_year import (  # noqa: E402,F401
    normalize_temporal_quarter_year,
    parse_temporal_quarter_year,
    validate_temporal_quarter_year,
)


# Aliases for scalars that share an implementation with another scalar.
# Identity.UserID reuses the Identity.UUID base62/canonical-UUID handling.
normalize_identity_user_id = normalize_identity_uuid
parse_identity_user_id = parse_identity_uuid
validate_identity_user_id = validate_identity_uuid
