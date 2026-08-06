package wdl

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

const benchmarkWorkflowSource = "version 1.2\nworkflow demo {\n}\n"

func BenchmarkLoadStringV1(b *testing.B) {
	ctx := context.Background()
	loader := NewWdlV1Loader()

	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		if _, err := loader.LoadString(ctx, benchmarkWorkflowSource, WithResolver(nil)); err != nil {
			b.Fatalf("LoadString failed: %v", err)
		}
	}
}

func BenchmarkLoadFileWithImportResolutionV1(b *testing.B) {
	ctx := context.Background()
	tmp := b.TempDir()
	importedPath := filepath.Join(tmp, "lib.wdl")
	rootPath := filepath.Join(tmp, "main.wdl")

	imported := "version 1.1\nworkflow libwf {\n}\n"
	root := "version 1.1\nimport \"lib.wdl\"\nworkflow mainwf {\n}\n"
	if err := os.WriteFile(importedPath, []byte(imported), 0o644); err != nil {
		b.Fatalf("write imported fixture: %v", err)
	}
	if err := os.WriteFile(rootPath, []byte(root), 0o644); err != nil {
		b.Fatalf("write root fixture: %v", err)
	}

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		b.Fatalf("resolver init failed: %v", err)
	}
	loader := NewWdlV1Loader()

	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		if _, err := loader.LoadFile(ctx, rootPath, WithResolver(resolver)); err != nil {
			b.Fatalf("LoadFile failed: %v", err)
		}
	}
}

func BenchmarkSemanticValidate(b *testing.B) {
	ctx := context.Background()
	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(ctx, benchmarkWorkflowSource, WithResolver(nil))
	if err != nil {
		b.Fatalf("fixture parse failed: %v", err)
	}
	validator := NewSemanticValidator(SemanticValidatorConfig{})

	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		if err := validator.Validate(ctx, doc); err != nil {
			b.Fatalf("semantic validation failed: %v", err)
		}
	}
}

func BenchmarkLintValidate(b *testing.B) {
	ctx := context.Background()
	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(ctx, benchmarkWorkflowSource, WithResolver(nil))
	if err != nil {
		b.Fatalf("fixture parse failed: %v", err)
	}
	validator := NewLintingValidator(SemanticValidatorConfig{})

	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		if err := validator.Validate(ctx, doc); err != nil {
			b.Fatalf("lint validation failed: %v", err)
		}
	}
}

func BenchmarkLoadAndValidateSemanticV1(b *testing.B) {
	ctx := context.Background()
	loader := NewWdlV1Loader()
	validator := NewSemanticValidator(SemanticValidatorConfig{})

	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		if _, err := loader.LoadString(ctx, benchmarkWorkflowSource, WithResolver(nil), WithValidator(validator)); err != nil {
			b.Fatalf("load+validate failed: %v", err)
		}
	}
}
