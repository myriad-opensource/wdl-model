package wdl

import (
	"context"
	"path/filepath"
	"testing"
)

func TestFunctionVersionMatrixFixtures(t *testing.T) {
	runSharedFixtureConformanceMatrix(
		t,
		filepath.Join("..", "..", "wdl_tests", "function_version_matrix"),
		NewStaticValidator(SemanticValidatorConfig{}),
		[]string{"v11_contains_key_fail.wdl", "v11_join_paths_fail.wdl", "v12_value_fail.wdl"},
		[]string{"v11_keys_ok.wdl", "v12_contains_ok.wdl", "v13_value_ok.wdl"},
	)
}

func TestStaticFunctionSignatureMatrixFixtures(t *testing.T) {
	runSharedFixtureConformanceMatrix(
		t,
		filepath.Join("..", "..", "wdl_tests", "static_function_signature_matrix"),
		NewStaticValidator(SemanticValidatorConfig{}),
		[]string{
			"keys_bad.wdl",
			"range_bad.wdl",
			"contains_bad.wdl",
			"size_bad_second.wdl",
			"chunk_bad.wdl",
			"cross_bad.wdl",
			"join_paths_bad_first.wdl",
			"join_paths_bad_tail.wdl",
			"basename_bad_first.wdl",
		},
		[]string{"static_signatures_ok.wdl"},
	)
}

func TestTypeAssignabilityMatrixFixtures(t *testing.T) {
	runSharedFixtureConformanceMatrix(
		t,
		filepath.Join("..", "..", "wdl_tests", "type_assignability_matrix"),
		NewStaticValidator(SemanticValidatorConfig{}),
		[]string{
			"required_from_none_fail.wdl",
			"array_member_type_fail.wdl",
			"required_string_to_int_fail.wdl",
			"array_string_to_int_fail.wdl",
			"map_value_type_fail.wdl",
			"struct_to_struct_incompatible_fail.wdl",
		},
		[]string{
			"optional_from_none_ok.wdl",
			"array_nested_ok.wdl",
			"map_value_type_ok.wdl",
			"file_directory_from_string_ok.wdl",
			"struct_to_struct_coercion_ok.wdl",
		},
	)
}

func TestExpressionOperatorSemanticsFixtures(t *testing.T) {
	runSharedFixtureConformanceMatrix(
		t,
		filepath.Join("..", "..", "wdl_tests", "expression_operator_semantics"),
		NewStaticValidator(SemanticValidatorConfig{}),
		[]string{
			"logical_operand_type_fail.wdl",
			"numeric_operand_type_fail.wdl",
			"order_comparison_type_fail.wdl",
			"ternary_condition_type_fail.wdl",
			"compound_equality_incompatible_fail.wdl",
		},
		[]string{"operators_ok.wdl", "operator_precedence_ok.wdl", "compound_equality_ok.wdl"},
	)
}

func TestImportEdgeCaseFixtures(t *testing.T) {
	root := filepath.Join("..", "..", "wdl_tests", "import_edge_cases")
	tests := []struct {
		scenario string
		wantErr  bool
	}{
		{scenario: "mixed_forms_ok", wantErr: false},
		{scenario: "duplicate_namespace", wantErr: true},
		{scenario: "namespace_conflicts_local", wantErr: true},
		{scenario: "member_alias_duplicate", wantErr: true},
		{scenario: "member_alias_conflicts_local", wantErr: true},
	}

	for _, tc := range tests {
		tc := tc
		t.Run(tc.scenario, func(t *testing.T) {
			resolver, err := NewDefaultResolver(ResolverConfig{})
			if err != nil {
				t.Fatalf("resolver init failed: %v", err)
			}
			loader := NewWdlV1Loader()
			validator := NewSemanticValidator(SemanticValidatorConfig{})
			rootWDL := filepath.Join(root, tc.scenario, "root.wdl")
			_, loadErr := loader.LoadFile(context.Background(), rootWDL, WithResolver(resolver), WithValidator(validator))
			if tc.wantErr && loadErr == nil {
				t.Fatalf("expected error for scenario %s", tc.scenario)
			}
			if !tc.wantErr && loadErr != nil {
				t.Fatalf("expected success for scenario %s, got: %v", tc.scenario, loadErr)
			}
		})
	}
}

func runSharedFixtureConformanceMatrix(t *testing.T, root string, validator Validator, failureFixtures []string, successFixtures []string) {
	t.Helper()
	resolver, err := NewDefaultResolver(ResolverConfig{})
	if err != nil {
		t.Fatalf("resolver init failed: %v", err)
	}
	loader := NewWdlV1Loader()

	for _, name := range failureFixtures {
		path := filepath.Join(root, name)
		_, loadErr := loader.LoadFile(context.Background(), path, WithResolver(resolver), WithValidator(validator))
		if loadErr == nil {
			t.Fatalf("expected failure for fixture %s", path)
		}
	}

	for _, name := range successFixtures {
		path := filepath.Join(root, name)
		_, loadErr := loader.LoadFile(context.Background(), path, WithResolver(resolver), WithValidator(validator))
		if loadErr != nil {
			t.Fatalf("expected success for fixture %s, got: %v", path, loadErr)
		}
	}
}
