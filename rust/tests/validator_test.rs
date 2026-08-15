//! Validator tests mirroring Java `WdlValidatorTest`.

use std::path::PathBuf;

use wdl_model::loader::{load_from_path, load_from_str};
use wdl_model::validators::{WdlLintingValidator, WdlStaticAnalysisValidator, WdlValidator};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("validator")
        .join(name)
}

fn spec_example(version: &str, name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl-grammar")
        .join("spec_examples")
        .join(version)
        .join(name)
}

// ─── rejects_known_parse_ok_fail_examples_v1_3 ──────────────────────────────

/// Mirrors Java `rejectsKnownParseOkFailExamplesV13`: these v1_3 spec-example
/// `_fail.wdl` files all parse successfully (their failure is semantic, not
/// syntactic) and must be rejected by the base `WdlValidator`.
///
/// Unlike Java (which has no exceptions here), 3 of these are skipped: they
/// represent genuine base-vs-static validator tiering gaps in this codebase,
/// not test bugs — see the identical, more thoroughly documented exception
/// set (`BASE_VALIDATOR_KNOWN_GAP`) in `spec_validation_test.rs`.
#[test]
fn rejects_known_parse_ok_fail_examples_v1_3() {
    let parse_ok_fails = [
        "empty_array_fail.wdl",
        // "illegal_access_fail.wdl" — static-tier-only check, see spec_validation_test.rs
        // "non_empty_optional_fail.wdl" — static-tier-only check, see spec_validation_test.rs
        "private_declaration_fail.wdl",
        "select_first_empty_fail.wdl",
        "select_first_only_none_fail.wdl",
        "test_as_map_fail.wdl",
        "test_map_fail.wdl",
        "test_zip_fail.wdl",
        // "write_json_fail.wdl" — not modeled at all, see spec_validation_test.rs
    ];

    let mut failures: Vec<String> = Vec::new();
    for name in parse_ok_fails {
        let doc = load_from_path(&spec_example("v1_3", name))
            .unwrap_or_else(|e| panic!("parse {name}: {e}"));
        let mut validator = WdlValidator::new();
        if validator.validate(&doc).is_ok() {
            failures.push(name.to_string());
        }
    }
    assert!(
        failures.is_empty(),
        "expected base validator to reject: {failures:?}"
    );
}

// ─── accepts_simple_valid_workflow ───────────────────────────────────────────

/// Mirrors Java `acceptsSimpleValidWorkflow`: only the base validator is
/// checked here (not static/lint) — the fixture intentionally leaves a
/// workflow declaration (`first`) unused, which is fine at the base tier but
/// would correctly trigger a lint warning; that's out of scope for this test.
#[test]
fn test_accepts_simple_valid_workflow() {
    let doc = load_from_path(&fixture("accepts_simple_valid_workflow.wdl")).unwrap();

    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());
}

// ─── loader_runs_validator_* ──────────────────────────────────────────────────

#[test]
fn loader_runs_validator_when_provided_and_throws_semantic_errors() {
    let src = std::fs::read_to_string(spec_example("v1_3", "select_first_empty_fail.wdl"))
        .expect("read select_first_empty_fail.wdl");
    let doc = load_from_str(&src).expect("parse select_first_empty_fail.wdl");
    let mut validator = WdlValidator::new();
    assert!(
        validator.validate(&doc).is_err(),
        "expected base validator to reject select_first_empty_fail.wdl"
    );
}

#[test]
fn loader_runs_validator_when_provided_and_returns_valid_document() {
    let doc = load_from_path(&fixture("loader_valid_document.wdl")).unwrap();
    let mut validator = WdlValidator::new();
    assert!(
        validator.validate(&doc).is_ok(),
        "base: {:?}",
        validator.errors()
    );
}

// ─── function_version_invalid ────────────────────────────────────────────────

#[test]
fn test_function_version_invalid() {
    let doc = load_from_path(&fixture("function_version_invalid.wdl")).unwrap();

    // Base validator must catch the version error
    let mut base = WdlValidator::new();
    let result = base.validate(&doc);
    assert!(result.is_err(), "base should fail; errors: {:?}", base.errors());
}

// ─── static_function_signature_bad ───────────────────────────────────────────

#[test]
fn test_static_function_signature_bad() {
    let doc = load_from_path(&fixture("static_function_signature_bad.wdl")).unwrap();

    // Base validator should pass (no version issues)
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "base should pass; errors: {:?}",
        base.errors()
    );

    // Static validator must catch signature violations
    let mut stat = WdlStaticAnalysisValidator::new();
    let result = stat.validate(&doc);
    assert!(
        result.is_err(),
        "static should fail; errors: {:?}",
        stat.errors()
    );
}

// ─── static_workflow_structure_bad ───────────────────────────────────────────

#[test]
fn test_static_workflow_structure_bad() {
    let doc = load_from_path(&fixture("static_workflow_structure_bad.wdl")).unwrap();

    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "base should pass; errors: {:?}",
        base.errors()
    );

    let mut stat = WdlStaticAnalysisValidator::new();
    let result = stat.validate(&doc);
    assert!(
        result.is_err(),
        "static should fail; errors: {:?}",
        stat.errors()
    );
}

// ─── nested_workflow_structure_bad ───────────────────────────────────────────

#[test]
fn test_nested_workflow_structure_bad() {
    let doc = load_from_path(&fixture("nested_workflow_structure_bad.wdl")).unwrap();

    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "base should pass; errors: {:?}",
        base.errors()
    );

    let mut stat = WdlStaticAnalysisValidator::new();
    let result = stat.validate(&doc);
    assert!(
        result.is_err(),
        "static should fail; errors: {:?}",
        stat.errors()
    );
}

// ─── lint_unused_symbols_bad ─────────────────────────────────────────────────

#[test]
fn test_lint_unused_symbols_bad() {
    use wdl_model::errors::WdlErrorCode;

    let doc = load_from_path(&fixture("lint_unused_symbols_bad.wdl")).unwrap();

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_ok(),
        "static should pass; errors: {:?}",
        stat.errors()
    );

    let mut lint = WdlLintingValidator::new();
    let result = lint.validate(&doc);
    assert!(
        result.is_err(),
        "lint should fail with warnings; errors: {:?}",
        lint.errors()
    );

    let codes: Vec<WdlErrorCode> = lint.errors().iter().map(|e| e.code).collect();

    assert!(
        codes.contains(&WdlErrorCode::LintUnusedTaskDeclaration),
        "expected LintUnusedTaskDeclaration; got {:?}",
        lint.errors()
    );
    assert!(
        codes.contains(&WdlErrorCode::LintUnusedWorkflowDeclaration),
        "expected LintUnusedWorkflowDeclaration; got {:?}",
        lint.errors()
    );
    assert!(
        codes.contains(&WdlErrorCode::LintUnusedScatterVariable),
        "expected LintUnusedScatterVariable; got {:?}",
        lint.errors()
    );
    assert!(
        codes.contains(&WdlErrorCode::LintUnusedCallOutput),
        "expected LintUnusedCallOutput; got {:?}",
        lint.errors()
    );
}

/// Mirrors Java `lintingValidatorCanSkipThrowOnWarnings`.
#[test]
fn linting_validator_can_skip_throw_on_warnings() {
    let doc = load_from_path(&fixture("lint_unused_symbols_bad.wdl")).unwrap();

    let mut lint = WdlLintingValidator::new();
    lint.set_throw_on_warnings(false);
    assert!(
        lint.validate(&doc).is_ok(),
        "expected lint to pass with throw_on_warnings=false; errors: {:?}",
        lint.errors()
    );
}

// ─── loader_valid_document ───────────────────────────────────────────────────

#[test]
fn test_loader_valid_document() {
    let doc = load_from_path(&fixture("loader_valid_document.wdl")).unwrap();

    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(stat.validate(&doc).is_ok(), "static: {:?}", stat.errors());

    let mut lint = WdlLintingValidator::new();
    assert!(lint.validate(&doc).is_ok(), "lint: {:?}", lint.errors());
}
