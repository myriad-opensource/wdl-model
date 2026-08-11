package wdl

import "context"

// LoadV1String parses WDL 1.x source text into a Document.
//
// Use options to enable import resolution, custom validators, and source
// location metadata.
func LoadV1String(ctx context.Context, source string, options ...LoadOption) (*Document, error) {
	return NewWdlV1Loader().LoadString(ctx, source, options...)
}

// LoadString parses WDL source text into a Document.
//
// Use options to enable import resolution, custom validators, and source
// location metadata.
func LoadString(ctx context.Context, source string, options ...LoadOption) (*Document, error) {
	return LoadV1String(ctx, source, options...)
}

// LoadV1File parses a WDL 1.x file into a Document.
//
// The loader sets SourceLocation to the absolute file path before resolving
// relative imports.
func LoadV1File(ctx context.Context, path string, options ...LoadOption) (*Document, error) {
	return NewWdlV1Loader().LoadFile(ctx, path, options...)
}

// LoadFile parses a WDL file into a Document.
//
// The loader sets SourceLocation to the absolute file path before resolving
// relative imports.
func LoadFile(ctx context.Context, path string, options ...LoadOption) (*Document, error) {
	return LoadV1File(ctx, path, options...)
}

// Validate runs a validator against a parsed document.
//
// If validator is nil, Validate uses SemanticValidator with default settings.
func Validate(ctx context.Context, doc *Document, validator Validator) error {
	if validator == nil {
		validator = NewSemanticValidator(SemanticValidatorConfig{})
	}
	return validator.Validate(ctx, doc)
}
