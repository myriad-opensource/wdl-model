//! Mirrors Java `WdlTypeAssignabilityMatrixTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path;
use wdl_model::validators::WdlStaticAnalysisValidator;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("type_assignability_matrix")
        .join(name)
}

#[rstest]
#[case("optional_from_none_ok.wdl")]
#[case("array_nested_ok.wdl")]
#[case("map_value_type_ok.wdl")]
fn accepts_compatible_assignment(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_ok(),
        "{name}: expected static to pass; errors: {:?}",
        stat.errors()
    );
}

#[rstest]
#[case("required_from_none_fail.wdl")]
#[case("array_member_type_fail.wdl")]
#[case("required_string_to_int_fail.wdl")]
#[case("array_string_to_int_fail.wdl")]
#[case("map_value_type_fail.wdl")]
fn rejects_incompatible_assignment(#[case] name: &str) {
    let doc = load_from_path(&fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_err(),
        "{name}: expected static to fail; errors: {:?}",
        stat.errors()
    );
}

// known_gap_mixed_array_literal.wdl  (Array[Int] xs = [1, "x"])
// known_gap_required_from_none.wdl   (Int i = None)
// Both represent type mismatches the static analyser cannot detect at model level;
// intentionally skipped, consistent with Java behaviour.
