package wdl

import "github.com/antlr4-go/antlr/v4"

// DeclarationKind indicates the top-level declaration category.
type DeclarationKind string

const (
	DeclarationTask     DeclarationKind = "task"
	DeclarationWorkflow DeclarationKind = "workflow"
	DeclarationStruct   DeclarationKind = "struct"
	DeclarationEnum     DeclarationKind = "enum"
)

// Declaration is a top-level named item in a WDL document.
type Declaration struct {
	// Kind is task, workflow, struct, or enum.
	Kind DeclarationKind
	// Name is the declaration identifier.
	Name string
	// Line is the 1-based source line of the first token.
	Line int
	// Column is the 0-based source column of the first token.
	Column int
	// EndLine is the 1-based source line of the last token.
	EndLine int
	// EndColumn is the 0-based exclusive-end column of the last token.
	EndColumn int
}

// ImportMember represents a selected import member and optional alias.
type ImportMember struct {
	Name  string
	Alias string
}

// ImportRecord stores resolved import source details for recursive loading.
type ImportRecord struct {
	// Line is the 1-based source line of the import statement.
	Line int
	// Column is the 0-based source column of the import statement.
	Column int
	// EndLine is the 1-based source line of the last token of the import statement.
	EndLine int
	// EndColumn is the 0-based exclusive-end column of the last token.
	EndColumn int
	// RawLocation is the location text as written in WDL source.
	RawLocation string
	// NamespaceAlias is the alias used by standard imports.
	NamespaceAlias string
	// Aliases lists member imports and optional renames.
	Aliases []ImportMember
	// ImportAllMembers indicates a star import.
	ImportAllMembers bool
	// ResolvedLocation is the final resolver location identifier.
	ResolvedLocation string
	// SourceText is the loaded content of the imported document.
	SourceText string
}

// Document is the parsed root model plus import graph metadata.
type Document struct {
	// Version is the parsed WDL language version.
	Version Version
	// SourceLocation identifies where the document came from.
	SourceLocation string
	// RawSource is the full source text used for parsing.
	RawSource string
	// ParseTree is the ANTLR parse tree for advanced traversal use cases.
	ParseTree antlr.Tree
	// Declarations are top-level model declarations in source order.
	Declarations []Declaration
	// ImportedDocs maps resolved import locations to loaded documents.
	ImportedDocs map[string]*Document
	// ImportStatements contains parsed import metadata in source order.
	ImportStatements []ImportRecord
}
