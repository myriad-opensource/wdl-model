package wdl

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestLoaderImportsRecursiveFixture(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	root := filepath.Join("..", "..", "wdl_tests", "loader_imports", "recursive", "root.wdl")
	loader := NewWdlV1Loader()
	doc, err := loader.LoadFile(context.Background(), root, WithResolver(resolver))
	if err != nil {
		t.Fatalf("load failed: %v", err)
	}
	if len(doc.ImportedDocs) != 1 {
		t.Fatalf("expected 1 direct imported document, got %d", len(doc.ImportedDocs))
	}
	var child *Document
	for _, d := range doc.ImportedDocs {
		child = d
		break
	}
	if child == nil || len(child.ImportedDocs) != 1 {
		t.Fatalf("expected child to have nested import, got %#v", child)
	}
}

func TestLoaderImportsFromStringFixture(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	rootPath := filepath.Join("..", "..", "wdl_tests", "loader_imports", "string_input", "root.wdl")
	buf, err := os.ReadFile(rootPath)
	if err != nil {
		t.Fatalf("read fixture failed: %v", err)
	}
	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(context.Background(), string(buf), WithResolver(resolver), WithSourceLocation(rootPath))
	if err != nil {
		t.Fatalf("load failed: %v", err)
	}
	if len(doc.ImportStatements) != 1 || len(doc.ImportedDocs) != 1 {
		t.Fatalf("expected one resolved import from string input, got statements=%d docs=%d", len(doc.ImportStatements), len(doc.ImportedDocs))
	}
}

func TestLoaderImportsCircularFixture(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	root := filepath.Join("..", "..", "wdl_tests", "loader_imports", "circular", "root.wdl")
	loader := NewWdlV1Loader()
	_, err = loader.LoadFile(context.Background(), root, WithResolver(resolver))
	if err == nil {
		t.Fatal("expected circular import error")
	}
	msg := err.Error()
	if !strings.Contains(msg, "circular import detected") || !strings.Contains(msg, "child.wdl") {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestLoaderImportsCircularRelativeFixture(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	root := filepath.Join("..", "..", "wdl_tests", "loader_imports", "circular_relative", "root.wdl")
	loader := NewWdlV1Loader()
	_, err = loader.LoadFile(context.Background(), root, WithResolver(resolver))
	if err == nil {
		t.Fatal("expected circular import error")
	}
	msg := err.Error()
	if !strings.Contains(msg, "circular import detected") || !strings.Contains(msg, "root.wdl") {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestResolverFilesystemFixture(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	rootPath := filepath.Join("..", "..", "wdl_tests", "resolver_filesystem", "root.wdl")
	resolved, src, err := resolver.ResolveImport(context.Background(), rootPath, "sub/imported.wdl")
	if err != nil {
		t.Fatalf("resolve failed: %v", err)
	}
	if resolved == "" || src == "" {
		t.Fatalf("expected resolved path and source, got resolved=%q src-len=%d", resolved, len(src))
	}
}

func TestProcessorImportsFixtureTraversal(t *testing.T) {
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	root := filepath.Join("..", "..", "wdl_tests", "processor_imports", "root.wdl")
	loader := NewWdlV1Loader()
	doc, err := loader.LoadFile(context.Background(), root, WithResolver(resolver))
	if err != nil {
		t.Fatalf("load failed: %v", err)
	}
	if len(doc.ImportStatements) != 4 {
		t.Fatalf("expected 4 import statements, got %d", len(doc.ImportStatements))
	}

	proc := &collectingProcessor{events: []string{}}
	if err := TraverseDocument(context.Background(), doc, proc); err != nil {
		t.Fatalf("traverse failed: %v", err)
	}
	if len(proc.events) == 0 {
		t.Fatal("expected traversal events")
	}
}
