package wdl

import "context"

// LoadString parses WDL source text into a Document.
//
// Use options to enable import resolution, custom validators, and source
// location metadata.
func LoadString(ctx context.Context, source string, options ...LoadOption) (*Document, error) {
	return NewLoader().LoadString(ctx, source, options...)
}

// LoadFile parses a WDL file into a Document.
//
// The loader sets SourceLocation to the absolute file path before resolving
// relative imports.
func LoadFile(ctx context.Context, path string, options ...LoadOption) (*Document, error) {
	return NewLoader().LoadFile(ctx, path, options...)
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
