// Smoke tests for the WDL loader (Phase 3).
//
// These tests exercise `load_from_str` and `load_from_path` against real WDL
// fixtures and verify that the resulting `WdlDocument` contains the expected
// top-level structure.

use std::path::Path;
use wdl_model::definitions::{WdlTaskElement, WdlWorkflowElement};
use wdl_model::loader::{load_from_path, load_from_str};
use wdl_model::version::WdlVersion;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn fixture(rel: &str) -> std::path::PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    root.join(rel)
}

// ---------------------------------------------------------------------------
// Basic version parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_version_1_3() {
    let src = "version 1.3\n\nworkflow empty_wf {\n}\n";
    let doc = load_from_str(src).expect("parse should succeed");
    assert_eq!(doc.wdl_version, Some(WdlVersion::V1_3));
}

#[test]
fn parse_version_1_1() {
    let src = "version 1.1\n\nworkflow empty_wf {\n}\n";
    let doc = load_from_str(src).expect("parse should succeed");
    assert_eq!(doc.wdl_version, Some(WdlVersion::V1_1));
}

// ---------------------------------------------------------------------------
// Task parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_task_with_input_output() {
    let src = r#"
version 1.3

task hello {
  input {
    String name
  }
  command <<< echo ~{name} >>>
  output {
    String greeting = read_string(stdout())
  }
}
"#;
    let doc = load_from_str(src).expect("parse should succeed");
    let tasks: Vec<_> = doc.tasks().collect();
    assert_eq!(tasks.len(), 1, "expected 1 task");
    let task = tasks[0];
    assert_eq!(task.name, "hello");

    let has_input = task
        .elements
        .iter()
        .any(|e| matches!(e, WdlTaskElement::Input(_)));
    let has_output = task
        .elements
        .iter()
        .any(|e| matches!(e, WdlTaskElement::Output(_)));
    let has_command = task
        .elements
        .iter()
        .any(|e| matches!(e, WdlTaskElement::Command(_)));

    assert!(has_input, "task should have an input section");
    assert!(has_output, "task should have an output section");
    assert!(has_command, "task should have a command section");
}

// ---------------------------------------------------------------------------
// Workflow parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_workflow_with_call() {
    let src = r#"
version 1.3

task t {
  input { Int x }
  command <<< echo ~{x} >>>
  output { Int out = x }
}

workflow ok {
  Int i = 1
  call t { x = i }
  output {
    Int y = t.out
  }
}
"#;
    let doc = load_from_str(src).expect("parse should succeed");
    let workflows: Vec<_> = doc.workflows().collect();
    assert_eq!(workflows.len(), 1, "expected 1 workflow");
    let wf = workflows[0];
    assert_eq!(wf.name, "ok");

    let has_output = wf
        .elements
        .iter()
        .any(|e| matches!(e, WdlWorkflowElement::Output(_)));
    let has_call = wf
        .elements
        .iter()
        .any(|e| matches!(e, WdlWorkflowElement::Call(_)));

    assert!(has_output, "workflow should have an output section");
    assert!(has_call, "workflow should have a call statement");
}

// ---------------------------------------------------------------------------
// Syntax error reporting
// ---------------------------------------------------------------------------

#[test]
fn syntax_error_does_not_panic() {
    let src = "version 1.3\n\ntask bad {{ broken";
    // Should either return Err or return a doc with syntax errors embedded.
    // We just verify it doesn't panic.
    let _ = load_from_str(src);
}

// ---------------------------------------------------------------------------
// Struct parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_struct() {
    let src = r#"
version 1.3

struct Point {
  Int x
  Int y
}
"#;
    let doc = load_from_str(src).expect("parse should succeed");
    let structs: Vec<_> = doc.structs().collect();
    assert_eq!(structs.len(), 1, "expected 1 struct");
    assert_eq!(structs[0].name, "Point");
}

// ---------------------------------------------------------------------------
// Load from path
// ---------------------------------------------------------------------------

#[test]
fn load_from_path_validator_fixture() {
    let path = fixture("wdl_tests/validator/loader_valid_document.wdl");
    let doc = load_from_path(&path).expect("should parse fixture file");
    assert_eq!(doc.wdl_version, Some(WdlVersion::V1_3));
    assert_eq!(doc.tasks().count(), 1, "expected 1 task");
    assert_eq!(doc.workflows().count(), 1, "expected 1 workflow");
}

#[test]
fn load_from_path_spec_example() {
    let path = fixture("wdl-grammar/spec_examples/v1_3/all_return_codes_task.wdl");
    let doc = load_from_path(&path).expect("should parse spec example");
    assert_eq!(doc.wdl_version, Some(WdlVersion::V1_3));
    let tasks: Vec<_> = doc.tasks().collect();
    assert_eq!(tasks.len(), 1, "expected 1 task");
    assert_eq!(tasks[0].name, "all_return_codes");
}

// ---------------------------------------------------------------------------
// Import parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_import_statement() {
    let src = r#"
version 1.3

import "other.wdl" as other

workflow w {
}
"#;
    let doc = load_from_str(src).expect("parse should succeed");
    let imports: Vec<_> = doc.import_statements().collect();
    assert_eq!(imports.len(), 1, "expected 1 import");
    let imp = imports[0];
    assert!(
        imp.source_text().contains("other.wdl"),
        "import source_text should contain 'other.wdl', got: {:?}",
        imp.source_text()
    );
}
