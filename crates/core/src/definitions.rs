//! `Definitions`: the assembled scalar definitions, with no implementations.
//!
//! WHY THIS IS ITS OWN TYPE. A consumer that needs only definitions -- a def
//! by id or canonical name, alias resolution, comparability, iterating the
//! catalog -- must not link the scalar implementations. `Registry` cannot give
//! it that: assembling one calls the built-in impl table and every extension's
//! `impls()`, and a call through `&dyn Extension` keeps `impls` reachable
//! through the vtable, so the phone-number library and its embedded metadata,
//! the regex engine and every hand-written scalar end up in the binary. In a
//! size-capped WASM bundle that is the difference between fitting and not.
//!
//! So this module assembles from plain `&'static [ScalarDef]` slices, which are
//! data, and nothing else. It must never name the `Scalar` trait, the
//! `Extension` trait, the directive engine, `regex` or an impl table;
//! `definitions_module_stays_free_of_implementations` below fails the build's
//! tests if it does. `Registry` assembles one of these first and delegates
//! every definition lookup to it, so the lookup and comparability rules have
//! exactly one home.
//!
//! The checks that run here are the ones that need only defs: no two defs
//! share an id (`DuplicateId`) or a canonical name (`DuplicateCanonical`),
//! every `namespace` is its canonical prefix (`NamespaceMismatch`), and every
//! `alias_of` names an assembled def (`DanglingAlias`) that is not itself an
//! alias (`AliasChain`). Pattern compilation, the impl checks, the id-block
//! checks and extension naming belong to `Registry` assembly, which runs them
//! around these in its documented order, so for the same input both report
//! the same `AssemblyError`.

use crate::catalog::{ScalarId, CATALOG};
use crate::extension::AssemblyError;
use crate::registry::ScalarDef;
use std::collections::{BTreeMap, HashMap};
use std::sync::LazyLock;

/// Owner name the built-in defs report in assembly errors. The same value
/// `Registry` reports for its built-in block.
pub(crate) const BUILTIN_OWNER: &str = "builtin";

/// One contribution to an assembly: the owner name assembly errors report
/// (`"acme"`) and the defs it declares, the same `'static` slice an
/// `Extension::defs` returns.
pub type DefSource = (&'static str, &'static [ScalarDef]);

/// The assembled scalar definitions: the built-ins plus zero or more def
/// slices, checked once and then read-only. Answers every definition question
/// a `Registry` answers, with the same semantics, and links no scalar
/// implementation. `Definitions::builtin()` is the process-wide built-in set;
/// `Registry::definitions` is the set a registry was assembled from.
pub struct Definitions {
    by_id: BTreeMap<u32, &'static ScalarDef>,
    by_canonical: HashMap<&'static str, ScalarId>,
}

static BUILTIN: LazyLock<Definitions> = LazyLock::new(|| Definitions::assemble(&[]));

impl Definitions {
    /// The built-in definitions, no extensions, assembled on first use.
    pub fn builtin() -> &'static Definitions {
        &BUILTIN
    }

    /// The built-ins plus `extensions`, each an `(owner, defs)` pair. Panics
    /// on any `AssemblyError`, naming both owners: an assembly that fails is a
    /// programming error in the def set, never a runtime condition.
    pub fn assemble(extensions: &[DefSource]) -> Definitions {
        match Self::try_assemble(extensions) {
            Ok(definitions) => definitions,
            Err(err) => panic!("scalar definitions assembly failed: {err}"),
        }
    }

    /// Non-panicking `assemble`. Runs the def-level checks in the documented
    /// order and stops at the first failure.
    pub fn try_assemble(extensions: &[DefSource]) -> Result<Definitions, AssemblyError> {
        Self::try_assemble_sources(
            std::iter::once((BUILTIN_OWNER, &CATALOG[..])).chain(extensions.iter().copied()),
        )
    }

    /// Assembly over every source, built-ins included. `Registry` calls this
    /// with its own built-in entry first.
    pub(crate) fn try_assemble_sources(
        sources: impl Iterator<Item = DefSource> + Clone,
    ) -> Result<Definitions, AssemblyError> {
        let by_canonical = check_unique_identities(sources.clone())?;
        let by_id: BTreeMap<u32, &'static ScalarDef> = sources
            .flat_map(|(_, defs)| defs.iter())
            .map(|def| (def.id.0, def))
            .collect();
        check_def_consistency(&by_id)?;
        Ok(Definitions {
            by_id,
            by_canonical,
        })
    }

    /// Every def keyed by id, for the `Registry` checks that run after these.
    pub(crate) fn by_id(&self) -> &BTreeMap<u32, &'static ScalarDef> {
        &self.by_id
    }

    /// The def for `id`, or `None` for an id no source declared.
    pub fn def(&self, id: ScalarId) -> Option<&'static ScalarDef> {
        self.by_id.get(&id.0).copied()
    }

    /// Look up a def by its canonical identity string, e.g. `"Contact.Email"`.
    /// Exact and case-sensitive.
    pub fn by_canonical(&self, name: &str) -> Option<&'static ScalarDef> {
        self.by_canonical.get(name).and_then(|id| self.def(*id))
    }

    /// Resolve an alias to the id that carries the implementation. An unknown
    /// id resolves to itself.
    pub fn resolved(&self, id: ScalarId) -> ScalarId {
        self.def(id).and_then(|def| def.alias_of).unwrap_or(id)
    }

    /// Whether a comparison or join between a column of scalar `a` and one of
    /// `b` is semantically meaningful. THIS is the comparability contract;
    /// `ScalarDef::comparability_class` is the data it reads, not the relation
    /// itself.
    ///
    /// The distinction matters because the obvious implementation is wrong.
    /// `a.comparability_class == b.comparability_class` answers `true` for two
    /// class-less scalars, and `None` is the value almost every scalar carries,
    /// so naive field equality makes almost the entire catalog mutually comparable --
    /// including `Contact.Email` against `Contact.PhoneNumber`, which
    /// the semantic-types design names as the case the relation exists to
    /// reject. Go has the sharper version of the same trap: it encodes absent
    /// as `""`, which is also the zero value a failed map lookup returns.
    ///
    /// The rule: a scalar is always comparable with itself, and two DISTINCT
    /// scalars are comparable only when both declare the SAME named class.
    /// `None` on either side is self-comparable-only and never matches across.
    /// An alias resolves first, since `alias_of` means one implementation under
    /// two ids (`Identity.UserID` and `Identity.UUID` are the only such pair),
    /// so they are the same scalar for this purpose. The class is read through
    /// the RESOLVED def so the alias inherits the target's class; reading the
    /// raw field on each side would break transitivity one hop out.
    ///
    /// Equivalence classes, not a subtype lattice. Reflexive, symmetric and
    /// transitive, asserted over the whole catalog by
    /// `comparability_is_an_equivalence_relation_over_the_catalog`.
    ///
    /// This answers ONE binary question: may these two be compared. It is not
    /// the validator's severity rule, which is three-valued --
    /// a query validator distinguishes different-class (error),
    /// typed-vs-unknown (warn) and unknown-vs-unknown (no diagnostic), and this
    /// predicate collapses the last two into `false`. The validator must read
    /// the classes itself to tell those apart; what it must NOT do is treat raw
    /// field equality as the comparability answer, which is the trap this
    /// exists for.
    ///
    /// The rule itself is `comparable_in`, which resolves the alias before it
    /// reads either class. This method only supplies these definitions as that
    /// function's lookup. The indirection is load-bearing rather than stylistic:
    /// reading the class off the raw row instead of off the alias TARGET breaks
    /// transitivity one hop out (`UserID ~ UUID` true by the identity
    /// short-circuit, `UUID ~ Other` true by the shared class, `UserID ~ Other`
    /// false), and no walk over the built-in catalog can see that, because the
    /// one alias pair carries no class on either side. Putting the resolution
    /// inside the rule is what lets a synthetic table catch it.
    pub fn comparable_with(&self, a: ScalarId, b: ScalarId) -> bool {
        comparable_in(a, b, |id| {
            self.def(id)
                .map_or((None, None), |def| (def.alias_of, def.comparability_class))
        })
    }

    /// Every assembled id, ascending.
    pub fn ids(&self) -> impl Iterator<Item = ScalarId> + '_ {
        self.by_id.keys().map(|id| ScalarId(*id))
    }

    /// Every assembled def, in ascending id order.
    pub fn defs(&self) -> impl Iterator<Item = &'static ScalarDef> + '_ {
        self.by_id.values().copied()
    }

    /// Number of assembled scalars.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Whether no scalar is assembled. Never true for an assembly that
    /// includes the built-ins; present for `clippy::len_without_is_empty`.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

/// Assembly checks 4 and 5: no two defs share an id (`DuplicateId`); no two
/// defs share a canonical name, compared exact and case-sensitive
/// (`DuplicateCanonical`). Returns the canonical-to-id index, which is well
/// defined only once check 5 has passed.
fn check_unique_identities(
    sources: impl Iterator<Item = DefSource> + Clone,
) -> Result<HashMap<&'static str, ScalarId>, AssemblyError> {
    let mut owner_by_id: BTreeMap<u32, &'static str> = BTreeMap::new();
    for (owner, defs) in sources.clone() {
        for def in defs {
            if let Some(first) = owner_by_id.insert(def.id.0, owner) {
                return Err(AssemblyError::DuplicateId {
                    id: def.id,
                    first,
                    second: owner,
                });
            }
        }
    }
    let mut by_canonical: HashMap<&'static str, ScalarId> = HashMap::new();
    let mut owner_by_canonical: HashMap<&'static str, &'static str> = HashMap::new();
    for (owner, defs) in sources {
        for def in defs {
            if let Some(first) = owner_by_canonical.insert(def.canonical, owner) {
                return Err(AssemblyError::DuplicateCanonical {
                    canonical: def.canonical,
                    first,
                    second: owner,
                });
            }
            by_canonical.insert(def.canonical, def.id);
        }
    }
    Ok(by_canonical)
}

/// Assembly checks 6, 7 and 8, over every def in id order: `namespace` is the
/// canonical prefix before the first `.` (`NamespaceMismatch`); every
/// `alias_of` target exists (`DanglingAlias`) and is not itself an alias
/// (`AliasChain`).
fn check_def_consistency(by_id: &BTreeMap<u32, &'static ScalarDef>) -> Result<(), AssemblyError> {
    for def in by_id.values() {
        let prefix = def.canonical.split('.').next().unwrap_or_default();
        if def.namespace != prefix {
            return Err(AssemblyError::NamespaceMismatch {
                canonical: def.canonical,
                namespace: def.namespace,
            });
        }
    }
    for def in by_id.values() {
        let Some(target) = def.alias_of else {
            continue;
        };
        let Some(target_def) = by_id.get(&target.0) else {
            return Err(AssemblyError::DanglingAlias {
                canonical: def.canonical,
                alias_of: target,
            });
        };
        if target_def.alias_of.is_some() {
            return Err(AssemblyError::AliasChain {
                canonical: def.canonical,
                alias_of: target,
            });
        }
    }
    Ok(())
}

/// The comparability relation over an alias-and-class TABLE, extracted from
/// [`Definitions::comparable_with`] so both of its arms can be table-tested.
///
/// `lookup` answers, for one key, that row's two RAW fields as a pair:
/// `(alias_of, comparability_class)`. Resolving the alias is this function's job
/// and deliberately not the caller's. That is the entire reason the extraction
/// exists: the defect this guards against is reading the class off the raw row
/// instead of off the alias TARGET, so if the caller did the resolving, the
/// mutation would live outside the unit under test and a table test would prove
/// nothing.
///
/// Undischargeable against the built-in catalog, which is why it takes a lookup
/// rather than reading a `Definitions` directly. Two separate holes, one cause -- a
/// guard whose input the catalog cannot produce:
///
/// * The ALIAS arm. `Identity.UserID` -> `Identity.UUID` is the only alias pair
///   and neither row carries a class, so the arm compares `None` against `None`
///   and answers the same either way.
/// * The TRANSITIVITY arm. With exactly one two-member class there is no
///   pairwise-distinct triple `a ~ b`, `b ~ c`, so the transitivity loop in
///   `comparability_is_an_equivalence_relation_over_the_catalog` can only reach
///   the reflexive and symmetric cases it already covers.
///
/// A mutation probe measured both: swapping the resolved read for a raw-field
/// read produced ZERO test failures across the workspace. Synthetic tables in
/// `comparability_rule_tests` close them. Same remedy, and the same reason for
/// it, as `json_shape_is_sortable` in `registry.rs` and `check_class_invariants` in
/// `crates/core/tests/semantic_metadata.rs`.
///
/// Alias resolution is SINGLE-HOP, matching [`Definitions::resolved`] exactly
/// rather than improving on it. A chain-following version here would be a
/// second, more permissive rule than the one production runs, and the point of
/// the extraction is that the tested rule and the shipped rule are one function.
fn comparable_in<K, F>(left: K, right: K, lookup: F) -> bool
where
    K: Copy + PartialEq,
    F: Fn(K) -> (Option<K>, Option<&'static str>),
{
    let left = lookup(left).0.unwrap_or(left);
    let right = lookup(right).0.unwrap_or(right);
    if left == right {
        return true;
    }
    match (lookup(left).1, lookup(right).1) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    }
}

/// The structural guard behind the module doc: the assembly and lookup code
/// above names nothing that would link a scalar implementation. A textual
/// check, because the property is about what the code REFERENCES and a
/// linker-level check needs a size-capped wasm build this crate does not own.
/// Comments are skipped (they explain the rule and must be free to name it),
/// and so is everything from the first  on.
#[cfg(test)]
mod implementation_free_guard {
    #[test]
    fn definitions_module_stays_free_of_implementations() {
        let source = include_str!("definitions.rs");
        let code_end = source
            .find("#[cfg(test)]")
            .expect("test modules follow the code");
        let forbidden = [
            "Scalar ",
            "Scalar>",
            "Scalar,",
            "Scalar;",
            "Scalar)",
            "dyn ",
            "Extension",
            "Registry",
            "DirectiveScalar",
            "regex",
            "Regex",
            "impls",
            "builtin::",
            "scalar_for",
            "crate::scalars",
            "crate::metadata",
            "crate::directive",
            "crate::coerce",
        ];
        for (number, line) in source[..code_end].lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            for token in forbidden {
                assert!(
                    !code.contains(token),
                    "definitions.rs line {}: {token:?} would reach a scalar                      implementation; this module must stay data-only so a                      definitions-only consumer links no impl",
                    number + 1
                );
            }
        }
    }
}

/// The comparability relation against tables the catalog cannot produce.
///
/// Every test here is a table test on purpose. The catalog-walking versions in
/// `crates/core/tests/semantic_metadata.rs` stay as the standing checks over
/// shipped data, but neither of the two arms below is DISCHARGEABLE by them: the
/// one alias pair carries no class, and one two-member class admits no
/// pairwise-distinct triple. These tables supply both, so neither guard depends
/// on any particular scalar being classed and both keep working when the class
/// table changes.
#[cfg(test)]
mod comparability_rule_tests {
    use super::comparable_in;

    /// `(name, alias_of, class)`. The shape [`comparable_in`] reads, as raw rows.
    type Row = (&'static str, Option<&'static str>, Option<&'static str>);

    fn lookup(
        rows: &[Row],
    ) -> impl Fn(&'static str) -> (Option<&'static str>, Option<&'static str>) + '_ {
        move |name| {
            rows.iter()
                .find(|(row, _, _)| *row == name)
                .map(|(_, alias_of, class)| (*alias_of, *class))
                .unwrap_or((None, None))
        }
    }

    /// The alias arm, which the catalog cannot exercise.
    ///
    /// `Alias` declares NO class of its own and points at `Target`, which is
    /// classed alongside `Other`. The relation must read `Alias`'s class through
    /// `Target`. Reading the raw field instead answers `None` for `Alias`, so
    /// `Alias ~ Other` comes back false while `Alias ~ Target` stays true on the
    /// identity short-circuit -- the inconsistency is invisible at the row where
    /// it was introduced and only surfaces one hop out, which is exactly why the
    /// shipped catalog cannot catch it.
    #[test]
    fn an_alias_inherits_its_targets_class_one_hop_out() {
        let rows: &[Row] = &[
            ("Identity.UserID", Some("Identity.UUID"), None),
            ("Identity.UUID", None, Some("opaque_id")),
            ("Other.Id", None, Some("opaque_id")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(
            comparable("Identity.UserID", "Other.Id"),
            "Identity.UserID is an alias of Identity.UUID, which shares the \
             opaque_id class with Other.Id; reading the class off the raw alias \
             row instead of off the alias target breaks this pair and leaves \
             Identity.UserID ~ Identity.UUID true, so the break only shows one \
             hop out"
        );
        assert!(comparable("Identity.UserID", "Identity.UUID"), "alias pair");
        assert!(
            comparable("Other.Id", "Identity.UserID"),
            "and symmetrically"
        );
    }

    /// The transitivity arm, which the catalog cannot exercise either: it needs a
    /// pairwise-distinct triple, and one two-member class has none.
    #[test]
    fn transitivity_holds_over_a_three_member_class() {
        let rows: &[Row] = &[
            ("A.One", None, Some("shared")),
            ("A.Two", None, Some("shared")),
            ("A.Three", None, Some("shared")),
            ("B.One", None, Some("other")),
            ("B.Two", None, Some("other")),
            ("C.Unclassed", None, None),
        ];
        let names = ["A.One", "A.Two", "A.Three", "B.One", "B.Two", "C.Unclassed"];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        let mut distinct_triples = 0;
        for &a in &names {
            assert!(comparable(a, a), "{a} reflexive");
            for &b in &names {
                assert_eq!(comparable(a, b), comparable(b, a), "{a} / {b} symmetric");
                if !comparable(a, b) {
                    continue;
                }
                for &c in &names {
                    if !comparable(b, c) {
                        continue;
                    }
                    if a != b && b != c && a != c {
                        distinct_triples += 1;
                    }
                    assert!(
                        comparable(a, c),
                        "transitivity: {a} ~ {b} and {b} ~ {c} but not {a} ~ {c}"
                    );
                }
            }
        }
        assert!(
            distinct_triples > 0,
            "the transitivity loop must reach a pairwise-distinct triple, which \
             is precisely what the shipped catalog cannot supply"
        );
    }

    /// Two classed scalars in DIFFERENT classes never compare, and an unclassed
    /// scalar is self-comparable only. The second half is the trap the doc
    /// comment on `comparable_with` names: raw field equality answers true for
    /// two class-less scalars, and `None` is what almost every catalog row
    /// carries.
    #[test]
    fn distinct_classes_and_absent_classes_never_match_across() {
        let rows: &[Row] = &[
            ("A.One", None, Some("shared")),
            ("B.One", None, Some("other")),
            ("C.Unclassed", None, None),
            ("D.Unclassed", None, None),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(!comparable("A.One", "B.One"), "different classes");
        assert!(
            !comparable("C.Unclassed", "D.Unclassed"),
            "two class-less scalars are NOT comparable; field equality would say \
             they are, and that is the majority of the catalog"
        );
        assert!(
            !comparable("A.One", "C.Unclassed"),
            "classed against absent"
        );
        assert!(comparable("C.Unclassed", "C.Unclassed"), "self");
    }

    /// Resolution is SINGLE-HOP, and this is the table that says so.
    ///
    /// [`comparable_in`]'s doc comment claims it matches `Definitions::resolved`
    /// exactly rather than improving on it, and that claim was undischargeable
    /// until this table: the catalog's one alias pair is a single hop, so
    /// replacing `unwrap_or` with a fixpoint loop passes every other test here.
    ///
    /// The chain is `A -> B -> C` with the class on `C`, and single-hop answers
    /// FALSE for every pair that crosses it -- including `A ~ B`, the adjacent
    /// alias pair. `A` resolves one hop to `B`; `B` resolves one hop to `C`, so
    /// the two sides are not equal and the identity short-circuit does not fire;
    /// `B` carries no class of its own because the alias arm of
    /// `check_class_invariants` forbids it. So a chain does not merely fail to
    /// propagate a class, it breaks the alias relation at its own first link.
    ///
    /// Assembly refuses a chain today (`AssemblyError::AliasChain`), so no
    /// assembled registry reaches this shape. The test pins the rule rather than
    /// the catalog: it is what fails if the assembly check is ever relaxed while
    /// a chain-following `comparable_in` and a single-hop `resolved()` would
    /// silently disagree about the same registry.
    #[test]
    fn resolution_is_single_hop_and_a_chain_breaks_at_its_first_link() {
        let rows: &[Row] = &[
            ("A.Head", Some("B.Middle"), None),
            ("B.Middle", Some("C.Tail"), None),
            ("C.Tail", None, Some("shared")),
            ("D.Other", None, Some("shared")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(
            !comparable("A.Head", "B.Middle"),
            "SINGLE-HOP: A.Head resolves to B.Middle and B.Middle resolves to \
             C.Tail, so the identity short-circuit does not fire and B.Middle \
             carries no class. A chain breaks even its own adjacent pair, which \
             is why the catalog must not grow one without deciding first"
        );
        assert!(
            !comparable("A.Head", "C.Tail"),
            "and it does not reach the class at the end of the chain; a fixpoint \
             resolver would answer true here and diverge from Definitions::resolved"
        );
        assert!(!comparable("A.Head", "D.Other"));
        assert!(
            comparable("C.Tail", "D.Other"),
            "the classed pair past the chain is unaffected"
        );

        // The contrast: the same shape WITHOUT a chain resolves cleanly, so the
        // assertions above are about the chain and not about aliases generally.
        let flat: &[Row] = &[
            ("A.Head", Some("C.Tail"), None),
            ("C.Tail", None, Some("shared")),
            ("D.Other", None, Some("shared")),
        ];
        assert!(comparable_in("A.Head", "C.Tail", lookup(flat)));
        assert!(comparable_in("A.Head", "D.Other", lookup(flat)));
    }

    /// An alias whose target is UNCLASSED stays self-comparable-only. Guards the
    /// over-correction: resolving the alias must not invent a class.
    #[test]
    fn an_alias_of_an_unclassed_target_matches_nothing_else() {
        let rows: &[Row] = &[
            ("Identity.UserID", Some("Identity.UUID"), None),
            ("Identity.UUID", None, None),
            ("Other.Id", None, Some("opaque_id")),
        ];
        let comparable = |a, b| comparable_in(a, b, lookup(rows));

        assert!(comparable("Identity.UserID", "Identity.UUID"), "alias pair");
        assert!(!comparable("Identity.UserID", "Other.Id"));
    }
}
