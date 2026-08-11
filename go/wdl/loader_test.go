package wdl

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
)

func TestLoadStringParsesVersion(t *testing.T) {
	src := "version 1.2\nworkflow demo {\n}\n"
	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(context.Background(), src, WithResolver(nil))
	if err != nil {
		t.Fatalf("expected parse success, got error: %v", err)
	}
	if got, want := doc.Version.String(), "1.2"; got != want {
		t.Fatalf("version mismatch: got %s want %s", got, want)
	}
	if len(doc.Declarations) != 1 {
		t.Fatalf("expected 1 declaration, got %d", len(doc.Declarations))
	}
	if doc.Declarations[0].Kind != DeclarationWorkflow || doc.Declarations[0].Name != "demo" {
		t.Fatalf("unexpected declaration: %+v", doc.Declarations[0])
	}
}

func TestLoadFileResolvesFilesystemImports(t *testing.T) {
	tmp := t.TempDir()
	importedPath := filepath.Join(tmp, "lib.wdl")
	rootPath := filepath.Join(tmp, "main.wdl")

	imported := "version 1.1\nworkflow libwf {\n}\n"
	root := "version 1.1\nimport \"lib.wdl\"\nworkflow mainwf {\n}\n"
	if err := os.WriteFile(importedPath, []byte(imported), 0o644); err != nil {
		t.Fatalf("write imported file: %v", err)
	}
	if err := os.WriteFile(rootPath, []byte(root), 0o644); err != nil {
		t.Fatalf("write root file: %v", err)
	}

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	loader := NewWdlV1Loader()
	doc, err := loader.LoadFile(context.Background(), rootPath, WithResolver(resolver))
	if err != nil {
		t.Fatalf("load failed: %v", err)
	}
	if len(doc.ImportStatements) != 1 {
		t.Fatalf("expected 1 import statement, got %d", len(doc.ImportStatements))
	}
	if len(doc.ImportedDocs) != 1 {
		t.Fatalf("expected 1 imported document, got %d", len(doc.ImportedDocs))
	}
}

func TestLoadStringResolvesHTTPImports(t *testing.T) {
	var srv *httptest.Server
	srv = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch r.URL.Path {
		case "/lib.wdl":
			_, _ = w.Write([]byte("version 1.0\nworkflow lib {\n}\n"))
		default:
			http.NotFound(w, r)
		}
	}))
	defer srv.Close()

	src := "version 1.0\nimport \"" + srv.URL + "/lib.wdl\"\nworkflow wf {\n}\n"
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(context.Background(), src, WithResolver(resolver), WithSourceLocation(filepath.Join(t.TempDir(), "root.wdl")))
	if err != nil {
		t.Fatalf("load failed: %v", err)
	}
	if len(doc.ImportedDocs) != 1 {
		t.Fatalf("expected 1 imported document, got %d", len(doc.ImportedDocs))
	}
}

func TestLoadStringExtractsImportVariants(t *testing.T) {
	src := "version 1.3\n" +
		"import \"base.wdl\" as base alias old as renamed\n" +
		"import * from \"all.wdl\"\n" +
		"import { foo as bar, baz } from \"members.wdl\"\n" +
		"workflow wf {}\n"

	loader := NewWdlV1Loader()
	doc, err := loader.LoadString(context.Background(), src, WithResolver(nil))
	if err != nil {
		t.Fatalf("expected parse success, got error: %v", err)
	}
	if len(doc.ImportStatements) != 3 {
		t.Fatalf("expected 3 imports, got %d", len(doc.ImportStatements))
	}
	if doc.ImportStatements[0].RawLocation != "base.wdl" || doc.ImportStatements[0].NamespaceAlias != "base" {
		t.Fatalf("unexpected first import: %+v", doc.ImportStatements[0])
	}
	if len(doc.ImportStatements[0].Aliases) != 1 || doc.ImportStatements[0].Aliases[0].Name != "old" || doc.ImportStatements[0].Aliases[0].Alias != "renamed" {
		t.Fatalf("unexpected standard aliases: %+v", doc.ImportStatements[0].Aliases)
	}
	if !doc.ImportStatements[1].ImportAllMembers {
		t.Fatalf("expected star import to set ImportAllMembers: %+v", doc.ImportStatements[1])
	}
	if len(doc.ImportStatements[2].Aliases) != 2 {
		t.Fatalf("expected 2 member aliases, got %d", len(doc.ImportStatements[2].Aliases))
	}
}

func TestLoadStringReturnsSyntaxError(t *testing.T) {
	src := "version 1.0\nworkflow bad {\ncall\n}\n"
	loader := NewWdlV1Loader()
	_, err := loader.LoadString(context.Background(), src, WithResolver(nil))
	if err == nil {
		t.Fatal("expected syntax error")
	}
	if _, ok := err.(Exception); !ok {
		t.Fatalf("expected Exception, got %T", err)
	}
}
