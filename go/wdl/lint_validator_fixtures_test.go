package wdl

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

func TestLintValidatorFixtureWarnings(t *testing.T) {
	root := filepath.Join("..", "..", "wdl_tests", "validator")
	fixture := filepath.Join(root, "lint_unused_symbols_bad.wdl")

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	loader := NewLoader()
	validator := NewLintingValidator(SemanticValidatorConfig{ThrowOnWarnings: true})

	_, loadErr := loader.LoadFile(context.Background(), fixture, WithResolver(resolver), WithValidator(validator))
	if loadErr == nil {
		t.Fatal("expected warning escalation for lint fixture")
	}
	if !strings.Contains(loadErr.Error(), string(CodeLintUnusedTaskDeclaration)) {
		t.Fatalf("expected unused task declaration lint code, got: %v", loadErr)
	}
	if !strings.Contains(loadErr.Error(), string(CodeLintUnusedWorkflowDeclaration)) {
		t.Fatalf("expected unused workflow declaration lint code, got: %v", loadErr)
	}
	if !strings.Contains(loadErr.Error(), string(CodeLintUnusedScatterVariable)) {
		t.Fatalf("expected unused scatter variable lint code, got: %v", loadErr)
	}
}
