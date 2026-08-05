package wdl

import (
	"context"
	"strings"
	"testing"
)

func TestSemanticValidatorRejectsDuplicateTopLevelNames(t *testing.T) {
	src := "version 1.3\ntask x {}\nworkflow x {}\n"
	loader := NewLoader()
	validator := NewSemanticValidator(SemanticValidatorConfig{})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err == nil {
		t.Fatal("expected duplicate definition error")
	}
	if !strings.Contains(err.Error(), string(CodeDuplicateDefinition)) {
		t.Fatalf("expected duplicate code in error, got: %v", err)
	}
}

func TestSemanticValidatorRequiresExactlyOneWorkflow(t *testing.T) {
	src := "version 1.2\ntask t {}\n"
	loader := NewLoader()
	validator := NewSemanticValidator(SemanticValidatorConfig{})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err == nil {
		t.Fatal("expected workflow count error")
	}
	if !strings.Contains(err.Error(), "exactly one workflow") {
		t.Fatalf("expected workflow count message, got: %v", err)
	}
}

func TestSemanticValidatorAcceptsSingleWorkflow(t *testing.T) {
	src := "version 1.3\nworkflow ok {}\n"
	loader := NewLoader()
	validator := NewSemanticValidator(SemanticValidatorConfig{})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err != nil {
		t.Fatalf("expected success, got: %v", err)
	}
}

func TestLintingValidatorDoesNotThrowWarningsByDefault(t *testing.T) {
	src := "version 1.3\nimport \"lib.wdl\" as lib\nworkflow wf {}\n"
	loader := NewLoader()
	validator := NewLintingValidator(SemanticValidatorConfig{})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err != nil {
		t.Fatalf("expected warning-only pass, got: %v", err)
	}
}

func TestLintingValidatorCanThrowWarnings(t *testing.T) {
	src := "version 1.3\nimport \"lib.wdl\" as lib\nworkflow wf {}\n"
	loader := NewLoader()
	validator := NewLintingValidator(SemanticValidatorConfig{ThrowOnWarnings: true})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err == nil {
		t.Fatal("expected warning escalation error")
	}
	if !strings.Contains(err.Error(), string(CodeLintUnusedImport)) {
		t.Fatalf("expected lint code in error, got: %v", err)
	}
}

func TestSemanticValidatorRejectsDuplicateImportNamespaceAliases(t *testing.T) {
	src := "version 1.3\n" +
		"import \"lib-a.wdl\" as dup\n" +
		"import \"lib-b.wdl\" as dup\n" +
		"workflow wf {}\n"
	loader := NewLoader()
	validator := NewSemanticValidator(SemanticValidatorConfig{})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err == nil {
		t.Fatal("expected duplicate import alias error")
	}
	if !strings.Contains(err.Error(), "duplicate import namespace alias") {
		t.Fatalf("expected duplicate import alias message, got: %v", err)
	}
}

func TestLintingValidatorReportsDeprecatedFileImportURI(t *testing.T) {
	src := "version 1.3\nimport \"file:///tmp/lib.wdl\" as lib\nworkflow wf {}\n"
	loader := NewLoader()
	validator := NewLintingValidator(SemanticValidatorConfig{ThrowOnWarnings: true})
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil), WithValidator(validator))
	if err == nil {
		t.Fatal("expected warning escalation error")
	}
	if !strings.Contains(err.Error(), string(CodeLintDeprecatedFeature)) {
		t.Fatalf("expected deprecated feature lint code, got: %v", err)
	}
}
