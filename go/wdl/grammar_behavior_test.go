package wdl

import (
	"context"
	"path/filepath"
	"testing"
)

func TestGrammarBehaviorAssociativityFixturesParse(t *testing.T) {
	fixturesRoot := filepath.Join("..", "..", "wdl_tests", "grammar_behavior")
	loader := NewWdlV1Loader()

	for _, name := range []string{
		"associativity_additive_chain.wdl",
		"associativity_multiplicative_chain.wdl",
		"associativity_logical_or_chain.wdl",
	} {
		path := filepath.Join(fixturesRoot, name)
		doc, err := loader.LoadFile(context.Background(), path, WithResolver(nil))
		if err != nil {
			t.Fatalf("expected %s to parse, got error: %v", name, err)
		}
		if len(doc.Declarations) == 0 {
			t.Fatalf("expected declarations in %s", name)
		}
	}
}

func TestGrammarBehaviorReservedKeywordFixturesFailParse(t *testing.T) {
	fixturesRoot := filepath.Join("..", "..", "wdl_tests", "grammar_behavior")
	loader := NewWdlV1Loader()

	for _, name := range []string{
		"keyword_decl_identifier_task.wdl",
		"keyword_decl_identifier_if.wdl",
		"keyword_task_input_in.wdl",
		"keyword_metadata_key_version.wdl",
	} {
		path := filepath.Join(fixturesRoot, name)
		if _, err := loader.LoadFile(context.Background(), path, WithResolver(nil)); err == nil {
			t.Fatalf("expected %s to fail parsing with reserved keyword identifier", name)
		}
	}
}
