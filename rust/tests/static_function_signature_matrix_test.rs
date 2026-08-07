//! Mirrors Java `WdlStaticFunctionSignatureMatrixTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path;
use wdl_model::validators::{WdlStaticAnalysisValidator, WdlValidator};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("static_function_signature_matrix")
        .join(name)
}

#[rstest]
#[case("keys_bad.wdl")]
#[case("range_bad.wdl")]
#[case("contains_bad.wdl")]
#[case("chunk_bad.wdl")]
#[case("cross_bad.wdl")]
#[case("join_paths_bad_first.wdl")]
#[case("join_paths_bad_tail.wdl")]
#[case("basename_bad_first.wdl")]
#[case("size_bad_second.wdl")]
fn base_passes_static_rejects_invalid_signature(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));

    let mut base = WdlValidator::new();
    assert!(
        base.validate(&doc).is_ok(),
        "{name}: base should pass; errors: {:?}",
        base.errors()
    );

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_err(),
        "{name}: static should fail; errors: {:?}",
        stat.errors()
    );
}

#[test]
fn accepts_valid_signatures_under_all_validators() {
    let doc = load_from_path(&fixture("static_signatures_ok.wdl")).expect("load ok fixture");

    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(stat.validate(&doc).is_ok(), "static: {:?}", stat.errors());
}
