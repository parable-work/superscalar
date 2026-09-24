package superscalar

import "testing"

// The emitted primitive table is what a Go reader uses instead of writing its
// own five-arm switch over a Rust function it cannot see. These pin the two
// properties that reader depends on: the lookup covers every primitive, and the
// two spellings really are different where the emitter says they are.

func TestPrimitiveDSLNameByTypeRefNameCoversEveryPrimitive(t *testing.T) {
	if len(PRIMITIVE_KINDS) == 0 {
		t.Fatal("no primitives were emitted")
	}
	if len(PrimitiveDSLNameByTypeRefName) != len(PRIMITIVE_KINDS) {
		t.Fatalf("index has %d entries for %d primitives: two primitives share an IR type-ref name",
			len(PrimitiveDSLNameByTypeRefName), len(PRIMITIVE_KINDS))
	}
	for _, primitive := range PRIMITIVE_KINDS {
		if primitive.Name == "" || primitive.TypeRefName == "" || primitive.DSLName == "" {
			t.Fatalf("primitive %+v was emitted without both spellings", primitive)
		}
		got, ok := PrimitiveDSLNameByTypeRefName[primitive.TypeRefName]
		if !ok {
			t.Fatalf("%s (%s) is missing from the index", primitive.Name, primitive.TypeRefName)
		}
		if got != primitive.DSLName {
			t.Fatalf("%s indexes to %q, want %q", primitive.Name, got, primitive.DSLName)
		}
	}
}

// The whole reason the pair is emitted rather than converted. If these two ever
// agree, the mapping has become an identity and this file can go -- but silently
// treating them as equal is how a derived Boolean column arrives at its
// consumer as an unknown type reference.
func TestTheIRAndDSLSpellingsDisagreeWhereTheEmitterSaysTheyDo(t *testing.T) {
	for typeRefName, want := range map[string]string{
		"Boolean": "Bool",
		"JSON":    "Type",
		"String":  "String",
		"Int":     "Int",
		"Float":   "Float",
	} {
		if got := PrimitiveDSLNameByTypeRefName[typeRefName]; got != want {
			t.Fatalf("type ref %q maps to %q, want %q", typeRefName, got, want)
		}
	}
	// The DSL spellings that differ are not type-ref names, and a scalar
	// reference is resolved through ScalarMetadataByCanonical instead. Nor is
	// the dotted Generic.JSON: it is a catalog scalar, and a bare primitive
	// must not read as one.
	for _, notATypeRef := range []string{"Bool", "Type", "scalars/Contact.Email", "Generic.JSON", ""} {
		if _, ok := PrimitiveDSLNameByTypeRefName[notATypeRef]; ok {
			t.Fatalf("%q must not resolve as an IR type-ref name", notATypeRef)
		}
	}
}
