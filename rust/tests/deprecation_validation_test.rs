//! Mirrors Java `WdlDeprecationValidationTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::errors::WdlErrorCode;
use wdl_model::loader::{load_from_path, load_from_str};
use wdl_model::validators::WdlLintingValidator;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("deprecations")
        .join(name)
}

// ─── fixtures loaded via path ─────────────────────────────────────────────────

#[rstest]
#[case("runtime_section_deprecated.wdl")]
#[case("object_type_deprecated.wdl")]
#[case("placeholder_options_deprecated.wdl")]
fn warns_on_deprecated_feature(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));

    let mut lint = WdlLintingValidator::new(); // throw_on_warnings = true
    let result = lint.validate(&doc);
    assert!(
        result.is_err(),
        "{name}: expected lint to fail with deprecation warning; errors: {:?}",
        lint.errors()
    );
    let has_deprecation = lint
        .errors()
        .iter()
        .any(|e| e.code == WdlErrorCode::LintDeprecatedFeature);
    assert!(
        has_deprecation,
        "{name}: expected at least one LintDeprecatedFeature; got: {:?}",
        lint.errors()
    );
}

// ─── file:// scheme import — loaded via string to skip resolver ───────────────

#[test]
fn warns_on_file_scheme_import_deprecated() {
    let path = fixture("file_scheme_import_deprecated.wdl");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let doc = load_from_str(&src).unwrap_or_else(|e| panic!("parse: {e}"));

    let mut lint = WdlLintingValidator::new();
    let result = lint.validate(&doc);
    assert!(
        result.is_err(),
        "expected lint to fail; errors: {:?}",
        lint.errors()
    );
    let has_deprecation = lint
        .errors()
        .iter()
        .any(|e| e.code == WdlErrorCode::LintDeprecatedFeature);
    assert!(
        has_deprecation,
        "expected LintDeprecatedFeature; got: {:?}",
        lint.errors()
    );
}

// ─── clean fixture ────────────────────────────────────────────────────────────

#[test]
fn no_deprecation_warning_on_clean_fixture() {
    let doc = load_from_path(&fixture("no_deprecations.wdl"))
        .expect("load no_deprecations.wdl");

    let mut lint = WdlLintingValidator::new();
    lint.set_throw_on_warnings(false);
    assert!(
        lint.validate(&doc).is_ok(),
        "expected no errors; got: {:?}",
        lint.errors()
    );
    let has_deprecation = lint
        .errors()
        .iter()
        .any(|e| e.code == WdlErrorCode::LintDeprecatedFeature);
    assert!(
        !has_deprecation,
        "expected no LintDeprecatedFeature; got: {:?}",
        lint.errors()
    );
}
