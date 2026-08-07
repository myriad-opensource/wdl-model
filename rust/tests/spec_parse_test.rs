// Parameterized spec-example parsing tests.
//
// Iterates all .wdl files under wdl-grammar/spec_examples/ and asserts that
// each one parses without panicking.

use std::path::{Path, PathBuf};
use wdl_model::loader::load_from_path;

fn spec_wdl_files(version: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    let dir = root.join("wdl-grammar/spec_examples").join(version);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("cannot read {}", dir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("wdl"))
        .collect();
    files.sort();
    files
}

fn check_no_panic(files: &[PathBuf]) {
    let mut panics: Vec<String> = Vec::new();
    for path in files {
        let path_str = path.display().to_string();
        let result = std::panic::catch_unwind(|| {
            let _ = load_from_path(path);
        });
        if let Err(e) = result {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "unknown panic payload".to_string()
            };
            panics.push(format!("{path_str}: {msg}"));
        }
    }
    if !panics.is_empty() {
        panic!(
            "{} spec file(s) panicked during parse:\n{}",
            panics.len(),
            panics.join("\n")
        );
    }
}

#[test]
fn all_v1_1_spec_examples_parse_without_panic() {
    let files = spec_wdl_files("v1_1");
    assert!(!files.is_empty(), "no v1_1 spec examples found");
    check_no_panic(&files);
}

#[test]
fn all_v1_2_spec_examples_parse_without_panic() {
    let files = spec_wdl_files("v1_2");
    assert!(!files.is_empty(), "no v1_2 spec examples found");
    check_no_panic(&files);
}

#[test]
fn all_v1_3_spec_examples_parse_without_panic() {
    let files = spec_wdl_files("v1_3");
    assert!(!files.is_empty(), "no v1_3 spec examples found");
    check_no_panic(&files);
}
