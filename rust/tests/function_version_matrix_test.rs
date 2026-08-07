//! Mirrors Java `WdlFunctionVersionMatrixTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::errors::WdlErrorCode;
use wdl_model::loader::load_from_path;
use wdl_model::validators::WdlValidator;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("function_version_matrix")
        .join(name)
}

#[rstest]
#[case("v11_keys_ok.wdl")]
#[case("v12_contains_ok.wdl")]
#[case("v13_value_ok.wdl")]
fn accepts_version_compatible_function(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));
    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{name}: expected base to pass; errors: {:?}",
        base.errors()
    );
}

#[rstest]
#[case("v11_contains_key_fail.wdl")]
#[case("v11_join_paths_fail.wdl")]
#[case("v12_value_fail.wdl")]
fn rejects_version_incompatible_function(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));
    let mut base = WdlValidator::new();
    let result = base.validate(&doc);
    assert!(
        result.is_err(),
        "{name}: expected base to fail; errors: {:?}",
        base.errors()
    );
    let has_version_err = base
        .errors()
        .iter()
        .any(|e| e.code == WdlErrorCode::FunctionNotAvailableInVersion);
    assert!(
        has_version_err,
        "{name}: expected FunctionNotAvailableInVersion; got: {:?}",
        base.errors()
    );
}
