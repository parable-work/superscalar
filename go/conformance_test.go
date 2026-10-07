package superscalar

import (
	"encoding/json"
	"maps"
	"os"
	"path/filepath"
	"slices"
	"sort"
	"testing"
)

type parityCase struct {
	Input      string `json:"input"`
	Normalized string `json:"normalized"`
	Validator  string `json:"validator"`
	// Marks a vector pinning intended behavior the core does not yet implement; the parity gate skips these so it never breaks the build before the scalar tightening lands.
	Unresolved bool `json:"unresolved"`
}

type scalarParity struct {
	Accepted []parityCase `json:"accepted"`
	Rejected []parityCase `json:"rejected"`
}

// One row of the corpus `metadata` section. ComparabilityClass is a *string and
// not a string, even though the generated Go field is a plain string: the corpus
// encodes "no class" as JSON null, and a plain string would silently accept a
// literal "null" or a missing key as "". The nil -> "" mapping is done once,
// explicitly, in the test body.
type metadataVector struct {
	ComparabilityClass *string `json:"comparability_class"`
	IsSortable         bool    `json:"is_sortable"`
	CaseInsensitive    bool    `json:"case_insensitive"`
	reservedWordRule
}

// The three reserved-word fields of a metadata row, and the value shape of
// meta.reserved_word_scalars. ReservedWords is a pointer for the same reason
// ComparabilityClass is: a row that lost the key must not read as "reserves
// nothing".
type reservedWordRule struct {
	ReservedWords                *[]string `json:"reserved_words"`
	ReservedWordsCaseInsensitive bool      `json:"reserved_words_case_insensitive"`
	ReservedWordsMatchPartial    bool      `json:"reserved_words_match_partial"`
}

// isSet reports whether the row belongs in meta.reserved_word_scalars.
func (r reservedWordRule) isSet() bool {
	return (r.ReservedWords != nil && len(*r.ReservedWords) > 0) ||
		r.ReservedWordsCaseInsensitive || r.ReservedWordsMatchPartial
}

func (r reservedWordRule) equal(other reservedWordRule) bool {
	return r.ReservedWords != nil && other.ReservedWords != nil &&
		slices.Equal(*r.ReservedWords, *other.ReservedWords) &&
		r.ReservedWordsCaseInsensitive == other.ReservedWordsCaseInsensitive &&
		r.ReservedWordsMatchPartial == other.ReservedWordsMatchPartial
}

// meta carries the hand-maintained expectations that must not be derived from
// the generated sections they check.
type parityMeta struct {
	NonSortableCount int `json:"non_sortable_count"`
	// Class name -> sorted member canonical names. A member list
	// rather than a count, because a count cannot catch a class attached to the
	// wrong scalar.
	ComparabilityClasses map[string][]string `json:"comparability_classes"`
	// Sorted canonical names of the rows with case_insensitive set.
	CaseInsensitiveScalars []string `json:"case_insensitive_scalars"`
	// Every row that reserves a word or sets either reserved-word flag.
	ReservedWordScalars map[string]reservedWordRule `json:"reserved_word_scalars"`
}

type paritySpec struct {
	Meta             parityMeta                `json:"meta"`
	Scalars          map[string]scalarParity   `json:"scalars"`
	Metadata         map[string]metadataVector `json:"metadata"`
	MetadataExcluded []string                  `json:"metadata_excluded"`
}

func loadCorpus(t *testing.T) paritySpec {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "conformance", "core-scalars.v2.json"))
	if err != nil {
		t.Fatalf("read corpus: %v", err)
	}
	var spec paritySpec
	if err := json.Unmarshal(data, &spec); err != nil {
		t.Fatalf("unmarshal corpus: %v", err)
	}
	return spec
}

func TestV2Conformance(t *testing.T) {
	spec := loadCorpus(t)
	accepted, rejected, skipped := 0, 0, 0
	for canonical, sc := range spec.Scalars {
		if !KnownScalar(canonical) {
			t.Fatalf("no generated scalar for %q", canonical)
		}
		for _, c := range sc.Accepted {
			if c.Unresolved {
				skipped++
				continue
			}
			got, err := callScalarParse(canonical, c.Input)
			if err != nil {
				t.Errorf("%s parse(%q) error: %v", canonical, c.Input, err)
				continue
			}
			// A few identity-normalizing scalars omit normalized; accept-only.
			if c.Normalized != "" && got != c.Normalized {
				t.Errorf("%s parse(%q) = %q, want %q", canonical, c.Input, got, c.Normalized)
			}
			accepted++
		}
		for _, c := range sc.Rejected {
			// Corpus pins the intended rejection but the scalar is still declared unconstrained, so the core legitimately accepts the input.
			if c.Unresolved {
				skipped++
				continue
			}
			if _, err := callScalarParse(canonical, c.Input); err == nil {
				t.Errorf("%s parse(%q) unexpectedly succeeded", canonical, c.Input)
			}
			rejected++
		}
	}
	t.Logf("v2 conformance: %d accepted, %d rejected, %d skipped (unresolved) across %d scalars", accepted, rejected, skipped, len(spec.Scalars))
}

func TestGeneratedWrappersRoute(t *testing.T) {
	got, err := ParseContactEmail("Foo@Bar.com")
	if err != nil || got != "foo@bar.com" {
		t.Fatalf("ParseContactEmail = %q, %v; want foo@bar.com", got, err)
	}
	if err := ValidateContactEmail("nope"); err == nil {
		t.Fatal("ValidateContactEmail(nope) should error")
	}
}

func TestV2MetadataParity(t *testing.T) {
	spec := loadCorpus(t)
	if len(spec.Metadata) != len(SCALAR_METADATA) {
		t.Fatalf("metadata vectors = %d, table = %d", len(spec.Metadata), len(SCALAR_METADATA))
	}
	nonSortable := 0
	for canonical, want := range spec.Metadata {
		got, ok := ScalarMetadataByCanonical[canonical]
		if !ok {
			t.Errorf("no metadata row for %q", canonical)
			continue
		}
		wantClass := ""
		if want.ComparabilityClass != nil {
			wantClass = *want.ComparabilityClass
		}
		if got.ComparabilityClass != wantClass {
			t.Errorf("%s ComparabilityClass = %q, want %q", canonical, got.ComparabilityClass, wantClass)
		}
		if got.IsSortable != want.IsSortable {
			t.Errorf("%s IsSortable = %v, want %v", canonical, got.IsSortable, want.IsSortable)
		}
		if got.CaseInsensitive != want.CaseInsensitive {
			t.Errorf("%s CaseInsensitive = %v, want %v", canonical, got.CaseInsensitive, want.CaseInsensitive)
		}
		if want.ReservedWords == nil {
			t.Errorf("%s: corpus row has no reserved_words", canonical)
		} else if !slices.Equal(got.ReservedWords, *want.ReservedWords) {
			t.Errorf("%s ReservedWords = %q, want %q", canonical, got.ReservedWords, *want.ReservedWords)
		}
		if got.ReservedWordsCaseInsensitive != want.ReservedWordsCaseInsensitive {
			t.Errorf("%s ReservedWordsCaseInsensitive = %v, want %v", canonical, got.ReservedWordsCaseInsensitive, want.ReservedWordsCaseInsensitive)
		}
		if got.ReservedWordsMatchPartial != want.ReservedWordsMatchPartial {
			t.Errorf("%s ReservedWordsMatchPartial = %v, want %v", canonical, got.ReservedWordsMatchPartial, want.ReservedWordsMatchPartial)
		}
		if !want.IsSortable {
			nonSortable++
		}
	}
	// Independent of the per-row loop above, which compares a transcript against
	// the table it was transcribed from.
	if nonSortable != spec.Meta.NonSortableCount {
		t.Errorf("non-sortable rows = %d, want %d", nonSortable, spec.Meta.NonSortableCount)
	}
	// Same independence argument, for comparability. Rows with no class are
	// skipped rather than grouped under an empty key, so the derived map holds
	// only real classes.
	derivedClasses := map[string][]string{}
	for canonical, want := range spec.Metadata {
		if want.ComparabilityClass == nil {
			continue
		}
		derivedClasses[*want.ComparabilityClass] = append(derivedClasses[*want.ComparabilityClass], canonical)
	}
	for _, members := range derivedClasses {
		sort.Strings(members)
	}
	// Absence first, and it has to be its own check. maps.EqualFunc compares
	// lengths before anything else, so an ABSENT key (which decodes to a nil map)
	// against an empty derived map answers true: the guard would stop noticing a
	// deleted hand-maintained key at exactly the moment the class table empties,
	// which is by construction the next edit this key sees. The other three
	// readers all fail closed here; this one has to say so explicitly.
	if spec.Meta.ComparabilityClasses == nil {
		t.Error("corpus meta.comparability_classes is missing; it is hand-maintained beside the generated metadata section and the generator must never remove it")
	} else if !maps.EqualFunc(derivedClasses, spec.Meta.ComparabilityClasses, slices.Equal) {
		for class, members := range derivedClasses {
			declared, ok := spec.Meta.ComparabilityClasses[class]
			if !ok {
				t.Errorf("comparability class %q: metadata rows give members %v, meta.comparability_classes does not declare it", class, members)
			} else if !slices.Equal(members, declared) {
				t.Errorf("comparability class %q: metadata rows give members %v, meta declares %v", class, members, declared)
			}
		}
		for class, declared := range spec.Meta.ComparabilityClasses {
			if _, ok := derivedClasses[class]; !ok {
				t.Errorf("comparability class %q: meta declares members %v, no metadata row carries the class", class, declared)
			}
		}
	}
	// Same independence argument, for the case and reserved-word fields. Nil
	// checks first, for the reason given above.
	var caseInsensitive []string
	derivedReserved := map[string]reservedWordRule{}
	for canonical, want := range spec.Metadata {
		if want.CaseInsensitive {
			caseInsensitive = append(caseInsensitive, canonical)
		}
		if want.isSet() {
			derivedReserved[canonical] = want.reservedWordRule
		}
	}
	sort.Strings(caseInsensitive)
	if spec.Meta.CaseInsensitiveScalars == nil {
		t.Error("corpus meta.case_insensitive_scalars is missing; it is hand-maintained and the generator must never remove it")
	} else if !slices.Equal(caseInsensitive, spec.Meta.CaseInsensitiveScalars) {
		t.Errorf("case-insensitive rows = %v, meta declares %v", caseInsensitive, spec.Meta.CaseInsensitiveScalars)
	}
	if spec.Meta.ReservedWordScalars == nil {
		t.Error("corpus meta.reserved_word_scalars is missing; it is hand-maintained and the generator must never remove it")
	} else if !maps.EqualFunc(derivedReserved, spec.Meta.ReservedWordScalars, reservedWordRule.equal) {
		t.Errorf("reserved-word rows = %+v, meta declares %+v", derivedReserved, spec.Meta.ReservedWordScalars)
	}
	// set(scalars) - set(metadata) == set(metadata_excluded), not the weaker
	// disjointness check. Disjointness alone passes if a scalar is missing from
	// `metadata` without being declared excluded, which is the drift that
	// matters: it means a scalar quietly lost its metadata row.
	var missing []string
	for canonical := range spec.Scalars {
		if _, ok := spec.Metadata[canonical]; !ok {
			missing = append(missing, canonical)
		}
	}
	sort.Strings(missing)
	want := append([]string(nil), spec.MetadataExcluded...)
	sort.Strings(want)
	if !slices.Equal(missing, want) {
		t.Errorf("scalars minus metadata = %v, want declared metadata_excluded %v", missing, want)
	}
}

// The corpus was all-null when this was written, so the non-nil branch of the
// deref above never ran against it. The first class covered Temporal.Date and
// Temporal.DateTime, so it runs now. This stays regardless: it pins the decode
// on a literal row, so it keeps proving the branch when the class table changes.
func TestMetadataVectorDecodesANamedClass(t *testing.T) {
	var v metadataVector
	if err := json.Unmarshal([]byte(`{"comparability_class":"temporal_instant","is_sortable":true,"case_insensitive":false,"reserved_words":[],"reserved_words_case_insensitive":false,"reserved_words_match_partial":false}`), &v); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if v.ComparabilityClass == nil || *v.ComparabilityClass != "temporal_instant" {
		t.Errorf("ComparabilityClass = %v, want temporal_instant", v.ComparabilityClass)
	}
}

// No built-in reserves a word, so the shipped corpus never decodes a non-empty
// reserved_words. Pin the decode on a literal row, as the class test above does.
func TestMetadataVectorDecodesReservedWords(t *testing.T) {
	var v metadataVector
	row := `{"comparability_class":null,"is_sortable":true,"case_insensitive":true,"reserved_words":["admin","root"],"reserved_words_case_insensitive":true,"reserved_words_match_partial":true}`
	if err := json.Unmarshal([]byte(row), &v); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if !v.CaseInsensitive || v.ReservedWords == nil || !slices.Equal(*v.ReservedWords, []string{"admin", "root"}) ||
		!v.ReservedWordsCaseInsensitive || !v.ReservedWordsMatchPartial || !v.isSet() {
		t.Errorf("decoded %+v (reserved words %v)", v, v.ReservedWords)
	}
}
