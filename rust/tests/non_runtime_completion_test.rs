//! Non-runtime completion checks — static analysis catches errors without
//! running the workflow.  Mirrors Java `WdlNonRuntimeCompletionTest`.
//!
//! Known gap: `unknown_struct_field_fail.wdl` requires struct-typed variable
//! declarations in workflow bodies, which our grammar does not yet support.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::{load_from_path, load_from_path_with_resolver};
use wdl_model::resolvers::FilesystemResolver;
use wdl_model::validators::WdlStaticAnalysisValidator;

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("non_runtime_completion")
        .join(rel)
}

// ── Reject cases ─────────────────────────────────────────────────────────────

#[rstest]
#[case("json_type_level_static_fail.wdl")]
#[case("baseline_function_args/contains_key_bad.wdl")]
#[case("baseline_function_args/length_bad.wdl")]
#[case("member_index_checks/index_out_of_bounds_fail.wdl")]
#[case("member_index_checks/unknown_call_output_fail.wdl")]
fn rejects_static(#[case] rel: &str) {
    let doc = load_from_path(&fixture(rel))
        .unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_err(),
        "{rel}: expected WdlStaticAnalysisValidator to fail; errors: {:?}",
        stat.errors()
    );
}

// ── Accept cases ─────────────────────────────────────────────────────────────

#[rstest]
#[case("placeholder_interpolation_ok.wdl")]
#[case("requirements_hints_syntax_ok.wdl")]
#[case("baseline_function_args/baseline_function_args_ok.wdl")]
#[case("member_index_checks/member_index_checks_ok.wdl")]
fn accepts_static(#[case] rel: &str) {
    let doc = load_from_path(&fixture(rel))
        .unwrap_or_else(|e| panic!("parse {rel}: {e}"));
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_ok(),
        "{rel}: expected WdlStaticAnalysisValidator to pass; errors: {:?}",
        stat.errors()
    );
}

/// Import with nested struct aliases resolves and validates cleanly.
#[test]
fn accepts_import_alias_nested() {
    let path = fixture("import_alias_nested/root.wdl");
    let doc = load_from_path_with_resolver(&path, &FilesystemResolver)
        .expect("load import_alias_nested/root.wdl");
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_ok(),
        "import_alias_nested: expected to pass; errors: {:?}",
        stat.errors()
    );
}
