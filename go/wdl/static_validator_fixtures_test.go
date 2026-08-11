package wdl

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

func TestStaticValidatorFixtures(t *testing.T) {
	root := filepath.Join("..", "..", "wdl_tests", "validator")

	tests := []struct {
		name        string
		fixture     string
		wantErrText string
	}{
		{name: "accepts simple valid workflow", fixture: "accepts_simple_valid_workflow.wdl"},
		{name: "static workflow structure bad", fixture: "static_workflow_structure_bad.wdl", wantErrText: "unknown type reference"},
		{name: "nested workflow structure bad", fixture: "nested_workflow_structure_bad.wdl", wantErrText: "duplicate declaration in scope"},
		{name: "function version invalid", fixture: "function_version_invalid.wdl", wantErrText: "not available before WDL 1.2"},
	}

	for _, tc := range tests {
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			resolver, err := NewDefaultResolver(ResolverConfig{})
			if err != nil {
				t.Fatalf("resolver init failed: %v", err)
			}
			loader := NewWdlV1Loader()
			validator := NewStaticValidator(SemanticValidatorConfig{})

			wdlPath := filepath.Join(root, tc.fixture)
			_, loadErr := loader.LoadFile(context.Background(), wdlPath, WithResolver(resolver), WithValidator(validator))
			if tc.wantErrText == "" {
				if loadErr != nil {
					t.Fatalf("expected success, got error: %v", loadErr)
				}
				return
			}
			if loadErr == nil {
				t.Fatalf("expected error containing %q", tc.wantErrText)
			}
			if !strings.Contains(loadErr.Error(), tc.wantErrText) {
				t.Fatalf("expected error containing %q, got: %v", tc.wantErrText, loadErr)
			}
		})
	}
}
