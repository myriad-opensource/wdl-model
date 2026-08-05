package wdl

import (
	"context"
	"io/fs"
	"path/filepath"
	"strings"
	"testing"
)

func TestSpecExamplesParse(t *testing.T) {
	root := filepath.Join("..", "..", "wdl-grammar", "spec_examples")
	loader := NewLoader()
	count := 0

	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() || !strings.HasSuffix(strings.ToLower(d.Name()), ".wdl") {
			return nil
		}
		if strings.Contains(strings.ToLower(d.Name()), "_fail") {
			return nil
		}
		count++
		if _, loadErr := loader.LoadFile(context.Background(), path, WithResolver(nil)); loadErr != nil {
			t.Fatalf("failed to parse spec example %s: %v", path, loadErr)
		}
		return nil
	})
	if err != nil {
		t.Fatalf("walk spec examples failed: %v", err)
	}
	if count == 0 {
		t.Fatalf("expected at least one spec example under %s", root)
	}
}
