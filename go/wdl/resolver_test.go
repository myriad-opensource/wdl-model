package wdl

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
)

func TestResolverReadsLocalFile(t *testing.T) {
	tmp := t.TempDir()
	target := filepath.Join(tmp, "a.wdl")
	if err := os.WriteFile(target, []byte("version 1.0\n"), 0o644); err != nil {
		t.Fatalf("write file failed: %v", err)
	}

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	resolved, src, err := resolver.ResolveImport(context.Background(), filepath.Join(tmp, "root.wdl"), "a.wdl")
	if err != nil {
		t.Fatalf("resolve failed: %v", err)
	}
	if resolved == "" || src == "" {
		t.Fatalf("unexpected empty resolve result: %q %q", resolved, src)
	}
}

func TestResolverReadsHTTPURL(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		_, _ = w.Write([]byte("version 1.0\nworkflow x {}\n"))
	}))
	defer srv.Close()

	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	resolved, src, err := resolver.ResolveImport(context.Background(), "/tmp/root.wdl", srv.URL)
	if err != nil {
		t.Fatalf("resolve failed: %v", err)
	}
	if resolved != srv.URL {
		t.Fatalf("resolved mismatch: got %s want %s", resolved, srv.URL)
	}
	if src == "" {
		t.Fatal("expected non-empty source")
	}
}
