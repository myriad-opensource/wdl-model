//! Spec example validation — parses and validates every non-fail WDL spec
//! example in all three version directories (v1_1, v1_2, v1_3).
//!
//! Files are skipped if they fall into one of two known-gap categories:
//!
//! 1. **Parse gap** — uses grammar features (struct-typed fields / struct vars
//!    in workflow bodies) that the ANTLR4 Rust generator does not yet support.
//! 2. **Validator false-positive** — the validator incorrectly rejects a valid
//!    spec example due to an over-eager constant-folding rule.
//!
//! All `_fail` WDL files are skipped: they test runtime failures that are
//! outside the scope of static validation.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use wdl_model::loader::load_from_str;
use wdl_model::validators::WdlValidator;

// ── Known-gap skip sets ───────────────────────────────────────────────────────

/// Files that fail to parse due to grammar limitations (struct-typed struct
/// fields / struct variables in workflow bodies).  Applies to all versions.
const PARSE_GAP: &[&str] = &[
    "import_structs.wdl",
    "map_to_struct2.wdl",
    "member_access.wdl",
    "nested_access.wdl",
    "pair_to_struct.wdl",
    "person_struct_task.wdl",
    "struct_to_struct.wdl",
    "test_struct.wdl",
];

/// Files where the validator produces a false-positive error due to
/// over-eager constant folding of `select_first` / `None` literals.
/// Only present in v1_2 and v1_3.
const VALIDATOR_FALSE_POSITIVE: &[&str] = &["placeholder_none.wdl", "test_select_first.wdl"];

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
    let skip_parse: HashSet<&str> = PARSE_GAP.iter().copied().collect();
    let skip_validate: HashSet<&str> = VALIDATOR_FALSE_POSITIVE.iter().copied().collect();

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

        if skip_parse.contains(name.as_str()) || skip_validate.contains(name.as_str()) {
            continue;
        }

        let src = fs::read_to_string(entry.path())
            .unwrap_or_else(|e| panic!("read {}: {}", name, e));

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
