//! Optional → non-optional assignability (Task C1).
//!
//! WDL treats `T?` as a distinct type from `T`: an optional value may be `None`,
//! so assigning `T?` into a `T`-typed declaration is unsound without an explicit
//! unwrap (`select_first`, `select_all`, or `if defined(x) then ... else ...`).
//! The 1.3 coercion table (`SPEC.md:1953`) lists `Y? ← X` as a widening
//! coercion and does not list the reverse.
//!
//! Java enforces this with a single guard at `WdlExpressionValidator.java:701`,
//! which is the *only* place optionality is checked in the whole of
//! `isTypeAssignable` — its `componentType` switch ignores optionality
//! entirely. Python (`wdl_semantic_validator.py:1168`) and TypeScript
//! (`wdl-semantic-validator.ts:1230`) have the same guard; Go does not, and
//! accepts `Int x = someOptionalInt`.
//!
//! Rust had no such guard. It nonetheless got the *common* cases right by
//! accident: for two primitives of identical kind the `expected == actual` fast
//! path fails on the optionality mismatch, no `match` arm matches, and control
//! falls to `_ => false`. That accident breaks down in every arm that matches
//! while *ignoring* optionality — the Int→Float promotion, the File/Directory
//! coercions, the TypeRef name-equality arm, and the Array/Map/Pair arms, which
//! destructure the outer constructor and recurse into the *inner* types without
//! ever consulting outer optionality.
//!
//! Nothing in the spec corpus reaches this path. Instrumenting
//! `is_type_assignable` across the whole suite recorded 320 calls, 46 with an
//! optional type, and **zero** with an optional `actual` — because `infer_type`
//! rarely propagates optionality (`find()` returning `String?` is the only
//! literal optional producer). That upstream gap is tracked under B2; these
//! tests are the only thing exercising the guard.

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

/// Wraps `body` in a minimal 1.3 workflow.
fn wf(body: &str) -> String {
    format!("version 1.3\n\nworkflow w {{\n{body}\n}}\n")
}

// ─── The guard: arms that previously matched while ignoring optionality ──────
//
// Each test below fails if the guard in `is_type_assignable` is removed.

#[test]
fn rejects_optional_int_assigned_to_float() {
    // Int → Float promotion arm matched on primitive kind alone.
    assert_rejects_with_type_mismatch(
        &wf("  Int? x = 1\n  Float y = x"),
        "Float ← Int? (promotion arm ignored optionality)",
    );
}

#[test]
fn rejects_optional_struct_assigned_to_struct() {
    // TypeRef arm compared names alone.
    assert_rejects_with_type_mismatch(
        "version 1.3\n\nstruct P { Int a }\n\nworkflow w {\n  P? p = P{a: 1}\n  P q = p\n}\n",
        "P ← P? (TypeRef arm ignored optionality)",
    );
}

#[test]
fn rejects_outer_optional_array() {
    // Array arm destructured the outer constructor and recursed into members,
    // never checking whether the *array itself* was optional.
    assert_rejects_with_type_mismatch(
        &wf("  Array[Int]? a = [1]\n  Array[Int] b = a"),
        "Array[Int] ← Array[Int]?",
    );
}

#[test]
fn rejects_outer_optional_map() {
    assert_rejects_with_type_mismatch(
        &wf("  Map[String, Int]? m = {\"a\": 1}\n  Map[String, Int] n = m"),
        "Map[String, Int] ← Map[String, Int]?",
    );
}

#[test]
fn rejects_outer_optional_pair() {
    assert_rejects_with_type_mismatch(
        &wf("  Pair[Int, Int]? p = (1, 2)\n  Pair[Int, Int] q = p"),
        "Pair[Int, Int] ← Pair[Int, Int]?",
    );
}

#[test]
fn rejects_optional_string_assigned_to_file() {
    // File ← String coercion arm matched on primitive kind alone.
    assert_rejects_with_type_mismatch(
        &wf("  String? s = \"x\"\n  File f = s"),
        "File ← String? (coercion arm ignored optionality)",
    );
}

#[test]
fn rejects_optional_file_assigned_to_string() {
    // The String ← File/Directory arm is Rust-specific: it is absent from the
    // spec's coercion table and from all four sibling implementations, but is
    // required by the normative spec example `placeholder_coercion.wdl`
    // (`File x` / `String x_as_str = x`). It too ignored optionality.
    assert_rejects_with_type_mismatch(
        &wf("  File? f = \"x\"\n  String s = f"),
        "String ← File? (coercion arm ignored optionality)",
    );
}

// ─── Regression pins: correct before C1, but correct *by accident* ───────────
//
// These pass with and without the guard, so they do not prove the fix. They pin
// the `_ => false` fall-through described in the module docs, which is fragile
// precisely because it is accidental: any future `match` arm added above `_`
// that matches while ignoring optionality would silently reopen a hole. With
// the guard in place that fragility is gone, and these become cheap invariants.

#[test]
fn pins_rejects_optional_primitive_assigned_to_required() {
    assert_rejects_with_type_mismatch(&wf("  Int? x = 1\n  Int y = x"), "Int ← Int?");
}

#[test]
fn pins_rejects_optional_array_member() {
    // Inner optionality, as distinct from `rejects_outer_optional_array`: the
    // Array arm recurses and the accident fires one level down.
    assert_rejects_with_type_mismatch(
        &wf("  Array[Int?] a = [1]\n  Array[Int] b = a"),
        "Array[Int] ← Array[Int?]",
    );
}

#[test]
fn pins_rejects_optional_function_result_assigned_to_required() {
    // `find()` is the only function in the signature table that returns an
    // optional type, so this is the one path where inference itself produces
    // the optional `actual` (see B2).
    assert_rejects_with_type_mismatch(
        &wf("  String s = find(\"a\", \"b\")"),
        "String ← String? from find()",
    );
}

#[test]
fn pins_rejects_optional_value_as_required_call_input() {
    assert_rejects_with_type_mismatch(
        "version 1.3\n\ntask t {\n  input { Int n }\n  command <<<>>>\n}\n\n\
         workflow w {\n  Int? x = 1\n  call t { input: n = x }\n}\n",
        "required call input ← optional value",
    );
}

// ─── The widening direction must stay legal ──────────────────────────────────

#[test]
fn accepts_required_assigned_to_optional() {
    // `Y? ← X` is an explicit row in the coercion table (SPEC.md:1953).
    assert_accepts(&wf("  Int x = 1\n  Int? y = x"), "Int? ← Int");
}

#[test]
fn accepts_optional_assigned_to_optional() {
    assert_accepts(&wf("  Int? x = 1\n  Int? y = x"), "Int? ← Int?");
}
