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

## Phase 1 — Confirm root cause (DONE)

| Step | Action | Result |
|---|---|---|
| 1.1 | Throwaway harness `rust/tests/probe_sync.rs` (deleted after this phase; not part of the permanent suite) parsing `struct Person { Address addr }` / `struct Address { String city }` and the real `wdl_tests/non_runtime_completion/import_alias_nested/lib.wdl` fixture | done |
| 1.2 | A probing `ErrorStrategy` wrapping `DefaultErrorStrategy` logged, at each `sync()` call: `recognizer.get_state()`, `atn().next_tokens(state)`, `get_expected_tokens()` (both boolean `.contains(la)` and the full `.to_token_string()`), plus an attached `ErrorListener` recording `syntax_error()` notifications in the same interleaved log | done |
| 1.3 | Original hypothesis ("`next_tokens` context-free set omits `IDENTIFIER`, `get_expected_tokens` context-aware set contains it") — **confirmed but refined**, see below | done, refined |

### Refined root cause: `IntervalSet::contains()` is unreliable, not just `next_tokens`

Original hypothesis: `sync()`'s early-return check `next_tokens.contains(la)` uses an
incomplete context-free set, while the context-aware `get_expected_tokens()` set is
correct. **This is only half the story.** Direct instrumentation of the real failing
fixture (`import_alias_nested/lib.wdl`) shows:

At the `structDefinition`'s `structItem*` loop-back decision (ATN state `346`, confirmed
`ATNSTATE_PLUS_LOOP_BACK`/`STAR_LOOP_BACK` type from `DefaultErrorStrategy::sync`'s
branch), parsing the **second** struct's field (`Address addr`, lookahead
`IDENTIFIER`/token id 52):

```
sync: state=346 la=52 next_tokens_contains_la=false expected_contains_la=false
       expected={..., KEYWORD_WORKFLOW, IDENTIFIER, KEYWORD_PARAMETER_META, CLOSE_BRACE}
SYNTAX_ERROR line=8 col=2 msg=mismatched input 'Address' expecting {..., IDENTIFIER, ...}
```

**`expected.contains(52)` returns `false`, yet the same `IntervalSet`'s own
`to_token_string()` rendering explicitly lists `IDENTIFIER` as a member.** These two
methods disagree about membership on the *same* `IntervalSet` instance:

- `IntervalSet::contains()` (`interval_set.rs`) uses `self.intervals.binary_search_by(...)`,
  which is only correct if `self.intervals` is sorted and non-overlapping.
- `IntervalSet::to_token_string()` just linearly iterates `self.intervals` in whatever
  order they happen to be in — no sortedness assumption, so it can't be fooled by the same
  invariant violation.

So the `intervals: Vec<Interval>` backing this specific `IntervalSet` (built via
`ATN::get_expected_tokens` in `atn.rs`, which repeatedly calls `expected.add_set(following)`
then `expected.remove_one(TOKEN_EPSILON)` once per stack frame in `states_stack`) ends up
with an interval containing `IDENTIFIER` that is **out of sorted order relative to the
rest of the vec**, breaking the invariant `contains()`'s binary search depends on — a
latent bug in `antlr4rust` 0.5.2's `IntervalSet`/`ATN::get_expected_tokens`/`remove_one`
interaction, not (only) an incomplete-set problem specific to `next_tokens`.

Practical implication: `next_tokens.contains(la)` **and** `get_expected_tokens().contains(la)`
are *both* unreliable membership checks in general — not just the cheap
context-free one. Any fix that gates on `.contains()` (from either set) inherits this
same defect and could still intermittently reject valid input, just less often.

### Why the minimal, in-isolation repros didn't reproduce a hard failure

`probe_struct_field_user_type`/`probe_workflow_struct_var_decl` (hand-written 2-struct/
1-struct fixtures) hit the same `next_tokens=false`/`expected=true` (not `false`) pattern
at ATN state `582` (the inner `wdlType` alt-selection decision) but the overall parse
still succeeded (`had_error=false`) — because `DefaultErrorStrategy::sync()` has an
earlier unconditional early-return: `if next_tokens.contains(TOKEN_EPSILON) { ...; return
Ok(()); }`. The decision points hit in the simpler repros happen to have a valid epsilon
(loop-exit) alternative, so `sync()` quietly defers to `adaptive_predict` without ever
reaching the `PLUS_LOOP_BACK`/`STAR_LOOP_BACK` match arm that calls
`report_unwanted_token`. The **real** fixture (`lib.wdl`, two struct definitions) reaches
a *different*, later decision point (state `346`, the loop-back after already having
matched one full `structItem` and re-entering the loop for its second iteration) where
this early-return doesn't apply, and the underlying `IntervalSet` bug then manifests as
a hard, listener-reported syntax error. This explains why a trivial single-declaration
repro can look fine while a two-struct real-world file fails — it depends on exactly
which ATN decision state is reached, which varies with grammar-position and
loop-iteration count, not just presence of a user-defined type reference.

### Consequence for Phase 2

The originally-planned surgical fix (`sync()` override that checks
`get_expected_tokens().contains(la)` before delegating) is now known to rely on **the
same buggy `contains()` method** that causes the failure in the first place — it could
still intermittently fail to unblock valid input, just less often, since it happens to
be correct along more code paths (having `TOKEN_EPSILON` handling, per-stack-frame
recomputation) than `next_tokens` alone but is *not proven reliable*. Given no public API
exists to iterate `IntervalSet`'s raw ranges directly (its `intervals` field is private;
`to_token_string()` is the only publicly-accessible order-independent view, and it's
string-based, not suitable for a hot-path equality check), the **full no-op `sync()`**
option (originally listed as a fallback, matching ANTLR's own `BailErrorStrategy`
behavior) is now the recommended primary approach for Phase 2, not the surgical variant:
it sidesteps `IntervalSet::contains()` entirely rather than trusting a different call
into the same broken machinery. Real syntax errors are still caught elsewhere: normal
token matching (`match_token`) uses direct token-id equality, not interval-set
membership, so genuinely malformed WDL will still fail via `recover_inline`/mismatched
`match_token` when the parser reaches an actual dead end.

## Phase 2 — Fix (revised: full no-op `sync()`, not the surgical `contains()` guard)

In `rust/src/loader.rs`, add alongside `WdlErrorListener` (~line 155):

```rust
/// antlr4rust 0.5.2's DefaultErrorStrategy::sync() gates its early-return decisions
/// on IntervalSet::contains(), which has a confirmed sortedness-invariant bug (see
/// rust_parser_fix_plan.md Phase 1): it can return `false` for tokens that a linear
/// scan of the very same set (IntervalSet::to_token_string) shows are present. Both
/// the context-free `next_tokens` set AND the context-aware `get_expected_tokens()`
/// set go through this same buggy `contains()`, so neither is safe to gate on.
///
/// sync() exists purely as an optimistic pre-check ahead of the real decision
/// (`adaptive_predict`, a separate ALL(*)/SLL simulation not affected by this bug).
/// Skipping it entirely — mirroring antlr4rust's own `BailErrorStrategy` — defers
/// fully to `adaptive_predict` and to ordinary token matching (`match_token`, a
/// direct token-id equality check, unaffected by IntervalSet) for detecting real
/// errors. Every other method delegates verbatim to `DefaultErrorStrategy`.
struct WdlErrorStrategy<'i, Ctx: ParserNodeType<'i>>(DefaultErrorStrategy<'i, Ctx>);

impl<'a, T: Parser<'a>> ErrorStrategy<'a, T> for WdlErrorStrategy<'a, T::Node> {
    fn sync(&mut self, _recognizer: &mut T) -> Result<(), ANTLRError> {
        Ok(())
    }
    // every other method delegates verbatim to self.0
}
```

Wire in at `loader.rs:197` — `WdlV1Parser::with_strategy(token_stream,
Box::new(WdlErrorStrategy::new()))` (generated at `wdlv1parser.rs:396`) instead of
`WdlV1Parser::new`.

(The originally-planned surgical `if get_expected_tokens().contains(la) { return Ok(()) }`
guard is no longer recommended — see Phase 1's "Consequence for Phase 2" section above.
Keep it in mind only as a fallback if the no-op version turns out to be too permissive
in practice, e.g. if it causes genuinely malformed WDL to silently mis-parse instead of
error — watch for this specifically in the mandatory negative-case verification below.)

Negative-case verification (mandatory):
- Every `_fail.wdl` under `wdl_tests/` still errors
- `spec_parse_test` still passes (no new panics)
- Spot-check that genuinely malformed WDL (e.g. `workflow w { Int }`) still yields a
  syntax error with a sensible message, not a silent partial parse

### Phase 2 result (DONE)

Implemented exactly as above in `rust/src/loader.rs`: added `WdlErrorStrategy` (wraps
`DefaultErrorStrategy`, no-ops `sync()`, delegates every other `ErrorStrategy` method
verbatim), wired in via `WdlV1Parser::with_strategy(token_stream,
Box::new(WdlErrorStrategy::new()))` replacing the old `WdlV1Parser::new(token_stream)`
call in `parse_document()`.

**Baseline before fix**: 5 failing tests across 4 binaries (`import_edge_cases_test`,
`import_validation_test`, `non_runtime_completion_test` ×2, `validator_test`).

**After fix**: 2 failing tests across 2 binaries — a net fix of 3 test failures, and
both remaining failures are pre-existing, unrelated to parsing:

| Binary | Test | Status |
|---|---|---|
| `import_edge_cases_test` | `accepts_mixed_forms_import` | **FIXED** |
| `import_validation_test` | `accepts_import::case_1` (`standard_alias`) | **FIXED** |
| `non_runtime_completion_test` | `accepts_static::case_4` (`member_index_checks_ok.wdl`) | **FIXED** |
| `non_runtime_completion_test` | `accepts_import_alias_nested` | Still fails, but **root cause moved**: now parses correctly (confirmed — no more `mismatched input` syntax error) and fails *validation* instead: `WdlSemanticError { code: UnknownReference, message: "Unknown type reference 'PersonAlias' at 'p'" }`. `WdlStaticAnalysisValidator` doesn't resolve struct type aliases introduced via `import ... alias X as Y`. This is a **separate, pre-existing validator gap**, out of scope for this parser-focused plan — worth a follow-up ticket, not fixed here. |
| `validator_test` | `test_accepts_simple_valid_workflow` | Still fails — the unrelated `LintUnusedWorkflowDeclaration` false-positive documented in Phase 0's baseline, untouched. |

**Negative-case verification — all passed** (throwaway harness, deleted after use):
- `workflow w { Int }` (missing declaration identifier) still correctly fails with
  `mismatched input '}' expecting {..., IDENTIFIER}` — genuine syntax errors are still
  caught; the no-op `sync()` did not make the parser silently permissive.
- All 18 `_fail.wdl` fixtures under `wdl_tests/` were re-checked: none newly parse when
  they shouldn't — all 18 "successfully parse" at the syntax level, which is *expected*
  since every one of them is a semantic/type/runtime-level failure (not a grammar-level
  one), matching `spec_validation_test.rs`'s existing documented design.
- `spec_parse_test` passes (3/3, all versions, no panics) — confirmed via full suite run.
- Bonus verification (not required, done anyway): re-parsed all 22 `PARSE_GAP` spec
  files across v1_1/v1_2/v1_3 (`import_structs.wdl`, `map_to_struct2.wdl`,
  `member_access.wdl`, `nested_access.wdl`, `pair_to_struct.wdl`,
  `person_struct_task.wdl`, `struct_to_struct.wdl` (v1_2/v1_3 only), `test_struct.wdl`)
  — **all 22 now parse successfully**, confirming Phase 3's `PARSE_GAP` skip-list removal
  will be a clean no-op change with nothing left to skip.

`cargo build`, `cargo clippy -- -D warnings`, and `rustfmt --check src/loader.rs` all
clean — no new warnings; remaining clippy warnings and loader.rs formatting diffs are
pre-existing and untouched by this change (verified via `git diff` that none of the
formatting-diff hunks intersect the added code).

## Phase 3 — Validate against pristine fixtures (DONE)

| Step | Action | Result |
|---|---|---|
| 3.1 | The 4 baseline-failing test files pass **with fixtures untouched** — no `wdl_tests/**` edits, this is the whole point | done in Phase 2 — 3 of the 4 originally-failing tests now pass; the 4th (`accepts_import_alias_nested`) now parses correctly but fails at a separate, pre-existing validator step (see Phase 2 writeup) |
| 3.2 | Confirm the 8 `PARSE_GAP` spec files now parse (`import_structs`, `map_to_struct2`, `member_access`, `nested_access`, `pair_to_struct`, `person_struct_task`, `struct_to_struct`, `test_struct`). If they do, remove the `PARSE_GAP` list from `spec_validation_test.rs:25-34` — a direct consequence of the fix, not scope creep. `VALIDATOR_FALSE_POSITIVE` stays. | done — removed `PARSE_GAP` const and its usage from `spec_validation_test.rs`; `cargo test --test spec_validation_test` still passes 3/3 (v1_1/v1_2/v1_3) with all 22 previously-skipped files (8 files × up to 3 versions each) now genuinely parsing and validating |
| 3.3 | Full regression: `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` | done — same 2 pre-existing failures as Phase 2 (`accepts_import_alias_nested`, `test_accepts_simple_valid_workflow`), no new clippy warnings, `rustfmt --check` clean on the touched file (one pre-existing-style line-wrap fixed as a direct side effect of removing the `skip_parse` condition, not unrelated drift) |

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
