//! Enum choice membership validation (Task C2).
//!
//! WDL 1.3 `SPEC.md:1965` defines the `Enum` ← `String` coercion as: "`String`
//! value must exactly match one of the enum's **choice names**". These tests
//! pin that rule.
//!
//! Java has no equivalent test — its four enum tests
//! (`WdlProcessorBaseEnumInferenceTest`) cover processor-level *inference*, not
//! validator assignability — but its implementation is correct
//! (`WdlExpressionValidator.java:216-224`). Rust's was not: `EnumShape` stored a
//! single `choices` list that mixed bare names with `"NAME=value"` renderings,
//! and the membership test compared bare names against that mixed list, so every
//! enum with assigned values rejected its own valid choice names.
//!
//! Nothing in the spec corpus reaches this path: the sole enum spec example
//! (`spec_examples/v1_3/test_enum_value.wdl`) assigns via member access
//! (`Color.Red`), which evaluates to `EvalValue::Unknown` and falls through.

use std::collections::HashMap;

use url::Url;
use wdl_model::errors::{WdlErrorCode, WdlSemanticError};
use wdl_model::loader::{load_from_str, load_from_str_with_resolver};
use wdl_model::resolvers::{ImportResolver, WdlImportError};
use wdl_model::validators::WdlValidator;

/// Parses `src` and runs the base validator, returning the collected errors.
fn validate(src: &str) -> Vec<WdlSemanticError> {
    let doc =
        load_from_str(src).unwrap_or_else(|e| panic!("parse failed: {e}\n--- source ---\n{src}"));
    let mut validator = WdlValidator::new();
    let _ = validator.validate(&doc);
    validator.errors().to_vec()
}

fn assert_accepts(src: &str, what: &str) {
    let errors = validate(src);
    assert!(
        errors.is_empty(),
        "{what}: expected zero diagnostics, got: {errors:?}"
    );
}

fn assert_rejects_with_type_mismatch(src: &str, what: &str) {
    let errors = validate(src);
    assert!(
        errors.iter().any(|e| e.code == WdlErrorCode::TypeMismatch),
        "{what}: expected a TypeMismatch, got: {errors:?}"
    );
}

/// Wraps `body` in a minimal 1.3 document with a workflow.
fn doc(enum_decl: &str, decl: &str) -> String {
    format!("version 1.3\n\n{enum_decl}\n\nworkflow w {{\n  {decl}\n}}\n")
}

// ─── Valued enums — the regression this task fixes ────────────────────────────

const COLOR_VALUED: &str = r##"enum Color[String] {
  Red = "#FF0000",
  Green = "#00FF00"
}"##;

#[test]
fn accepts_valid_choice_name_on_valued_enum() {
    // The C2 regression: before the fix this compared "Red" against
    // ["Red=\"#FF0000\"", "Green=\"#00FF00\""] and failed.
    assert_accepts(
        &doc(COLOR_VALUED, r#"Color c = "Red""#),
        "valued enum, valid choice name",
    );
}

#[test]
fn rejects_unknown_choice_name_on_valued_enum() {
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUED, r#"Color c = "Purple""#),
        "valued enum, unknown choice name",
    );
}

#[test]
fn rejects_choice_value_used_in_place_of_choice_name() {
    // SPEC.md:1965 matches on choice *names*, not values. "#FF0000" is Red's
    // value, not its name, so it must be rejected.
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUED, r##"Color c = "#FF0000""##),
        "valued enum, value supplied instead of name",
    );
}

// ─── Valueless enums — must keep working ─────────────────────────────────────

const COLOR_VALUELESS: &str = "enum Color {\n  Red,\n  Green\n}";

#[test]
fn accepts_valid_choice_name_on_valueless_enum() {
    // Worked before the fix too (both renderings coincide when there are no
    // values); this is the regression guard.
    assert_accepts(
        &doc(COLOR_VALUELESS, r#"Color c = "Red""#),
        "valueless enum, valid choice name",
    );
}

#[test]
fn rejects_unknown_choice_name_on_valueless_enum() {
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUELESS, r#"Color c = "Purple""#),
        "valueless enum, unknown choice name",
    );
}

#[test]
fn rejects_the_internal_shape_rendering() {
    // Guards against a regression that re-merges the two collections: the
    // `NAME=value` form is an internal identity rendering and must never be
    // accepted as a choice name.
    //
    // Uses the valueless enum deliberately. Its shape rendering is
    // `"Red=<none>"`, which contains no escapes and so constant-folds cleanly
    // to an `EvalValue::Str`. The valued enum's shape (`Red="#FF0000"`) has
    // embedded quotes that defeat constant folding, making it fall through to
    // the type check — it would pass this assertion for the wrong reason and
    // would not detect a re-merge.
    assert_rejects_with_type_mismatch(
        &doc(COLOR_VALUELESS, r#"Color c = "Red=<none>""#),
        "valueless enum, internal shape rendering",
    );
}

// ─── Non-String value types ──────────────────────────────────────────────────

#[test]
fn accepts_choice_name_on_non_string_valued_enum() {
    // Membership is on names, so the value type is irrelevant to this check.
    let priority = "enum Priority {\n  Low = 1,\n  High = 10\n}";
    assert_accepts(
        &doc(priority, r#"Priority p = "Low""#),
        "Int-valued enum, valid choice name",
    );
}

// ─── Member-access form must remain unaffected ───────────────────────────────

#[test]
fn accepts_member_access_choice_reference() {
    // `Color.Red` evaluates to EvalValue::Unknown and falls through to the
    // type-based check. This is the form used by
    // `spec_examples/v1_3/test_enum_value.wdl`, so it guards that example.
    assert_accepts(
        &doc(COLOR_VALUED, "Color c = Color.Red"),
        "valued enum, member-access reference",
    );
}

// ═══ Cross-import enum compatibility (Task B1) ═══════════════════════════════
//
// These scenarios need multiple documents, but deliberately do *not* add
// fixtures under `wdl_tests/`. That directory is a shared corpus consumed by
// all five implementations (`python/wdl_tests` and `typescript/wdl_tests` are
// symlinks to it), and no other implementation tests — or even implements —
// cross-import enum compatibility:
//
//   * Java has all three diagnostics (`WdlValidator.java:610`, `:433`, `:444`)
//     but no test exercises any of them; its only import-conflict test is the
//     *struct* one (`WdlImportValidationTest.java:96`).
//   * Python, TypeScript and Go have no enum shape tracking at all — every
//     `enum` symbol in their sources is generated ANTLR code. Go's fixture list
//     even omits `struct_conflict` for the same reason
//     (`go/wdl/import_validation_fixtures_test.go:18-23`).
//
// A fixture there would therefore be read by exactly one language while
// implying a conformance expectation the other four would fail. Promoting
// these to the shared corpus is a deliberate all-languages decision, not a
// side effect of Rust parity work.

/// Serves imported documents from memory, keyed by final path segment.
///
/// Uses a non-`file` URL scheme deliberately: `load_with_resolver_inner`
/// (`src/loader.rs:138-144`) short-circuits `file:` URLs to
/// `std::fs::read_to_string` and never consults the resolver, so a `file://`
/// root would silently fall through to the real filesystem. Any other scheme
/// takes the resolver path, and `resolve_import_uri`'s unknown-scheme fallback
/// (`src/resolvers/mod.rs:226`) URL-joins `"a.wdl"` onto `mem://test/root.wdl`
/// to give `mem://test/a.wdl` — so matching on the last path segment is
/// sufficient here.
struct MemoryResolver(HashMap<String, String>);

impl MemoryResolver {
    fn new(files: &[(&str, &str)]) -> Self {
        Self(
            files
                .iter()
                .map(|(name, src)| ((*name).to_string(), (*src).to_string()))
                .collect(),
        )
    }
}

impl ImportResolver for MemoryResolver {
    fn dispatch_import(
        &self,
        import_url: &Url,
        original_import_location: &str,
    ) -> Result<String, WdlImportError> {
        let name = import_url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .unwrap_or_default();
        self.0
            .get(name)
            .cloned()
            .ok_or_else(|| WdlImportError::InvalidPath {
                location: original_import_location.to_string(),
            })
    }
}

/// Canonical location of the root document; only used to resolve relative
/// import paths against. See [`MemoryResolver`] for why the scheme is not `file`.
fn mem_root() -> Url {
    Url::parse("mem://test/root.wdl").expect("valid URL")
}

/// Loads `root` with `files` available as imports, then runs the base validator.
fn validate_with_imports(root: &str, files: &[(&str, &str)]) -> Vec<WdlSemanticError> {
    let resolver = MemoryResolver::new(files);
    let doc = load_from_str_with_resolver(root, &mem_root(), &resolver)
        .unwrap_or_else(|e| panic!("load failed: {e}\n--- root ---\n{root}"));
    let mut validator = WdlValidator::new();
    let _ = validator.validate(&doc);
    validator.errors().to_vec()
}

fn assert_imports_reject(root: &str, files: &[(&str, &str)], needle: &str, what: &str) {
    let errors = validate_with_imports(root, files);
    assert!(
        errors
            .iter()
            .any(|e| e.code == WdlErrorCode::TypeMismatch && e.message.contains(needle)),
        "{what}: expected a TypeMismatch containing {needle:?}, got: {errors:?}"
    );
}

/// Code-only variant, for the choice-membership path. `is_assignable_from`
/// (`src/validators/mod.rs:542`) returns a bool, so a membership failure is
/// reported with the same generic "Cannot assign ..." text as any other type
/// mismatch — there is no distinguishing message to match on.
fn assert_imports_reject_type_mismatch(root: &str, files: &[(&str, &str)], what: &str) {
    let errors = validate_with_imports(root, files);
    assert!(
        errors.iter().any(|e| e.code == WdlErrorCode::TypeMismatch),
        "{what}: expected a TypeMismatch, got: {errors:?}"
    );
}

fn assert_imports_accept(root: &str, files: &[(&str, &str)], what: &str) {
    let errors = validate_with_imports(root, files);
    assert!(
        errors.is_empty(),
        "{what}: expected zero diagnostics, got: {errors:?}"
    );
}

// Two mutually incompatible `Color` definitions, plus an exact twin of the
// first. Incompatibility is in the choice *values*; the names coincide, so a
// shape comparison that ignored values would not distinguish them.

const A_COLOR_HEX: &str = r##"version 1.3
enum Color[String] {
  Red = "#FF0000",
  Green = "#00FF00"
}
workflow a {}
"##;

const B_COLOR_HEX: &str = r##"version 1.3
enum Color[String] {
  Red = "#FF0000",
  Green = "#00FF00"
}
workflow b {}
"##;

const B_COLOR_WORDS: &str = r##"version 1.3
enum Color[String] {
  Red = "red",
  Green = "green"
}
workflow b {}
"##;

const IMPORT_BOTH: &str = "version 1.3\nimport \"a.wdl\"\nimport \"b.wdl\"\nworkflow root {}\n";

#[test]
fn rejects_incompatible_imported_enums_without_alias() {
    // The B1 regression: `to_enum_shape` was computed and immediately dropped,
    // so two imports could contribute conflicting `Color` types silently.
    assert_imports_reject(
        IMPORT_BOTH,
        &[("a.wdl", A_COLOR_HEX), ("b.wdl", B_COLOR_WORDS)],
        "Imported enum 'Color' has incompatible definitions across imports",
        "two imports, incompatible Color",
    );
}

#[test]
fn accepts_identical_imported_enums() {
    // Compatible shapes must not be treated as a redefinition — the second
    // import simply overwrites the first.
    assert_imports_accept(
        IMPORT_BOTH,
        &[("a.wdl", A_COLOR_HEX), ("b.wdl", B_COLOR_HEX)],
        "two imports, identical Color",
    );
}

#[test]
fn rejects_local_enum_conflicting_with_imported_enum() {
    // Distinct from the case above: imports are indexed before local document
    // elements (`index_top_level_contracts`), so this must report the *local*
    // message, not the cross-import one.
    let root = r##"version 1.3
import "a.wdl"
enum Color[String] {
  Red = "red",
  Green = "green"
}
workflow root {}
"##;
    assert_imports_reject(
        root,
        &[("a.wdl", A_COLOR_HEX)],
        "Enum 'Color' conflicts with imported enum definition",
        "local Color vs imported incompatible Color",
    );
}

#[test]
fn accepts_incompatible_imported_enums_when_aliased() {
    // Every B1 diagnostic advises "use aliases to disambiguate". Before the
    // alias fix, imported types were always indexed under their *source* name,
    // so following that advice changed nothing and the conflict persisted.
    let root = r##"version 1.3
import "a.wdl" as a
  alias Color as ColorA
import "b.wdl" as b
  alias Color as ColorB
workflow root {}
"##;
    assert_imports_accept(
        root,
        &[("a.wdl", A_COLOR_HEX), ("b.wdl", B_COLOR_WORDS)],
        "incompatible Color imports, both aliased apart",
    );
}

// ─── Members-form imports now index types (a C2 hole) ────────────────────────
//
// `import { Color } from "a.wdl"` previously indexed no types at all, so C2's
// choice-membership check silently did nothing on this path.
//
// Of the pair below, `accepts_valid_choice_on_members_imported_enum` is the
// load-bearing one: with no shape indexed, `Color ← String` falls through to
// the plain type check and *both* "Red" and "Purple" are rejected. The reject
// case therefore passes with or without the fix; it is kept as the other half
// of the membership contract, not as proof of it.

const IMPORT_COLOR_MEMBER: &str =
    "version 1.3\nimport { Color } from \"a.wdl\"\nworkflow root {\n  Color c = ";

#[test]
fn rejects_unknown_choice_on_members_imported_enum() {
    let root = format!("{IMPORT_COLOR_MEMBER}\"Purple\"\n}}\n");
    assert_imports_reject_type_mismatch(
        &root,
        &[("a.wdl", A_COLOR_HEX)],
        "members-form import, unknown choice name",
    );
}

#[test]
fn accepts_valid_choice_on_members_imported_enum() {
    let root = format!("{IMPORT_COLOR_MEMBER}\"Red\"\n}}\n");
    assert_imports_accept(
        &root,
        &[("a.wdl", A_COLOR_HEX)],
        "members-form import, valid choice name",
    );
}
