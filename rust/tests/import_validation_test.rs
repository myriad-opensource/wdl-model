//! Mirrors Java `WdlImportValidationTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path_with_resolver;
use wdl_model::resolvers::FilesystemResolver;
use wdl_model::validators::{WdlStaticAnalysisValidator, WdlValidator};

fn fixture_root(dir: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("import_validation")
        .join(dir)
        .join("root.wdl")
}

fn load(dir: &str) -> wdl_model::document::WdlDocument {
    let path = fixture_root(dir);
    load_from_path_with_resolver(&path, &FilesystemResolver)
        .unwrap_or_else(|e| panic!("load {dir}/root.wdl: {e}"))
}

// ── Reject cases ─────────────────────────────────────────────────────────────

#[rstest]
#[case("bad_alias")]
#[case("duplicate_namespace")]
#[case("struct_conflict")]
#[case("unknown_member")]
#[case("version_mismatch")]
fn rejects_import(#[case] dir: &str) {
    let doc = load(dir);
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_err(),
        "{dir}: expected WdlValidator to fail; errors: {:?}",
        base.errors()
    );
}

// ── Accept cases ─────────────────────────────────────────────────────────────

#[rstest]
#[case("standard_alias")]
#[case("star_members")]
fn accepts_import(#[case] dir: &str) {
    let doc = load(dir);
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{dir}: expected WdlValidator to pass; errors: {:?}",
        base.errors()
    );
}

// ── Spec-examples-based import cases ──────────────────────────────────────────
//
// Mirrors Java's `positiveImportExamples`/`negativeImportExamples`, which
// enumerate (version, filename) pairs across v1_1/v1_2/v1_3 and skip any pair
// where the file doesn't exist on disk. The pairs below were confirmed to
// exist at the time of writing; `illegal_access_fail.wdl` has no v1_1 variant.

fn spec_example_path(version: &str, filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl-grammar")
        .join("spec_examples")
        .join(version)
        .join(filename)
}

fn load_spec_example(version: &str, filename: &str) -> wdl_model::document::WdlDocument {
    let path = spec_example_path(version, filename);
    load_from_path_with_resolver(&path, &FilesystemResolver)
        .unwrap_or_else(|e| panic!("load {version}/{filename}: {e}"))
}

#[rstest]
#[case("v1_1", "call_example.wdl")]
#[case("v1_1", "call_imported.wdl")]
#[case("v1_2", "call_example.wdl")]
#[case("v1_2", "call_imported.wdl")]
#[case("v1_3", "call_example.wdl")]
#[case("v1_3", "call_imported.wdl")]
fn validates_positive_import_spec_example(#[case] version: &str, #[case] filename: &str) {
    let doc = load_spec_example(version, filename);
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{version}/{filename}: expected WdlValidator to pass; errors: {:?}",
        base.errors()
    );
}

/// Mirrors Java's `assertThrows(WdlException.class, () -> WdlV1Loader.load(file,
/// new WdlValidator()))`: in Java, `load` performs both parsing and
/// validation, and a syntax error is just as much a `WdlException` as a
/// semantic one. Most of these fixtures actually fail at the *parse* stage in
/// this implementation (a syntax error counts as "rejected", matching Java's
/// combined load+validate semantics); `illegal_access_fail.wdl` parses
/// successfully and is only caught by the static-analysis tier here (unknown
/// type/member-reference detection is a static-tier-only check in this
/// codebase's validator architecture — see `type_assignability_matrix_test.rs`
/// for the same base-vs-static architecture note).
#[rstest]
#[case("v1_1", "call_subworkflow_fail.wdl")]
#[case("v1_1", "incomplete_struct_fail.wdl")]
#[case("v1_2", "call_subworkflow_fail.wdl")]
#[case("v1_2", "incomplete_struct_fail.wdl")]
#[case("v1_2", "illegal_access_fail.wdl")]
#[case("v1_3", "call_subworkflow_fail.wdl")]
#[case("v1_3", "incomplete_struct_fail.wdl")]
#[case("v1_3", "illegal_access_fail.wdl")]
fn rejects_negative_import_spec_example(#[case] version: &str, #[case] filename: &str) {
    let path = spec_example_path(version, filename);
    match load_from_path_with_resolver(&path, &FilesystemResolver) {
        Err(_) => {} // syntax error also counts as "rejected"
        Ok(doc) => {
            let mut stat = WdlStaticAnalysisValidator::new();
            assert!(
                stat.validate(&doc).is_err(),
                "{version}/{filename}: expected static analysis to fail; errors: {:?}",
                stat.errors()
            );
        }
    }
}
