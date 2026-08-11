package wdl

import (
	"context"
	"path/filepath"
	"testing"
)

func TestDiagnosticParityGate(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}

	tests := []struct {
		name         string
		path         string
		validator    Validator
		withResolver bool
		wantCode     SemanticCode
	}{
		{
			name:         "function version contains_key gate",
			path:         filepath.Join("..", "..", "wdl_tests", "function_version_matrix", "v11_contains_key_fail.wdl"),
			validator:    NewStaticValidator(SemanticValidatorConfig{}),
			withResolver: true,
			wantCode:     CodeFunctionNotAvailableInVersion,
		},
		{
			name:         "static signature keys gate",
			path:         filepath.Join("..", "..", "wdl_tests", "static_function_signature_matrix", "keys_bad.wdl"),
			validator:    NewStaticValidator(SemanticValidatorConfig{}),
			withResolver: true,
			wantCode:     CodeInvalidFunctionArguments,
		},
		{
			name:         "type assignability none gate",
			path:         filepath.Join("..", "..", "wdl_tests", "type_assignability_matrix", "required_from_none_fail.wdl"),
			validator:    NewStaticValidator(SemanticValidatorConfig{}),
			withResolver: true,
			wantCode:     CodeTypeMismatch,
		},
		{
			name:         "operator semantics logical gate",
			path:         filepath.Join("..", "..", "wdl_tests", "expression_operator_semantics", "logical_operand_type_fail.wdl"),
			validator:    NewStaticValidator(SemanticValidatorConfig{}),
			withResolver: true,
			wantCode:     CodeTypeMismatch,
		},
		{
			name:         "import validation unknown member gate",
			path:         filepath.Join("..", "..", "wdl_tests", "import_validation", "unknown_member", "root.wdl"),
			validator:    NewSemanticValidator(SemanticValidatorConfig{}),
			withResolver: true,
			wantCode:     CodeUnknownReference,
		},
		{
			name:         "deprecation gate",
			path:         filepath.Join("..", "..", "wdl_tests", "deprecations", "runtime_section_deprecated.wdl"),
			validator:    NewLintingValidator(SemanticValidatorConfig{ThrowOnWarnings: true}),
			withResolver: false,
			wantCode:     CodeLintDeprecatedFeature,
		},
	}

	loader := NewWdlV1Loader()
	for _, tc := range tests {
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			opts := []LoadOption{WithValidator(tc.validator)}
			if tc.withResolver {
				opts = append(opts, WithResolver(resolver))
			} else {
				opts = append(opts, WithResolver(nil))
			}

			_, loadErr := loader.LoadFile(context.Background(), tc.path, opts...)
			if loadErr == nil {
				t.Fatalf("expected failure for fixture %s", tc.path)
			}

			codes := collectSemanticCodes(loadErr)
			if !hasSemanticCode(codes, tc.wantCode) {
				t.Fatalf("expected code %s in %v for fixture %s", tc.wantCode, codes, tc.path)
			}
		})
	}
}

func collectSemanticCodes(err error) []SemanticCode {
	ex, ok := err.(Exception)
	if !ok {
		return nil
	}
	codes := make([]SemanticCode, 0)
	for _, d := range ex.Diagnostics {
		se, ok := d.(SemanticError)
		if !ok {
			continue
		}
		codes = append(codes, se.CodeValue)
	}
	return codes
}

func hasSemanticCode(codes []SemanticCode, want SemanticCode) bool {
	for _, got := range codes {
		if got == want {
			return true
		}
	}
	return false
}
