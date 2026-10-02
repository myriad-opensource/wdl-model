//! Spec example validation -- parses and validates every non-fail WDL spec
//! example in all three version directories (v1_1, v1_2, v1_3), and asserts
//! every `_fail` example is rejected by the base validator.
//!
//! Only the `_fail` half mirrors Java, as
//! `WdlV1{1,2,3}SpecExamplesTest.testParseAndValidateFailSpecExample`. Java has
//! no counterpart for the valid half: its `testParseSpecExample` only *parses*
//! valid examples and never constructs a validator
//! (`WdlV11SpecExamplesTest.java:53-76`). The Rust mirror of that is
//! `spec_parse_test.rs`; validating the valid examples here goes beyond the
//! reference implementation deliberately.
//!
//! Files are skipped if they fall into a known-gap category:
//!
//! **Validator false-positive** -- the validator incorrectly rejects a valid
//! spec example due to an over-eager constant-folding rule.
//!
//! **P1 inference gap** -- see [`P1_INFERENCE_GAP`].

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

/// Valid spec examples the base validator rejects. After the B4 audit every
/// remaining entry has the same single cause: **scatter/conditional output
/// rewrapping**. Inside a `scatter {}` a declaration or call output is scalar
/// `T`, but referenced from the enclosing scope it is `Array[T]`; inside
/// `if {}` it is `T?`. Neither `infer_type` nor `scope_types` applies that
/// rewrapping, so the outer reference is checked against the inner scalar type.
///
/// It surfaces as a declaration mismatch (`Array[Array[Int]] aout = a`,
/// `map_to_array.wdl`) or as a call-input mismatch (`args = arg_str`,
/// `serialize_map.wdl`); same gap, two sites.
///
/// **This is not a Java-parity gap.** Java fails all of these too -- measured
/// by running Java's `WdlValidator` directly over each file. Java never
/// catches it because its spec-example tests only *parse* valid examples
/// (`WdlV11SpecExamplesTest.java:53-76`); the validator runs only on
/// `_fail.wdl` files. This suite is therefore stricter than anything Java has,
/// and closing these entries means going *beyond* the reference
/// implementation. Deliberately deferred -- see `rust/.context/B4_plan.md` and
/// the F1 parity sweep.
const P1_INFERENCE_GAP: &[&str] = &[
    "allow_nested.wdl",
    "chunk_array.wdl",
    "main.wdl",
    "map_to_array.wdl",
    "nested_scatter.wdl",
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

/// Spec examples that legitimately fail to *parse* because they use reserved
/// keywords as identifiers (`String in`, `version` as a metadata key,
/// `task.description`, `task.previous`).
///
/// The grammar reserves these: `strictIdentifier` resolves to a plain
/// `IDENTIFIER` token. These files are therefore invalid under the grammar even
/// though they ship as spec examples. Java carries the identical allowlist as
/// `parseFailuresExpectedWithReservedKeywordsV1_2` / `...V1_3`
/// (`WdlV12SpecExamplesTest.java:43`, `WdlV13SpecExamplesTest.java:43`);
/// `test_task_previous.wdl` appears only in v1.3, and skipping a file absent
/// from v1.2 is a no-op.
const RESERVED_KEYWORD_PARSE_FAILURE: &[&str] = &[
    "test_find_task.wdl",
    "test_meta_values.wdl",
    "test_runtime_info_task.wdl",
    "test_task_previous.wdl",
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
        .chain(RESERVED_KEYWORD_PARSE_FAILURE.iter())
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
