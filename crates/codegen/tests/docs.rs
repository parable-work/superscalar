//! The docs generator writes one page per scalar plus an index, and its check
//! refuses a scalar with no vectors or no description.

mod common;

use std::fs;
use std::path::PathBuf;
use superscalar::{
    Extension, LegacyAlias, PrimitiveKind, Registry, Scalar, ScalarDef, ScalarHooks, ScalarId,
    ScalarTag,
};
use superscalar_codegen::docs::{self, page_slug, Vectors};

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("scalar-lib-docs-{tag}-{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear temp dir");
    }
    dir
}

#[test]
fn full_registry_writes_one_page_per_scalar_plus_index() {
    let config = common::config();
    let registry = Registry::builtin();
    let vectors = docs::load_vectors(&config).expect("vectors");
    let out = temp_dir("full");

    let report = docs::write(registry, &config, &vectors, &out).expect("write docs");

    assert_eq!(report.pages, 44, "one page per registered scalar");
    assert_eq!(report.pages, registry.len());
    let mut files: Vec<String> = fs::read_dir(&out)
        .expect("read out dir")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    files.sort();
    assert_eq!(files.len(), 45, "pages plus index.md");
    assert!(files.contains(&"index.md".to_string()));
    assert!(files.contains(&"contact-email.md".to_string()));
    assert!(files.contains(&"crypto-rsa-private-key.md".to_string()));

    let email = fs::read_to_string(out.join("contact-email.md")).expect("page");
    assert!(email.starts_with(
        "---\ntitle: Contact.Email\ndescription: \"An email address\"\nsidebar:\n  order: 3\n---\n"
    ));
    assert!(email.contains("| Canonical name | `Contact.Email` |"));
    assert!(email.contains("| Id | 8 |"));
    assert!(email.contains("| Namespace | Contact |"));
    assert!(email.contains("| Primitive | String |"));
    assert!(email.contains("| SQL type | `CITEXT` |"));
    assert!(email.contains("| JSON Schema type | `string` |"));
    assert!(email.contains("| Max length | 255 |"));
    assert!(email.contains("| Pattern | `"));
    assert!(
        email.contains("- `test@example.com`"),
        "examples from the def"
    );
    assert!(
        email.contains("| `TEST@EXAMPLE.COM` | `test@example.com` |"),
        "accepted vectors from the corpus"
    );
    assert!(
        email.contains("| `not-an-email` | pattern |"),
        "rejected vectors from the corpus"
    );
    assert!(email.contains("`ParseContactEmail`, `NormalizeContactEmail`, `ValidateContactEmail`"));
    assert!(email.contains("`parseContactEmail`, `normalizeContactEmail`, `validateContactEmail`"));
    assert!(email
        .contains("`parse_contact_email`, `normalize_contact_email`, `validate_contact_email`"));
    assert!(email.contains("`Registry::scalar(ScalarId(8))`"));

    // Object scalars have no Go functions, and the Go UUID override reaches
    // the per-language table.
    let location = fs::read_to_string(out.join("geo-location.md")).expect("page");
    assert!(location.contains("| Go | `struct{ Lat float64; Lon float64 }` | none (object-valued"));
    let uuid = fs::read_to_string(out.join("identity-uuid.md")).expect("page");
    assert!(uuid.contains("| Go | `UUID` |"));
    let datetime = fs::read_to_string(out.join("temporal-date-time.md")).expect("page");
    assert!(uuid.contains("| TypeScript | `string` branded as `IdentityUUID` |"));
    assert!(
        datetime.contains("| TypeScript | `JSDate` |"),
        "TS type renames apply"
    );

    // Prose from the registry renders literally: emphasis and raw-HTML
    // characters are escaped outside code spans, code spans keep their text.
    let size = fs::read_to_string(out.join("file-size-bytes.md")).expect("page");
    assert!(size.contains("Number.MAX\\_SAFE\\_INTEGER"));
    let slug = fs::read_to_string(out.join("identity-slug.md")).expect("page");
    assert!(slug.contains("| Pattern | `^[a-z0-9]+(?:[-_][a-z0-9]+)*$` |"));

    // Every page is frontmatter first, and the index links each one with the
    // configured prefix.
    let index = fs::read_to_string(out.join("index.md")).expect("index");
    assert!(index.starts_with("---\ntitle: Scalar reference\n"));
    for def in registry.defs() {
        let slug = page_slug(def.canonical);
        let page = fs::read_to_string(out.join(format!("{slug}.md"))).expect("page");
        assert!(
            page.starts_with("---\ntitle: "),
            "{slug} starts with frontmatter"
        );
        assert!(
            page.contains("\nsidebar:\n  order: "),
            "{slug} has a sidebar order"
        );
        assert!(
            index.contains(&format!(
                "[{}](/superscalar/reference/{slug}/)",
                def.canonical
            )),
            "index links {slug}"
        );
    }
    assert_eq!(index.matches("](/superscalar/reference/").count(), 44);

    // The corpus documents every built-in scalar today, so the check is clean.
    assert_eq!(docs::check(registry, &vectors), Vec::<String>::new());

    fs::remove_dir_all(&out).expect("remove temp dir");
}

const fn acme_def(id: u32, canonical: &'static str, description: &'static str) -> ScalarDef {
    ScalarDef {
        id: ScalarId(id),
        namespace: "Acme",
        canonical,
        primitive: PrimitiveKind::String,
        sql_type: "TEXT",
        metadata_primitive: "String",
        json_schema_type: "string",
        tag: ScalarTag::PatternOnly,
        pattern: Some("^[a-z]+$"),
        min_length: None,
        max_length: None,
        minimum: None,
        maximum: None,
        case_insensitive: false,
        reserved_words: &[],
        examples: &["widget"],
        description,
        type_mappings: &[
            ("typescript", "string"),
            ("python", "str"),
            ("go", "string"),
        ],
        file_upload: None,
        image_constraints: None,
        docstring: "",
        alias_of: None,
        schema_primitive_override: None,
        schema_omit: false,
        format: None,
        reserved_words_case_insensitive: false,
        reserved_words_match_partial: false,
        comparability_class: None,
        hooks: ScalarHooks::NONE,
        metadata_omit: false,
    }
}

static ACME_DEFS: [ScalarDef; 2] = [
    acme_def(4096, "Acme.Widget", "A widget name"),
    acme_def(4097, "Acme.Undescribed", ""),
];

struct Acme;

impl Extension for Acme {
    fn name(&self) -> &'static str {
        "acme"
    }
    fn id_base(&self) -> u32 {
        4096
    }
    fn defs(&self) -> &'static [ScalarDef] {
        &ACME_DEFS
    }
    fn impls(&self) -> Vec<(ScalarId, Box<dyn Scalar>)> {
        Vec::new()
    }
    fn aliases(&self) -> &'static [LegacyAlias] {
        &[]
    }
}

#[test]
fn check_fails_a_scalar_with_no_vectors_or_no_description() {
    let config = common::config();
    let registry = Registry::assemble(&[&Acme]);
    assert_eq!(registry.len(), 46);
    let vectors = docs::load_vectors(&config).expect("vectors");

    let problems = docs::check(&registry, &vectors);
    assert_eq!(
        problems,
        vec![
            "Acme.Widget: no conformance vectors".to_string(),
            "Acme.Undescribed: no description".to_string(),
            "Acme.Undescribed: no conformance vectors".to_string(),
        ]
    );

    // A vector file naming an unknown scalar is also a problem, and an
    // extension scalar with vectors passes.
    let dir = temp_dir("acme-vectors");
    fs::create_dir_all(&dir).expect("create temp dir");
    let corpus = dir.join("acme.v2.json");
    fs::write(
        &corpus,
        r#"{"scalars": {"Acme.Widget": {"accepted": [{"input": "widget", "normalized": "widget"}], "rejected": [{"input": "1", "validator": "pattern"}]}, "Acme.Typo": {"accepted": [], "rejected": []}}}"#,
    )
    .expect("write corpus");
    let mut paths: Vec<PathBuf> = config
        .docs
        .as_ref()
        .expect("[docs]")
        .vectors
        .iter()
        .map(|p| config.resolve(p))
        .collect();
    paths.push(corpus);
    let merged = Vectors::load(&paths).expect("merge");
    let problems = docs::check(&registry, &merged);
    assert_eq!(
        problems,
        vec![
            "Acme.Undescribed: no description".to_string(),
            "Acme.Undescribed: no conformance vectors".to_string(),
            "Acme.Typo: vectors name a scalar the registry does not hold".to_string(),
        ]
    );

    // Pages are still written for the assembled registry, with the owner named.
    let out = temp_dir("acme-pages");
    let report = docs::write(&registry, &config, &merged, &out).expect("write");
    assert_eq!(report.pages, 46);
    let widget = fs::read_to_string(out.join("acme-widget.md")).expect("page");
    assert!(widget.contains("| Owner | acme |"));
    assert!(widget.contains("| Id | 4096 |"));
    assert!(widget.contains("| `widget` | `widget` |"));
    let undescribed = fs::read_to_string(out.join("acme-undescribed.md")).expect("page");
    assert!(undescribed.contains("No accepted vectors in the conformance corpus."));

    fs::remove_dir_all(&dir).expect("remove temp dir");
    fs::remove_dir_all(&out).expect("remove temp dir");
}

#[test]
fn a_scalar_repeated_across_vector_files_is_refused() {
    let config = common::config();
    let paths: Vec<PathBuf> = config
        .docs
        .as_ref()
        .expect("[docs]")
        .vectors
        .iter()
        .map(|p| config.resolve(p))
        .collect();
    let twice: Vec<PathBuf> = paths.iter().chain(paths.iter()).cloned().collect();
    let err = Vectors::load(&twice).expect_err("duplicate scalar");
    assert!(err.to_string().contains("repeats vectors for"), "{err}");
}
