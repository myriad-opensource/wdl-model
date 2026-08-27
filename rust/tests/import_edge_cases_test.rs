//! Mirrors Java `WdlImportEdgeCasesTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path;
use wdl_model::validators::WdlValidator;

fn fixture_root(dir: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("import_edge_cases")
        .join(dir)
        .join("root.wdl")
}

#[rstest]
#[case("duplicate_namespace")]
#[case("namespace_conflicts_local")]
#[case("member_alias_conflicts_local")]
#[case("member_alias_duplicate")]
fn rejects_import_edge_case(#[case] dir: &str) {
    let doc = load_from_path(&fixture_root(dir))
        .unwrap_or_else(|e| panic!("load {dir}/root.wdl: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_err(),
        "{dir}: expected WdlValidator to fail; errors: {:?}",
        base.errors()
    );
}

#[test]
fn accepts_mixed_forms_import() {
    let doc = load_from_path(&fixture_root("mixed_forms_ok"))
        .expect("load mixed_forms_ok/root.wdl");
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "mixed_forms_ok: expected to pass; errors: {:?}",
        base.errors()
    );
}
