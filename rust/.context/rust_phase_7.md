# Phase 7 Implementation Plan: Tests

## Overview

9 new integration test files + a validator gap fix (3 missing deprecation checks).
The existing 5 test files (`loader_test.rs`, `processor_test.rs`, `resolver_test.rs`,
`spec_parse_test.rs`, `validator_test.rs`) are kept as-is and count toward the final total.

**Estimated total tests after Phase 7: ~95–110 functions (~270 individual cases)**
(up from 26, dominated by 200+ parameterized spec-validation cases)

---

## Step 0 — Fix Missing Deprecation Checks (prerequisite for Step 6)

Three lint checks are described in `phase6_plan.md` Chunk 6 but were never implemented in
`validators/mod.rs`. They are needed before any deprecation tests can pass:

| Missing check | Fixture | Location in `validators/mod.rs` |
|---|---|---|
| `runtime` section present → `LintDeprecatedFeature` | `runtime_section_deprecated.wdl` | `lint_task()` — add match arm on `WdlTaskElement::Runtime` |
| `Object` primitive type used → `LintDeprecatedFeature` | `object_type_deprecated.wdl` | New helper `lint_deprecated_type_usage(ty, name)` called from `lint_task()` + `lint_workflow()` on every declaration's type |
| String placeholder option present (`sep=`, `default=`, `true=`/`false=`) → `LintDeprecatedFeature` | `placeholder_options_deprecated.wdl` | `collect_string_literal_usage()` — already iterates `WdlStringComponent::Placeholder`; add `LintDeprecatedFeature` emission when `option` is `Some(_)` |

The `file://` import scheme check is already implemented and working.

### `lint_deprecated_type_usage` helper (new)

```rust
fn lint_deprecated_type_usage(&mut self, ty: &WdlType, name: &str) {
    match ty {
        WdlType::Primitive(p) if p.primitive_kind == WdlPrimitiveKind::Object => {
            self.add_error(
                WdlErrorCode::LintDeprecatedFeature,
                format!("Declaration '{}' uses deprecated 'Object' type", name),
            );
        }
        WdlType::Array(a) => self.lint_deprecated_type_usage(&a.member_type, name),
        WdlType::Map(m) => {
            self.lint_deprecated_type_usage(&m.key_type, name);
            self.lint_deprecated_type_usage(&m.value_type, name);
        }
        WdlType::Pair(p) => {
            self.lint_deprecated_type_usage(&p.left_type, name);
            self.lint_deprecated_type_usage(&p.right_type, name);
        }
        _ => {}
    }
}
```

Call this in `lint_task()` and `lint_workflow()` for every Input/BoundDeclaration/Output type.

### Placeholder option deprecation (change to `collect_string_literal_usage`)

When iterating `WdlStringComponent::Placeholder { option, .. }`:

```rust
if let Some(_opt) = option {
    self.add_error(
        WdlErrorCode::LintDeprecatedFeature,
        "String placeholder option syntax (sep=, default=, true=/false=) is deprecated",
    );
    // existing option recursion stays
}
```

### Runtime section deprecation (change to `lint_task`)

```rust
WdlTaskElement::Runtime(_) => {
    self.add_error(
        WdlErrorCode::LintDeprecatedFeature,
        format!("Task '{}' uses deprecated 'runtime' section; use 'requirements' instead", task.name),
    );
}
```

---

## Step 1 — `rust/tests/spec_validation_test.rs`

Mirrors `WdlV11/12/13SpecExamplesTest.java`.

### Fixture roots

```
wdl-grammar/spec_examples/v1_1/   (471 files, 18 _fail)
wdl-grammar/spec_examples/v1_2/   (551 files, 18 _fail)
wdl-grammar/spec_examples/v1_3/   (557 files, 72 _fail)
```

### Exception sets

Files where `WdlValidator` cannot detect the failure (execution-time, coercion, or out-of-scope):

**v1_1 exceptions** (parse OK, validator cannot detect):
- `select_first_only_none_fail.wdl`
- `empty_array_fail.wdl`
- `test_as_map_fail.wdl`
- `write_json_fail.wdl`
- `test_map_fail.wdl`
- `select_first_empty_fail.wdl`
- `private_declaration_fail.wdl`
- `non_empty_optional_fail.wdl`
- `test_zip_fail.wdl`
- `bash_comment_fail_task.wdl`
- `bash_variables_fail_task.wdl`
- `coercion_fail.wdl`
- `multi_return_code_fail_task.wdl`
- `test_prefix_fail.wdl`
- `test_suffix_fail.wdl`

**v1_2 exceptions**: same as v1_1 + `illegal_access_fail.wdl`

**v1_3 exceptions**: same as v1_2

### Test functions

```
fn fail_exception_set_v1_1() -> HashSet<&'static str>
fn fail_exception_set_v1_2() -> HashSet<&'static str>
fn fail_exception_set_v1_3() -> HashSet<&'static str>
fn spec_fail_wdl_files(version: &str) -> Vec<PathBuf>  // only _fail.wdl files
```

**`all_v1_1_fail_examples_rejected_by_base_validator`**
**`all_v1_2_fail_examples_rejected_by_base_validator`**
**`all_v1_3_fail_examples_rejected_by_base_validator`**

For each `_fail.wdl` file NOT in the exception set:
- Load via `load_from_path`
- Assert `WdlValidator::new().validate(&doc).is_err()`
- Collect all failures; panic once at the end with a combined message

**`all_v1_1_import_files_have_populated_import_map`**
**`all_v1_2_import_files_have_populated_import_map`**
**`all_v1_3_import_files_have_populated_import_map`**

For each `.wdl` file whose text matches `^\s*import\s+"` but NOT `^\s*import\s+"https?://`:
- Load via `load_from_path` (runs filesystem resolver)
- Assert `!doc.imported_documents.is_empty()`
- Assert `doc.imported_documents.len() == doc.import_statements().count()`
- Skip if load returns `Err` (file may have intentionally broken imports)

### Key implementation notes

- Use `std::fs::read_to_string` + `contains` to detect local imports before loading
- Use collect-failures pattern (accumulate strings, single panic at end) matching existing `spec_parse_test.rs`
- Total: 6 test functions, ~200 lines

---

## Step 2 — `rust/tests/type_assignability_matrix_test.rs`

Mirrors `WdlTypeAssignabilityMatrixTest.java`.

```
Fixture dir: wdl_tests/type_assignability_matrix/
Validator:   WdlStaticAnalysisValidator (type checking is Static-level)
```

### Test functions

```rust
#[rstest]
#[case("optional_from_none_ok.wdl")]
#[case("array_nested_ok.wdl")]
#[case("map_value_type_ok.wdl")]
fn accepts_compatible_assignment(#[case] fixture: &str)
// assert static.validate(&doc).is_ok()

#[rstest]
#[case("required_from_none_fail.wdl")]
#[case("array_member_type_fail.wdl")]
#[case("required_string_to_int_fail.wdl")]
#[case("array_string_to_int_fail.wdl")]
#[case("map_value_type_fail.wdl")]
fn rejects_incompatible_assignment(#[case] fixture: &str)
// assert static.validate(&doc).is_err()
```

**`known_gap_*` files are intentionally skipped** with a comment. Both
`known_gap_mixed_array_literal.wdl` (`Array[Int] xs = [1, "x"]`) and
`known_gap_required_from_none.wdl` (`Int i = None`) represent cases where the current
static type inference cannot detect the mismatch, consistent with Java behavior.

### Total: 2 `rstest` functions = 8 individual test cases

---

## Step 3 — `rust/tests/function_version_matrix_test.rs`

Mirrors `WdlFunctionVersionMatrixTest.java`.

```
Fixture dir: wdl_tests/function_version_matrix/
Validator:   WdlValidator (version checks are base-level)
```

### Test functions

```rust
#[rstest]
#[case("v11_keys_ok.wdl")]
#[case("v12_contains_ok.wdl")]
#[case("v13_value_ok.wdl")]
fn accepts_version_compatible_function(#[case] fixture: &str)
// assert base.validate(&doc).is_ok()

#[rstest]
#[case("v11_contains_key_fail.wdl")]
#[case("v11_join_paths_fail.wdl")]
#[case("v12_value_fail.wdl")]
fn rejects_version_incompatible_function(#[case] fixture: &str)
// assert base.validate(&doc).is_err()
// assert errors contain FunctionNotAvailableInVersion
```

### Total: 2 `rstest` functions = 6 individual test cases

---

## Step 4 — `rust/tests/static_function_signature_matrix_test.rs`

Mirrors `WdlStaticFunctionSignatureMatrixTest.java`.

```
Fixture dir: wdl_tests/static_function_signature_matrix/
```

### Test functions

```rust
#[rstest]
#[case("keys_bad.wdl")]
#[case("range_bad.wdl")]
#[case("contains_bad.wdl")]
#[case("chunk_bad.wdl")]
#[case("cross_bad.wdl")]
#[case("join_paths_bad_first.wdl")]
#[case("join_paths_bad_tail.wdl")]
#[case("basename_bad_first.wdl")]
#[case("size_bad_second.wdl")]
fn base_passes_static_rejects_invalid_signature(#[case] fixture: &str)
// assert base.validate(&doc).is_ok()
// assert static.validate(&doc).is_err()

fn accepts_valid_signatures_under_all_validators()
// static_signatures_ok.wdl: both base and static pass
```

### Total: 1 `rstest` function (9 cases) + 1 plain test = 10 individual test cases

---

## Step 5 — `rust/tests/expression_operator_semantics_test.rs`

Mirrors `WdlExpressionOperatorSemanticsTest.java`.

```
Fixture dir: wdl_tests/expression_operator_semantics/
```

### Test functions

```rust
#[rstest]
#[case("logical_operand_type_fail.wdl")]
#[case("numeric_operand_type_fail.wdl")]
#[case("order_comparison_type_fail.wdl")]
#[case("ternary_condition_type_fail.wdl")]
fn base_passes_static_rejects_operator_type_mismatch(#[case] fixture: &str)
// base passes, static fails with TypeMismatch

fn accepts_valid_operator_expressions()
// operators_ok.wdl: both base and static pass

fn accepts_operator_precedence_and_compound_equality()
// operator_precedence_ok.wdl + compound_equality_ok.wdl: static passes

fn rejects_incompatible_compound_equality()
// compound_equality_incompatible_fail.wdl: base passes, static fails
```

### Total: 1 `rstest` function (4 cases) + 3 plain tests = 7 individual test cases

---

## Step 6 — `rust/tests/deprecation_validation_test.rs`

Mirrors `WdlDeprecationValidationTest.java`. **Requires Step 0.**

```
Fixture dir: wdl_tests/deprecations/
Validator:   WdlLintingValidator (throw_on_warnings = true by default)
```

### Test functions

```rust
#[rstest]
#[case("runtime_section_deprecated.wdl")]
#[case("object_type_deprecated.wdl")]
#[case("placeholder_options_deprecated.wdl")]
fn warns_on_deprecated_feature(#[case] fixture: &str)
// load_from_path, assert lint.validate(&doc).is_err()
// assert lint.errors() contains at least one LintDeprecatedFeature

fn warns_on_file_scheme_import_deprecated()
// SPECIAL: load via load_from_str (avoids filesystem resolver trying to find file://example.wdl)
// read file contents, load_from_str(&contents), assert lint err with LintDeprecatedFeature

fn no_deprecation_warning_on_clean_fixture()
// no_deprecations.wdl
// lint.set_throw_on_warnings(false); assert lint.validate(&doc).is_ok()
// assert no error has code LintDeprecatedFeature
```

### Why `load_from_str` for the file-scheme fixture

`file_scheme_import_deprecated.wdl` contains `import "file://example.wdl"` which the
filesystem resolver would try to open (and fail). The Java test also loads it from string.
In Rust: `std::fs::read_to_string(path)` then `load_from_str(&contents)`.

### Total: 1 `rstest` function (3 cases) + 2 plain tests = 5 individual test cases

---

## Step 7 — `rust/tests/import_edge_cases_test.rs`

Mirrors `WdlImportEdgeCasesTest.java`.

```
Fixture dirs: wdl_tests/import_edge_cases/<subdir>/root.wdl
Validator:    WdlValidator
```

### Test functions

```rust
#[rstest]
#[case("duplicate_namespace")]
#[case("namespace_conflicts_local")]
#[case("member_alias_conflicts_local")]
#[case("member_alias_duplicate")]
fn rejects_import_edge_case(#[case] dir: &str)
// load root.wdl, assert WdlValidator err

fn accepts_mixed_forms_import()
// mixed_forms_ok/root.wdl: WdlValidator ok
```

### Total: 1 `rstest` function (4 cases) + 1 plain test = 5 individual test cases

---

## Step 8 — `rust/tests/import_validation_test.rs`

Mirrors `WdlImportValidationTest.java`.

```
Fixture dirs:
  wdl-grammar/spec_examples/v1_1|v1_2|v1_3/<file>   (spec import examples)
  wdl_tests/import_validation/<subdir>/root.wdl       (structural fixtures)
Validator: WdlValidator
```

### Spec example parameterization

Enumerate (version, filename) pairs that exist on disk (skip missing files):

```
Positive spec examples (validated via WdlValidator, assert ok):
  (v1_1, call_example.wdl), (v1_2, call_example.wdl), (v1_3, call_example.wdl)
  (v1_1, call_imported.wdl), (v1_2, call_imported.wdl), (v1_3, call_imported.wdl)

Negative spec examples (validated via WdlValidator, assert err):
  (v1_1, call_subworkflow_fail.wdl), (v1_2, ...), (v1_3, ...)
  (v1_1, incomplete_struct_fail.wdl), (v1_2, ...), (v1_3, ...)
  (v1_1, illegal_access_fail.wdl), (v1_2, ...), (v1_3, ...)
  [only emit a test case for pairs where the file actually exists]
```

### Test functions

```rust
#[rstest]
#[case("v1_1", "call_example.wdl")]
#[case("v1_2", "call_example.wdl")]
#[case("v1_3", "call_example.wdl")]
#[case("v1_1", "call_imported.wdl")]
// ... etc for all (version, file) pairs that exist
fn validates_positive_import_spec_example(#[case] version: &str, #[case] file: &str)

#[rstest]
#[case("v1_1", "call_subworkflow_fail.wdl")]
// ... etc
fn rejects_negative_import_spec_example(#[case] version: &str, #[case] file: &str)

fn validates_star_and_members_import_forms()         // star_members/root.wdl
fn validates_standard_import_struct_aliases()        // standard_alias/root.wdl
fn rejects_unknown_member_import()                   // unknown_member/root.wdl
fn rejects_duplicate_import_namespaces()             // duplicate_namespace/root.wdl
fn rejects_import_alias_target_not_exist()           // bad_alias/root.wdl
fn rejects_incompatible_imported_structs()           // struct_conflict/root.wdl
fn rejects_import_from_higher_minor_version()        // version_mismatch/root.wdl
```

### Total: 2 `rstest` functions (~12 cases) + 7 plain tests = ~19 individual test cases

---

## Step 9 — `rust/tests/non_runtime_completion_test.rs`

Mirrors `WdlNonRuntimeCompletionValidationTest.java`.

```
Fixture dir: wdl_tests/non_runtime_completion/
```

### Test functions

```rust
fn validates_nested_import_type_aliases()
// import_alias_nested/root.wdl: WdlValidator ok

#[rstest]
#[case("baseline_function_args/length_bad.wdl")]
#[case("baseline_function_args/contains_key_bad.wdl")]
fn rejects_invalid_baseline_function_args(#[case] fixture: &str)
// WdlValidator err

fn accepts_valid_baseline_function_args()
// baseline_function_args/baseline_function_args_ok.wdl: WdlValidator ok

#[rstest]
#[case("member_index_checks/unknown_struct_field_fail.wdl")]
#[case("member_index_checks/unknown_call_output_fail.wdl")]
#[case("member_index_checks/index_out_of_bounds_fail.wdl")]
fn rejects_invalid_member_index_access(#[case] fixture: &str)
// WdlValidator err

fn accepts_valid_member_index_access()
// member_index_checks/member_index_checks_ok.wdl: WdlValidator ok

fn validates_placeholder_interpolation_and_section_syntax()
// placeholder_interpolation_ok.wdl + requirements_hints_syntax_ok.wdl
// WdlStaticAnalysisValidator ok for both

fn rejects_json_type_level_static_usage()
// json_type_level_static_fail.wdl: base passes, static fails
```

### Total: 2 `rstest` functions (5 cases) + 5 plain tests = 10 individual test cases

---

## Summary Table

| Step | File | New test cases |
|---|---|---|
| 0 | `src/validators/mod.rs` (patch) | — prerequisite, no new test file |
| 1 | `tests/spec_validation_test.rs` | ~6 functions (~200+ parameterized by file) |
| 2 | `tests/type_assignability_matrix_test.rs` | 8 |
| 3 | `tests/function_version_matrix_test.rs` | 6 |
| 4 | `tests/static_function_signature_matrix_test.rs` | 10 |
| 5 | `tests/expression_operator_semantics_test.rs` | 7 |
| 6 | `tests/deprecation_validation_test.rs` | 5 |
| 7 | `tests/import_edge_cases_test.rs` | 5 |
| 8 | `tests/import_validation_test.rs` | ~19 |
| 9 | `tests/non_runtime_completion_test.rs` | 10 |

**Cumulative after Phase 7: ~96 test functions / ~270 individual cases**

---

## Fixture Inventory (all pre-existing, no new WDL files needed)

```
wdl_tests/
  appending_processor/          ← covered by existing processor_test.rs
  deprecations/                 ← Step 6
    file_scheme_import_deprecated.wdl
    no_deprecations.wdl
    object_type_deprecated.wdl
    placeholder_options_deprecated.wdl
    runtime_section_deprecated.wdl
  expression_operator_semantics/ ← Step 5
    compound_equality_incompatible_fail.wdl
    compound_equality_ok.wdl
    logical_operand_type_fail.wdl
    numeric_operand_type_fail.wdl
    operator_precedence_ok.wdl
    operators_ok.wdl
    order_comparison_type_fail.wdl
    ternary_condition_type_fail.wdl
  function_version_matrix/       ← Step 3
    v11_contains_key_fail.wdl
    v11_join_paths_fail.wdl
    v11_keys_ok.wdl
    v12_contains_ok.wdl
    v12_value_fail.wdl
    v13_value_ok.wdl
  import_edge_cases/             ← Step 7
    duplicate_namespace/root.wdl + lib.wdl
    member_alias_conflicts_local/root.wdl + lib.wdl
    member_alias_duplicate/root.wdl + lib.wdl
    mixed_forms_ok/root.wdl + lib.wdl + members.wdl + star.wdl
    namespace_conflicts_local/root.wdl + lib.wdl
  import_validation/             ← Step 8
    bad_alias/root.wdl + lib.wdl
    duplicate_namespace/root.wdl + lib.wdl
    standard_alias/root.wdl + lib.wdl
    star_members/root.wdl + members_lib.wdl + star_lib.wdl
    struct_conflict/a.wdl + b.wdl + root.wdl
    unknown_member/root.wdl + lib.wdl
    version_mismatch/root.wdl + lib.wdl
  non_runtime_completion/        ← Step 9
    baseline_function_args/baseline_function_args_ok.wdl + contains_key_bad.wdl + length_bad.wdl
    import_alias_nested/root.wdl + lib.wdl
    json_type_level_static_fail.wdl
    member_index_checks/index_out_of_bounds_fail.wdl + member_index_checks_ok.wdl +
                         unknown_call_output_fail.wdl + unknown_struct_field_fail.wdl
    placeholder_interpolation_ok.wdl
    requirements_hints_syntax_ok.wdl
  static_function_signature_matrix/ ← Step 4
    basename_bad_first.wdl + chunk_bad.wdl + contains_bad.wdl + cross_bad.wdl +
    join_paths_bad_first.wdl + join_paths_bad_tail.wdl + keys_bad.wdl +
    range_bad.wdl + size_bad_second.wdl + static_signatures_ok.wdl
  type_assignability_matrix/     ← Step 2
    array_member_type_fail.wdl + array_nested_ok.wdl + array_string_to_int_fail.wdl +
    known_gap_mixed_array_literal.wdl + known_gap_required_from_none.wdl +
    map_value_type_fail.wdl + map_value_type_ok.wdl + optional_from_none_ok.wdl +
    required_from_none_fail.wdl + required_string_to_int_fail.wdl
  validator/                     ← covered by existing validator_test.rs

wdl-grammar/spec_examples/
  v1_1/  (471 files)             ← Steps 1 + 8
  v1_2/  (551 files)             ← Steps 1 + 8
  v1_3/  (557 files)             ← Steps 1 + 8
```

---

## Open Questions

Two decisions needed before implementation begins:

**Q1 — Spec `_fail` validation strategy**

The `_fail.wdl` spec files include both WDL model-level failures (detectable by `WdlValidator`)
and WDL execution-level failures (runtime coercion, bash script errors, etc.). Options:

- **(a) Generous exception set (recommended)**: Add all execution/runtime `_fail` files to the
  exception set upfront. Safe; no false test failures. The exception set in the plan above covers
  all known non-model failures for v1.1/v1.2/v1.3.
- **(b) Aggressive / discovery mode**: Start with a minimal exception set, run, and see which
  `_fail` files cause false passes — use this to find validator gaps. Higher upfront noise.
- **(c) Skip `_fail` validation entirely**: Only add the import-map-population assertions.
  Very conservative; leaves the spec test suite weaker than the Java one.

**Q2 — Step 0 timing**

The 3 missing deprecation checks in `validators/mod.rs` are ~40 lines combined. Options:

- **(a) Implement in Phase 7 as Step 0 (recommended)**: All deprecation tests are green from
  the start. No `#[ignore]` markers needed.
- **(b) Defer and use `#[ignore]`**: Write `deprecation_validation_test.rs` with
  `#[ignore = "deprecation check not yet implemented"]` on the 3 unimplemented cases.
  Keeps Phase 7 purely test-writing, but leaves known failures in the test suite.

---

## Execution Order

Recommended order to minimize rework:

```
Step 0  →  Step 6  (implement deprecations, then test them immediately)
Step 2  →  Step 3  →  Step 4  (matrix tests, simplest first)
Step 5                         (expression semantics)
Step 7  →  Step 8              (import tests, edge cases before full validation)
Step 9                         (non-runtime completion)
Step 1                         (spec validation last — largest, needs confidence in validators)
```

Each step: write test file → `cargo test --test <file>` → fix any issues → mark done.
Full suite regression after every step: `cargo test`.

---

## Critical Context

- **`rstest` version**: `0.23` already in `[dev-dependencies]`; use `#[rstest]` + `#[case(...)]`
- **Fixture path helper**: reuse pattern from existing tests —
  `PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("wdl_tests").join(...)`
- **Load via path vs string**:
  - `load_from_path` → triggers filesystem import resolver (needed for import tests)
  - `load_from_str` → no resolver (needed for `file_scheme_import_deprecated.wdl`)
- **Collect-failures pattern**: match `spec_parse_test.rs` — accumulate into `Vec<String>`,
  single `panic!` at end with joined messages; no `#[should_panic]`
- **`set_throw_on_warnings(false)`**: use on `WdlLintingValidator` when testing clean fixtures
  to distinguish "no lint errors" from "no errors at all"
- **`cargo test` command**: `source "$HOME/.cargo/env" && cargo test` from `rust/` directory
- **Test binary naming**: each file in `tests/` becomes its own integration test binary;
  run a single file with `cargo test --test <stem>` (e.g. `cargo test --test type_assignability_matrix_test`)
