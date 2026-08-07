//! Tests for the Phase 4 processor layer.
//!
//! Mirrors Java's `WdlAppendingProcessorTest` and `WdlExpressionProcessorBaseTest`.

use std::path::PathBuf;

use wdl_model::expressions::{
    WdlArrayLiteral, WdlBinaryOperation, WdlExpression,
    WdlMapEntry, WdlMapLiteral, WdlStringComponent,
    WdlStringLiteral, WdlStringPlaceholderOption, WdlUnaryOperation, BinaryOperator, StringDelimiter,
    UnaryOperator,
};
use wdl_model::processors::appending::WdlAppendingProcessor;
use wdl_model::processors::base::WdlProcessor;
use wdl_model::processors::expression::WdlExpressionProcessor;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn fixtures_root() -> PathBuf {
    // Workspace root is one level up from `rust/`; tests run with `rust/` as cwd.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("parent of rust/ crate dir")
        .join("wdl_tests")
        .join("appending_processor")
}

fn load(filename: &str) -> wdl_model::document::WdlDocument {
    let path = fixtures_root().join(filename);
    wdl_model::loader::load_from_path(&path)
        .unwrap_or_else(|e| panic!("Failed to load {filename}: {e}"))
}

// ---------------------------------------------------------------------------
// WdlAppendingProcessorTest — mirrors Java's WdlAppendingProcessorTest
// ---------------------------------------------------------------------------

/// Mirrors Java `rendersRecursiveWorkflowStatements`.
#[test]
fn renders_recursive_workflow_statements() {
    let doc = load("recursive_workflow_statements.wdl");

    let mut ap = WdlAppendingProcessor::new();
    ap.process_document(&doc);
    let out = ap.into_string();

    assert!(
        out.contains("if (x == 1) {"),
        "expected 'if (x == 1) {{' in output:\n{out}"
    );
    assert!(
        out.contains("scatter (n in [1, 2]) {"),
        "expected 'scatter (n in [1, 2]) {{' in output:\n{out}"
    );
    assert!(
        out.contains("Int y = n"),
        "expected 'Int y = n' in output:\n{out}"
    );
    assert!(
        !out.contains("{ ... }"),
        "should not contain placeholder '{{ ... }}' in output:\n{out}"
    );
}

/// Mirrors Java `rendersMetadataContent`.
#[test]
fn renders_metadata_content() {
    let doc = load("metadata_content.wdl");

    let mut ap = WdlAppendingProcessor::new();
    ap.process_document(&doc);
    let out = ap.into_string();

    assert!(out.contains("meta {"), "expected 'meta {{' in output:\n{out}");
    assert!(out.contains("author:"), "expected 'author:' in output:\n{out}");
    assert!(
        out.contains("parameter_meta {"),
        "expected 'parameter_meta {{' in output:\n{out}"
    );
    assert!(out.contains("x:"), "expected 'x:' in output:\n{out}");
    assert!(
        !out.contains("meta { ... }"),
        "should not contain 'meta {{ ... }}' in output:\n{out}"
    );
    assert!(
        !out.contains("parameter_meta { ... }"),
        "should not contain 'parameter_meta {{ ... }}' in output:\n{out}"
    );
}

// ---------------------------------------------------------------------------
// WdlExpressionProcessorBaseTest — mirrors Java's WdlExpressionProcessorBaseTest
// ---------------------------------------------------------------------------

/// Recording processor that captures `enter_expression` and `enter_string_component` events.
struct RecordingProcessor {
    events: Vec<String>,
}

impl RecordingProcessor {
    fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl WdlExpressionProcessor for RecordingProcessor {
    fn enter_expression(&mut self, expression: &WdlExpression) {
        let tag = match expression.component_type() {
            wdl_model::expressions::ExprComponentType::BoolLit => "BOOL_LIT",
            wdl_model::expressions::ExprComponentType::FloatLit => "FLOAT_LIT",
            wdl_model::expressions::ExprComponentType::IntLit => "INT_LIT",
            wdl_model::expressions::ExprComponentType::ArrayLit => "ARRAY_LIT",
            wdl_model::expressions::ExprComponentType::MapLit => "MAP_LIT",
            wdl_model::expressions::ExprComponentType::NullLit => "NULL_LIT",
            wdl_model::expressions::ExprComponentType::ObjLit => "OBJ_LIT",
            wdl_model::expressions::ExprComponentType::PairLit => "PAIR_LIT",
            wdl_model::expressions::ExprComponentType::StrLit => "STR_LIT",
            wdl_model::expressions::ExprComponentType::StructLit => "STRUCT_LIT",
            wdl_model::expressions::ExprComponentType::Variable => "VARIABLE",
            wdl_model::expressions::ExprComponentType::BinaryOp => "BINARY_OP",
            wdl_model::expressions::ExprComponentType::FuncOp => "FUNC_OP",
            wdl_model::expressions::ExprComponentType::IdxOp => "IDX_OP",
            wdl_model::expressions::ExprComponentType::MemberOp => "MEMBER_OP",
            wdl_model::expressions::ExprComponentType::TernaryOp => "TERNARY_OP",
            wdl_model::expressions::ExprComponentType::UnaryOp => "UNARY_OP",
        };
        self.events.push(tag.to_string());
    }

    fn enter_string_component(
        &mut self,
        _context: &WdlStringLiteral,
        component: &WdlStringComponent,
    ) {
        let tag = match component {
            WdlStringComponent::Text(_) => "SC:TEXT",
            WdlStringComponent::Escape(_) => "SC:ESC",
            WdlStringComponent::Special(_) => "SC:SPECIAL",
            WdlStringComponent::Placeholder { .. } => "SC:PLACEHOLDER",
        };
        self.events.push(tag.to_string());
    }
}

/// Mirrors Java `walksExpressionsDepthFirstUsingComponentTypeDispatch`.
///
/// Tree:
/// ```
/// ArrayLit [
///   BinaryOp(IntLit(1) + IntLit(2)),
///   StrLit(DoubleQuote, [ Text("pre"), Placeholder(default='d', Variable("v")) ]),
///   MapLit { Variable("k"): UnaryOp(- FloatLit(3.0)) }
/// ]
/// ```
///
/// Expected event order (pre-order DFS):
/// ARRAY_LIT, BINARY_OP, INT_LIT, INT_LIT,
/// STR_LIT, SC:TEXT, SC:PLACEHOLDER, STR_LIT, SC:TEXT, VARIABLE,
/// MAP_LIT, VARIABLE, UNARY_OP, FLOAT_LIT
#[test]
fn walks_expressions_depth_first() {
    // Build the expression tree.

    // BinaryOp(1 + 2)
    let bin_op = WdlExpression::BinaryOp(Box::new(WdlBinaryOperation {
        left: Box::new(WdlExpression::IntLit(1)),
        operator: BinaryOperator::Add,
        right: Box::new(WdlExpression::IntLit(2)),
    }));

    // StrLit: "pre${default='d' v}"
    let default_val = {
        let mut s = WdlStringLiteral::new(StringDelimiter::SingleQuote);
        s.components.push(WdlStringComponent::Text("d".to_string()));
        s
    };
    let str_lit = {
        let mut s = WdlStringLiteral::new(StringDelimiter::DoubleQuote);
        s.components.push(WdlStringComponent::Text("pre".to_string()));
        s.components.push(WdlStringComponent::Placeholder {
            symbol: wdl_model::expressions::PlaceholderSymbol::Dollar,
            option: Some(Box::new(WdlStringPlaceholderOption::Default(default_val))),
            expression: Box::new(WdlExpression::Variable("v".to_string())),
        });
        s
    };

    // MapLit { k: -3.0 }
    let map_lit = WdlExpression::MapLit(WdlMapLiteral {
        entries: vec![WdlMapEntry {
            key: WdlExpression::Variable("k".to_string()),
            value: Some(WdlExpression::UnaryOp(Box::new(WdlUnaryOperation {
                operator: UnaryOperator::Negative,
                operand: Box::new(WdlExpression::FloatLit(3.0)),
            }))),
        }],
    });

    // Root: Array[bin_op, str_lit, map_lit]
    let root = WdlExpression::ArrayLit(WdlArrayLiteral {
        entries: vec![bin_op, WdlExpression::StrLit(str_lit), map_lit],
    });

    let mut proc = RecordingProcessor::new();
    proc.walk_expression(&root);

    let expected: Vec<&str> = vec![
        "ARRAY_LIT",
        "BINARY_OP",
        "INT_LIT",
        "INT_LIT",
        "STR_LIT",
        "SC:TEXT",
        "SC:PLACEHOLDER",
        // placeholder option walks the default value string literal
        "STR_LIT",
        "SC:TEXT",
        // then the placeholder's own expression
        "VARIABLE",
        "MAP_LIT",
        "VARIABLE",
        "UNARY_OP",
        "FLOAT_LIT",
    ];

    assert_eq!(
        proc.events,
        expected.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        "Depth-first walk event order mismatch"
    );
}

// ---------------------------------------------------------------------------
// Additional smoke tests for the render helpers
// ---------------------------------------------------------------------------

#[test]
fn expression_to_wdl_primitives() {
    use wdl_model::processors::render::expression_to_wdl;
    assert_eq!(expression_to_wdl(&WdlExpression::BoolLit(true)), "true");
    assert_eq!(expression_to_wdl(&WdlExpression::BoolLit(false)), "false");
    assert_eq!(expression_to_wdl(&WdlExpression::IntLit(42)), "42");
    assert_eq!(expression_to_wdl(&WdlExpression::FloatLit(3.14)), "3.14");
    assert_eq!(expression_to_wdl(&WdlExpression::NullLit), "None");
    assert_eq!(
        expression_to_wdl(&WdlExpression::Variable("x".to_string())),
        "x"
    );
}

#[test]
fn type_to_wdl_basic() {
    use wdl_model::processors::render::type_to_wdl;
    use wdl_model::types::{WdlArrayType, WdlPrimitiveKind, WdlPrimitiveType, WdlType};

    let int_t = WdlType::Primitive(WdlPrimitiveType::new(WdlPrimitiveKind::Int));
    assert_eq!(type_to_wdl(&int_t), "Int");

    let arr_t = WdlType::Array(WdlArrayType::new(int_t));
    assert_eq!(type_to_wdl(&arr_t), "Array[Int]");

    let opt_t = WdlType::Primitive(WdlPrimitiveType::optional(WdlPrimitiveKind::String));
    assert_eq!(type_to_wdl(&opt_t), "String?");
}
