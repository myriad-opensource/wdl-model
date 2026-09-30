//! Enum choice membership validation (Task C2).
//!
//! WDL 1.3 `SPEC.md:1965` defines the `Enum` ← `String` coercion as: "`String`
//! value must exactly match one of the enum's **choice names**". These tests
//! pin that rule.
//!
//! Java has no equivalent test — its four enum tests
//! (`WdlProcessorBaseEnumInferenceTest`) cover processor-level *inference*, not
//! validator assignability — but its implementation is correct
//! (`WdlExpressionValidator.java:216-224`). Rust's was not: `EnumShape` stored a
//! single `choices` list that mixed bare names with `"NAME=value"` renderings,
//! and the membership test compared bare names against that mixed list, so every
//! enum with assigned values rejected its own valid choice names.
//!
//! Nothing in the spec corpus reaches this path: the sole enum spec example
//! (`spec_examples/v1_3/test_enum_value.wdl`) assigns via member access
//! (`Color.Red`), which evaluates to `EvalValue::Unknown` and falls through.

use wdl_model::errors::WdlErrorCode;
use wdl_model::loader::load_from_str;
use wdl_model::validators::WdlValidator;

/// Parses `src` and runs the base validator, returning the collected errors.
fn validate(src: &str) -> Vec<wdl_model::errors::WdlSemanticError> {
    let doc =
        load_from_str(src).unwrap_or_else(|e| panic!("parse failed: {e}\n--- source ---\n{src}"));
    let mut validator = WdlValidator::new();
    let _ = validator.validate(&doc);
    validator.errors().to_vec()
}

fn assert_accepts(src: &str, what: &str) {
    let errors = validate(src);
    assert!(
        errors.is_empty(),
        "{what}: expected zero diagnostics, got: {errors:?}"
    );
}

fn assert_rejects_with_type_mismatch(src: &str, what: &str) {
    let errors = validate(src);
    assert!(
        errors.iter().any(|e| e.code == WdlErrorCode::TypeMismatch),
        "{what}: expected a TypeMismatch, got: {errors:?}"
    );
}

/// Wraps `body` in a minimal 1.3 document with a workflow.
fn doc(enum_decl: &str, decl: &str) -> String {
    format!("version 1.3\n\n{enum_decl}\n\nworkflow w {{\n  {decl}\n}}\n")
}

// ─── Valued enums — the regression this task fixes ────────────────────────────

const COLOR_VALUED: &str = r##"enum Color[String] {
  Red = "#FF0000",
  Green = "#00FF00"
}"##;

#[test]
fn accepts_valid_choice_name_on_valued_enum() {
    // The C2 regression: before the fix this compared "Red" against
    // ["Red=\"#FF0000\"", "Green=\"#00FF00\""] and failed.
    assert_accepts(
        &doc(COLOR_VALUED, r#"Color c = "Red""#),
        "valued enum, valid choice name",
    );
}

#[test]
fn rejects_unknown_choice_name_on_valued_enum() {
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUED, r#"Color c = "Purple""#),
        "valued enum, unknown choice name",
    );
}

#[test]
fn rejects_choice_value_used_in_place_of_choice_name() {
    // SPEC.md:1965 matches on choice *names*, not values. "#FF0000" is Red's
    // value, not its name, so it must be rejected.
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUED, r##"Color c = "#FF0000""##),
        "valued enum, value supplied instead of name",
    );
}

// ─── Valueless enums — must keep working ─────────────────────────────────────

const COLOR_VALUELESS: &str = "enum Color {\n  Red,\n  Green\n}";

#[test]
fn accepts_valid_choice_name_on_valueless_enum() {
    // Worked before the fix too (both renderings coincide when there are no
    // values); this is the regression guard.
    assert_accepts(
        &doc(COLOR_VALUELESS, r#"Color c = "Red""#),
        "valueless enum, valid choice name",
    );
}

#[test]
fn rejects_unknown_choice_name_on_valueless_enum() {
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUELESS, r#"Color c = "Purple""#),
        "valueless enum, unknown choice name",
    );
}

#[test]
fn rejects_the_internal_shape_rendering() {
    // Guards against a regression that re-merges the two collections: the
    // `NAME=value` form is an internal identity rendering and must never be
    // accepted as a choice name.
    //
    // Uses the valueless enum deliberately. Its shape rendering is
    // `"Red=<none>"`, which contains no escapes and so constant-folds cleanly
    // to an `EvalValue::Str`. The valued enum's shape (`Red="#FF0000"`) has
    // embedded quotes that defeat constant folding, making it fall through to
    // the type check — it would pass this assertion for the wrong reason and
    // would not detect a re-merge.
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUELESS, r#"Color c = "Red=<none>""#),
        "valueless enum, internal shape rendering",
    );
}

// ─── Non-String value types ──────────────────────────────────────────────────

#[test]
fn accepts_choice_name_on_non_string_valued_enum() {
    // Membership is on names, so the value type is irrelevant to this check.
    let priority = "enum Priority {\n  Low = 1,\n  High = 10\n}";
    assert_accepts(
        &doc(priority, r#"Priority p = "Low""#),
        "Int-valued enum, valid choice name",
    );
}

// ─── Member-access form must remain unaffected ───────────────────────────────

#[test]
fn accepts_member_access_choice_reference() {
    // `Color.Red` evaluates to EvalValue::Unknown and falls through to the
    // type-based check. This is the form used by
    // `spec_examples/v1_3/test_enum_value.wdl`, so it guards that example.
    assert_accepts(
        &doc(COLOR_VALUED, "Color c = Color.Red"),
        "valued enum, member-access reference",
    );
}
