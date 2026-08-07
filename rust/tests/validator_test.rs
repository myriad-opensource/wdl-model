//! Validator tests mirroring Java `WdlValidatorTest`.

use std::path::PathBuf;

use wdl_model::loader::load_from_path;
use wdl_model::validators::{
    WdlLintingValidator, WdlStaticAnalysisValidator, WdlValidator,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("validator")
        .join(name)
}

// ─── accepts_simple_valid_workflow ───────────────────────────────────────────

#[test]
fn test_accepts_simple_valid_workflow() {
    let doc = load_from_path(&fixture("accepts_simple_valid_workflow.wdl")).unwrap();

    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(stat.validate(&doc).is_ok(), "static: {:?}", stat.errors());

    let mut lint = WdlLintingValidator::new();
    assert!(lint.validate(&doc).is_ok(), "lint: {:?}", lint.errors());
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
