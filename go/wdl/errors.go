package wdl

import (
	"fmt"
	"strings"
)

// Severity defines diagnostic severity.
type Severity string

const (
	SeverityWarning Severity = "WARNING"
	SeverityError   Severity = "ERROR"
)

// SemanticCode identifies semantic and lint diagnostics.
type SemanticCode string

const (
	CodeGenericSemanticError          SemanticCode = "GENERIC_SEMANTIC_ERROR"
	CodeFunctionNotAvailableInVersion SemanticCode = "FUNCTION_NOT_AVAILABLE_IN_VERSION"
	CodeDuplicateDefinition           SemanticCode = "DUPLICATE_DEFINITION"
	CodeUnknownReference              SemanticCode = "UNKNOWN_REFERENCE"
	CodeTypeMismatch                  SemanticCode = "TYPE_MISMATCH"
	CodeInvalidFunctionArguments      SemanticCode = "INVALID_FUNCTION_ARGUMENTS"
	CodeLintUnusedWorkflowDeclaration SemanticCode = "LINT_UNUSED_WORKFLOW_DECLARATION"
	CodeLintUnusedTaskDeclaration     SemanticCode = "LINT_UNUSED_TASK_DECLARATION"
	CodeLintUnusedScatterVariable     SemanticCode = "LINT_UNUSED_SCATTER_VARIABLE"
	CodeLintUnusedCallOutput          SemanticCode = "LINT_UNUSED_CALL_OUTPUT"
	CodeLintUnusedImport              SemanticCode = "LINT_UNUSED_IMPORT"
	CodeLintDeprecatedFeature         SemanticCode = "LINT_DEPRECATED_FEATURE"
)

// Diagnostic represents a syntax or semantic issue.
type Diagnostic interface {
	error
	Line() int
	Column() int
	DebugMessage() string
}

// SyntaxError is a parser-level error.
type SyntaxError struct {
	Message string
	AtLine  int
	AtCol   int
}

func (e SyntaxError) Error() string { return e.DebugMessage() }
func (e SyntaxError) Line() int     { return e.AtLine }
func (e SyntaxError) Column() int   { return e.AtCol }
func (e SyntaxError) DebugMessage() string {
	return fmt.Sprintf("WdlSyntaxError:%d:%d:%s", e.AtLine, e.AtCol, e.Message)
}

// SemanticError is a semantic or lint validation diagnostic.
type SemanticError struct {
	CodeValue SemanticCode
	Severity  Severity
	Message   string
	AtLine    int
	AtCol     int
}

func (e SemanticError) Error() string { return e.DebugMessage() }
func (e SemanticError) Line() int     { return e.AtLine }
func (e SemanticError) Column() int   { return e.AtCol }
func (e SemanticError) DebugMessage() string {
	return fmt.Sprintf("WdlSemanticError:%d:%d:%s:%s:%s", e.AtLine, e.AtCol, e.CodeValue, e.Severity, e.Message)
}

// Exception is an aggregate diagnostic error.
type Exception struct {
	Diagnostics []Diagnostic
}

func (e Exception) Error() string {
	parts := make([]string, 0, len(e.Diagnostics))
	for _, d := range e.Diagnostics {
		parts = append(parts, d.DebugMessage())
	}
	return strings.Join(parts, "\n")
}

// HasErrors returns true when at least one ERROR-severity semantic diagnostic exists,
// or any syntax diagnostic is present.
func (e Exception) HasErrors() bool {
	for _, d := range e.Diagnostics {
		s, ok := d.(SemanticError)
		if !ok {
			return true
		}
		if s.Severity == SeverityError {
			return true
		}
	}
	return false
}
