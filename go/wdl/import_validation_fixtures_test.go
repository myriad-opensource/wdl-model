package wdl

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

func TestImportValidationFixtures(t *testing.T) {
	root := filepath.Join("..", "..", "wdl_tests", "import_validation")

	tests := []struct {
		name        string
		relRootWDL  string
		wantErrText string
	}{
		{name: "standard alias ok", relRootWDL: filepath.Join("standard_alias", "root.wdl")},
		{name: "star members ok", relRootWDL: filepath.Join("star_members", "root.wdl")},
		{name: "bad alias fails", relRootWDL: filepath.Join("bad_alias", "root.wdl"), wantErrText: "unknown imported symbol"},
		{name: "unknown member fails", relRootWDL: filepath.Join("unknown_member", "root.wdl"), wantErrText: "unknown imported symbol"},
		{name: "duplicate namespace fails", relRootWDL: filepath.Join("duplicate_namespace", "root.wdl"), wantErrText: "duplicate import namespace alias"},
		{name: "version mismatch fails", relRootWDL: filepath.Join("version_mismatch", "root.wdl"), wantErrText: "is newer than importing document version"},
	}

	for _, tc := range tests {
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			resolver, err := NewDefaultResolver(ResolverConfig{})
			if err != nil {
				t.Fatalf("resolver init failed: %v", err)
			}
			loader := NewWdlV1Loader()
			validator := NewSemanticValidator(SemanticValidatorConfig{})

			path := filepath.Join(root, tc.relRootWDL)
			_, loadErr := loader.LoadFile(context.Background(), path, WithResolver(resolver), WithValidator(validator))
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
