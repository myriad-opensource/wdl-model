package wdl

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

func TestDeprecationFixtures(t *testing.T) {
	root := filepath.Join("..", "..", "wdl_tests", "deprecations")
	tests := []struct {
		file     string
		wantWarn bool
	}{
		{file: "runtime_section_deprecated.wdl", wantWarn: true},
		{file: "object_type_deprecated.wdl", wantWarn: true},
		{file: "placeholder_options_deprecated.wdl", wantWarn: true},
		{file: "file_scheme_import_deprecated.wdl", wantWarn: true},
		{file: "no_deprecations.wdl", wantWarn: false},
	}

	loader := NewLoader()
	validator := NewLintingValidator(SemanticValidatorConfig{ThrowOnWarnings: true})

	for _, tc := range tests {
		tc := tc
		t.Run(tc.file, func(t *testing.T) {
			path := filepath.Join(root, tc.file)
			_, loadErr := loader.LoadFile(context.Background(), path, WithResolver(nil), WithValidator(validator))
			if tc.wantWarn {
				if loadErr == nil {
					t.Fatalf("expected warning/error for %s", tc.file)
				}
				if !strings.Contains(loadErr.Error(), string(CodeLintDeprecatedFeature)) {
					t.Fatalf("expected deprecated-feature code for %s, got: %v", tc.file, loadErr)
				}
				return
			}
			if loadErr != nil {
				t.Fatalf("expected no warning for %s, got: %v", tc.file, loadErr)
			}
		})
	}
}
