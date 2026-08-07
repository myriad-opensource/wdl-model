//! WDL statement model (declarations, imports, calls, scatters, conditionals).

use crate::expressions::{WdlExpression, WdlStringLiteral};
use crate::types::WdlType;

/// High-level statement family for traversal and validation dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatementComponentType {
    Call,
    Conditional,
    Declaration,
    Scatter,
}

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

/// Unbound declaration: `[env] Type name`.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlDeclaration {
    pub wdl_type: WdlType,
    pub name: String,
    pub environment_variable: bool,
}

impl WdlDeclaration {
    pub fn new(wdl_type: WdlType, name: impl Into<String>) -> Self {
        Self {
            wdl_type,
            name: name.into(),
            environment_variable: false,
        }
    }
}

/// Bound declaration: `[env] Type name = expression`.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlBoundDeclaration {
    pub wdl_type: WdlType,
    pub name: String,
    pub environment_variable: bool,
    pub expression: WdlExpression,
}

impl WdlBoundDeclaration {
    pub fn new(wdl_type: WdlType, name: impl Into<String>, expression: WdlExpression) -> Self {
        Self {
            wdl_type,
            name: name.into(),
            environment_variable: false,
            expression,
        }
    }
}

// ---------------------------------------------------------------------------
// Imports
// ---------------------------------------------------------------------------

/// A single member in an import alias or use list.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlImportMember {
    pub member: String,
    pub alias: Option<String>,
}

impl WdlImportMember {
    pub fn new(member: impl Into<String>, alias: Option<String>) -> Self {
        Self {
            member: member.into(),
            alias,
        }
    }
}

/// Standard import: `import "source" [as alias] (alias Old as New)*`
#[derive(Debug, Clone, PartialEq)]
pub struct WdlImportStandard {
    pub source: WdlStringLiteral,
    pub source_text: String,
    pub import_identifier: Option<String>,
    pub alias: Option<String>,
    pub members: Vec<WdlImportMember>,
}

/// Star import: `import * from "source"`
#[derive(Debug, Clone, PartialEq)]
pub struct WdlImportStar {
    pub source: WdlStringLiteral,
    pub source_text: String,
    pub import_identifier: Option<String>,
}

/// Named member import: `import { A [as B], … } from "source"`
#[derive(Debug, Clone, PartialEq)]
pub struct WdlImportMembers {
    pub source: WdlStringLiteral,
    pub source_text: String,
    pub import_identifier: Option<String>,
    pub members: Vec<WdlImportMember>,
}

/// Any import form.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlImport {
    Standard(WdlImportStandard),
    Star(WdlImportStar),
    Members(WdlImportMembers),
}

impl WdlImport {
    pub fn source_text(&self) -> &str {
        match self {
            WdlImport::Standard(i) => &i.source_text,
            WdlImport::Star(i) => &i.source_text,
            WdlImport::Members(i) => &i.source_text,
        }
    }

    pub fn import_identifier(&self) -> Option<&str> {
        match self {
            WdlImport::Standard(i) => i.import_identifier.as_deref(),
            WdlImport::Star(i) => i.import_identifier.as_deref(),
            WdlImport::Members(i) => i.import_identifier.as_deref(),
        }
    }

    pub fn set_import_identifier(&mut self, id: String) {
        match self {
            WdlImport::Standard(i) => i.import_identifier = Some(id),
            WdlImport::Star(i) => i.import_identifier = Some(id),
            WdlImport::Members(i) => i.import_identifier = Some(id),
        }
    }
}

// ---------------------------------------------------------------------------
// Call
// ---------------------------------------------------------------------------

/// A single `name = expression` binding in a call input block.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlCallInput {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlCallInput {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// Workflow call statement.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlCall {
    /// Dotted path segments, e.g. `["lib", "task_name"]`.
    pub target_path: Vec<String>,
    pub alias: Option<String>,
    pub inputs: Vec<WdlCallInput>,
    pub after_dependencies: Vec<String>,
    pub legacy_input_colon_used: bool,
}

impl WdlCall {
    pub fn new() -> Self {
        Self {
            target_path: Vec::new(),
            alias: None,
            inputs: Vec::new(),
            after_dependencies: Vec::new(),
            legacy_input_colon_used: false,
        }
    }

    /// Returns the dotted target path joined by `.`.
    pub fn target_path_as_string(&self) -> String {
        self.target_path.join(".")
    }
}

impl Default for WdlCall {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Scatter
// ---------------------------------------------------------------------------

/// `scatter (name in collection) { … }` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlScatter {
    pub name: String,
    pub collection: WdlExpression,
    pub statements: Vec<WdlStatement>,
}

impl WdlScatter {
    pub fn new(name: impl Into<String>, collection: WdlExpression) -> Self {
        Self {
            name: name.into(),
            collection,
            statements: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Conditional
// ---------------------------------------------------------------------------

/// A single `else if (condition) { … }` branch.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlConditionalElseIf {
    pub condition: WdlExpression,
    pub then_statements: Vec<WdlStatement>,
}

/// `if (condition) { … } [else if …]* [else { … }]` statement.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlConditional {
    pub condition: WdlExpression,
    pub then_statements: Vec<WdlStatement>,
    pub else_ifs: Vec<WdlConditionalElseIf>,
    pub else_statements: Vec<WdlStatement>,
}

impl WdlConditional {
    pub fn new(condition: WdlExpression) -> Self {
        Self {
            condition,
            then_statements: Vec::new(),
            else_ifs: Vec::new(),
            else_statements: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// WdlStatement — top-level statement enum
// ---------------------------------------------------------------------------

/// Any statement that can appear inside a workflow or scatter/conditional body.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlStatement {
    Declaration(WdlDeclaration),
    BoundDeclaration(WdlBoundDeclaration),
    Call(WdlCall),
    Scatter(WdlScatter),
    Conditional(WdlConditional),
}

impl WdlStatement {
    pub fn component_type(&self) -> StatementComponentType {
        match self {
            WdlStatement::Declaration(_) | WdlStatement::BoundDeclaration(_) => {
                StatementComponentType::Declaration
            }
            WdlStatement::Call(_) => StatementComponentType::Call,
            WdlStatement::Scatter(_) => StatementComponentType::Scatter,
            WdlStatement::Conditional(_) => StatementComponentType::Conditional,
        }
    }
}
