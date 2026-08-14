//! WDL document loader — parses WDL source into a [`crate::document::WdlDocument`].
//!
//! # Entry points
//! - [`load_from_str`] — parse from an in-memory string
//! - [`load_from_path`] — read a UTF-8 file then parse
//! - [`load_from_path_with_resolver`] — parse + recursively resolve all imports

#![allow(non_snake_case)] // ANTLR visitor methods use camelCase

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr4rust::errors::ANTLRError;
use antlr4rust::input_stream::InputStream;
use antlr4rust::parser::ParserNodeType;
use antlr4rust::tree::{ErrorNode, ParseTree, ParseTreeVisitor, TerminalNode, VisitableDyn};
use antlr4rust::{tid, Parser};

use crate::definitions::{
    WdlEnum, WdlEnumChoice, WdlStruct, WdlStructElement, WdlStructMember, WdlTask, WdlTaskElement,
    WdlWorkflow, WdlWorkflowElement,
};
use crate::document::{WdlDocument, WdlDocumentElement};
use crate::errors::{WdlError, WdlSyntaxError};
use crate::expressions::{
    BinaryOperator, PlaceholderSymbol, StringDelimiter, UnaryOperator, WdlArrayLiteral,
    WdlBinaryOperation, WdlExpression, WdlFunctionCallOperation, WdlIndexAccessOperation,
    WdlMapEntry, WdlMapLiteral, WdlMemberAccessOperation, WdlObjectEntry, WdlObjectLiteral,
    WdlPairLiteral, WdlStringComponent, WdlStringLiteral, WdlStringPlaceholderOption,
    WdlStructEntry, WdlStructLiteral, WdlTernaryOperation, WdlUnaryOperation,
};
use crate::grammar::wdlv1lexer::WdlV1Lexer;
use crate::grammar::wdlv1parser::*;
use crate::grammar::wdlv1parservisitor::WdlV1ParserVisitor;
use crate::sections::{
    InputDeclaration, WdlCommand, WdlInput, WdlMetadata, WdlMetadataEntry, WdlOutput,
    WdlParameterMetadata, WdlRequirementEntry, WdlRequirements, WdlRuntime, WdlRuntimeEntry,
    WdlTaskHint, WdlTaskHints, WdlWorkflowHint, WdlWorkflowHints,
};
use crate::statements::{
    WdlBoundDeclaration, WdlCall, WdlCallInput, WdlConditional, WdlConditionalElseIf,
    WdlDeclaration, WdlImport, WdlImportMember, WdlImportMembers, WdlImportStar,
    WdlImportStandard, WdlScatter, WdlStatement,
};
use crate::types::{WdlArrayType, WdlMapType, WdlPairType, WdlPrimitiveKind, WdlPrimitiveType, WdlType, WdlTypeRefType};
use crate::version::WdlVersion;

// ============================================================================
// Public entry points
// ============================================================================

/// Parse WDL source text into a [`WdlDocument`]. No import resolution is performed.
pub fn load_from_str(source: &str) -> Result<WdlDocument, WdlError> {
    parse_document(source)
}

/// Read a UTF-8 file and parse it into a [`WdlDocument`]. No import resolution.
pub fn load_from_path(path: &std::path::Path) -> Result<WdlDocument, WdlError> {
    let source = std::fs::read_to_string(path)?;
    parse_document(&source)
}

/// Read a UTF-8 file, parse it, and recursively resolve all imports using `resolver`.
///
/// Each imported document is stored in [`WdlDocument::imported_documents`] keyed by
/// its canonical URL string. Import cycles are silently skipped (the already-seen document
/// is not re-parsed).
///
/// The `import_identifier` field on each [`crate::statements::WdlImport`] is set to the
/// canonical URL string of the resolved document.
pub fn load_from_path_with_resolver(
    path: &std::path::Path,
    resolver: &dyn crate::resolvers::ImportResolver,
) -> Result<WdlDocument, WdlError> {
    let url = url::Url::from_file_path(path).map_err(|_| {
        WdlError::import(
            "cannot create file URL from path",
            path.display().to_string(),
        )
    })?;
    let mut seen: HashSet<String> = HashSet::new();
    load_with_resolver_inner(&url, resolver, &mut seen)
}

// ---------------------------------------------------------------------------
// Internal recursive loader
// ---------------------------------------------------------------------------

fn load_with_resolver_inner(
    doc_url: &url::Url,
    resolver: &dyn crate::resolvers::ImportResolver,
    seen: &mut HashSet<String>,
) -> Result<WdlDocument, WdlError> {
    let url_str = doc_url.to_string();

    // Detect cycles — return an empty stub so the parent can continue.
    if !seen.insert(url_str.clone()) {
        return Ok(WdlDocument::new());
    }

    // Load source content.
    let content = match doc_url.scheme() {
        "file" => {
            let path = doc_url.to_file_path().map_err(|_| {
                WdlError::import("invalid file URL — cannot convert to path", &url_str)
            })?;
            std::fs::read_to_string(&path)?
        }
        _ => resolver
            .resolve_import(None, doc_url.as_str())
            .map_err(|e| WdlError::import(e.to_string(), doc_url.as_str()))?,
    };

    let mut doc = parse_document(&content)?;
    doc.source_location = Some(url_str.clone());

    // Collect (element-index, source_text) pairs for all import statements.
    let import_indices: Vec<(usize, String)> = doc
        .elements
        .iter()
        .enumerate()
        .filter_map(|(i, el)| {
            if let WdlDocumentElement::Import(imp) = el {
                Some((i, imp.source_text().to_owned()))
            } else {
                None
            }
        })
        .collect();

    for (idx, source_text) in import_indices {
        let resolved_url = resolver
            .resolve_import_location(Some(doc_url), &source_text)
            .map_err(|e| WdlError::import(e.to_string(), &source_text))?;
        let resolved_str = resolved_url.to_string();

        // Write back the canonical import identifier.
        if let WdlDocumentElement::Import(imp) = &mut doc.elements[idx] {
            imp.set_import_identifier(resolved_str.clone());
        }

        // Recursively load the imported document (unless it was already inserted by a
        // sibling import that resolved to the same URL).
        if !doc.imported_documents.contains_key(&resolved_str) {
            let imported = load_with_resolver_inner(&resolved_url, resolver, seen)?;
            doc.imported_documents.insert(resolved_str, imported);
        }
    }

    Ok(doc)
}

// ============================================================================
// Error listener — mirrors Java's WdlErrorListener
// ============================================================================

struct WdlErrorListener {
    errors: Rc<RefCell<Vec<WdlSyntaxError>>>,
}

impl WdlErrorListener {
    fn new(errors: Rc<RefCell<Vec<WdlSyntaxError>>>) -> Self {
        Self { errors }
    }
}

impl<'a, T: antlr4rust::recognizer::Recognizer<'a>> ErrorListener<'a, T> for WdlErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as antlr4rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr4rust::errors::ANTLRError>,
    ) {
        self.errors
            .borrow_mut()
            .push(WdlSyntaxError::new(msg, line as i32, column as i32));
    }
}

// ============================================================================
// Error strategy — works around an antlr4rust 0.5.2 bug (see
// rust/.context/rust_parser_fix_plan.md Phase 1)
// ============================================================================
//
// `DefaultErrorStrategy::sync()` gates its early-return decisions on
// `IntervalSet::contains()`, which has a confirmed sortedness-invariant bug in
// antlr4rust 0.5.2: it can return `false` for tokens that a linear scan of the
// very same set (`IntervalSet::to_token_string()`) shows are present. Both the
// context-free `ATN::next_tokens` set and the context-aware
// `Parser::get_expected_tokens()` set go through this same buggy `contains()`,
// so neither is safe to gate on — a "smarter" check that also calls `contains()`
// would just be a different call into the same broken machinery.
//
// This surfaces as spurious syntax errors when a user-defined type name
// (IDENTIFIER) appears as a struct field type or in a bound declaration's type
// position — e.g. `struct Person { Address addr }` or
// `workflow w { S s = S { x: 1 } }` — even though the grammar and the ANTLR4
// ALL(*) prediction algorithm (`adaptive_predict`, a separate simulation not
// affected by this bug) both correctly support these constructs.
//
// `sync()` exists purely as an optimistic pre-check ahead of the real decision
// point; skipping it entirely (mirroring antlr4rust's own `BailErrorStrategy`,
// which also no-ops recovery-related hooks) defers fully to `adaptive_predict`
// and to ordinary token matching (`match_token`, a direct token-id equality
// check, unaffected by `IntervalSet`) for detecting genuine errors. Every other
// method delegates verbatim to `DefaultErrorStrategy`.
struct WdlErrorStrategy<'input, Ctx: ParserNodeType<'input>>(DefaultErrorStrategy<'input, Ctx>);

impl<'input, Ctx: ParserNodeType<'input>> WdlErrorStrategy<'input, Ctx> {
    fn new() -> Self {
        Self(DefaultErrorStrategy::new())
    }
}

tid! { impl<'input, Ctx> TidAble<'input> for WdlErrorStrategy<'input, Ctx> where Ctx: ParserNodeType<'input> }

impl<'input, T: Parser<'input>> ErrorStrategy<'input, T> for WdlErrorStrategy<'input, T::Node> {
    fn reset(&mut self, recognizer: &mut T) {
        self.0.reset(recognizer)
    }

    fn recover_inline(
        &mut self,
        recognizer: &mut T,
    ) -> Result<<T::TF as antlr4rust::token_factory::TokenFactory<'input>>::Tok, ANTLRError> {
        self.0.recover_inline(recognizer)
    }

    fn recover(&mut self, recognizer: &mut T, e: &ANTLRError) -> Result<(), ANTLRError> {
        self.0.recover(recognizer, e)
    }

    fn sync(&mut self, _recognizer: &mut T) -> Result<(), ANTLRError> {
        Ok(())
    }

    fn in_error_recovery_mode(&mut self, recognizer: &mut T) -> bool {
        self.0.in_error_recovery_mode(recognizer)
    }

    fn report_error(&mut self, recognizer: &mut T, e: &ANTLRError) {
        self.0.report_error(recognizer, e)
    }

    fn report_match(&mut self, recognizer: &mut T) {
        self.0.report_match(recognizer)
    }
}

// ============================================================================
// Internal parse function
// ============================================================================

fn parse_document(source: &str) -> Result<WdlDocument, WdlError> {
    let errors: Rc<RefCell<Vec<WdlSyntaxError>>> = Rc::new(RefCell::new(Vec::new()));

    let input = InputStream::new(source.as_bytes());
    let mut lexer = WdlV1Lexer::new(input);
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(WdlErrorListener::new(Rc::clone(&errors))));

    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = WdlV1Parser::with_strategy(token_stream, Box::new(WdlErrorStrategy::new()));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(WdlErrorListener::new(Rc::clone(&errors))));

    let tree = parser
        .document()
        .map_err(|e| WdlError::Syntax(vec![WdlSyntaxError::new(e.to_string(), 0, 0)]))?;

    if !errors.borrow().is_empty() {
        let errs = errors.borrow().clone();
        return Err(WdlError::Syntax(errs));
    }

    let mut builder = WdlV1Builder::new();
    builder.visit_document(&tree);

    Ok(builder.document)
}

// ============================================================================
// StackItem — all types that flow through the builder stack
// ============================================================================

enum StackItem {
    // Imports
    ImportStandard(WdlImportStandard),
    ImportStar(WdlImportStar),
    ImportMembers(WdlImportMembers),
    ImportMember(WdlImportMember),
    // Definitions
    Struct(WdlStruct),
    StructMember(WdlStructMember),
    Enum(WdlEnum),
    EnumChoice(WdlEnumChoice),
    Task(WdlTask),
    Workflow(WdlWorkflow),
    // Declarations
    Declaration(WdlDeclaration),
    BoundDeclaration(WdlBoundDeclaration),
    // Types
    Type(WdlType),
    // Sections
    Input(WdlInput),
    Output(WdlOutput),
    Command(WdlCommand),
    Runtime(WdlRuntime),
    RuntimeEntry(WdlRuntimeEntry),
    Requirements(WdlRequirements),
    RequirementEntry(WdlRequirementEntry),
    TaskHints(WdlTaskHints),
    TaskHint(WdlTaskHint),
    WorkflowHints(WdlWorkflowHints),
    WorkflowHint(WdlWorkflowHint),
    Metadata(WdlMetadata),
    ParameterMetadata(WdlParameterMetadata),
    MetadataEntry(WdlMetadataEntry),
    // Statements
    Call(WdlCall),
    CallInput(WdlCallInput),
    Scatter(WdlScatter),
    Conditional(WdlConditional),
    Statement(WdlStatement),
    // Expressions / strings
    Expr(WdlExpression),
    StringLiteral(WdlStringLiteral),
    StringComponent(WdlStringComponent),
    PlaceholderOption(WdlStringPlaceholderOption),
}

// ============================================================================
// WdlV1Builder
// ============================================================================

struct WdlV1Builder {
    stack: Vec<StackItem>,
    document: WdlDocument,
}

impl WdlV1Builder {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            document: WdlDocument::new(),
        }
    }

    // -------------------------------------------------------------------------
    // Typed pop helpers — panic with assertion on type mismatch (mirrors Java)
    // -------------------------------------------------------------------------

    fn pop_expr(&mut self) -> WdlExpression {
        match self.stack.pop() {
            Some(StackItem::Expr(e)) => e,
            // A string literal in expression position is lifted to WdlExpression::StrLit.
            Some(StackItem::StringLiteral(s)) => WdlExpression::StrLit(s),
            other => panic!(
                "pop_expr: expected Expr on stack, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_type(&mut self) -> WdlType {
        match self.stack.pop() {
            Some(StackItem::Type(t)) => t,
            other => panic!(
                "pop_type: expected Type on stack, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_string_literal(&mut self) -> WdlStringLiteral {
        match self.stack.pop() {
            Some(StackItem::StringLiteral(s)) => s,
            other => panic!(
                "pop_string_literal: expected StringLiteral, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_string_component(&mut self) -> WdlStringComponent {
        match self.stack.pop() {
            Some(StackItem::StringComponent(c)) => c,
            other => panic!(
                "pop_string_component: expected StringComponent, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_metadata_entry(&mut self) -> WdlMetadataEntry {
        match self.stack.pop() {
            Some(StackItem::MetadataEntry(e)) => e,
            other => panic!(
                "pop_metadata_entry: expected MetadataEntry, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_placeholder_option(&mut self) -> WdlStringPlaceholderOption {
        match self.stack.pop() {
            Some(StackItem::PlaceholderOption(o)) => o,
            other => panic!(
                "pop_placeholder_option: expected PlaceholderOption, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_import_member(&mut self) -> WdlImportMember {
        match self.stack.pop() {
            Some(StackItem::ImportMember(m)) => m,
            other => panic!(
                "pop_import_member: expected ImportMember, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_declaration(&mut self) -> WdlDeclaration {
        match self.stack.pop() {
            Some(StackItem::Declaration(d)) => d,
            other => panic!(
                "pop_declaration: expected Declaration, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_bound_declaration(&mut self) -> WdlBoundDeclaration {
        match self.stack.pop() {
            Some(StackItem::BoundDeclaration(d)) => d,
            other => panic!(
                "pop_bound_declaration: expected BoundDeclaration, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_call_input(&mut self) -> WdlCallInput {
        match self.stack.pop() {
            Some(StackItem::CallInput(i)) => i,
            other => panic!(
                "pop_call_input: expected CallInput, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_runtime_entry(&mut self) -> WdlRuntimeEntry {
        match self.stack.pop() {
            Some(StackItem::RuntimeEntry(e)) => e,
            other => panic!(
                "pop_runtime_entry: expected RuntimeEntry, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_requirement_entry(&mut self) -> WdlRequirementEntry {
        match self.stack.pop() {
            Some(StackItem::RequirementEntry(e)) => e,
            other => panic!(
                "pop_requirement_entry: expected RequirementEntry, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_task_hint(&mut self) -> WdlTaskHint {
        match self.stack.pop() {
            Some(StackItem::TaskHint(h)) => h,
            other => panic!(
                "pop_task_hint: expected TaskHint, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_workflow_hint(&mut self) -> WdlWorkflowHint {
        match self.stack.pop() {
            Some(StackItem::WorkflowHint(h)) => h,
            other => panic!(
                "pop_workflow_hint: expected WorkflowHint, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_statement(&mut self) -> WdlStatement {
        match self.stack.pop() {
            Some(StackItem::Statement(s)) => s,
            other => panic!(
                "pop_statement: expected Statement, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_enum_choice(&mut self) -> WdlEnumChoice {
        match self.stack.pop() {
            Some(StackItem::EnumChoice(c)) => c,
            other => panic!(
                "pop_enum_choice: expected EnumChoice, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_task(&mut self) -> WdlTask {
        match self.stack.pop() {
            Some(StackItem::Task(t)) => t,
            other => panic!(
                "pop_task: expected Task, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_workflow(&mut self) -> WdlWorkflow {
        match self.stack.pop() {
            Some(StackItem::Workflow(w)) => w,
            other => panic!(
                "pop_workflow: expected Workflow, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_struct(&mut self) -> WdlStruct {
        match self.stack.pop() {
            Some(StackItem::Struct(s)) => s,
            other => panic!(
                "pop_struct: expected Struct, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_enum(&mut self) -> WdlEnum {
        match self.stack.pop() {
            Some(StackItem::Enum(e)) => e,
            other => panic!(
                "pop_enum: expected Enum, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_input(&mut self) -> WdlInput {
        match self.stack.pop() {
            Some(StackItem::Input(i)) => i,
            other => panic!(
                "pop_input: expected Input, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_output(&mut self) -> WdlOutput {
        match self.stack.pop() {
            Some(StackItem::Output(o)) => o,
            other => panic!(
                "pop_output: expected Output, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_command(&mut self) -> WdlCommand {
        match self.stack.pop() {
            Some(StackItem::Command(c)) => c,
            other => panic!(
                "pop_command: expected Command, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_runtime(&mut self) -> WdlRuntime {
        match self.stack.pop() {
            Some(StackItem::Runtime(r)) => r,
            other => panic!(
                "pop_runtime: expected Runtime, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_requirements(&mut self) -> WdlRequirements {
        match self.stack.pop() {
            Some(StackItem::Requirements(r)) => r,
            other => panic!(
                "pop_requirements: expected Requirements, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_task_hints(&mut self) -> WdlTaskHints {
        match self.stack.pop() {
            Some(StackItem::TaskHints(h)) => h,
            other => panic!(
                "pop_task_hints: expected TaskHints, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_workflow_hints(&mut self) -> WdlWorkflowHints {
        match self.stack.pop() {
            Some(StackItem::WorkflowHints(h)) => h,
            other => panic!(
                "pop_workflow_hints: expected WorkflowHints, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_metadata(&mut self) -> WdlMetadata {
        match self.stack.pop() {
            Some(StackItem::Metadata(m)) => m,
            other => panic!(
                "pop_metadata: expected Metadata, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_parameter_metadata(&mut self) -> WdlParameterMetadata {
        match self.stack.pop() {
            Some(StackItem::ParameterMetadata(m)) => m,
            other => panic!(
                "pop_parameter_metadata: expected ParameterMetadata, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_call(&mut self) -> WdlCall {
        match self.stack.pop() {
            Some(StackItem::Call(c)) => c,
            other => panic!(
                "pop_call: expected Call, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_scatter(&mut self) -> WdlScatter {
        match self.stack.pop() {
            Some(StackItem::Scatter(s)) => s,
            other => panic!(
                "pop_scatter: expected Scatter, got {}",
                stack_item_name(&other)
            ),
        }
    }

    fn pop_conditional(&mut self) -> WdlConditional {
        match self.stack.pop() {
            Some(StackItem::Conditional(c)) => c,
            other => panic!(
                "pop_conditional: expected Conditional, got {}",
                stack_item_name(&other)
            ),
        }
    }

    // -------------------------------------------------------------------------
    // drain helpers — collect items from top of stack while they match a variant
    // Returns items in source order (reverses the LIFO popping order).
    // -------------------------------------------------------------------------

    fn drain_while_import_member(&mut self) -> Vec<WdlImportMember> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::ImportMember(_))) {
            out.push(self.pop_import_member());
        }
        out.reverse();
        out
    }

    #[allow(dead_code)]
    fn drain_while_declaration(&mut self) -> Vec<WdlDeclaration> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::Declaration(_))) {
            out.push(self.pop_declaration());
        }
        out.reverse();
        out
    }

    fn drain_while_bound_decl(&mut self) -> Vec<WdlBoundDeclaration> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::BoundDeclaration(_))) {
            out.push(self.pop_bound_declaration());
        }
        out.reverse();
        out
    }

    fn drain_while_call_input(&mut self) -> Vec<WdlCallInput> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::CallInput(_))) {
            out.push(self.pop_call_input());
        }
        out.reverse();
        out
    }

    fn drain_while_runtime_entry(&mut self) -> Vec<WdlRuntimeEntry> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::RuntimeEntry(_))) {
            out.push(self.pop_runtime_entry());
        }
        out.reverse();
        out
    }

    fn drain_while_req_entry(&mut self) -> Vec<WdlRequirementEntry> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::RequirementEntry(_))) {
            out.push(self.pop_requirement_entry());
        }
        out.reverse();
        out
    }

    fn drain_while_task_hint(&mut self) -> Vec<WdlTaskHint> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::TaskHint(_))) {
            out.push(self.pop_task_hint());
        }
        out.reverse();
        out
    }

    fn drain_while_workflow_hint(&mut self) -> Vec<WdlWorkflowHint> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::WorkflowHint(_))) {
            out.push(self.pop_workflow_hint());
        }
        out.reverse();
        out
    }

    fn drain_while_metadata_entry(&mut self) -> Vec<WdlMetadataEntry> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::MetadataEntry(_))) {
            out.push(self.pop_metadata_entry());
        }
        out.reverse();
        out
    }

    fn drain_while_enum_choice(&mut self) -> Vec<WdlEnumChoice> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::EnumChoice(_))) {
            out.push(self.pop_enum_choice());
        }
        out.reverse();
        out
    }

    fn drain_while_string_component(&mut self) -> Vec<WdlStringComponent> {
        let mut out = Vec::new();
        while matches!(self.stack.last(), Some(StackItem::StringComponent(_))) {
            out.push(self.pop_string_component());
        }
        out.reverse();
        out
    }

    fn drain_while_expr(&mut self) -> Vec<WdlExpression> {
        let mut out = Vec::new();
        loop {
            match self.stack.last() {
                Some(StackItem::Expr(_)) | Some(StackItem::StringLiteral(_)) => {
                    out.push(self.pop_expr());
                }
                _ => break,
            }
        }
        out.reverse();
        out
    }

    /// Drain Expr/StringLiteral items from the top of the stack, stopping once the stack
    /// depth reaches `sentinel`.  This prevents greedily consuming items that were already
    /// on the stack before the current `visit_children` call (e.g. map/array literals
    /// nested inside binary expressions or string placeholders).
    fn drain_expr_above(&mut self, sentinel: usize) -> Vec<WdlExpression> {
        let mut out = Vec::new();
        while self.stack.len() > sentinel {
            match self.stack.last() {
                Some(StackItem::Expr(_)) | Some(StackItem::StringLiteral(_)) => {
                    out.push(self.pop_expr());
                }
                _ => break,
            }
        }
        out.reverse();
        out
    }

    /// Drain exactly `n` Statement items from the top of the stack.
    /// Returns them in source order (reverses LIFO).
    fn drain_n_statements(&mut self, n: usize) -> Vec<WdlStatement> {
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.pop_statement());
        }
        out.reverse();
        out
    }

    // -------------------------------------------------------------------------
    // findWithType equivalents — search stack backwards
    // -------------------------------------------------------------------------

    fn find_task_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Task(_)))
            .expect("find_task_idx: no Task on stack")
    }

    fn find_workflow_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Workflow(_)))
            .expect("find_workflow_idx: no Workflow on stack")
    }

    fn find_struct_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Struct(_)))
            .expect("find_struct_idx: no Struct on stack")
    }

    fn find_enum_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Enum(_)))
            .expect("find_enum_idx: no Enum on stack")
    }

    fn find_scatter_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Scatter(_)))
            .expect("find_scatter_idx: no Scatter on stack")
    }

    fn find_conditional_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Conditional(_)))
            .expect("find_conditional_idx: no Conditional on stack")
    }

    fn find_call_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::Call(_)))
            .expect("find_call_idx: no Call on stack")
    }

    fn find_string_literal_idx(&self) -> usize {
        self.stack
            .iter()
            .rposition(|item| matches!(item, StackItem::StringLiteral(_)))
            .expect("find_string_literal_idx: no StringLiteral on stack")
    }

    // Mutable reference accessors for in-place mutation

    fn task_at_mut(&mut self, idx: usize) -> &mut WdlTask {
        match &mut self.stack[idx] {
            StackItem::Task(t) => t,
            _ => panic!("task_at_mut: wrong item at index"),
        }
    }

    fn workflow_at_mut(&mut self, idx: usize) -> &mut WdlWorkflow {
        match &mut self.stack[idx] {
            StackItem::Workflow(w) => w,
            _ => panic!("workflow_at_mut: wrong item at index"),
        }
    }

    fn scatter_at_mut(&mut self, idx: usize) -> &mut WdlScatter {
        match &mut self.stack[idx] {
            StackItem::Scatter(s) => s,
            _ => panic!("scatter_at_mut: wrong item at index"),
        }
    }

    fn conditional_at_mut(&mut self, idx: usize) -> &mut WdlConditional {
        match &mut self.stack[idx] {
            StackItem::Conditional(c) => c,
            _ => panic!("conditional_at_mut: wrong item at index"),
        }
    }

    fn struct_at_mut(&mut self, idx: usize) -> &mut WdlStruct {
        match &mut self.stack[idx] {
            StackItem::Struct(s) => s,
            _ => panic!("struct_at_mut: wrong item at index"),
        }
    }

    fn call_at_mut(&mut self, idx: usize) -> &mut WdlCall {
        match &mut self.stack[idx] {
            StackItem::Call(c) => c,
            _ => panic!("call_at_mut: wrong item at index"),
        }
    }

    fn string_literal_at_mut(&mut self, idx: usize) -> &mut WdlStringLiteral {
        match &mut self.stack[idx] {
            StackItem::StringLiteral(s) => s,
            _ => panic!("string_literal_at_mut: wrong item at index"),
        }
    }

    // Helper: collect metadata entries from stack top into WdlObjectLiteral
    fn collect_metadata_entries_as_obj_lit(&mut self) -> WdlObjectLiteral {
        let entries = self.drain_while_metadata_entry();
        WdlObjectLiteral {
            entries: entries
                .into_iter()
                .map(|e| WdlObjectEntry {
                    key: e.key,
                    value: e.value,
                })
                .collect(),
        }
    }
}

// ============================================================================
// Helper function for diagnostic messages
// ============================================================================

fn stack_item_name(item: &Option<StackItem>) -> &'static str {
    match item {
        None => "None",
        Some(StackItem::ImportStandard(_)) => "ImportStandard",
        Some(StackItem::ImportStar(_)) => "ImportStar",
        Some(StackItem::ImportMembers(_)) => "ImportMembers",
        Some(StackItem::ImportMember(_)) => "ImportMember",
        Some(StackItem::Struct(_)) => "Struct",
        Some(StackItem::StructMember(_)) => "StructMember",
        Some(StackItem::Enum(_)) => "Enum",
        Some(StackItem::EnumChoice(_)) => "EnumChoice",
        Some(StackItem::Task(_)) => "Task",
        Some(StackItem::Workflow(_)) => "Workflow",
        Some(StackItem::Declaration(_)) => "Declaration",
        Some(StackItem::BoundDeclaration(_)) => "BoundDeclaration",
        Some(StackItem::Type(_)) => "Type",
        Some(StackItem::Input(_)) => "Input",
        Some(StackItem::Output(_)) => "Output",
        Some(StackItem::Command(_)) => "Command",
        Some(StackItem::Runtime(_)) => "Runtime",
        Some(StackItem::RuntimeEntry(_)) => "RuntimeEntry",
        Some(StackItem::Requirements(_)) => "Requirements",
        Some(StackItem::RequirementEntry(_)) => "RequirementEntry",
        Some(StackItem::TaskHints(_)) => "TaskHints",
        Some(StackItem::TaskHint(_)) => "TaskHint",
        Some(StackItem::WorkflowHints(_)) => "WorkflowHints",
        Some(StackItem::WorkflowHint(_)) => "WorkflowHint",
        Some(StackItem::Metadata(_)) => "Metadata",
        Some(StackItem::ParameterMetadata(_)) => "ParameterMetadata",
        Some(StackItem::MetadataEntry(_)) => "MetadataEntry",
        Some(StackItem::Call(_)) => "Call",
        Some(StackItem::CallInput(_)) => "CallInput",
        Some(StackItem::Scatter(_)) => "Scatter",
        Some(StackItem::Conditional(_)) => "Conditional",
        Some(StackItem::Statement(_)) => "Statement",
        Some(StackItem::Expr(_)) => "Expr",
        Some(StackItem::StringLiteral(_)) => "StringLiteral",
        Some(StackItem::StringComponent(_)) => "StringComponent",
        Some(StackItem::PlaceholderOption(_)) => "PlaceholderOption",
    }
}

// ============================================================================
// ParseTreeVisitor implementation — drives visit_children
// ============================================================================

impl<'input> ParseTreeVisitor<'input, WdlV1ParserContextType> for WdlV1Builder {
    fn visit_children(&mut self, node: &(dyn WdlV1ParserContext<'input> + 'input)) {
        let n = node.get_child_count();
        for i in 0..n {
            if let Some(child) = node.get_child(i) {
                child.accept_dyn(self);
            }
        }
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'input, WdlV1ParserContextType>) {}
    fn visit_error_node(&mut self, _node: &ErrorNode<'input, WdlV1ParserContextType>) {}
}

// ============================================================================
// WdlV1ParserVisitor implementation
// ============================================================================

impl<'input> WdlV1ParserVisitor<'input> for WdlV1Builder {
    // =========================================================================
    // Document & Version
    // =========================================================================

    fn visit_document(&mut self, ctx: &DocumentContext<'input>) {
        self.document = WdlDocument::new();
        self.visit_children(ctx);
    }

    fn visit_versionStatement(&mut self, ctx: &VersionStatementContext<'input>) {
        if let Some(tok) = ctx.FLOAT() {
            let text = tok.get_text();
            if let Some(v) = WdlVersion::from_str(&text) {
                self.document.wdl_version = Some(v);
            }
        }
    }

    // =========================================================================
    // Imports
    // =========================================================================

    fn visit_importStatementStandard(&mut self, ctx: &ImportStatementStandardContext<'input>) {
        self.visit_children(ctx);

        // collect alias members (importAlias nodes push ImportMember)
        let mut members = self.drain_while_import_member();
        members.reverse(); // drain already reverses; reverse again for Java-compat addFirst behaviour
        // Actually drain_while already returns source-order (reversed LIFO), so keep it.
        // Re-reverse to match: Java adds with push (LIFO front), so order is reversed.
        members.reverse();

        let alias = if ctx.KEYWORD_AS().is_some() {
            // The strictIdentifier for `as <alias>` is the last one after the URI
            // When AS is present: import "uri" as Alias  -> the strictIdentifier is the alias
            ctx.strictIdentifier().map(|id| id.get_text().to_owned())
        } else {
            None
        };

        let source = self.pop_string_literal();
        let source_text = string_literal_to_text(&source);

        let imp = WdlImportStandard {
            source,
            source_text,
            import_identifier: None,
            alias,
            members,
        };
        self.document
            .elements
            .push(WdlDocumentElement::Import(WdlImport::Standard(imp)));
    }

    fn visit_importStatementStar(&mut self, ctx: &ImportStatementStarContext<'input>) {
        self.visit_children(ctx);
        let source = self.pop_string_literal();
        let source_text = string_literal_to_text(&source);
        let imp = WdlImportStar {
            source,
            source_text,
            import_identifier: None,
        };
        self.document
            .elements
            .push(WdlDocumentElement::Import(WdlImport::Star(imp)));
    }

    fn visit_importStatementMembers(&mut self, ctx: &ImportStatementMembersContext<'input>) {
        self.visit_children(ctx);
        // Grammar order: importMembers KEYWORD_FROM importUriLiteral
        // After visit_children the stack (top-to-bottom) is:
        //   StringLiteral  ← pushed by importUriLiteral (visited last)
        //   ImportMember*  ← pushed by each importMember (visited first)
        // So pop the source literal first, then drain members.
        let source = self.pop_string_literal();
        let members = self.drain_while_import_member();
        let source_text = string_literal_to_text(&source);
        let imp = WdlImportMembers {
            source,
            source_text,
            import_identifier: None,
            members,
        };
        self.document
            .elements
            .push(WdlDocumentElement::Import(WdlImport::Members(imp)));
    }

    fn visit_importMembers(&mut self, ctx: &ImportMembersContext<'input>) {
        self.visit_children(ctx);
        // members are pushed by visit_importMember; nothing else to do here
    }

    fn visit_importMember(&mut self, ctx: &ImportMemberContext<'input>) {
        let member = ctx
            .strictIdentifier(0)
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let alias = if ctx.KEYWORD_AS().is_some() {
            ctx.strictIdentifier(1).map(|id| id.get_text().to_owned())
        } else {
            None
        };
        self.stack
            .push(StackItem::ImportMember(WdlImportMember::new(member, alias)));
    }

    fn visit_importAlias(&mut self, ctx: &ImportAliasContext<'input>) {
        let member = ctx
            .strictIdentifier(0)
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let alias = if ctx.KEYWORD_AS().is_some() {
            ctx.strictIdentifier(1).map(|id| id.get_text().to_owned())
        } else {
            None
        };
        self.stack
            .push(StackItem::ImportMember(WdlImportMember::new(member, alias)));
    }

    fn visit_importUriLiteral(&mut self, ctx: &ImportUriLiteralContext<'input>) {
        let delim = if ctx.SINGLE_QUOTE().is_some() {
            StringDelimiter::SingleQuote
        } else {
            StringDelimiter::DoubleQuote
        };
        self.stack
            .push(StackItem::StringLiteral(WdlStringLiteral::new(delim)));
        self.visit_children(ctx);
        // components are added directly to the StringLiteral by visit_importUriElement
    }

    fn visit_importUriElement(&mut self, ctx: &ImportUriElementContext<'input>) {
        let idx = self.find_string_literal_idx();
        let lit = self.string_literal_at_mut(idx);
        if let Some(tok) = ctx.STRING_TEXT() {
            lit.components
                .push(WdlStringComponent::Text(tok.get_text().to_owned()));
        } else if let Some(tok) = ctx.STRING_ESCAPE() {
            lit.components
                .push(WdlStringComponent::Escape(tok.get_text().to_owned()));
        }
    }

    // =========================================================================
    // Struct Definitions
    // =========================================================================

    fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Struct(WdlStruct::new(name)));
        self.visit_children(ctx);
        let s = self.pop_struct();
        self.document
            .elements
            .push(WdlDocumentElement::Struct(s));
    }

    fn visit_structItemMemberDeclaration(
        &mut self,
        ctx: &StructItemMemberDeclarationContext<'input>,
    ) {
        self.visit_children(ctx);
        let wdl_type = self.pop_type();
        let name = ctx
            .structDeclaration()
            .and_then(|d| d.strictIdentifier())
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let member = WdlStructMember::new(wdl_type, name);
        let idx = self.find_struct_idx();
        self.struct_at_mut(idx)
            .elements
            .push(WdlStructElement::Member(member));
    }

    fn visit_structItemMetadata(&mut self, ctx: &StructItemMetadataContext<'input>) {
        self.visit_children(ctx);
        let meta = self.pop_metadata();
        let idx = self.find_struct_idx();
        self.struct_at_mut(idx)
            .elements
            .push(WdlStructElement::Meta(meta));
    }

    fn visit_structItemParameterMetadata(
        &mut self,
        ctx: &StructItemParameterMetadataContext<'input>,
    ) {
        self.visit_children(ctx);
        let pm = self.pop_parameter_metadata();
        let idx = self.find_struct_idx();
        self.struct_at_mut(idx)
            .elements
            .push(WdlStructElement::ParameterMeta(pm));
    }

    // =========================================================================
    // Enum Definitions
    // =========================================================================

    fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Enum(WdlEnum::new(name)));
        self.visit_children(ctx);
        // Drain choices FIRST (they sit above the Enum sentinel), then pop sentinel.
        let choices = self.drain_while_enum_choice();
        // optional value type (pushed below the choices by visit_enumTypeParameter, if any)
        let value_type = if ctx.enumTypeParameter().is_some() {
            Some(self.pop_type())
        } else {
            None
        };
        let mut enum_def = self.pop_enum();
        enum_def.elements = choices;
        enum_def.value_type = value_type;
        self.document
            .elements
            .push(WdlDocumentElement::Enum(enum_def));
    }

    fn visit_enumChoice(&mut self, ctx: &EnumChoiceContext<'input>) {
        self.visit_children(ctx);
        let value = if ctx.ASSIGNMENT().is_some() {
            Some(self.pop_expr())
        } else {
            None
        };
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let choice = if let Some(v) = value {
            WdlEnumChoice::with_value(name, v)
        } else {
            WdlEnumChoice::new(name)
        };
        self.stack.push(StackItem::EnumChoice(choice));
    }

    // Enum literal expressions — mirror their non-enum counterparts
    fn visit_enumQuotedString(&mut self, ctx: &EnumQuotedStringContext<'input>) {
        let delim = if ctx.SINGLE_QUOTE().is_some() {
            StringDelimiter::SingleQuote
        } else {
            StringDelimiter::DoubleQuote
        };
        self.stack
            .push(StackItem::StringLiteral(WdlStringLiteral::new(delim)));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        let idx = self.find_string_literal_idx();
        self.string_literal_at_mut(idx).components = components;
    }

    fn visit_enumStringElement(&mut self, ctx: &EnumStringElementContext<'input>) {
        if let Some(tok) = ctx.STRING_TEXT() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.STRING_ESCAPE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Escape(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.STRING_DOLLAR_SIGN() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.STRING_TILDE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_enumMultilineString(&mut self, ctx: &EnumMultilineStringContext<'input>) {
        self.stack.push(StackItem::StringLiteral(WdlStringLiteral::new(
            StringDelimiter::Multiline,
        )));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        let idx = self.find_string_literal_idx();
        self.string_literal_at_mut(idx).components = components;
    }

    fn visit_enumMultilineStringElement(&mut self, ctx: &EnumMultilineStringElementContext<'input>) {
        if let Some(tok) = ctx.MULTILINE_STRING_TEXT() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.MULTILINE_STRING_ESCAPE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Escape(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.MULTILINE_STRING_SINGLE_CLOSE_ANGLE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.MULTILINE_STRING_DOLLAR_SIGN() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        } else if let Some(tok) = ctx.MULTILINE_STRING_TILDE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_enumArrayLiteral(&mut self, ctx: &EnumArrayLiteralContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let entries = self.drain_expr_above(sentinel);
        self.stack
            .push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    fn visit_enumMapLiteral(&mut self, ctx: &EnumMapLiteralContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let exprs = self.drain_expr_above(sentinel);
        // exprs: key0, val0, key1, val1, ... (source order after drain_while reversal)
        let entries = exprs
            .chunks(2)
            .map(|pair| WdlMapEntry {
                key: pair[0].clone(),
                value: Some(pair[1].clone()),
            })
            .collect();
        self.stack
            .push(StackItem::Expr(WdlExpression::MapLit(WdlMapLiteral { entries })));
    }

    fn visit_enumMapLiteralItem(&mut self, ctx: &EnumMapLiteralItemContext<'input>) {
        self.visit_children(ctx);
        // key and value both pushed as Expr by child visitors
    }

    fn visit_enumObjectLiteral(&mut self, ctx: &EnumObjectLiteralContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_enumObjectLiteralItem(&mut self, ctx: &EnumObjectLiteralItemContext<'input>) {
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack.push(StackItem::MetadataEntry(
            WdlMetadataEntry::with_value(key, value),
        ));
    }

    fn visit_enumStructLiteral(&mut self, ctx: &EnumStructLiteralContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        // Push a struct-literal sentinel as Expr
        self.stack.push(StackItem::Expr(WdlExpression::StructLit(
            WdlStructLiteral { name, entries: Vec::new() },
        )));
        self.visit_children(ctx);
        // struct entries are added inline by visit_enumStructLiteralItem
    }

    fn visit_enumStructLiteralItem(&mut self, ctx: &EnumStructLiteralItemContext<'input>) {
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        // find the struct literal sentinel and add entry directly
        let entry = WdlStructEntry { key, value: Some(value) };
        for item in self.stack.iter_mut().rev() {
            if let StackItem::Expr(WdlExpression::StructLit(sl)) = item {
                sl.entries.push(entry);
                return;
            }
        }
        panic!("visit_enumStructLiteralItem: no StructLit sentinel on stack");
    }

    fn visit_enumPairLiteral(&mut self, ctx: &EnumPairLiteralContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::PairLit(Box::new(
            WdlPairLiteral {
                left: Box::new(left),
                right: Box::new(right),
            },
        ))));
    }

    // =========================================================================
    // Declarations
    // =========================================================================

    fn visit_unboundDeclaration(&mut self, ctx: &UnboundDeclarationContext<'input>) {
        self.visit_children(ctx);
        let wdl_type = self.pop_type();
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let env = ctx.KEYWORD_ENV().is_some();
        let mut decl = WdlDeclaration::new(wdl_type, name);
        decl.environment_variable = env;
        self.stack.push(StackItem::Declaration(decl));
    }

    fn visit_boundDeclaration(&mut self, ctx: &BoundDeclarationContext<'input>) {
        self.visit_children(ctx);
        let expression = self.pop_expr();
        let wdl_type = self.pop_type();
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let env = ctx.KEYWORD_ENV().is_some();
        let mut decl = WdlBoundDeclaration::new(wdl_type, name, expression);
        decl.environment_variable = env;
        self.stack.push(StackItem::BoundDeclaration(decl));
    }

    // =========================================================================
    // Types
    // =========================================================================

    fn visit_primitiveType(&mut self, ctx: &PrimitiveTypeContext<'input>) {
        let kind = if ctx.KEYWORD_BOOLEAN_TYPE().is_some() {
            WdlPrimitiveKind::Boolean
        } else if ctx.KEYWORD_INT_TYPE().is_some() {
            WdlPrimitiveKind::Int
        } else if ctx.KEYWORD_FLOAT_TYPE().is_some() {
            WdlPrimitiveKind::Float
        } else if ctx.KEYWORD_STRING_TYPE().is_some() {
            WdlPrimitiveKind::String
        } else if ctx.KEYWORD_FILE_TYPE().is_some() {
            WdlPrimitiveKind::File
        } else if ctx.KEYWORD_DIRECTORY_TYPE().is_some() {
            WdlPrimitiveKind::Directory
        } else {
            panic!("visit_primitiveType: unknown primitive type token");
        };
        let optional = ctx.QUESTION_MARK().is_some();
        let pt = if optional {
            WdlPrimitiveType::optional(kind)
        } else {
            WdlPrimitiveType::new(kind)
        };
        self.stack.push(StackItem::Type(WdlType::Primitive(pt)));
    }

    fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) {
        self.visit_children(ctx);
        let member_type = self.pop_type();
        let non_empty = ctx.PLUS().is_some();
        let optional = ctx.QUESTION_MARK().is_some();
        let mut at = WdlArrayType::new(member_type);
        at.non_empty = non_empty;
        at.optional = optional;
        self.stack.push(StackItem::Type(WdlType::Array(at)));
    }

    fn visit_mapType(&mut self, ctx: &MapTypeContext<'input>) {
        self.visit_children(ctx);
        let value_type = self.pop_type();
        let key_type = self.pop_type();
        let optional = ctx.QUESTION_MARK().is_some();
        let mut mt = WdlMapType::new(key_type, value_type);
        mt.optional = optional;
        self.stack.push(StackItem::Type(WdlType::Map(Box::new(mt))));
    }

    fn visit_pairType(&mut self, ctx: &PairTypeContext<'input>) {
        self.visit_children(ctx);
        let right_type = self.pop_type();
        let left_type = self.pop_type();
        let optional = ctx.QUESTION_MARK().is_some();
        let mut pt = WdlPairType::new(left_type, right_type);
        pt.optional = optional;
        self.stack.push(StackItem::Type(WdlType::Pair(Box::new(pt))));
    }

    fn visit_objectType(&mut self, ctx: &ObjectTypeContext<'input>) {
        let optional = ctx.QUESTION_MARK().is_some();
        let mut trt = WdlTypeRefType::new("Object");
        trt.optional = optional;
        self.stack.push(StackItem::Type(WdlType::TypeRef(trt)));
    }

    fn visit_typeRefType(&mut self, ctx: &TypeRefTypeContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let optional = ctx.QUESTION_MARK().is_some();
        let mut trt = WdlTypeRefType::new(name);
        trt.optional = optional;
        self.stack.push(StackItem::Type(WdlType::TypeRef(trt)));
    }

    // =========================================================================
    // Input / Output Sections
    // =========================================================================

    fn visit_inputSection(&mut self, ctx: &InputSectionContext<'input>) {
        self.stack.push(StackItem::Input(WdlInput::new()));
        self.visit_children(ctx);
        // Drain both unbound (Declaration) and bound (BoundDeclaration) declarations.
        // They alternate at the top of the stack; collect until we hit the Input sentinel.
        let mut elements: Vec<InputDeclaration> = Vec::new();
        loop {
            match self.stack.last() {
                Some(StackItem::BoundDeclaration(_)) => {
                    let d = self.pop_bound_declaration();
                    elements.push(InputDeclaration::Bound(d));
                }
                Some(StackItem::Declaration(_)) => {
                    let d = self.pop_declaration();
                    elements.push(InputDeclaration::Unbound(d));
                }
                _ => break,
            }
        }
        elements.reverse(); // restore source order
        match self.stack.last_mut() {
            Some(StackItem::Input(inp)) => inp.elements = elements,
            _ => panic!("visit_inputSection: Input sentinel not on stack"),
        }
    }

    fn visit_outputSection(&mut self, ctx: &OutputSectionContext<'input>) {
        self.stack.push(StackItem::Output(WdlOutput::new()));
        self.visit_children(ctx);
        let decls = self.drain_while_bound_decl();
        match self.stack.last_mut() {
            Some(StackItem::Output(out)) => out.elements = decls,
            _ => panic!("visit_outputSection: Output sentinel not on stack"),
        }
    }

    // =========================================================================
    // Task Definitions
    // =========================================================================

    fn visit_taskDefinition(&mut self, ctx: &TaskDefinitionContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Task(WdlTask::new(name)));
        self.visit_children(ctx);
        let task = self.pop_task();
        self.document.elements.push(WdlDocumentElement::Task(task));
    }

    fn visit_taskInputSection(&mut self, ctx: &TaskInputSectionContext<'input>) {
        self.visit_children(ctx);
        let inp = self.pop_input();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Input(inp));
    }

    fn visit_taskOutputSection(&mut self, ctx: &TaskOutputSectionContext<'input>) {
        self.visit_children(ctx);
        let out = self.pop_output();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Output(out));
    }

    fn visit_taskCommandSection(&mut self, ctx: &TaskCommandSectionContext<'input>) {
        self.stack.push(StackItem::Command(WdlCommand::new(
            WdlStringLiteral::new(StringDelimiter::DoubleQuote),
            false,
        )));
        self.visit_children(ctx);
        let cmd = self.pop_command();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Command(cmd));
    }

    fn visit_commandSection(&mut self, ctx: &CommandSectionContext<'input>) {
        self.visit_children(ctx);
        let lit = self.pop_string_literal();
        // find command on stack and set text
        for item in self.stack.iter_mut().rev() {
            if let StackItem::Command(cmd) = item {
                cmd.command_text = lit;
                return;
            }
        }
        panic!("visit_commandSection: no Command on stack");
    }

    fn visit_multilineStringCommand(&mut self, ctx: &MultilineStringCommandContext<'input>) {
        // Set multiline flag on the Command
        for item in self.stack.iter_mut().rev() {
            if let StackItem::Command(cmd) = item {
                cmd.multiline = true;
                break;
            }
        }
        self.stack.push(StackItem::StringLiteral(WdlStringLiteral::new(
            StringDelimiter::Multiline,
        )));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        match self.stack.last_mut() {
            Some(StackItem::StringLiteral(lit)) => lit.components = components,
            _ => panic!("visit_multilineStringCommand: StringLiteral not on stack"),
        }
    }

    fn visit_bracedCommand(&mut self, ctx: &BracedCommandContext<'input>) {
        for item in self.stack.iter_mut().rev() {
            if let StackItem::Command(cmd) = item {
                cmd.multiline = false;
                break;
            }
        }
        self.stack.push(StackItem::StringLiteral(WdlStringLiteral::new(
            StringDelimiter::DoubleQuote,
        )));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        match self.stack.last_mut() {
            Some(StackItem::StringLiteral(lit)) => lit.components = components,
            _ => panic!("visit_bracedCommand: StringLiteral not on stack"),
        }
    }

    fn visit_taskRuntimeSection(&mut self, ctx: &TaskRuntimeSectionContext<'input>) {
        self.visit_children(ctx);
        let rt = self.pop_runtime();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Runtime(rt));
    }

    fn visit_runtimeSection(&mut self, ctx: &RuntimeSectionContext<'input>) {
        self.stack.push(StackItem::Runtime(WdlRuntime::new()));
        self.visit_children(ctx);
        let entries = self.drain_while_runtime_entry();
        match self.stack.last_mut() {
            Some(StackItem::Runtime(rt)) => rt.elements = entries,
            _ => panic!("visit_runtimeSection: Runtime sentinel not on stack"),
        }
    }

    fn visit_runtimeItem(&mut self, ctx: &RuntimeItemContext<'input>) {
        self.visit_children(ctx);
        let value = self.pop_expr();
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack
            .push(StackItem::RuntimeEntry(WdlRuntimeEntry::with_value(key, value)));
    }

    fn visit_taskRequirementsSection(&mut self, ctx: &TaskRequirementsSectionContext<'input>) {
        self.visit_children(ctx);
        let req = self.pop_requirements();
        let idx = self.find_task_idx();
        self.task_at_mut(idx)
            .elements
            .push(WdlTaskElement::Requirements(req));
    }

    fn visit_requirementsSection(&mut self, ctx: &RequirementsSectionContext<'input>) {
        self.stack
            .push(StackItem::Requirements(WdlRequirements::new()));
        self.visit_children(ctx);
        let entries = self.drain_while_req_entry();
        match self.stack.last_mut() {
            Some(StackItem::Requirements(req)) => req.elements = entries,
            _ => panic!("visit_requirementsSection: Requirements sentinel not on stack"),
        }
    }

    fn visit_requirementsItem(&mut self, ctx: &RequirementsItemContext<'input>) {
        self.visit_children(ctx);
        let value = self.pop_expr();
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack
            .push(StackItem::RequirementEntry(WdlRequirementEntry::with_value(key, value)));
    }

    fn visit_taskHintsSection(&mut self, ctx: &TaskHintsSectionContext<'input>) {
        self.visit_children(ctx);
        let hints = self.pop_task_hints();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Hints(hints));
    }

    fn visit_hintsSectionTask(&mut self, ctx: &HintsSectionTaskContext<'input>) {
        self.stack.push(StackItem::TaskHints(WdlTaskHints::new()));
        self.visit_children(ctx);
        let hints = self.drain_while_task_hint();
        match self.stack.last_mut() {
            Some(StackItem::TaskHints(th)) => th.elements = hints,
            _ => panic!("visit_hintsSectionTask: TaskHints sentinel not on stack"),
        }
    }

    fn visit_hintsItemTask(&mut self, ctx: &HintsItemTaskContext<'input>) {
        self.visit_children(ctx);
        let value = self.pop_expr();
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack
            .push(StackItem::TaskHint(WdlTaskHint::with_value(key, value)));
    }

    fn visit_hintsTypedObjectTask(&mut self, ctx: &HintsTypedObjectTaskContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_hintsObjectItemTask(&mut self, ctx: &HintsObjectItemTaskContext<'input>) {
        let key = ctx
            .dottedIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_inputHintsObjectTask(&mut self, ctx: &InputHintsObjectTaskContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_inputHintsItemTask(&mut self, ctx: &InputHintsItemTaskContext<'input>) {
        let key = ctx
            .dottedIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_outputHintsObjectTask(&mut self, ctx: &OutputHintsObjectTaskContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_outputHintsItemTask(&mut self, ctx: &OutputHintsItemTaskContext<'input>) {
        let key = ctx
            .dottedIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_taskHintsArray(&mut self, ctx: &TaskHintsArrayContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let entries = self.drain_expr_above(sentinel);
        self.stack.push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    fn visit_taskMetadataSection(&mut self, ctx: &TaskMetadataSectionContext<'input>) {
        self.visit_children(ctx);
        let meta = self.pop_metadata();
        let idx = self.find_task_idx();
        self.task_at_mut(idx).elements.push(WdlTaskElement::Meta(meta));
    }

    fn visit_taskParameterMetadataSection(
        &mut self,
        ctx: &TaskParameterMetadataSectionContext<'input>,
    ) {
        self.visit_children(ctx);
        let pm = self.pop_parameter_metadata();
        let idx = self.find_task_idx();
        self.task_at_mut(idx)
            .elements
            .push(WdlTaskElement::ParameterMeta(pm));
    }

    fn visit_taskDeclaration(&mut self, ctx: &TaskDeclarationContext<'input>) {
        self.visit_children(ctx);
        let decls = self.drain_while_bound_decl();
        let idx = self.find_task_idx();
        for d in decls {
            self.task_at_mut(idx)
                .elements
                .push(WdlTaskElement::BoundDeclaration(d));
        }
    }

    // =========================================================================
    // Metadata Sections
    // =========================================================================

    fn visit_metadataSection(&mut self, ctx: &MetadataSectionContext<'input>) {
        self.stack.push(StackItem::Metadata(WdlMetadata::new()));
        self.visit_children(ctx);
        let entries = self.drain_while_metadata_entry();
        match self.stack.last_mut() {
            Some(StackItem::Metadata(m)) => m.elements = entries,
            _ => panic!("visit_metadataSection: Metadata sentinel not on stack"),
        }
    }

    fn visit_parameterMetadataSection(&mut self, ctx: &ParameterMetadataSectionContext<'input>) {
        self.stack
            .push(StackItem::ParameterMetadata(WdlParameterMetadata::new()));
        self.visit_children(ctx);
        let entries = self.drain_while_metadata_entry();
        match self.stack.last_mut() {
            Some(StackItem::ParameterMetadata(pm)) => pm.elements = entries,
            _ => panic!("visit_parameterMetadataSection: ParameterMetadata sentinel not on stack"),
        }
    }

    fn visit_metadataObjectItem(&mut self, ctx: &MetadataObjectItemContext<'input>) {
        let key = ctx
            .dottedIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_metadataObject(&mut self, ctx: &MetadataObjectContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_metadataArray(&mut self, ctx: &MetadataArrayContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let entries = self.drain_expr_above(sentinel);
        self.stack
            .push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    // visit_metadataValue: default visit_children is sufficient (child pushes expression)

    // =========================================================================
    // Workflow Definitions
    // =========================================================================

    fn visit_workflowDefinition(&mut self, ctx: &WorkflowDefinitionContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Workflow(WdlWorkflow::new(name)));
        self.visit_children(ctx);
        let wf = self.pop_workflow();
        self.document
            .elements
            .push(WdlDocumentElement::Workflow(wf));
    }

    fn visit_workflowInputSection(&mut self, ctx: &WorkflowInputSectionContext<'input>) {
        self.visit_children(ctx);
        let inp = self.pop_input();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Input(inp));
    }

    fn visit_workflowOutputSection(&mut self, ctx: &WorkflowOutputSectionContext<'input>) {
        self.visit_children(ctx);
        let out = self.pop_output();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Output(out));
    }

    fn visit_workflowHintsSection(&mut self, ctx: &WorkflowHintsSectionContext<'input>) {
        self.visit_children(ctx);
        let hints = self.pop_workflow_hints();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Hints(hints));
    }

    fn visit_hintsSectionWorkflow(&mut self, ctx: &HintsSectionWorkflowContext<'input>) {
        self.stack
            .push(StackItem::WorkflowHints(WdlWorkflowHints::new()));
        self.visit_children(ctx);
        let hints = self.drain_while_workflow_hint();
        match self.stack.last_mut() {
            Some(StackItem::WorkflowHints(wh)) => wh.elements = hints,
            _ => panic!("visit_hintsSectionWorkflow: WorkflowHints not on stack"),
        }
    }

    fn visit_hintsItemWorkflow(&mut self, ctx: &HintsItemWorkflowContext<'input>) {
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::WorkflowHint(WdlWorkflowHint::with_value(key, value)));
    }

    fn visit_workflowHintValueObject(&mut self, ctx: &WorkflowHintValueObjectContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_workflowHintValueArray(&mut self, ctx: &WorkflowHintValueArrayContext<'input>) {
        self.visit_children(ctx);
        let entries = self.drain_while_expr();
        self.stack
            .push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    fn visit_hintsObjectWorkflow(&mut self, ctx: &HintsObjectWorkflowContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_hintsObjectItemWorkflow(&mut self, ctx: &HintsObjectItemWorkflowContext<'input>) {
        let key = ctx
            .dottedIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_workflowHintsArray(&mut self, ctx: &WorkflowHintsArrayContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let entries = self.drain_expr_above(sentinel);
        self.stack
            .push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    fn visit_workflowMetadataSection(&mut self, ctx: &WorkflowMetadataSectionContext<'input>) {
        self.visit_children(ctx);
        let meta = self.pop_metadata();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Meta(meta));
    }

    fn visit_workflowParameterMetadataSection(
        &mut self,
        ctx: &WorkflowParameterMetadataSectionContext<'input>,
    ) {
        self.visit_children(ctx);
        let pm = self.pop_parameter_metadata();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::ParameterMeta(pm));
    }

    fn visit_workflowDeclaration(&mut self, ctx: &WorkflowDeclarationContext<'input>) {
        self.visit_children(ctx);
        let decls = self.drain_while_bound_decl();
        let idx = self.find_workflow_idx();
        for d in decls {
            self.workflow_at_mut(idx)
                .elements
                .push(WdlWorkflowElement::BoundDeclaration(d));
        }
    }

    // =========================================================================
    // Call Statements
    // =========================================================================

    fn visit_workflowCallStatement(&mut self, ctx: &WorkflowCallStatementContext<'input>) {
        self.visit_children(ctx);
        let call = self.pop_call();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Call(call));
    }

    fn visit_callStatement(&mut self, ctx: &CallStatementContext<'input>) {
        self.stack.push(StackItem::Call(WdlCall::new()));
        self.visit_children(ctx);
        // call stays on stack; parent (visit_workflowCallStatement or scatter/conditional
        // via visit_workflowStatement) pops it
    }

    fn visit_callTarget(&mut self, ctx: &CallTargetContext<'input>) {
        let idx = self.find_call_idx();
        for id in ctx.strictIdentifier_all() {
            self.call_at_mut(idx)
                .target_path
                .push(id.get_text().to_owned());
        }
    }

    fn visit_callAlias(&mut self, ctx: &CallAliasContext<'input>) {
        let alias = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned());
        let idx = self.find_call_idx();
        self.call_at_mut(idx).alias = alias;
    }

    fn visit_callAfterClause(&mut self, ctx: &CallAfterClauseContext<'input>) {
        let dep = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let idx = self.find_call_idx();
        self.call_at_mut(idx).after_dependencies.push(dep);
    }

    fn visit_callInputBlock(&mut self, ctx: &CallInputBlockContext<'input>) {
        let legacy = ctx.KEYWORD_INPUT().is_some();
        let idx = self.find_call_idx();
        self.call_at_mut(idx).legacy_input_colon_used = legacy;
        self.visit_children(ctx);
        let inputs = self.drain_while_call_input();
        let idx = self.find_call_idx();
        self.call_at_mut(idx).inputs = inputs;
    }

    fn visit_callInputItem(&mut self, ctx: &CallInputItemContext<'input>) {
        self.visit_children(ctx);
        let value = if ctx.expression().is_some() {
            Some(self.pop_expr())
        } else {
            None
        };
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let ci = if let Some(v) = value {
            WdlCallInput::with_value(key, v)
        } else {
            WdlCallInput::new(key)
        };
        self.stack.push(StackItem::CallInput(ci));
    }

    // =========================================================================
    // Workflow Statement routing
    // =========================================================================

    fn visit_workflowStatement(&mut self, ctx: &WorkflowStatementContext<'input>) {
        self.visit_children(ctx);
        // Wrap whatever the child left on the stack into a WdlStatement
        let stmt = match self.stack.pop() {
            Some(StackItem::Call(c)) => WdlStatement::Call(c),
            Some(StackItem::Scatter(s)) => WdlStatement::Scatter(s),
            Some(StackItem::Conditional(c)) => WdlStatement::Conditional(c),
            Some(StackItem::Declaration(d)) => WdlStatement::Declaration(d),
            Some(StackItem::BoundDeclaration(d)) => WdlStatement::BoundDeclaration(d),
            other => panic!(
                "visit_workflowStatement: unexpected item on stack: {}",
                stack_item_name(&other)
            ),
        };
        self.stack.push(StackItem::Statement(stmt));
    }

    // =========================================================================
    // Conditional Statements
    // =========================================================================

    fn visit_workflowConditionalStatement(
        &mut self,
        ctx: &WorkflowConditionalStatementContext<'input>,
    ) {
        self.visit_children(ctx);
        let cond = self.pop_conditional();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Conditional(cond));
    }

    fn visit_conditionalStatement(&mut self, ctx: &ConditionalStatementContext<'input>) {
        // Push conditional sentinel with a dummy condition; condition is set after children
        self.stack.push(StackItem::Conditional(WdlConditional::new(
            WdlExpression::NullLit,
        )));
        self.visit_children(ctx);
        let n = ctx.workflowStatement_all().len();
        let then_stmts = self.drain_n_statements(n);
        let condition = self.pop_expr();
        match self.stack.last_mut() {
            Some(StackItem::Conditional(c)) => {
                c.condition = condition;
                c.then_statements = then_stmts;
            }
            _ => panic!("visit_conditionalStatement: Conditional not on stack"),
        }
    }

    fn visit_conditionalElseIfClause(&mut self, ctx: &ConditionalElseIfClauseContext<'input>) {
        self.visit_children(ctx);
        let n = ctx.workflowStatement_all().len();
        let then_stmts = self.drain_n_statements(n);
        let condition = self.pop_expr();
        let elif = WdlConditionalElseIf {
            condition,
            then_statements: then_stmts,
        };
        let idx = self.find_conditional_idx();
        self.conditional_at_mut(idx).else_ifs.push(elif);
    }

    fn visit_conditionalElseClause(&mut self, ctx: &ConditionalElseClauseContext<'input>) {
        self.visit_children(ctx);
        let n = ctx.workflowStatement_all().len();
        let else_stmts = self.drain_n_statements(n);
        let idx = self.find_conditional_idx();
        self.conditional_at_mut(idx).else_statements = else_stmts;
    }

    // =========================================================================
    // Scatter Statements
    // =========================================================================

    fn visit_workflowScatterStatement(
        &mut self,
        ctx: &WorkflowScatterStatementContext<'input>,
    ) {
        self.visit_children(ctx);
        let scatter = self.pop_scatter();
        let idx = self.find_workflow_idx();
        self.workflow_at_mut(idx)
            .elements
            .push(WdlWorkflowElement::Scatter(scatter));
    }

    fn visit_scatterStatement(&mut self, ctx: &ScatterStatementContext<'input>) {
        let var_name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        // Push sentinel with dummy collection; filled after children
        self.stack.push(StackItem::Scatter(WdlScatter::new(
            var_name,
            WdlExpression::NullLit,
        )));
        self.visit_children(ctx);
        let collection = self.pop_expr();
        match self.stack.last_mut() {
            Some(StackItem::Scatter(s)) => s.collection = collection,
            _ => panic!("visit_scatterStatement: Scatter not on stack"),
        }
    }

    fn visit_scatterBody(&mut self, ctx: &ScatterBodyContext<'input>) {
        self.visit_children(ctx);
        let n = ctx.workflowStatement_all().len();
        let stmts = self.drain_n_statements(n);
        let idx = self.find_scatter_idx();
        self.scatter_at_mut(idx).statements = stmts;
    }

    // =========================================================================
    // Primary Expression — promotes StringLiteral → Expr in expression context
    // =========================================================================

    fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) {
        self.visit_children(ctx);
        // If visiting children left a StringLiteral on the stack, it means a
        // quoted or multiline string appeared in expression position.  Promote
        // it to StackItem::Expr so that expression consumers see it correctly.
        if matches!(self.stack.last(), Some(StackItem::StringLiteral(_))) {
            let lit = self.pop_string_literal();
            self.stack.push(StackItem::Expr(WdlExpression::StrLit(lit)));
        }
    }

    // =========================================================================
    // String Literals & Placeholders
    // =========================================================================

    fn visit_quotedString(&mut self, ctx: &QuotedStringContext<'input>) {
        let delim = if ctx.SINGLE_QUOTE().is_some() {
            StringDelimiter::SingleQuote
        } else {
            StringDelimiter::DoubleQuote
        };
        self.stack
            .push(StackItem::StringLiteral(WdlStringLiteral::new(delim)));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        match self.stack.last_mut() {
            Some(StackItem::StringLiteral(lit)) => lit.components = components,
            _ => panic!("visit_quotedString: StringLiteral not on stack"),
        }
    }

    fn visit_multilineString(&mut self, ctx: &MultilineStringContext<'input>) {
        self.stack.push(StackItem::StringLiteral(WdlStringLiteral::new(
            StringDelimiter::Multiline,
        )));
        self.visit_children(ctx);
        let components = self.drain_while_string_component();
        match self.stack.last_mut() {
            Some(StackItem::StringLiteral(lit)) => lit.components = components,
            _ => panic!("visit_multilineString: StringLiteral not on stack"),
        }
    }

    fn visit_stringElementText(&mut self, ctx: &StringElementTextContext<'input>) {
        if let Some(tok) = ctx.STRING_TEXT() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_stringElementEscape(&mut self, ctx: &StringElementEscapeContext<'input>) {
        if let Some(tok) = ctx.STRING_ESCAPE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Escape(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_stringElementDollarSign(&mut self, ctx: &StringElementDollarSignContext<'input>) {
        if let Some(tok) = ctx.STRING_DOLLAR_SIGN() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_stringElementTilde(&mut self, ctx: &StringElementTildeContext<'input>) {
        if let Some(tok) = ctx.STRING_TILDE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_stringPlaceholder(&mut self, ctx: &StringPlaceholderContext<'input>) {
        let symbol = match ctx.STRING_PLACEHOLDER_START().map(|t| t.get_text().to_owned()).as_deref() {
            Some("~{") => PlaceholderSymbol::Tilde,
            Some("${") => PlaceholderSymbol::Dollar,
            other => panic!("visit_stringPlaceholder: unknown symbol {:?}", other),
        };
        self.visit_children(ctx);
        // After visiting children, top is expression; optionally a PlaceholderOption above
        let expression = self.pop_expr();
        let option = if matches!(self.stack.last(), Some(StackItem::PlaceholderOption(_))) {
            Some(Box::new(self.pop_placeholder_option()))
        } else {
            None
        };
        self.stack
            .push(StackItem::StringComponent(WdlStringComponent::Placeholder {
                symbol,
                option,
                expression: Box::new(expression),
            }));
    }

    fn visit_stringPlaceholderOptionSepDefault(
        &mut self,
        ctx: &StringPlaceholderOptionSepDefaultContext<'input>,
    ) {
        self.visit_children(ctx);
        let value = self.pop_string_literal();
        let kw = ctx
            .IDENTIFIER()
            .map(|t| t.get_text().to_owned())
            .unwrap_or_default();
        let opt = match kw.as_str() {
            "sep" => WdlStringPlaceholderOption::Sep(value),
            "default" => WdlStringPlaceholderOption::Default(value),
            other => panic!("visit_stringPlaceholderOptionSepDefault: unknown keyword {other}"),
        };
        self.stack.push(StackItem::PlaceholderOption(opt));
    }

    fn visit_stringPlaceholderOptionTrueFalse(
        &mut self,
        ctx: &StringPlaceholderOptionTrueFalseContext<'input>,
    ) {
        self.visit_children(ctx);
        let false_value = self.pop_string_literal();
        let true_value = self.pop_string_literal();
        self.stack.push(StackItem::PlaceholderOption(
            WdlStringPlaceholderOption::TrueFalse {
                true_value,
                false_value,
            },
        ));
    }

    fn visit_stringPlaceholderOptionFalseTrue(
        &mut self,
        ctx: &StringPlaceholderOptionFalseTrueContext<'input>,
    ) {
        self.visit_children(ctx);
        let true_value = self.pop_string_literal();
        let false_value = self.pop_string_literal();
        self.stack.push(StackItem::PlaceholderOption(
            WdlStringPlaceholderOption::FalseTrue {
                false_value,
                true_value,
            },
        ));
    }

    fn visit_multilineStringElementText(
        &mut self,
        ctx: &MultilineStringElementTextContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_TEXT() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringElementEscape(
        &mut self,
        ctx: &MultilineStringElementEscapeContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_ESCAPE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Escape(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringElementDoubleCloseAngle(
        &mut self,
        ctx: &MultilineStringElementDoubleCloseAngleContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_DOUBLE_CLOSE_ANGLE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringElementSingleCloseAngle(
        &mut self,
        ctx: &MultilineStringElementSingleCloseAngleContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_SINGLE_CLOSE_ANGLE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Text(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringElementDollarSign(
        &mut self,
        ctx: &MultilineStringElementDollarSignContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_DOLLAR_SIGN() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringElementTilde(
        &mut self,
        ctx: &MultilineStringElementTildeContext<'input>,
    ) {
        if let Some(tok) = ctx.MULTILINE_STRING_TILDE() {
            self.stack
                .push(StackItem::StringComponent(WdlStringComponent::Special(
                    tok.get_text().to_owned(),
                )));
        }
    }

    fn visit_multilineStringPlaceholder(
        &mut self,
        ctx: &MultilineStringPlaceholderContext<'input>,
    ) {
        let symbol = if ctx.MULTILINE_STRING_TILDE_PLACEHOLDER_START().is_some() {
            PlaceholderSymbol::Tilde
        } else if ctx.MULTILINE_STRING_DOLLAR_PLACEHOLDER_START().is_some() {
            PlaceholderSymbol::Dollar
        } else {
            panic!("visit_multilineStringPlaceholder: unknown placeholder symbol");
        };
        self.visit_children(ctx);
        // Pop expression first (it's on top), then optionally a PlaceholderOption below it.
        let expression = self.pop_expr();
        let option = if matches!(self.stack.last(), Some(StackItem::PlaceholderOption(_))) {
            Some(Box::new(self.pop_placeholder_option()))
        } else {
            None
        };
        self.stack
            .push(StackItem::StringComponent(WdlStringComponent::Placeholder {
                symbol,
                option,
                expression: Box::new(expression),
            }));
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    fn visit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) {
        self.stack.push(StackItem::Expr(WdlExpression::NullLit));
    }

    fn visit_noneLiteral(&mut self, _ctx: &NoneLiteralContext<'input>) {
        self.stack.push(StackItem::Expr(WdlExpression::NullLit));
    }

    fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) {
        let value = ctx.KEYWORD_TRUE().is_some();
        self.stack
            .push(StackItem::Expr(WdlExpression::BoolLit(value)));
    }

    fn visit_numberLiteralInt(&mut self, ctx: &NumberLiteralIntContext<'input>) {
        let text = ctx
            .INTEGER()
            .map(|t| t.get_text().to_owned())
            .unwrap_or_default();
        // handle octal (0-prefix) and hex (0x-prefix)
        let value: i64 = if text.starts_with("0x") || text.starts_with("0X") {
            i64::from_str_radix(&text[2..], 16).unwrap_or(0)
        } else if text.len() > 1 && text.starts_with('0') {
            i64::from_str_radix(&text[1..], 8).unwrap_or(0)
        } else {
            text.parse().unwrap_or(0)
        };
        self.stack.push(StackItem::Expr(WdlExpression::IntLit(value)));
    }

    fn visit_numberLiteralFloat(&mut self, ctx: &NumberLiteralFloatContext<'input>) {
        let text = ctx
            .FLOAT()
            .map(|t| t.get_text().to_owned())
            .unwrap_or_default();
        let value: f64 = text.parse().unwrap_or(0.0);
        self.stack
            .push(StackItem::Expr(WdlExpression::FloatLit(value)));
    }

    fn visit_numberLiteralSigned(&mut self, ctx: &NumberLiteralSignedContext<'input>) {
        self.visit_children(ctx);
        if ctx.MINUS().is_some() {
            // negate the top expression in place
            match self.stack.last_mut() {
                Some(StackItem::Expr(WdlExpression::IntLit(ref mut v))) => *v = v.wrapping_neg(),
                Some(StackItem::Expr(WdlExpression::FloatLit(ref mut v))) => *v = -*v,
                _ => panic!("visit_numberLiteralSigned: top of stack is not a number literal"),
            }
        }
    }

    fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        let entries = self.drain_expr_above(sentinel);
        self.stack
            .push(StackItem::Expr(WdlExpression::ArrayLit(WdlArrayLiteral { entries })));
    }

    fn visit_mapLiteral(&mut self, ctx: &MapLiteralContext<'input>) {
        let sentinel = self.stack.len();
        self.visit_children(ctx);
        // drain_expr_above returns all exprs in source order: key0, val0, key1, val1, ...
        let exprs = self.drain_expr_above(sentinel);
        let entries = exprs
            .chunks(2)
            .filter_map(|pair| {
                if pair.len() == 2 {
                    Some(WdlMapEntry {
                        key: pair[0].clone(),
                        value: Some(pair[1].clone()),
                    })
                } else {
                    None
                }
            })
            .collect();
        self.stack
            .push(StackItem::Expr(WdlExpression::MapLit(WdlMapLiteral { entries })));
    }

    // visit_mapLiteralItem: default visit_children pushes key then value as Expr items

    fn visit_objectLiteral(&mut self, ctx: &ObjectLiteralContext<'input>) {
        self.visit_children(ctx);
        let obj = self.collect_metadata_entries_as_obj_lit();
        self.stack.push(StackItem::Expr(WdlExpression::ObjLit(obj)));
    }

    fn visit_objectLiteralItem(&mut self, ctx: &ObjectLiteralItemContext<'input>) {
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        self.stack
            .push(StackItem::MetadataEntry(WdlMetadataEntry::with_value(key, value)));
    }

    fn visit_structLiteral(&mut self, ctx: &StructLiteralContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Expr(WdlExpression::StructLit(
            WdlStructLiteral { name, entries: Vec::new() },
        )));
        self.visit_children(ctx);
        // entries are added inline by visit_structLiteralItem
    }

    fn visit_structLiteralItem(&mut self, ctx: &StructLiteralItemContext<'input>) {
        let key = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.visit_children(ctx);
        let value = self.pop_expr();
        let entry = WdlStructEntry { key, value: Some(value) };
        for item in self.stack.iter_mut().rev() {
            if let StackItem::Expr(WdlExpression::StructLit(sl)) = item {
                sl.entries.push(entry);
                return;
            }
        }
        panic!("visit_structLiteralItem: no StructLit sentinel on stack");
    }

    fn visit_pairLiteral(&mut self, ctx: &PairLiteralContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::PairLit(Box::new(
            WdlPairLiteral {
                left: Box::new(left),
                right: Box::new(right),
            },
        ))));
    }

    // visit_groupedExpression: default visit_children — inner expression stays on stack

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
        let name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Expr(WdlExpression::Variable(name)));
    }

    fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) {
        let fn_name = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        let sentinel_depth = self.stack.len();
        self.visit_children(ctx);
        // pop all Expr items pushed above the sentinel depth as arguments
        let mut args = Vec::new();
        while self.stack.len() > sentinel_depth {
            if matches!(self.stack.last(), Some(StackItem::Expr(_))) {
                args.push(self.pop_expr());
            } else {
                break;
            }
        }
        args.reverse(); // restore source order
        let mut func = WdlFunctionCallOperation::new(fn_name);
        func.arguments = args;
        self.stack.push(StackItem::Expr(WdlExpression::FuncOp(func)));
    }

    fn visit_ifExpression(&mut self, ctx: &IfExpressionContext<'input>) {
        self.visit_children(ctx);
        let false_value = self.pop_expr();
        let true_value = self.pop_expr();
        let condition = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::TernaryOp(Box::new(
            WdlTernaryOperation {
                condition: Box::new(condition),
                true_value: Box::new(true_value),
                false_value: Box::new(false_value),
            },
        ))));
    }

    // Binary operators

    fn visit_logicalOrExprOperation(&mut self, ctx: &LogicalOrExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_logicalAndExprOperation(&mut self, ctx: &LogicalAndExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_equalityExprOperation(&mut self, ctx: &EqualityExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        let op = if ctx.EQUAL().is_some() {
            BinaryOperator::Eq
        } else {
            BinaryOperator::Neq
        };
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_comparisonExprOperation(&mut self, ctx: &ComparisonExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        let op = if ctx.LESS().is_some() {
            BinaryOperator::Lt
        } else if ctx.LESS_EQUAL().is_some() {
            BinaryOperator::Lte
        } else if ctx.GREATER().is_some() {
            BinaryOperator::Gt
        } else if ctx.GREATER_EQUAL().is_some() {
            BinaryOperator::Gte
        } else {
            panic!("visit_comparisonExprOperation: unknown operator");
        };
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_additiveExprOperation(&mut self, ctx: &AdditiveExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        let op = if ctx.PLUS().is_some() {
            BinaryOperator::Add
        } else {
            BinaryOperator::Subtract
        };
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_multiplicativeExprOperation(
        &mut self,
        ctx: &MultiplicativeExprOperationContext<'input>,
    ) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        let op = if ctx.ASTERISK().is_some() {
            BinaryOperator::Multiply
        } else if ctx.SLASH().is_some() {
            BinaryOperator::Divide
        } else if ctx.PERCENT().is_some() {
            BinaryOperator::Modulo
        } else {
            panic!("visit_multiplicativeExprOperation: unknown operator");
        };
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_powerExprOperation(&mut self, ctx: &PowerExprOperationContext<'input>) {
        self.visit_children(ctx);
        let right = self.pop_expr();
        let left = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::BinaryOp(Box::new(
            WdlBinaryOperation {
                left: Box::new(left),
                operator: BinaryOperator::Power,
                right: Box::new(right),
            },
        ))));
    }

    fn visit_unaryExprOperation(&mut self, ctx: &UnaryExprOperationContext<'input>) {
        self.visit_children(ctx);
        let operand = self.pop_expr();
        let op = if ctx.MINUS().is_some() {
            UnaryOperator::Negative
        } else if ctx.EXCLAMATION().is_some() {
            UnaryOperator::Not
        } else {
            panic!("visit_unaryExprOperation: unknown operator");
        };
        self.stack.push(StackItem::Expr(WdlExpression::UnaryOp(Box::new(
            WdlUnaryOperation {
                operator: op,
                operand: Box::new(operand),
            },
        ))));
    }

    fn visit_postfixExprArrayIndex(&mut self, ctx: &PostfixExprArrayIndexContext<'input>) {
        self.visit_children(ctx);
        let index = self.pop_expr();
        let target = self.pop_expr();
        self.stack.push(StackItem::Expr(WdlExpression::IdxOp(Box::new(
            WdlIndexAccessOperation {
                target: Box::new(target),
                index: Box::new(index),
            },
        ))));
    }

    fn visit_postfixExprField(&mut self, ctx: &PostfixExprFieldContext<'input>) {
        self.visit_children(ctx);
        let target = self.pop_expr();
        let member = ctx
            .strictIdentifier()
            .map(|id| id.get_text().to_owned())
            .unwrap_or_default();
        self.stack.push(StackItem::Expr(WdlExpression::MemberOp(Box::new(
            WdlMemberAccessOperation {
                target: Box::new(target),
                member,
            },
        ))));
    }
}

// ============================================================================
// Utility: reconstruct the plain text of a string literal (for import URIs)
// ============================================================================

fn string_literal_to_text(lit: &WdlStringLiteral) -> String {
    let mut out = String::new();
    for comp in &lit.components {
        match comp {
            WdlStringComponent::Text(t) => out.push_str(t),
            WdlStringComponent::Escape(e) => out.push_str(e),
            WdlStringComponent::Special(s) => out.push_str(s),
            WdlStringComponent::Placeholder { .. } => {
                // Import URIs may not contain placeholders; skip
            }
        }
    }
    out
}
