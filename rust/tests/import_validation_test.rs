//! Mirrors Java `WdlImportValidationTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path_with_resolver;
use wdl_model::resolvers::FilesystemResolver;
use wdl_model::validators::WdlValidator;

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
