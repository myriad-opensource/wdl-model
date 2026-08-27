//! Non-runtime completion checks — static analysis catches errors without
//! running the workflow. Mirrors Java `WdlNonRuntimeCompletionValidationTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::{load_from_path, load_from_path_with_resolver};
use wdl_model::resolvers::FilesystemResolver;
use wdl_model::validators::{WdlStaticAnalysisValidator, WdlValidator};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("non_runtime_completion")
        .join(rel)
}

/// Import with nested struct aliases resolves and validates cleanly under the
/// base validator.
#[test]
fn validates_nested_import_type_aliases() {
    let path = fixture("import_alias_nested/root.wdl");
    let doc = load_from_path_with_resolver(&path, &FilesystemResolver)
        .expect("load import_alias_nested/root.wdl");
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "import_alias_nested: expected base validator to pass; errors: {:?}",
        base.errors()
    );
}

// ── Baseline function argument checks (base validator) ───────────────────────

#[rstest]
#[case("baseline_function_args/length_bad.wdl")]
#[case("baseline_function_args/contains_key_bad.wdl")]
fn rejects_invalid_baseline_function_args(#[case] rel: &str) {
    let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_err(),
        "{rel}: expected base validator to fail; errors: {:?}",
        base.errors()
    );
}

#[test]
fn accepts_valid_baseline_function_args() {
    let rel = "baseline_function_args/baseline_function_args_ok.wdl";
    let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{rel}: expected base validator to pass; errors: {:?}",
        base.errors()
    );
}

// ── Member/index access checks (base validator) ──────────────────────────────

#[rstest]
#[case("member_index_checks/unknown_struct_field_fail.wdl")]
#[case("member_index_checks/unknown_call_output_fail.wdl")]
#[case("member_index_checks/index_out_of_bounds_fail.wdl")]
fn rejects_invalid_member_and_index_access(#[case] rel: &str) {
    let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_err(),
        "{rel}: expected base validator to fail; errors: {:?}",
        base.errors()
    );
}

#[test]
fn accepts_valid_member_and_index_access() {
    let rel = "member_index_checks/member_index_checks_ok.wdl";
    let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{rel}: expected base validator to pass; errors: {:?}",
        base.errors()
    );
}

// ── Static-analysis-tier checks ───────────────────────────────────────────────

#[test]
fn validates_placeholder_interpolation_and_section_syntax() {
    for rel in ["placeholder_interpolation_ok.wdl", "requirements_hints_syntax_ok.wdl"] {
        let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));
        let mut stat = WdlStaticAnalysisValidator::new();
        assert!(
            stat.validate(&doc).is_ok(),
            "{rel}: expected WdlStaticAnalysisValidator to pass; errors: {:?}",
            stat.errors()
        );
    }
}

/// `json_type_level_static_fail.wdl` parses and passes the base validator, but
/// is rejected once static analysis is applied.
#[test]
fn rejects_invalid_json_type_level_static_usage() {
    let rel = "json_type_level_static_fail.wdl";
    let doc = load_from_path(&fixture(rel)).unwrap_or_else(|e| panic!("parse {rel}: {e}"));

    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{rel}: expected base validator to pass; errors: {:?}",
        base.errors()
    );

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_err(),
        "{rel}: expected WdlStaticAnalysisValidator to fail"
    );
}
