//! Spec example validation — parses and validates every non-fail WDL spec
//! example in all three version directories (v1_1, v1_2, v1_3), and asserts
//! every `_fail` example is rejected by the base validator. Mirrors Java's
//! `WdlV1{1,2,3}SpecExamplesTest.testParseSpecExample` /
//! `testParseAndValidateFailSpecExample`.
//!
//! Files are skipped if they fall into a known-gap category:
//!
//! **Validator false-positive** — the validator incorrectly rejects a valid
//! spec example due to an over-eager constant-folding rule.
//!
//! **P1 inference gap** — the validator rejects a valid spec example because
//! the type inference or assignability logic is missing a rule that Java has.
//! Documented in `rust/.context/p1_plan.md`. Should be removed when P1 lands.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use wdl_model::loader::load_from_str;
use wdl_model::validators::WdlValidator;

// ── Known-gap skip sets ───────────────────────────────────────────────────────

/// Files where the validator produces a false-positive error due to
/// over-eager constant folding of `select_first` / `None` literals.
/// Only present in v1_2 and v1_3.
const VALIDATOR_FALSE_POSITIVE: &[&str] = &["placeholder_none.wdl", "test_select_first.wdl"];

/// Valid spec examples that trigger P1 inference gaps once the P0 baseline
/// assignability check is enabled. Each falls into one of these categories,
/// all tracked in `rust/.context/p1_plan.md`:
///
/// - **Scatter/conditional output rewrapping** — inside a `scatter {}` a call
///   output is scalar, but outside it is `Array[T]`; inside `if {}` it is
///   `T?`. Rust's `infer_type` doesn't apply this rewrapping when a symbol
///   defined inside a scatter/conditional is referenced from an outer scope.
///   (`test_scatter.wdl`, `nested_scatter.wdl`, `test_conditional.wdl`,
///   `main.wdl`, `test_keys.wdl`, `test_range.wdl`, `allow_nested.wdl`,
///   `chunk_array.wdl`, `test_values.wdl`)
///
/// - **Weak `ArrayLit`/`MapLit`/`PairLit` element inference** — P1 Task 1.6:
///   `infer_type` uses only the first entry rather than folding all entries
///   via `merge_types`. (`map_to_array.wdl`, `pair_to_array.wdl`,
///   `test_map_ordering.wdl`)
///
/// - **Missing struct/object/map literal assignability walk** — P1 Task 1.2
///   (`Object ← struct/object/Map[String,_]`) and 1.3 (`Map[String,V] ← struct`).
///   (`map_to_struct.wdl`)
///
/// - **Missing `String → File`/`File → String` bidirectional coercion in
///   placeholder contexts** — P1 type-rule expansion.
///   (`placeholder_coercion.wdl`)
///
/// - **Missing struct-literal per-member walk on call inputs** —
///   P1 Task 1.4 / §Task 5 in P2 (`isAssignableFrom` for struct/pair literals
///   as arguments). (`serde_homogeneous_pair.wdl`, `serde_pair.wdl`,
///   `serialize_map.wdl`, `non_empty_optional.wdl`)
///
/// Every entry here should be removable once P1 lands and Rust inference
/// matches Java.
const P1_INFERENCE_GAP: &[&str] = &[
    "allow_nested.wdl",
    "chunk_array.wdl",
    "main.wdl",
    "map_to_array.wdl",
    "map_to_struct.wdl",
    "nested_scatter.wdl",
    "non_empty_optional.wdl",
    "pair_to_array.wdl",
    "serde_homogeneous_pair.wdl",
    "serde_pair.wdl",
    "serialize_map.wdl",
    "test_conditional.wdl",
    "test_keys.wdl",
    "test_map_ordering.wdl",
    "test_range.wdl",
    "test_scatter.wdl",
    "test_values.wdl",
];

/// `_fail.wdl` files that the base `WdlValidator` does not currently reject.
/// Unlike Java's equivalent test (which has zero exceptions here, since Java's
/// base validator performs deeper semantic checks), these represent genuine
/// checks this codebase's base validator does not implement:
///
/// - `non_empty_optional_fail.wdl`: assigning an empty array literal to an
///   `Array[T]+` (non-empty) type — this check exists, but only runs in the
///   static-analysis tier, not the base tier (same base-vs-static tiering
///   noted in `type_assignability_matrix_test.rs` and
///   `import_validation_test.rs`).
/// - `write_json_fail.wdl`: `write_json` on a `Map[Int, ...]` value (JSON
///   requires string keys) — not modeled at all; would require function-return
///   serializability checking not currently implemented.
/// - `illegal_access_fail.wdl` (v1_2/v1_3 only): unknown struct
///   field/call-output access via an imported type — also a static-tier-only
///   check in this codebase (see `import_validation_test.rs`).
const BASE_VALIDATOR_KNOWN_GAP: &[&str] = &[
    "non_empty_optional_fail.wdl",
    "write_json_fail.wdl",
    "illegal_access_fail.wdl",
];

// ── Helper ────────────────────────────────────────────────────────────────────

fn spec_dir(version: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("wdl-grammar")
        .join("spec_examples")
        .join(version)
}

fn run_version(version: &str) {
    let skip_validate: HashSet<&str> = VALIDATOR_FALSE_POSITIVE
        .iter()
        .chain(P1_INFERENCE_GAP.iter())
        .copied()
        .collect();

    let dir = spec_dir(version);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", dir.display(), e))
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.ends_with(".wdl") && !n.contains("_fail")
        })
        .collect();
    files.sort_by_key(|e| e.file_name());

    let mut failures: Vec<String> = Vec::new();

    for entry in &files {
        let name = entry.file_name().to_string_lossy().to_string();

        if skip_validate.contains(name.as_str()) {
            continue;
        }

        let src =
            fs::read_to_string(entry.path()).unwrap_or_else(|e| panic!("read {}: {}", name, e));

        let doc = match load_from_str(&src) {
            Ok(d) => d,
            Err(e) => {
                failures.push(format!("PARSE  {}: {:?}", name, e));
                continue;
            }
        };

        let mut v = WdlValidator::new();
        if let Err(e) = v.validate(&doc) {
            failures.push(format!("VALID  {}: {:?}", name, e));
        }
    }

    if !failures.is_empty() {
        panic!(
            "{} unexpected failure(s) in {}:\n{}",
            failures.len(),
            version,
            failures.join("\n")
        );
    }
}

/// Asserts every `_fail.wdl` spec example in `version` is rejected by the base
/// `WdlValidator` — either at parse time (a syntax error also counts, mirroring
/// Java's combined `load(content, validator)` semantics where both syntax and
/// semantic errors are `WdlException`) or at validation time. Mirrors Java's
/// `testParseAndValidateFailSpecExample`, minus `BASE_VALIDATOR_KNOWN_GAP`.
fn run_fail_examples(version: &str) {
    let skip: HashSet<&str> = BASE_VALIDATOR_KNOWN_GAP.iter().copied().collect();

    let dir = spec_dir(version);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", dir.display(), e))
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.ends_with("_fail.wdl")
        })
        .collect();
    files.sort_by_key(|e| e.file_name());

    let mut failures: Vec<String> = Vec::new();

    for entry in &files {
        let name = entry.file_name().to_string_lossy().to_string();
        if skip.contains(name.as_str()) {
            continue;
        }

        let src =
            fs::read_to_string(entry.path()).unwrap_or_else(|e| panic!("read {}: {}", name, e));

        let rejected = match load_from_str(&src) {
            Err(_) => true, // syntax error also counts as "rejected"
            Ok(doc) => {
                let mut v = WdlValidator::new();
                v.validate(&doc).is_err()
            }
        };
        if !rejected {
            failures.push(format!("NOT REJECTED  {}", name));
        }
    }

    if !failures.is_empty() {
        panic!(
            "{} _fail.wdl file(s) unexpectedly accepted in {}:\n{}",
            failures.len(),
            version,
            failures.join("\n")
        );
    }
}

// ── Per-version tests ─────────────────────────────────────────────────────────

#[test]
fn spec_v1_1() {
    run_version("v1_1");
}

#[test]
fn spec_v1_2() {
    run_version("v1_2");
}

#[test]
fn spec_v1_3() {
    run_version("v1_3");
}

#[test]
fn all_v1_1_fail_examples_rejected_by_base_validator() {
    run_fail_examples("v1_1");
}

#[test]
fn all_v1_2_fail_examples_rejected_by_base_validator() {
    run_fail_examples("v1_2");
}

#[test]
fn all_v1_3_fail_examples_rejected_by_base_validator() {
    run_fail_examples("v1_3");
}
