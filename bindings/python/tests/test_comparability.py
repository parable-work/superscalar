"""Mirror of ``comparability_across_distinct_scalars_is_exactly_the_temporal_instant_pair``
(``core/tests/semantic_metadata.rs``) for the Python binding.

The Rust core owns the rule. This pins that the binding exposes it, because
``SCALAR_METADATA`` ships ``comparability_class`` and the obvious consumer
implementation over that field is wrong.
"""

from superscalar import comparable_with
from superscalar._generated import SCALAR_METADATA, _SCALAR_ALIAS_TARGETS


def test_a_scalar_is_comparable_with_itself():
    assert comparable_with("Contact.Email", "Contact.Email")
    assert comparable_with("Contact.PhoneNumber", "Contact.PhoneNumber")


def test_two_class_less_scalars_are_not_comparable():
    # The spec's named counter-example (0084-semantic-types.mdx). Naive field
    # equality answers True here because both classes are None. This is the
    # assertion the whole ticket exists for.
    assert not comparable_with("Contact.Email", "Contact.PhoneNumber")
    assert not comparable_with("Contact.PhoneNumber", "Contact.Email")
    assert (
        SCALAR_METADATA["Contact.Email"]["comparability_class"]
        == SCALAR_METADATA["Contact.PhoneNumber"]["comparability_class"]
    ), "classes differ; the assertions above no longer prove field equality is wrong"


def test_an_alias_is_the_same_scalar():
    # One implementation under two ids. Identity.UserID -> Identity.UUID is the
    # catalog's only alias pair.
    assert _SCALAR_ALIAS_TARGETS["Identity.UserID"] == "Identity.UUID"
    assert comparable_with("Identity.UserID", "Identity.UUID")
    assert comparable_with("Identity.UUID", "Identity.UserID")


def test_a_name_with_no_metadata_row_fails_closed():
    assert not comparable_with("Not.AScalar", "Not.AScalar")
    assert not comparable_with("Not.AScalar", "Contact.Email")


def test_the_temporal_instant_pair_is_comparable():
    # The one named class: Temporal.Date and Temporal.DateTime share
    # temporal_instant, so the predicate answers True across them.
    assert comparable_with("Temporal.Date", "Temporal.DateTime")
    assert comparable_with("Temporal.DateTime", "Temporal.Date")
