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
	parseFailuresExpectedWithReservedKeywords := map[string]struct{}{
		"test_find_task.wdl":         {},
		"test_meta_values.wdl":       {},
		"test_runtime_info_task.wdl": {},
		"test_task_previous.wdl":     {},
	}

	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		name := strings.ToLower(d.Name())
		if d.IsDir() || !strings.HasSuffix(name, ".wdl") {
			return nil
		}
		if strings.Contains(name, "_fail") {
			return nil
		}
		if _, expected := parseFailuresExpectedWithReservedKeywords[d.Name()]; expected {
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
