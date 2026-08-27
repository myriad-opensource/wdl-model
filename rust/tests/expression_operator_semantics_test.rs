//! Mirrors Java `WdlExpressionOperatorSemanticsTest`.

use std::path::PathBuf;

use rstest::rstest;
use wdl_model::loader::load_from_path;
use wdl_model::validators::{WdlStaticAnalysisValidator, WdlValidator};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl_tests")
        .join("expression_operator_semantics")
        .join(name)
}

#[rstest]
#[case("logical_operand_type_fail.wdl")]
#[case("numeric_operand_type_fail.wdl")]
#[case("order_comparison_type_fail.wdl")]
#[case("ternary_condition_type_fail.wdl")]
fn base_passes_static_rejects_operator_type_mismatch(#[case] name: &str) {
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
fn accepts_valid_operator_expressions() {
    let doc = load_from_path(&fixture("operators_ok.wdl")).expect("load operators_ok");
    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());
    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(stat.validate(&doc).is_ok(), "static: {:?}", stat.errors());
}

#[test]
fn accepts_operator_precedence_and_compound_equality() {
    let prec_doc =
        load_from_path(&fixture("operator_precedence_ok.wdl")).expect("load precedence");
    let eq_doc =
        load_from_path(&fixture("compound_equality_ok.wdl")).expect("load compound_equality");

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&prec_doc).is_ok(),
        "precedence: {:?}",
        stat.errors()
    );

    let mut stat2 = WdlStaticAnalysisValidator::new();
    assert!(
        stat2.validate(&eq_doc).is_ok(),
        "compound_equality: {:?}",
        stat2.errors()
    );
}

#[test]
fn rejects_incompatible_compound_equality() {
    let doc = load_from_path(&fixture("compound_equality_incompatible_fail.wdl"))
        .expect("load compound_equality_incompatible_fail");

    let mut base = WdlValidator::new();
    assert!(base.validate(&doc).is_ok(), "base: {:?}", base.errors());

    let mut stat = WdlStaticAnalysisValidator::new();
    assert!(
        stat.validate(&doc).is_err(),
        "static should fail; errors: {:?}",
        stat.errors()
    );
}
