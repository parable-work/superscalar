//! `Scalar::magnitude` and `Scalar::parts`: the refusing defaults every
//! built-in keeps, and an extension scalar that implements both. The trait
//! methods and the name-keyed `Registry::magnitude` and `Registry::parts`
//! give the same answers.

mod common;

use common::{alias_def, def, TestExtension};
use superscalar::{ErrorKind, Registry, Scalar, ScalarDef, ScalarError, ScalarTag};

const MASS: &str = "Acme.Mass";
const WEIGHT: &str = "Acme.Weight";

static DEFS: [ScalarDef; 2] = [
    def("Acme", MASS, ScalarTag::CustomLogic, None),
    alias_def("Acme", WEIGHT, MASS),
];

/// A mass written as ASCII digits and a unit, `g` or `kg`: `750g`, `2kg`.
/// Its magnitude is the mass in grams. Its parts are the amount as written
/// and the unit.
struct Mass;

impl Mass {
    fn read(input: &str) -> Result<(u128, &'static str), ScalarError> {
        let refuse = || ScalarError::new(ErrorKind::Pattern, "a mass is digits then g or kg");
        let (digits, unit) = match input.strip_suffix("kg") {
            Some(digits) => (digits, "kg"),
            None => (input.strip_suffix('g').ok_or_else(refuse)?, "g"),
        };
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(refuse());
        }
        let amount = digits.parse::<u128>().map_err(|_| refuse())?;
        Ok((amount, unit))
    }
}

impl Scalar for Mass {
    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let (amount, unit) = Self::read(input)?;
        Ok(format!("{amount}{unit}"))
    }

    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        Self::read(input).map(|_| ())
    }

    fn magnitude(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let (amount, unit) = Self::read(input)?;
        let grams = if unit == "kg" { amount * 1000 } else { amount };
        Ok(grams.to_string())
    }

    fn parts(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        let (amount, unit) = Self::read(input)?;
        Ok(serde_json::json!({ "amount": amount.to_string(), "unit": unit }).to_string())
    }
}

fn impls() -> Vec<(&'static str, Box<dyn Scalar>)> {
    vec![(MASS, Box::new(Mass))]
}

fn assembled() -> Registry {
    Registry::assemble(&[&TestExtension::new("acme", &DEFS).with_impls(impls)])
}

/// No built-in has a magnitude or parts. Each one keeps the trait defaults,
/// which refuse a value the scalar accepts, through the trait and through the
/// registry alike.
#[test]
fn every_builtin_refuses_magnitude_and_parts() {
    let registry = Registry::builtin();
    let mut checked = 0;
    for def in registry.defs() {
        let name = def.canonical;
        let scalar = registry.scalar(name).expect("assembled");
        // An example is a value the scalar accepts; a scalar without one is
        // asked about a value it may refuse, and the default refuses first.
        let input = def.examples.first().copied().unwrap_or("value");
        for (what, by_trait, by_name) in [
            (
                "magnitude",
                scalar.magnitude(registry, input),
                registry.magnitude(name, input),
            ),
            (
                "parts",
                scalar.parts(registry, input),
                registry.parts(name, input),
            ),
        ] {
            let error = by_trait.expect_err(name);
            assert_eq!(error.kind, ErrorKind::Custom, "{name} {what}");
            assert_eq!(error.message, format!("the scalar has no {what}"), "{name}");
            assert_eq!(by_name, Err(error), "{name} {what} by name");
        }
        checked += 1;
    }
    assert_eq!(checked, registry.len());
}

#[test]
fn an_extension_scalar_answers_its_magnitude() {
    let registry = assembled();
    let mass = registry.scalar(MASS).expect("assembled");
    for (input, grams) in [
        ("750g", "750"),
        ("2kg", "2000"),
        ("0g", "0"),
        // Beyond 64 bits: the answer is text, so no digit is lost.
        ("36893488147419103232kg", "36893488147419103232000"),
    ] {
        assert_eq!(mass.magnitude(&registry, input).as_deref(), Ok(grams));
        assert_eq!(registry.magnitude(MASS, input).as_deref(), Ok(grams));
    }
}

#[test]
fn an_extension_scalar_answers_its_parts() {
    let registry = assembled();
    let mass = registry.scalar(MASS).expect("assembled");
    let by_trait = mass.parts(&registry, "2kg").expect("a valid mass");
    assert_eq!(
        registry.parts(MASS, "2kg").as_deref(),
        Ok(by_trait.as_str())
    );
    let parts: serde_json::Value = serde_json::from_str(&by_trait).expect("a JSON object");
    assert_eq!(parts, serde_json::json!({ "amount": "2", "unit": "kg" }));
}

/// A value the scalar refuses fails with the scalar's own error, not with the
/// default refusal.
#[test]
fn a_refused_value_fails_with_the_scalars_error() {
    let registry = assembled();
    for input in ["", "kg", "2", "2 kg", "two kg", "-2kg"] {
        for result in [registry.magnitude(MASS, input), registry.parts(MASS, input)] {
            let error = result.expect_err(input);
            assert_eq!(error.kind, ErrorKind::Pattern, "{input:?}");
        }
    }
}

/// An alias shares its target's implementation, so it has the same magnitude
/// and parts.
#[test]
fn an_alias_answers_as_its_target() {
    let registry = assembled();
    assert_eq!(registry.magnitude(WEIGHT, "2kg").as_deref(), Ok("2000"));
    assert_eq!(registry.parts(WEIGHT, "2kg"), registry.parts(MASS, "2kg"));
}

#[test]
fn an_unknown_name_fails_with_enum() {
    let registry = assembled();
    for result in [
        registry.magnitude("Acme.Missing", "2kg"),
        registry.parts("Acme.Missing", "2kg"),
        Registry::builtin().magnitude(MASS, "2kg"),
    ] {
        let error = result.expect_err("unknown name");
        assert_eq!(error.kind, ErrorKind::Enum);
        assert!(
            error.message.contains("unknown scalar"),
            "{}",
            error.message
        );
    }
}
