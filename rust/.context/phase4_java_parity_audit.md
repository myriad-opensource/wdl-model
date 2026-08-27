# Phase 4 — Java/Python/TypeScript/Go Parity Audit

Report-only audit per `rust_parser_fix_plan.md` Phase 4. No test or fixture files were
modified as part of this phase. Findings below are grouped by Rust test file, with a
cross-language comparison table and a final prioritized action list for anyone picking
this up later.

Validator naming key (same concept, different names per language):
- **base**: `WdlValidator` (Rust/Java) / `WdlSemanticValidator` (Python/TS) / `NewSemanticValidator` (Go)
- **static**: `WdlStaticAnalysisValidator` (Rust/Java) / `WdlStaticAnalysisSemanticValidator` (Python/TS) / `NewStaticValidator` (Go)
- **lint**: `WdlLintingValidator` (Rust/Java) / `WdlLintingSemanticValidator` (Python/TS) / `NewLintingValidator` (Go)

---

## 1. `type_assignability_matrix_test.rs`

**Missing from Rust** (present in Java/Python/TS/Go):
- `file_directory_from_string_ok.wdl` (success case)
- `struct_to_struct_coercion_ok.wdl` (success case)
- `struct_to_struct_incompatible_fail.wdl` (failure case)

Rust covers 8 of 11 actionable fixtures (excludes the two `known_gap_*` files
intentionally and consistently with Java — that part is fine — but is missing these 3
struct/file-coercion cases that every other language exercises).

**Extra in Rust:** none.

**Validator mismatch:** Rust uses the **static** validator; Java/Python/TS use **base**.
Go uses static (matches Rust). This is a genuine cross-language inconsistency in which
tier is asserted here — worth resolving alongside adding the missing fixtures, since
`struct_to_struct_incompatible_fail.wdl` may behave differently under base vs. static.

**Assertion semantics:** map correctly everywhere. Go additionally checks the specific
diagnostic code `CodeTypeMismatch` for one fixture — a level of specificity Rust doesn't
have (informational only, not a blocking gap).

---

## 2. `function_version_matrix_test.rs`

**Missing from Rust:** none — full 6/6 fixture parity across all 5 languages.

**Extra in Rust:** none.

**Validator:** consistent (**base**) across Rust/Java/Python/TS; Go uses static as its
one universal validator (doesn't change fixture behavior here).

**Assertion semantics:** Rust is *stricter* than Java/Python/TS — it additionally asserts
the specific `WdlErrorCode::FunctionNotAvailableInVersion` for all 3 failure fixtures,
where Java/Python/TS only check "some semantic exception was thrown." This is a Rust
strength, not a gap.

---

## 3. `static_function_signature_matrix_test.rs`

**Missing from Rust:** none — full 10/10 fixture parity, identical across all 5 languages.

**Extra in Rust:** none.

**Validator:** Rust and Java are exact matches (base-passes / static-fails two-step
design); Python/TS mirror this with their own naming. Go is the outlier — it uses a
single validator and doesn't replicate the base/static contrast (a Go gap, not Rust's).

**Assertion semantics:** all equivalent. Go additionally checks a specific diagnostic
code for one fixture, not done in Rust (informational only).

---

## 4. `expression_operator_semantics_test.rs`

**Missing from Rust:** none — full 8/8 fixture parity across all 5 languages.

**Extra in Rust:** none.

**Validator:** consistent design across Rust/Java/Python/TS (base-passes + static-fails
two-step, or static-only for the `_ok` cases, matching intent per fixture). Go collapses
this into a single static-only check (structural difference, not a fixture gap).

**Assertion semantics:** all map correctly, no mismatches.

---

## 5. `deprecation_validation_test.rs`

**Missing from Rust:** none — full 5/5 fixture parity across all 5 languages.

**Extra in Rust:** none.

**Validator:** consistent (**lint**, `throw_on_warnings` toggled per case) across all 5
languages — no type mismatch.

**Assertion semantics:** all map correctly. Two minor asymmetries, informational only:
- TypeScript additionally asserts `severity() === WARNING` on the matched error (Rust doesn't).
- Go checks the deprecation code via a message substring match rather than a structured
  error-code filter (Rust does the structured check, which is stronger).

---

## 6. `import_edge_cases_test.rs`

**Missing from Rust:** none — full 5/5 fixture parity (4 reject + 1 accept), matching
Java/Python/TS exactly. (Go is the one missing a case here — `duplicate_namespace` — not
Rust; noted for completeness, out of scope for this repo's Rust work.)

**Extra in Rust:** none.

**Validator:** consistent (**base**) across Rust/Java/Python/TS/Go — no mismatch.

**Assertion semantics:** all map correctly.

---

## 7. `import_validation_test.rs`

**Missing from Rust** (present in Java/Python/TS):
Rust only exercises the 7 `wdl_tests/import_validation/` fixture directories (5 reject +
2 accept) — full parity on those. But it has **zero** coverage of the spec-examples-based
import cases that Java, Python, and TypeScript all implement:
- Positive (must load+validate OK): `call_example.wdl`, `call_imported.wdl` — checked
  across v1_1/v1_2/v1_3 (up to 6 cases)
- Negative (must fail): `call_subworkflow_fail.wdl`, `incomplete_struct_fail.wdl`,
  `illegal_access_fail.wdl` — checked across v1_1/v1_2/v1_3 (up to 9 cases)

This is the largest concrete, actionable gap found in this audit: up to ~15 missing test
cases, all using fixtures that already exist on disk under
`wdl-grammar/spec_examples/{v1_1,v1_2,v1_3}/` — no new fixtures needed, purely a missing
test-writing task (per Phase 7's original Step 8 design in `rust_phase_7.md`, which
explicitly planned this and was apparently never fully executed for the spec-examples
half).

**Extra in Rust:** none.

**Validator:** consistent (**base**) across Rust/Java/Python/TS for the overlapping
fixture-dir cases — no mismatch.

**Assertion semantics:** map correctly wherever cases overlap.

---

## 8. `non_runtime_completion_test.rs`

**Missing from Rust:**
- `member_index_checks/unknown_struct_field_fail.wdl` — explicitly documented in Rust's
  own doc-comment as excluded due to a "known gap" (struct-typed variable declarations in
  workflow bodies not supported by the grammar). **This exact limitation is exactly what
  Phase 2 of this plan fixed** (the antlr4rust `sync()` bug). This fixture should very
  likely now work and can probably be un-skipped — worth verifying directly as a
  follow-up, since Java and Python both include and test it.

**Extra in Rust:** none.

**Validator mismatch:** Rust uses **static** for every case in this file. Java/Python use
**base** for the baseline-function-args and member-index cases, and switch to **static**
specifically for the two-tier `json_type_level_static_fail.wdl` check (asserting base
passes AND static fails). Rust only checks the static validator's failure for that
fixture — it never asserts the base validator alone succeeds, which is a real
strictness gap (Rust could be masking a case where base *also* incorrectly rejects the
fixture, since it's never checked).

---

## 9. `validator_test.rs`

**Missing from Rust** (present in Java/Python/TS):
- **The "rejects known parse-ok fail examples" v1_3 batch test** — a batch of 10 spec-
  example `_fail.wdl` files (`empty_array_fail`, `illegal_access_fail`,
  `non_empty_optional_fail`, `private_declaration_fail`, `select_first_empty_fail`,
  `select_first_only_none_fail`, `test_as_map_fail`, `test_map_fail`, `test_zip_fail`,
  `write_json_fail`) that Java/Python/TS all assert are rejected by the **base**
  `WdlValidator`. None of these appear anywhere in `rust/tests/validator_test.rs`.
- The "loader runs validator and throws on invalid document" test (loader-constructor
  variant, using `select_first_empty_fail.wdl` passed through the loader's
  validator-argument path) — present in Java/Python/TS, absent in Rust.

**Extra in Rust (strengths, not gaps):**
- Rust checks all 4 specific `WdlErrorCode` lint codes for `lint_unused_symbols_bad.wdl`
  (`LintUnusedTaskDeclaration`, `LintUnusedWorkflowDeclaration`,
  `LintUnusedScatterVariable`, `LintUnusedCallOutput`); Java/Python/TS only check generic
  severity/exception-thrown, and Go checks only 3 of the 4 codes.

**Validator:** naming differs by language but maps consistently for the shared cases —
no cross-language type mismatch beyond what's already noted per-case above.

---

## 10. `spec_validation_test.rs` + `spec_parse_test.rs`

**The single largest structural gap found in this audit.**

Java (`WdlV11/12/13SpecExamplesTest`), Python (`test_spec_examples.py`), and TypeScript
(`spec-examples.test.ts`) each have a dedicated test —
`testParseAndValidateFailSpecExample` (Java) / equivalent — that asserts **every**
`_fail.wdl` spec example, across all 3 versions, with **zero exceptions**, is rejected
when loaded with the base validator attached
(`WdlV1Loader.load(content, new WdlValidator())` must throw). This is dozens of test
cases (one per `_fail.wdl` file × 3 versions) with no Rust (or Go) counterpart at all.

Rust's `spec_validation_test.rs` currently filters out **all** `_fail.wdl` files
entirely — `!n.contains("_fail")` — meaning zero validation assertions are ever made
against any `_fail.wdl` spec example. This mirrors the design noted in
`rust_parser_fix_plan.md`'s own "Out of scope (deferred)" list ("Removing the `_fail.wdl`
skip in `spec_validation_test.rs`" was explicitly deferred), but this audit confirms via
direct Java/Python/TS comparison exactly what's missing and how large the gap is.

Additionally: Rust's `spec_parse_test.rs` only asserts "doesn't panic" for every spec
file (`Ok` or `Err` both count as pass) — much weaker than Java/Python/TS's parse tests,
which assert specific per-filename pass/fail expectations, including reserved-keyword
parse-failure exception sets (`test_find_task.wdl`, `test_meta_values.wdl`,
`test_runtime_info_task.wdl`, `test_task_previous.wdl` — 3-4 files per version, expected
to fail to parse due to reserved-keyword grammar limitations). Rust has no equivalent
assertions for these.

**Rust's `VALIDATOR_FALSE_POSITIVE` skip set** (`placeholder_none.wdl`,
`test_select_first.wdl`) is a Rust-specific "known gap" workaround not found identically
in any other language's spec-validation test — suggesting either a Rust-specific
validator false-positive bug (over-eager `select_first`/`None` constant folding, per
`validators/mod.rs`) that other languages don't exhibit, or that other languages simply
don't test this exact scenario as rigorously. Worth investigating independently.

---

## 11. `loader_test.rs`

**Missing from Rust** (present in Java/Python/TS/Go):

*Grammar-behavior fixtures* (`wdl_tests/grammar_behavior/`) — completely untested in Rust:
- `associativity_additive_chain.wdl`, `associativity_multiplicative_chain.wdl`,
  `associativity_logical_or_chain.wdl` — verify left-associative operator parse trees
- `keyword_decl_identifier_task.wdl`, `keyword_decl_identifier_if.wdl`,
  `keyword_task_input_in.wdl`, `keyword_metadata_key_version.wdl` — verify reserved
  keywords are rejected as identifiers in various positions

*Loader-imports fixtures* (`wdl_tests/loader_imports/`) — completely untested in Rust:
- `recursive/{root,child,grandchild}.wdl` — recursive `imported_documents` population,
  source-location correctness, non-null import source text
- `string_input/{root,child}.wdl` — loading from raw source string + explicit
  source-location URI + filesystem resolver + validator
- `circular/{root,child}.wdl` — circular-import detection (error message content check)
- `circular_relative/root.wdl` + `nested/child.wdl` — circular-import detection survives
  relative-path normalization

This is a real, actionable functional gap: Rust has essentially zero coverage of
associativity/precedence AST-shape correctness, reserved-keyword identifier rejection,
and recursive/circular import resolution — despite all the necessary fixtures already
existing on disk and being used by every other language's test suite.

**Extra in Rust:** a few loose smoke tests (`parse_version_1_1`,
`load_from_path_spec_example`, `syntax_error_does_not_panic`) with no direct counterpart
elsewhere — harmless, low-value duplication, not a concern.

---

## 12. `processor_test.rs`

**Missing from Rust** (present in Java/Python, TS partially):

Rust's own header comment says it mirrors only `WdlAppendingProcessorTest` and
`WdlExpressionProcessorBaseTest` — an intentional, self-documented scope limit. Per that
comment, Rust has **zero** coverage of:
- `WdlFunctionProcessorBaseTest` — dispatch-to-function-specific-method behavior (e.g.
  `processFloor` vs `processNonstandard`). Also missing from TypeScript (shared gap, not
  Rust-unique).
- `WdlProcessorBaseEnumInferenceTest` — enum value type inference (implicit `String`,
  int/float widening to `Float`, empty-Optional on incompatible choices), plus
  struct/enum introspection helpers (`hasMember`/`memberType`, `hasChoice`/`choice`). TS
  covers this via `type-inference-helpers.test.ts`; Rust has no equivalent at all.
- `WdlProcessorBaseImportResolutionTest` — `resolveImportedTasks`/`Workflows`/`Structs`/
  `Enums`/`Document` helper methods against `wdl_tests/processor_imports/` fixtures
  (namespace-qualified, star-import, member-import-with-alias forms). TS covers this via
  `processor-import-resolution.test.ts`; Rust has zero equivalent (Go's coverage here is
  much looser too — non-empty-traversal-only, not the specific named-resolution
  assertions).

**Extra in Rust:** `expression_to_wdl_primitives`, `type_to_wdl_basic` — direct unit
tests of render helpers with no counterpart elsewhere, but harmless/reasonable additions.

---

## 13. `resolver_test.rs`

**Missing from Rust:** none for filesystem- or HTTP-resolver-specific behavior — full
parity with Java/Python (Rust's HTTP resolver tests are gated behind the
`http-resolver` feature flag, meaning they don't run in a default `cargo test`, unlike
Java/Python which always run theirs — worth considering whether CI should also build/test
with `--features http-resolver`).

**Extra in Rust:** `loader_populates_imported_documents_via_resolver` is a weak,
degenerate test — it uses `resolver_filesystem/root.wdl`, a fixture with **no imports at
all**, and merely asserts `imported_documents.is_empty()`. This can't exercise actual
resolver-driven recursive import loading. The equivalent real functional coverage (using
`loader_imports/recursive`, `loader_imports/circular`, etc.) exists in every other
language's *loader* test file — and, per item 11 above, is entirely absent from Rust's
`loader_test.rs` too. This is the same underlying gap as item 11, just visible from a
different angle.

---

## Cross-Language Fixture Coverage Summary Table

| Rust test file | Fixture parity | Validator-tier mismatch | Missing test cases (actionable) |
|---|---|---|---|
| `type_assignability_matrix_test.rs` | 8/11 (missing 3) | Rust/Go: static vs Java/Py/TS: base | 3 fixtures |
| `function_version_matrix_test.rs` | 6/6 full | none | 0 |
| `static_function_signature_matrix_test.rs` | 10/10 full | none (Go differs, not Rust) | 0 |
| `expression_operator_semantics_test.rs` | 8/8 full | none | 0 |
| `deprecation_validation_test.rs` | 5/5 full | none | 0 |
| `import_edge_cases_test.rs` | 5/5 full | none | 0 |
| `import_validation_test.rs` | 7/7 fixture-dirs, **0/~15 spec-examples** | none | up to ~15 |
| `non_runtime_completion_test.rs` | one fixture explicitly skipped (now fixable, see below) | Rust always static; Java/Py mix base+static | 1 fixture + 1 base-tier assertion per case |
| `validator_test.rs` | missing a 10-file batch + 1 loader-integration test | none | ~11 |
| `spec_validation_test.rs` | **0 validation assertions on any `_fail.wdl`** (dozens missing) | none (validator choice consistent where tested) | dozens |
| `loader_test.rs` | 0/7 grammar-behavior, 0/7ish loader-imports fixtures | n/a | ~14 |
| `processor_test.rs` | 2/5 Java processor test classes mirrored | n/a | 3 whole test classes |
| `resolver_test.rs` | full parity, but 1 weak placeholder test | n/a | 0 new, but see loader_test.rs |

## Prioritized Follow-Up List

**Update: items 1–6 were all implemented in Phase 5 (see
`rust_parser_fix_plan.md`'s Phase 5 section for the full writeup, including 5 genuine
production-code bugs discovered and fixed along the way — associativity, boolean/null
literal parsing, circular-import detection, relative-path normalization, and two
type-assignability coercion gaps). Item 7 remains open/deferred.**

1. ~~**Quick win, directly related to this plan's Phase 2 fix**: re-check
   `non_runtime_completion/member_index_checks/unknown_struct_field_fail.wdl` — the
   grammar limitation it was documented as hitting is exactly what Phase 2 fixed. If it
   now parses, un-skip it (mirrors the Phase 3 `PARSE_GAP` cleanup already done for
   `spec_validation_test.rs`).~~ **Done** — un-skipped; also fixed
   `accepts_import_alias_nested` in the same file by correcting the validator tier used.
2. ~~**Largest, most mechanical gap**: `import_validation_test.rs`'s missing
   spec-examples-based cases (~15 tests, fixtures already exist on disk).~~ **Done.**
3. ~~**Second-largest, most mechanical gap**: `spec_validation_test.rs`'s missing
   "`_fail.wdl` must be rejected by base validator" assertions (dozens of cases,
   fixtures already exist on disk) — the original Phase 7 plan intended this
   (`rust_phase_7.md` Step 1) but it was apparently descoped to "skip all `_fail` files"
   instead.~~ **Done**, with a small documented exception list for 3 genuine base-tier
   validator-architecture gaps (narrower than originally feared).
4. ~~**Structural, higher-effort gap**: `loader_test.rs` missing grammar-behavior
   (associativity/reserved-keyword) and loader-imports (recursive/circular) fixture
   coverage — real functional gaps, not just missing assertions, since Rust currently has
   no test proving circular-import detection or recursive `imported_documents`
   population work correctly at all.~~ **Done** — and this is exactly where the
   associativity, boolean/null-literal, circular-import-detection, and path-normalization
   bugs were found; all fixed.
5. ~~**Structural, higher-effort gap**: `processor_test.rs` missing 3 whole Java test
   classes' worth of coverage (function-dispatch, enum-inference, import-resolution
   helpers).~~ **Done** — required adding a few genuinely-missing (not buggy) small API
   surfaces (`WdlStruct::has_member`/`member_type`, `WdlEnum::has_choice`/`choice`,
   `infer_enum_value_type`, `resolve_imported_document`) to have something to test.
6. ~~**Smaller/targeted fixes**: add the 3 missing `type_assignability_matrix` fixtures and
   resolve the base-vs-static validator tier inconsistency in that file and in
   `non_runtime_completion_test.rs`; add the missing `validator_test.rs` 10-file batch +
   loader-integration test; consider running `http-resolver` feature tests in CI.~~
   **Done**, except the `http-resolver` CI suggestion (still open, low priority).
7. **Worth independent investigation, not just a test-writing task**: the
   `VALIDATOR_FALSE_POSITIVE` skip (`placeholder_none.wdl`, `test_select_first.wdl`) in
   `spec_validation_test.rs` may indicate a genuine Rust-specific validator bug
   (over-eager `select_first`/`None` constant folding) not present in other languages —
   still open/deferred, per `rust_parser_fix_plan.md`'s "Out of scope" list.
