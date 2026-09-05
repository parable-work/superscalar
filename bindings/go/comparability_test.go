package superscalar

import "testing"

// Mirrors comparability_across_distinct_scalars_is_exactly_the_temporal_instant_pair in
// core/tests/semantic_metadata.rs. The Rust core owns the rule; this pins that
// the Go binding exposes it, because the binding ships ComparabilityClass as a
// bare string and the obvious consumer implementation over that field is wrong.
func TestComparableWith(t *testing.T) {
	// Self-comparable, which is what an absent class means.
	if !ComparableWith("Contact.Email", "Contact.Email") {
		t.Error("Contact.Email is not comparable with itself")
	}
	if !ComparableWith("Contact.PhoneNumber", "Contact.PhoneNumber") {
		t.Error("Contact.PhoneNumber is not comparable with itself")
	}

	// The spec's named counter-example (0084-semantic-types.mdx). Naive field
	// equality answers true here because both classes are absent. This is the
	// assertion the whole ticket exists for.
	if ComparableWith("Contact.Email", "Contact.PhoneNumber") {
		t.Error("Contact.Email and Contact.PhoneNumber are comparable")
	}
	if ComparableWith("Contact.PhoneNumber", "Contact.Email") {
		t.Error("comparability is not symmetric")
	}
	emailClass := ScalarMetadataByCanonical["Contact.Email"].ComparabilityClass
	phoneClass := ScalarMetadataByCanonical["Contact.PhoneNumber"].ComparabilityClass
	if emailClass != phoneClass {
		t.Errorf("classes differ (%q vs %q); the two assertions above no longer "+
			"prove field equality is the wrong implementation", emailClass, phoneClass)
	}

	// An alias is one implementation under two ids, so it is the same scalar
	// for comparison purposes. Identity.UserID -> Identity.UUID is the catalog's
	// only alias pair.
	if !ComparableWith("Identity.UserID", "Identity.UUID") {
		t.Error("the alias pair is not comparable")
	}
	if !ComparableWith("Identity.UUID", "Identity.UserID") {
		t.Error("the alias pair is not comparable in reverse")
	}

	// Go's sharp edge: absent class and map miss are both the empty string, so a
	// field-equality implementation cannot tell them apart and calls two unknown
	// names comparable.
	if ComparableWith("Not.AScalar", "Not.AScalar") {
		t.Error("an unknown scalar is comparable with itself")
	}
	if ComparableWith("Not.AScalar", "Contact.Email") {
		t.Error("an unknown scalar is comparable with a known one")
	}

	// The one named class: Temporal.Date and Temporal.DateTime share
	// temporal_instant, so the predicate answers true across them and nowhere
	// else.
	if !ComparableWith("Temporal.Date", "Temporal.DateTime") {
		t.Error("Temporal.Date and Temporal.DateTime share temporal_instant")
	}
	if !ComparableWith("Temporal.DateTime", "Temporal.Date") {
		t.Error("the temporal_instant pair is not comparable in reverse")
	}
}
