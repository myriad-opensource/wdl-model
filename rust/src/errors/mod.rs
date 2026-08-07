//! WDL error and diagnostic model.

use thiserror::Error;

/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Warning,
    Error,
}

/// Stable semantic and lint diagnostic codes emitted by this library.
/// Mirrors `WdlSemanticError.Code` in the Java implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WdlErrorCode {
    GenericSemanticError,
    FunctionNotAvailableInVersion,
    DuplicateDefinition,
    UnknownReference,
    TypeMismatch,
    InvalidFunctionArguments,
    LintDeprecatedFeature,
    LintUnusedWorkflowDeclaration,
    LintUnusedTaskDeclaration,
    LintUnusedScatterVariable,
    LintUnusedCallOutput,
}

impl WdlErrorCode {
    /// Returns the severity implied by this error code.
    pub fn severity(self) -> Severity {
        match self {
            WdlErrorCode::GenericSemanticError
            | WdlErrorCode::FunctionNotAvailableInVersion
            | WdlErrorCode::DuplicateDefinition
            | WdlErrorCode::UnknownReference
            | WdlErrorCode::TypeMismatch
            | WdlErrorCode::InvalidFunctionArguments => Severity::Error,

            WdlErrorCode::LintDeprecatedFeature
            | WdlErrorCode::LintUnusedWorkflowDeclaration
            | WdlErrorCode::LintUnusedTaskDeclaration
            | WdlErrorCode::LintUnusedScatterVariable
            | WdlErrorCode::LintUnusedCallOutput => Severity::Warning,
        }
    }
}

/// A semantic or lint diagnostic with a stable code and source location.
/// Mirrors `WdlSemanticError` in the Java implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlSemanticError {
    pub code: WdlErrorCode,
    pub message: String,
    pub line: i32,
    pub char_position_in_line: i32,
}

impl WdlSemanticError {
    pub fn new(
        code: WdlErrorCode,
        message: impl Into<String>,
        line: i32,
        char_position_in_line: i32,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            line,
            char_position_in_line,
        }
    }

    pub fn generic(message: impl Into<String>, line: i32, char_position_in_line: i32) -> Self {
        Self::new(
            WdlErrorCode::GenericSemanticError,
            message,
            line,
            char_position_in_line,
        )
    }

    /// Returns the severity implied by this diagnostic's code.
    pub fn severity(&self) -> Severity {
        self.code.severity()
    }

    pub fn to_debug_message(&self) -> String {
        format!(
            "WdlSemanticError:{}:{}:{}:{:?}:{:?}",
            self.line,
            self.char_position_in_line,
            self.message,
            self.code,
            self.severity()
        )
    }
}

impl std::fmt::Display for WdlSemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] {:?}: {}",
            self.line, self.char_position_in_line, self.code, self.message
        )
    }
}

/// A syntax diagnostic produced while lexing or parsing WDL source.
/// Mirrors `WdlSyntaxError` in the Java implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlSyntaxError {
    pub message: String,
    pub line: i32,
    pub char_position_in_line: i32,
}

impl WdlSyntaxError {
    pub fn new(message: impl Into<String>, line: i32, char_position_in_line: i32) -> Self {
        Self {
            message: message.into(),
            line,
            char_position_in_line,
        }
    }

    pub fn to_debug_message(&self) -> String {
        format!(
            "WdlSyntaxError:{}:{}:{}",
            self.line, self.char_position_in_line, self.message
        )
    }
}

impl std::fmt::Display for WdlSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] syntax error: {}",
            self.line, self.char_position_in_line, self.message
        )
    }
}

/// Top-level error type returned by loading and validation operations.
/// Mirrors the checked exception types (`WdlException`, `WdlImportException`) in Java.
#[derive(Debug, Error)]
pub enum WdlError {
    /// One or more syntax errors were encountered during parsing.
    #[error("WDL syntax errors: {0:?}")]
    Syntax(Vec<WdlSyntaxError>),

    /// One or more semantic errors were produced during validation.
    #[error("WDL semantic errors: {0:?}")]
    Semantic(Vec<WdlSemanticError>),

    /// A failure occurred while resolving or loading an imported document.
    #[error("WDL import error at '{location}': {message}")]
    Import { message: String, location: String },

    /// An I/O error occurred while reading a WDL source file.
    #[error("WDL I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl WdlError {
    pub fn import(message: impl Into<String>, location: impl Into<String>) -> Self {
        WdlError::Import {
            message: message.into(),
            location: location.into(),
        }
    }
}
