# Plan: Fix the antlr4rust parse failure for user-defined types

Follow-up to the open question at the end of `session-ses_0273.md` (line 4341):
"why did you modify the test wdl files?" — decision made: fix the visitor/parser
instead of watering down fixtures.

## Root cause (to be confirmed in Phase 1)

`DefaultErrorStrategy::sync()` in `antlr4rust` 0.5.2 (`error_strategy.rs:468`) gates on
`atn().next_tokens(state)` — the **context-free, cached** `LL1Analyzer::look(s, None, None)`
set — which is missing `IDENTIFIER` for the `wdlType -> typeRefType -> strictIdentifier ->
anyIdentBase -> IDENTIFIER` path. It aborts with `InputMismatchError` *before*
`adaptive_predict` runs. The error message then prints the **context-aware**
`get_expected_tokens()` set, which *does* contain `IDENTIFIER` — hence the
self-contradictory `mismatched input 'Address' expecting {... IDENTIFIER ...}`.

Affects two constructs, both valid per the shared grammar and parsed correctly by the
Java/Python/TS/Go implementations:
- `struct Person { Address addr }` — user-defined type as a struct field
- `workflow w { S s = S { x: 1 } }` — user-defined type in a bound declaration

## Phase 0 — Unblock the build (DONE)

| Step | Action | Result |
|---|---|---|
| 0.1 | `git submodule update --init wdl-grammar` | done — checked out `27defc20` |
| 0.2 | `asdf plugin add java && asdf install java temurin-11.0.27+6` (per `.tool-versions`) | done — `openjdk 11.0.27+6` on PATH via asdf |
| 0.3 | `make spec_examples` | done — 471/551/557 files in `wdl-grammar/spec_examples/{v1_1,v1_2,v1_3}` |
| 0.4 | Harden `rust/build.rs` | done, with a scope change — see below |
| 0.5 | `cargo test` baseline | done — see failure list below |

### 0.4 finding and follow-up: build.rs had a pre-existing, independent bug

`grammar_dir` was hard-coded to `../wdl-grammar/antrl4/v1` (typo: `antrl4`), while the
submodule's real directory is `antlr4`. This mismatch meant codegen silently never ran —
by luck, since the `!jar.exists()` / (initially added) `!grammar_dir.exists()` fallback
always fired.

Fixing the typo exposed a **second**, deeper bug: `antlr4-rust-tool.jar` fails against
the real submodule grammar (`error(114): cannot find tokens file ... WdlV1Lexer.tokens`
— a lexer/parser generation-order issue in the two-file invocation) but **exits 0
anyway**, so `status.success()` can't detect the failure on its own.

Resolved as follows (final state):
- Added `rust/antlr4`, `rust/spec_examples`, `rust/wdl_tests` symlinks into the
  `wdl-grammar` submodule and `wdl_tests`, mirroring `python/`'s layout
- Removed the stale, hand-maintained `rust/grammar/*.g4` copies (no longer the source
  of truth — the submodule via the symlink is)
- Removed the unreferenced, accidentally-committed `rust/src/wdl-grammar/antrl4/v1/`
  debris copy of generated output (dead code, not part of `mod.rs`)
- `build.rs`'s `grammar_dir` now correctly points at `antlr4/v1` (through the symlink)
- Codegen now writes into a scratch directory under Cargo's `OUT_DIR` rather than
  directly into `src/grammar/`, because the tool sometimes mirrors the `-lib` path's
  directory structure into its output instead of writing flat files (confirmed:
  produced a stray `src/grammar/antlr4/v1/...` nested copy during testing). After
  invocation, `build.rs` recursively scans the scratch dir for exactly the 6 filenames
  `mod.rs` declares, copies only those flat into `src/grammar/`, and errors out
  (falls back, doesn't touch tracked sources) if any are missing — which correctly
  catches the `error(114)`-exits-0 case: verified via a clean-scratch-dir rebuild that
  it now prints `cargo:warning=ANTLR4 codegen failed; falling back to pre-generated
  grammar files` and leaves `src/grammar/*.rs` byte-identical to the committed version
- Still checks `jar.exists()`, `grammar_dir` existence, and `java -version` success
  before attempting codegen, warning + falling back (not panicking) if any are absent

Net effect: build is unblocked, and — given the tool's `error(114)` issue against the
real submodule grammar remains unfixed — always ends up using the committed
pre-generated `src/grammar/*.rs` sources, deterministically and safely, regardless of
environment. The underlying `error(114)` bug in `antlr4-rust-tool.jar` (lexer/parser
generation ordering) is a separate, deeper tooling issue not fixed here; `build.rs` now
degrades gracefully around it instead of silently emitting stale/partial output.

### 0.5 baseline (`cargo test --no-fail-fast`, pristine fixtures, no `wdl_tests/**` edits)

16 test binaries, 5 failing:

| Binary | Failing test(s) | Cause |
|---|---|---|
| `import_edge_cases_test` | `accepts_mixed_forms_import` | parser bug (struct var in workflow body) |
| `import_validation_test` | `accepts_import::case_1` (`standard_alias`) | parser bug (struct-typed decl) |
| `non_runtime_completion_test` | `accepts_import_alias_nested`, `accepts_static::case_4` (`member_index_checks_ok.wdl`) | parser bug (struct field type / struct var) |
| `validator_test` | `test_accepts_simple_valid_workflow` | **unrelated** pre-existing bug: `WdlLintingValidator` raises a false-positive `LintUnusedWorkflowDeclaration` on a workflow named `first` that IS used. Not a parse error, not touched by Phase 2. Out of scope; noted for a future pass. |

All other binaries pass, including `spec_parse_test` (all spec examples parse without
panic) and `spec_validation_test` (with the existing `PARSE_GAP`/`VALIDATOR_FALSE_POSITIVE`
skips still in place). Re-confirmed identical after the symlink/build.rs cleanup above.

## Phase 1 — Confirm root cause

| Step | Action |
|---|---|
| 1.1 | Throwaway harness (`rust/tests/probe_sync.rs`, deleted afterward) parsing `struct P { A a }` and `workflow w { S s = S { x: 1 } }` |
| 1.2 | Instrument via a temporary `ErrorStrategy` that logs, at each `sync()`: `recognizer.get_state()`, `atn().next_tokens(state)`, `get_expected_tokens()`, and `la(1)` |
| 1.3 | Assert: `next_tokens` omits `IDENTIFIER` while `get_expected_tokens` contains it -> hypothesis confirmed. If **not** confirmed, stop and re-plan rather than applying a fix that masks a different defect. |

## Phase 2 — Surgical fix

In `rust/src/loader.rs`, add alongside `WdlErrorListener` (~line 155):

```rust
struct WdlErrorStrategy<'i, Ctx: ParserNodeType<'i>>(DefaultErrorStrategy<'i, Ctx>);

impl<'a, T: Parser<'a>> ErrorStrategy<'a, T> for WdlErrorStrategy<'a, T::Node> {
    fn sync(&mut self, recognizer: &mut T) -> Result<(), ANTLRError> {
        // antlr4rust's DefaultErrorStrategy::sync gates on the context-free
        // ATN::next_tokens set, which is incomplete for `wdlType -> typeRefType`.
        // If the context-aware expected set accepts the lookahead, let
        // adaptive_predict make the real decision.
        let la = recognizer.get_input_stream_mut().la(1);
        if recognizer.get_expected_tokens().contains(la) {
            return Ok(());
        }
        self.0.sync(recognizer)
    }
    // every other method delegates verbatim to self.0
}
```

Wire in at `loader.rs:197` — `WdlV1Parser::with_strategy(token_stream,
Box::new(WdlErrorStrategy::new()))` (generated at `wdlv1parser.rs:396`) instead of
`WdlV1Parser::new`.

Fallback if `get_expected_tokens` isn't reachable through the public `Parser` trait or the
surgical guard proves insufficient: full no-op `sync()`, matching ANTLR's own
`BailErrorStrategy`.

Negative-case verification (mandatory):
- Every `_fail.wdl` under `wdl_tests/` still errors
- `spec_parse_test` still passes (no new panics)
- Spot-check that genuinely malformed WDL (e.g. `workflow w { Int }`) still yields a
  syntax error with a sensible message, not a silent partial parse

## Phase 3 — Validate against pristine fixtures

| Step | Action |
|---|---|
| 3.1 | The 4 baseline-failing test files pass **with fixtures untouched** — no `wdl_tests/**` edits, this is the whole point |
| 3.2 | Confirm the 8 `PARSE_GAP` spec files now parse (`import_structs`, `map_to_struct2`, `member_access`, `nested_access`, `pair_to_struct`, `person_struct_task`, `struct_to_struct`, `test_struct`). If they do, remove the `PARSE_GAP` list from `spec_validation_test.rs:25-34` — a direct consequence of the fix, not scope creep. `VALIDATOR_FALSE_POSITIVE` stays. |
| 3.3 | Full regression: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` |

## Phase 4 — Java parity audit (report only, no edits)

Systematically diff each `rust/tests/*.rs` against its counterpart in
`java/src/test/java/org/openwdl/wdl/model/**`, and cross-check `typescript/test/`,
`python/tests/`, `go/wdl/` for the shared `wdl_tests/` fixtures. Deliver a table of:
missing cases, extra cases, and validator-level mismatches.

Already-identified gaps to seed it:
- `type_assignability_matrix_test.rs` is missing `file_directory_from_string_ok.wdl`,
  `struct_to_struct_coercion_ok.wdl`, `struct_to_struct_incompatible_fail.wdl` (all
  present in Java/TS/Python/Go)
- Same file uses `WdlStaticAnalysisValidator`; Java uses the base `WdlValidator`
- `spec_validation_test.rs` skips all `_fail.wdl` files; the Java spec tests assert
  they're rejected
- `spec_validation_test.rs` skips `placeholder_none.wdl` / `test_select_first.wdl` for
  over-eager `select_first`/`None` constant folding in `validators/mod.rs`

## Phase 5 — Upstream

File an issue at `github.com/antlr4rust/antlr4` with the minimal repro from Phase 1.

## Out of scope (deferred)

- Fixing the `select_first`/`None` constant-folding false positives
- Adding the missing Java-parity test cases
- Removing the `_fail.wdl` skip in `spec_validation_test.rs`
- Phase 7 Step 0 deprecation checks from `rust_phase_7.md`

Nothing under `wdl_tests/` or `wdl-grammar/` will be modified by this plan.
