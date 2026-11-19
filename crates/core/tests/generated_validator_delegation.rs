//! A schema runtime generated from the defs calls the core validator only for
//! a def that sets `hooks.validate`. For any other def it checks the
//! directives alone: length, pattern and reserved words. A custom scalar that
//! refuses a value its directives accept must therefore set the hook.
//! Otherwise a generated validator in one language accepts a value that the
//! core, and so every binding that calls it, refuses.
//!
//! Every custom built-in sets the hook or is in `EXEMPT` with its reason. A
//! new custom scalar fails here until its author picks one.

use serde::Deserialize;
use superscalar::directive::DirectiveScalar;
use superscalar::{names, Registry, Scalar, ScalarDef};

const VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../conformance/core-scalars.v2.json"
));

enum Why {
    /// The custom validator refuses nothing its directives accept, so the
    /// directive checks are enough.
    /// `exempt_scalars_refuse_nothing_their_directives_accept` checks this
    /// claim against every input in the conformance vectors.
    DirectivesAgree,
    /// The custom validator refuses more, but a generated validator must not
    /// delegate to it, for the stated reason.
    NotDelegated(&'static str),
}

const EXEMPT: &[(&str, Why)] = &[
    (
        names::EMBEDDING_VECTOR,
        Why::NotDelegated(
            "a generated validator receives a decoded number array, not the text the core validates",
        ),
    ),
    (names::GENERIC_DECIMAL, Why::DirectivesAgree),
    (
        names::GENERIC_STRING_MAP,
        Why::NotDelegated(
            "JSON-parsed: the def sets the parse hook, and the core parse refuses bad input",
        ),
    ),
    (
        names::GEO_LOCATION,
        Why::NotDelegated(
            "JSON-parsed: the def sets the parse hook, and the core parse refuses bad input",
        ),
    ),
    (
        names::IDENTITY_UUID,
        Why::NotDelegated(
            "the def sets the parse hook: the pattern accepts the short encodings that the \
             core parse canonicalizes, and the core validate refuses them",
        ),
    ),
    (names::NETWORK_DNS_LABEL, Why::DirectivesAgree),
    (names::NETWORK_URL, Why::DirectivesAgree),
    (
        names::TEMPORAL_DATE_TIME,
        Why::NotDelegated(
            "a generated validator receives a decoded date-time value; \
             the core parse already refused bad text",
        ),
    ),
    (names::TEMPORAL_TIME, Why::DirectivesAgree),
];

fn exemption(canonical: &str) -> Option<&'static Why> {
    EXEMPT
        .iter()
        .find(|(exempt, _)| *exempt == canonical)
        .map(|(_, why)| why)
}

/// An alias shares its target's impl, and the alias test below holds its
/// hook to the target's, so only implementing defs need a decision.
fn implementing_defs() -> impl Iterator<Item = &'static ScalarDef> {
    Registry::builtin()
        .defs()
        .filter(|def| def.alias_of.is_none())
}

fn scalar(canonical: &str) -> &'static dyn Scalar {
    Registry::builtin().scalar(canonical).expect("a built-in")
}

#[test]
fn every_custom_scalar_is_delegated_or_exempt() {
    let mut failures = Vec::new();
    for def in implementing_defs() {
        let name = def.canonical;
        let custom = !scalar(name).is_directive();
        match (custom, def.hooks.validate, exemption(name)) {
            (true, false, Some(Why::NotDelegated(reason))) if reason.trim().is_empty() => {
                failures.push(format!("{name}: EXEMPT gives no reason"))
            }
            (true, true, None) | (true, false, Some(_)) | (false, _, None) => {}
            (true, false, None) => failures.push(format!(
                "{name}: custom impl, but hooks.validate is not set and it is not in EXEMPT"
            )),
            (true, true, Some(_)) => failures.push(format!(
                "{name}: sets hooks.validate and is in EXEMPT; remove one"
            )),
            (false, _, Some(_)) => failures.push(format!(
                "{name}: served by the directive engine; remove it from EXEMPT"
            )),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    for (name, _) in EXEMPT {
        assert!(
            Registry::builtin().def(name).is_some(),
            "{name} is in EXEMPT but is not a built-in"
        );
    }
}

#[derive(Deserialize)]
struct Vectors {
    scalars: std::collections::BTreeMap<String, Cases>,
}

#[derive(Deserialize)]
struct Cases {
    #[serde(default)]
    accepted: Vec<Case>,
    #[serde(default)]
    rejected: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    input: String,
}

/// Every input in the conformance vectors, whichever scalar it was written
/// for. One scalar's accepted and rejected values probe another's edges well.
fn corpus() -> Vec<String> {
    let vectors: Vectors = serde_json::from_str(VECTORS).expect("v2 vectors parse");
    let mut inputs: Vec<String> = vectors
        .scalars
        .into_values()
        .flat_map(|cases| cases.accepted.into_iter().chain(cases.rejected))
        .map(|case| case.input)
        .collect();
    inputs.sort();
    inputs.dedup();
    inputs
}

#[test]
fn exempt_scalars_refuse_nothing_their_directives_accept() {
    let registry = Registry::builtin();
    let inputs = corpus();
    let mut failures = Vec::new();
    let mut probed = 0usize;
    for (name, why) in EXEMPT {
        if !matches!(why, Why::DirectivesAgree) {
            continue;
        }
        let def = registry.def(name).expect("a built-in");
        let directives = DirectiveScalar::from_def(def);
        let custom = scalar(name);
        for input in &inputs {
            if directives.validate(registry, input).is_ok() {
                probed += 1;
                if custom.validate(registry, input).is_err() {
                    failures.push(format!(
                        "{name}: the directives accept {input:?} but the core refuses it; \
                         set hooks.validate on the def"
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(probed > 0, "no input passed any exempt scalar's directives");
}

#[test]
fn aliases_delegate_as_their_target() {
    let registry = Registry::builtin();
    let mut aliases = 0usize;
    for def in registry.defs() {
        if let Some(target) = def.alias_of {
            let target = registry.def(target).expect("the alias target is assembled");
            assert_eq!(
                def.hooks.validate, target.hooks.validate,
                "{}: alias of {} but hooks.validate differs",
                def.canonical, target.canonical,
            );
            aliases += 1;
        }
    }
    assert!(aliases > 0, "the built-ins have an alias");
}
