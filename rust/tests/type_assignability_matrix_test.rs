//! Mirrors Java `WdlTypeAssignabilityMatrixTest`.
//!
//! Note: Java's equivalent test uses the base `WdlValidator` for these cases,
//! since Java's base validator performs type-assignability checking directly.
//! In this implementation, type-assignability checking lives in the static
//! analysis tier only — the base `WdlValidator` does not reject any of the
//! `_fail.wdl` fixtures here (confirmed empirically). Using
//! `WdlStaticAnalysisValidator` throughout this file, consistent with the rest
//! of the suite and with Go's equivalent test, is intentional and correct for
//! this codebase's validator architecture, not a parity gap to "fix" by
//! switching to the base validator.

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
#[case("file_directory_from_string_ok.wdl")]
#[case("struct_to_struct_coercion_ok.wdl")]
fn accepts_compatible_assignment(#[case] name: &str) {
    let doc = load_from_path(&fixture(name)).unwrap_or_else(|e| panic!("load {name}: {e}"));
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
#[case("struct_to_struct_incompatible_fail.wdl")]
fn rejects_incompatible_assignment(#[case] name: &str) {
    let doc = load_from_path(&fixture(name)).unwrap_or_else(|e| panic!("load {name}: {e}"));
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

// ─────────────────────────────────────────────────────────────────────────────
// Task B2 — literal type widening (`merge_types`)
//
// These are in-memory rather than fixture-backed: `wdl_tests/` is a shared
// cross-language corpus, and adding Rust-only fixtures there would silently
// change the other four implementations' suites. Same precedent as B1.
//
// Reference: `WdlExpressionValidator.inferType` (Java :262-421) — *not*
// `WdlTypeInference.inferLiteralExpressionType`, which only ever sees scalar,
// Object and Struct literals and folds arrays with different (bail-on-null)
// semantics. See `rust/.context/B2_plan.md` §1.1.
// ─────────────────────────────────────────────────────────────────────────────

use wdl_model::loader::load_from_str;

/// Parses `src`, runs static analysis, and reports whether it was accepted.
fn accepts(src: &str) -> bool {
    let doc =
        load_from_str(src).unwrap_or_else(|e| panic!("parse failed: {e}\n--- source ---\n{src}"));
    WdlStaticAnalysisValidator::new().validate(&doc).is_ok()
}

/// Wraps `body` in a minimal 1.2 workflow.
fn wf(body: &str) -> String {
    format!("version 1.2\n\nworkflow w {{\n{body}\n}}\n")
}

/// `[1, 2.5]` must widen to `Array[Float]`, not stop at the first entry.
///
/// Java: `mergeTypes(Int, Float)` finds neither direction assignable and falls
/// through to the explicit `{Int, Float} → Float` rule (:686-693).
///
/// Note the nesting. The obvious spelling, `Array[Int] a = [1, 2.5]`, cannot
/// observe inference at all: `validate_bound_declaration` has a separate
/// per-element check (`validators/mod.rs:2168`) that compares each entry
/// against the *declared* member type directly, so it rejects that source even
/// when the literal wrongly infers as `Array[Int]`. Wrapping the literal one
/// level down moves it onto the inference path, where the merge is what decides.
#[test]
fn mixed_int_float_array_literal_widens_to_float() {
    assert!(
        !accepts(&wf("  Array[Array[Int]] a = [[1, 2.5]]")),
        "[1, 2.5] must not infer Array[Int] by sampling only the first entry"
    );
    // Non-regression: widening must not over-reject. (This direction does not
    // discriminate on its own — Array[Int] is assignable to Array[Float] via
    // Int→Float promotion — but it pins that the merge produces *something*
    // compatible rather than failing to None.)
    assert!(
        accepts(&wf("  Array[Array[Float]] a = [[1, 2.5]]")),
        "[1, 2.5] should infer Array[Float]"
    );
}

/// Widening must consider *every* entry, not just the first two.
#[test]
fn array_literal_folds_all_entries() {
    assert!(
        !accepts(&wf("  Array[Array[Int]] a = [[1, 2, 3, 4.5]]")),
        "a Float in the last position must still widen the member type"
    );
}

/// An empty array literal infers `Array[<unknown member>]`.
///
/// Java builds `new WdlArrayType(null, …)` unconditionally (:306), and
/// `isTypeAssignable` short-circuits to true when either side is null (:698).
/// So the member type is assignable to anything...
#[test]
fn empty_array_literal_is_assignable_to_any_array() {
    for decl in [
        "  Array[Int] a = []",
        "  Array[String] a = []",
        "  Array[Pair[Int, File]] a = []",
    ] {
        assert!(accepts(&wf(decl)), "{decl} should produce no diagnostics");
    }
}

/// ...but the `Array` constructor around it is still real.
///
/// This is what rules out "just return `None` for `[]`": that would accept
/// `Int a = []`, which Java rejects.
#[test]
fn empty_array_literal_is_not_assignable_to_a_non_array() {
    assert!(
        !accepts(&wf("  Int a = []")),
        "[] is Array[unknown], not unknown — the Array constructor must still bite"
    );
}

/// Ternary branches are merged, not first-wins.
///
/// Java :414-418 routes both branches through `mergeTypes`; Rust previously
/// used `.or_else`, which returned whichever branch inferred first.
#[test]
fn ternary_branches_merge_rather_than_taking_the_first() {
    assert!(
        accepts(&wf("  Float f = if true then 1 else 2.5")),
        "ternary over Int and Float should infer Float"
    );
    assert!(
        !accepts(&wf("  Int i = if true then 1 else 2.5")),
        "ternary must not infer Int from the true branch alone"
    );
}

/// Map keys and values are each folded across all entries.
#[test]
fn map_literal_folds_keys_and_values() {
    assert!(
        accepts(&wf(r#"  Map[String, Float] m = {"a": 1, "b": 2.5}"#)),
        "map values Int and Float should widen to Float"
    );
    assert!(
        !accepts(&wf(r#"  Map[String, Int] m = {"a": 1, "b": 2.5}"#)),
        "map value type must not be sampled from the first entry alone"
    );
}

/// A pair with an un-inferable side is itself un-inferable (Java :309-316),
/// rather than silently substituting `String`.
#[test]
fn pair_with_uninferable_side_is_uninferable() {
    assert!(
        accepts(&wf("  Pair[Int, Int] p = (1, None)")),
        "(1, None) should infer nothing, not Pair[Int, String]"
    );
}

/// The `Unknown`/`None` invariant: a bare unknown must never reach a fold.
///
/// `scatter (x in [])` binds `x` to the empty literal's member type. That is
/// `Unknown`, which is legal *nested* but must surface from `infer_type` as
/// `None` so the array fold skips it — exactly as Java's `continue` on a null
/// item does (:298-300).
///
/// If it leaked through as a bare `Unknown`, the fold would merge it with the
/// `1` (since `Unknown` is assignable to everything) and infer
/// `Array[Unknown]`, which is assignable to `Array[String]` — so the source
/// would be *accepted*. Nested for the same reason as
/// `mixed_int_float_array_literal_widens_to_float`: the per-element check at
/// `validators/mod.rs:2168` would otherwise reject the direct spelling on its
/// own and mask the difference.
#[test]
fn bare_unknown_is_skipped_by_the_array_fold() {
    let src = wf("  scatter (x in []) {\n    Array[Array[String]] ys = [[x, 1]]\n  }");
    assert!(
        !accepts(&src),
        "[x, 1] should infer Array[Int] once the unknown x is skipped"
    );
}
