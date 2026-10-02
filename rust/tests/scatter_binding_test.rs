//! Scatter-variable binding (Task B4.1).
//!
//! Rust binds the scatter variable into `scope_types`
//! (`validators/mod.rs`, `process_workflow_scatter`); Java deliberately does
//! not (`WdlValidator.processWorkflowScatter`, :876-881 — it validates the
//! collection and recurses, and never touches `scopeTypes`). That makes Rust
//! strictly stronger here, and the extra checking is worth keeping: Go does the
//! same (`go/wdl/static_checks.go:233`).
//!
//! The cost is that Rust needs a sound answer for "what is the element type
//! when the collection type cannot be inferred?". Binding a concrete `Object`
//! asserts something false and produces false positives: it made the validator
//! reject the normative spec examples `serde_homogeneous_pair.wdl` and
//! `serde_pair.wdl`, both of which pass a scatter variable over
//! `as_pairs(...)` into a `Pair[_, _]` call input. `as_pairs` has no arm in
//! `infer_function_type`, so the collection type is `None`.
//!
//! Binding `WdlType::Unknown` instead reproduces Java's behaviour exactly —
//! there `scopeTypes.get(name)` is null, `inferType` returns null, and
//! `isAssignableFrom` short-circuits to true
//! (`WdlExpressionValidator.java:252-253`) — while preserving Rust's extra
//! strictness for every collection whose type *is* known.
//!
//! These tests pin both halves of that: no false positive when the element type
//! is unknowable, and no loss of checking when it is knowable.

use wdl_model::errors::WdlErrorCode;
use wdl_model::loader::load_from_str;
use wdl_model::validators::WdlValidator;

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

// ── Un-inferable collection: must not produce a false positive ───────────────

/// Reduced from the spec example `serde_homogeneous_pair.wdl`. `as_pairs` has
/// no return-type arm, so the collection infers as `None`. Binding `Object`
/// made `Pair[String,String] <- Object` fail at the call input.
#[test]
fn scatter_over_uninferable_collection_does_not_reject_call_input() {
    assert_accepts(
        r#"
version 1.1

task consume {
  input {
    Pair[String, String] p
  }
  command <<< >>>
  output {
    String out = "x"
  }
}

workflow w {
  input {
    Map[String, String] m
  }
  scatter (pair in as_pairs(m)) {
    call consume { input: p = pair }
  }
}
"#,
        "scatter var over as_pairs() passed to a Pair-typed call input",
    );
}

/// Same gap reached through a declaration rather than a call input.
#[test]
fn scatter_over_uninferable_collection_does_not_reject_declaration() {
    assert_accepts(
        r#"
version 1.1

workflow w {
  input {
    Map[String, String] m
  }
  scatter (pair in as_pairs(m)) {
    Pair[String, String] echoed = pair
  }
}
"#,
        "scatter var over as_pairs() assigned to a Pair-typed declaration",
    );
}

/// Regression guard, **not** an A/B case: verified to pass against unfixed
/// code too. Member access does not consult the scatter binding unless it is a
/// `TypeRef`, so `Object` and `Unknown` behave identically here. Kept because
/// it is reduced from the spec example `serialize_map.wdl` and pins the shape.
#[test]
fn member_access_on_uninferable_scatter_var_is_not_rejected() {
    assert_accepts(
        r#"
version 1.1

workflow w {
  input {
    Map[String, String] m
  }
  scatter (pair in as_pairs(m)) {
    String joined = "~{pair.left}=~{pair.right}"
  }
}
"#,
        "left/right access on a scatter var whose type is unknown",
    );
}

// ── Inferable collection: checking must be preserved ─────────────────────────

/// Control for the three tests above. If the fix had worked by disabling
/// scatter-variable checking wholesale rather than by narrowing it to the
/// unknown case, this test would stop failing.
#[test]
fn scatter_over_inferable_collection_still_rejects_bad_assignment() {
    assert_rejects_with_type_mismatch(
        r#"
version 1.1

workflow w {
  Array[Int] nums = [1, 2, 3]
  scatter (n in nums) {
    String s = n
  }
}
"#,
        "Int scatter element assigned to a String declaration",
    );
}

/// Second control, through a call input rather than a declaration.
#[test]
fn scatter_over_inferable_collection_still_rejects_bad_call_input() {
    assert_rejects_with_type_mismatch(
        r#"
version 1.1

task consume {
  input {
    Boolean flag
  }
  command <<< >>>
  output {
    String out = "x"
  }
}

workflow w {
  Array[Int] nums = [1, 2, 3]
  scatter (n in nums) {
    call consume { input: flag = n }
  }
}
"#,
        "Int scatter element passed to a Boolean call input",
    );
}

#[test]
fn scatter_over_inferable_collection_accepts_good_assignment() {
    assert_accepts(
        r#"
version 1.1

workflow w {
  Array[Int] nums = [1, 2, 3]
  scatter (n in nums) {
    Int doubled = n * 2
  }
}
"#,
        "Int scatter element assigned to an Int declaration",
    );
}

/// Regression guard, not an A/B case: `[]` already yielded
/// `Array[<unknown member>]` before B4.1, so the element type was already
/// `Unknown` on this path. Pinned because B4.1 makes `Unknown` reachable from a
/// second direction and the two must not diverge.
#[test]
fn scatter_over_empty_literal_binds_unknown() {
    assert_accepts(
        r#"
version 1.1

workflow w {
  scatter (x in []) {
    Int y = x
  }
}
"#,
        "scatter over an empty array literal",
    );
}
