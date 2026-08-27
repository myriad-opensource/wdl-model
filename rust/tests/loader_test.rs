// Smoke tests for the WDL loader (Phase 3).
//
// These tests exercise `load_from_str` and `load_from_path` against real WDL
// fixtures and verify that the resulting `WdlDocument` contains the expected
// top-level structure.

use std::path::Path;
use wdl_model::definitions::{WdlTaskElement, WdlWorkflowElement};
use wdl_model::expressions::{BinaryOperator, WdlExpression};
use wdl_model::loader::{load_from_path, load_from_path_with_resolver, load_from_str, load_from_str_with_resolver};
use wdl_model::resolvers::FilesystemResolver;
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

// ---------------------------------------------------------------------------
// Grammar behavior — operator associativity, reserved-keyword identifiers
// Mirrors Java `WdlV1LoaderGrammarBehaviorTest`.
// ---------------------------------------------------------------------------

fn grammar_behavior_fixture(name: &str) -> std::path::PathBuf {
    fixture(&format!("wdl_tests/grammar_behavior/{name}"))
}

fn first_workflow_declaration_expr(name: &str) -> WdlExpression {
    let doc = load_from_path(&grammar_behavior_fixture(name))
        .unwrap_or_else(|e| panic!("load {name}: {e}"));
    let workflows: Vec<_> = doc.workflows().collect();
    let wf = workflows.first().unwrap_or_else(|| panic!("{name}: no workflow"));
    match wf.elements.first() {
        Some(WdlWorkflowElement::BoundDeclaration(d)) => d.expression.clone(),
        other => panic!("{name}: expected first workflow element to be a bound declaration, got {other:?}"),
    }
}

fn as_binary_op(expr: &WdlExpression) -> &wdl_model::expressions::WdlBinaryOperation {
    match expr {
        WdlExpression::BinaryOp(op) => op,
        other => panic!("expected BinaryOp, got {other:?}"),
    }
}

#[test]
fn parses_additive_chains_as_left_associative() {
    let expr = first_workflow_declaration_expr("associativity_additive_chain.wdl");
    let root = as_binary_op(&expr);
    assert_eq!(root.operator, BinaryOperator::Subtract);
    assert!(matches!(root.right.as_ref(), WdlExpression::IntLit(3)));

    let left = as_binary_op(&root.left);
    assert_eq!(left.operator, BinaryOperator::Subtract);
    assert!(matches!(left.left.as_ref(), WdlExpression::IntLit(1)));
    assert!(matches!(left.right.as_ref(), WdlExpression::IntLit(2)));
}

#[test]
fn parses_multiplicative_chains_as_left_associative() {
    let expr = first_workflow_declaration_expr("associativity_multiplicative_chain.wdl");
    let root = as_binary_op(&expr);
    assert_eq!(root.operator, BinaryOperator::Divide);
    assert!(matches!(root.right.as_ref(), WdlExpression::IntLit(2)));

    let left = as_binary_op(&root.left);
    assert_eq!(left.operator, BinaryOperator::Divide);
    assert!(matches!(left.left.as_ref(), WdlExpression::IntLit(8)));
    assert!(matches!(left.right.as_ref(), WdlExpression::IntLit(4)));
}

#[test]
fn parses_logical_or_chains_as_left_associative() {
    let expr = first_workflow_declaration_expr("associativity_logical_or_chain.wdl");
    let root = as_binary_op(&expr);
    assert_eq!(root.operator, BinaryOperator::Or);
    assert!(matches!(root.right.as_ref(), WdlExpression::BoolLit(true)));

    let left = as_binary_op(&root.left);
    assert_eq!(left.operator, BinaryOperator::Or);
    assert!(matches!(left.left.as_ref(), WdlExpression::BoolLit(true)));
    assert!(matches!(left.right.as_ref(), WdlExpression::BoolLit(false)));
}

// Unlike Java (whose grammar treats these as strictly reserved and rejects
// them as identifiers), this implementation's grammar deliberately supports a
// broad "keyword compatibility set" (`anyIdentBase` in
// `wdl-grammar/antlr4/v1/WdlV1Parser.g4`), allowing many keywords to be used
// as plain identifiers. These fixtures therefore parse successfully here,
// which is the intentional, documented divergence from Java rather than a
// gap — confirmed empirically before writing these assertions.

#[test]
fn accepts_reserved_keyword_as_declaration_identifier_task() {
    load_from_path(&grammar_behavior_fixture("keyword_decl_identifier_task.wdl"))
        .expect("keyword_decl_identifier_task.wdl should parse: keywords are valid identifiers in this grammar");
}

#[test]
fn accepts_reserved_keyword_as_declaration_identifier_if() {
    load_from_path(&grammar_behavior_fixture("keyword_decl_identifier_if.wdl"))
        .expect("keyword_decl_identifier_if.wdl should parse: keywords are valid identifiers in this grammar");
}

#[test]
fn accepts_reserved_keyword_as_task_input_identifier() {
    load_from_path(&grammar_behavior_fixture("keyword_task_input_in.wdl"))
        .expect("keyword_task_input_in.wdl should parse: keywords are valid identifiers in this grammar");
}

#[test]
fn accepts_reserved_keyword_as_metadata_key() {
    load_from_path(&grammar_behavior_fixture("keyword_metadata_key_version.wdl"))
        .expect("keyword_metadata_key_version.wdl should parse: keywords are valid metadata keys in this grammar");
}

// ---------------------------------------------------------------------------
// Import resolution — recursive population, string-source loading, circular
// detection. Mirrors Java `WdlV1LoaderImportResolutionTest`.
// ---------------------------------------------------------------------------

fn loader_imports_fixture(rel: &str) -> std::path::PathBuf {
    fixture(&format!("wdl_tests/loader_imports/{rel}"))
}

#[test]
fn recursively_loads_imported_documents_into_map() {
    let root = loader_imports_fixture("recursive/root.wdl");
    let child = loader_imports_fixture("recursive/child.wdl");
    let grandchild = loader_imports_fixture("recursive/grandchild.wdl");

    let root_doc = load_from_path_with_resolver(&root, &FilesystemResolver)
        .expect("load recursive/root.wdl");

    assert_eq!(root_doc.imported_documents.len(), 1);
    let child_doc = root_doc.imported_documents.values().next().unwrap();
    let child_location = child_doc
        .source_location
        .as_ref()
        .expect("child source_location should be set");
    assert_eq!(
        std::fs::canonicalize(url::Url::parse(child_location).unwrap().to_file_path().unwrap())
            .unwrap(),
        std::fs::canonicalize(&child).unwrap()
    );

    assert_eq!(child_doc.imported_documents.len(), 1);
    let grandchild_doc = child_doc.imported_documents.values().next().unwrap();
    let grandchild_location = grandchild_doc
        .source_location
        .as_ref()
        .expect("grandchild source_location should be set");
    assert_eq!(
        std::fs::canonicalize(
            url::Url::parse(grandchild_location).unwrap().to_file_path().unwrap()
        )
        .unwrap(),
        std::fs::canonicalize(&grandchild).unwrap()
    );

    let root_imports: Vec<_> = root_doc.import_statements().collect();
    let child_imports: Vec<_> = child_doc.import_statements().collect();
    assert!(!root_imports[0].source_text().is_empty());
    assert!(!child_imports[0].source_text().is_empty());
}

#[test]
fn loads_from_source_code_with_source_location_resolver_then_validator() {
    use wdl_model::validators::WdlValidator;

    let root = loader_imports_fixture("string_input/root.wdl");
    let root_source = std::fs::read_to_string(&root).expect("read string_input/root.wdl");
    let source_url = url::Url::from_file_path(&root).unwrap();

    let root_doc = load_from_str_with_resolver(&root_source, &source_url, &FilesystemResolver)
        .expect("load string_input/root.wdl from source string");

    assert_eq!(root_doc.import_statements().count(), 1);
    assert_eq!(root_doc.imported_documents.len(), 1);
    assert!(root_doc.imported_documents.values().next().is_some());

    let mut validator = WdlValidator::new();
    assert!(
        validator.validate(&root_doc).is_ok(),
        "expected base validator to pass; errors: {:?}",
        validator.errors()
    );
}

#[test]
fn throws_on_direct_circular_imports() {
    let root = loader_imports_fixture("circular/root.wdl");
    let err = load_from_path_with_resolver(&root, &FilesystemResolver)
        .expect_err("expected circular import to fail");
    let message = format!("{err}");
    assert!(message.contains("Circular import detected"), "{message}");
    assert!(message.contains("root.wdl"), "{message}");
    assert!(message.contains("child.wdl"), "{message}");
}

#[test]
fn throws_on_circular_imports_with_relative_path_normalization() {
    let root = loader_imports_fixture("circular_relative/root.wdl");
    let err = load_from_path_with_resolver(&root, &FilesystemResolver)
        .expect_err("expected circular import to fail");
    let message = format!("{err}");
    assert!(message.contains("Circular import detected"), "{message}");
    assert!(message.contains("root.wdl"), "{message}");
    assert!(
        message.contains("nested/child.wdl")
            || message.contains("nested%2Fchild.wdl")
            || message.contains("nested\\child.wdl"),
        "{message}"
    );
}
