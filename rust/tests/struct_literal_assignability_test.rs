//! Struct-literal assignability (Task B4.4).
//!
//! Java's `isAssignableFrom(WdlType, WdlExpression)` recurses over the
//! *expression* before falling back to comparing inferred types
//! (`WdlExpressionValidator.java:200-256`). For a struct-typed target it
//! delegates to `isStructAssignableFromExpression` (:776-836), which walks the
//! literal's entries member by member. That is what makes map coercion to a
//! struct legal -- `Words w = { "a": 1, ... }` infers as `Map[String, Int]`,
//! which is not assignable to `Words` by type comparison alone. It is also the
//! only place Java type-checks struct literal *members* at all.
//!
//! Rust had no such walk: `infer_type(StructLit)` returns `TypeRef(name)`
//! unconditionally, so a struct literal was accepted on name equality alone,
//! with its members never checked, and a map literal was always rejected.
//!
//! **Deliberate divergence.** Java compares the literal's key set to the
//! struct's member set for *equality*, so it rejects any literal that omits a
//! member -- including an optional one. That is a Java bug: the normative spec
//! example `test_struct.wdl` omits the optional `String? username` and says so
//! in a comment, and running Java's validator over it rejects the file. Java
//! never notices because its spec-example tests only parse valid examples.
//! Adopting the rule would newly reject `test_struct.wdl` and
//! `import_structs.wdl`, which this validator accepts, so omissions are
//! permitted here. Tracked in `java/TODO.md`.
//!
//! **A/B verification.** Every test below was confirmed to fail against
//! unfixed code, but note that simply deleting the whole `is_assignable_from`
//! dispatch branch is *not* a sufficient mutation: with the branch gone a map
//! literal is unassignable to a struct by type comparison alone, so the
//! `map_literal_*_is_rejected` tests keep passing for the wrong reason. Each
//! test records the targeted mutation that actually makes it bite.

use wdl_model::errors::WdlErrorCode;
use wdl_model::loader::load_from_str;
use wdl_model::validators::WdlValidator;

fn validate(src: &str) -> Vec<wdl_model::errors::WdlSemanticError> {
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

const WORDS: &str = "struct Words {\n  Int a\n  Int b\n  Int c\n}";

/// Wraps declarations in a 1.1 document carrying the `Words` struct.
fn words_doc(decl: &str) -> String {
    format!("version 1.1\n\n{WORDS}\n\nworkflow w {{\n  {decl}\n}}\n")
}

// ---- Map coercion to a struct (the point of the change) --------------------

/// Reduced from the spec example `map_to_struct.wdl`.
///
/// Bites when the `is_assignable_from` struct-literal branch is removed.
#[test]
fn map_literal_with_matching_members_is_assignable_to_struct() {
    assert_accepts(
        &words_doc(r#"Words coerced = { "a": 10, "b": 11, "c": 12 }"#),
        "map literal coerced to a struct",
    );
}

/// Bites when `keyed_entries_match_members` stops checking entry values
/// (`is_assignable_from(ty, expr)` -> `true`). Does *not* bite on removing the
/// dispatch branch, which would reject this for the wrong reason.
#[test]
fn map_literal_with_wrong_member_type_is_rejected() {
    assert_rejects_with_type_mismatch(
        &words_doc(r#"Words coerced = { "a": 10, "b": "not an int", "c": 12 }"#),
        "map literal with a String where the struct declares Int",
    );
}

/// Unknown keys are rejected twice over: by the explicit `contains_key` guard
/// and again by the `is_some_and` lookup in the fold. Bites only when *both*
/// are defeated, and not on removing the dispatch branch.
#[test]
fn map_literal_with_unknown_member_is_rejected() {
    assert_rejects_with_type_mismatch(
        &words_doc(r#"Words coerced = { "a": 10, "b": 11, "zzz": 12 }"#),
        "map literal naming a member the struct does not declare",
    );
}

/// Bites when the walk's non-string-key rejection is softened (`return false`
/// -> skip the entry). Does *not* bite on removing the dispatch branch: such a
/// map was already unassignable to a struct by type comparison.
#[test]
fn map_literal_with_non_string_key_is_rejected() {
    assert_rejects_with_type_mismatch(
        &words_doc("Words coerced = { 1: 10, 2: 11, 3: 12 }"),
        "map literal with Int keys assigned to a struct",
    );
}

// ---- Struct literal members are now actually checked -----------------------

#[test]
fn struct_literal_with_matching_members_is_accepted() {
    assert_accepts(
        &words_doc("Words lit = Words { a: 10, b: 11, c: 12 }"),
        "well-formed struct literal",
    );
}

/// Bites on removing the dispatch branch, and again on defeating the entry
/// value check. Before B4.4 a struct literal was accepted on name alone.
#[test]
fn struct_literal_with_wrong_member_type_is_rejected() {
    assert_rejects_with_type_mismatch(
        &words_doc(r#"Words lit = Words { a: 10, b: "not an int", c: 12 }"#),
        "struct literal with a String where the struct declares Int",
    );
}

/// Bites on removing the dispatch branch.
#[test]
fn struct_literal_with_unknown_member_is_rejected() {
    assert_rejects_with_type_mismatch(
        &words_doc("Words lit = Words { a: 10, b: 11, zzz: 12 }"),
        "struct literal naming a member the struct does not declare",
    );
}

/// Bites on removing the dispatch branch. Pins that the walk recurses: the
/// inner literal is reached through `is_assignable_from` on the member type.
#[test]
fn nested_struct_literal_members_are_checked() {
    assert_rejects_with_type_mismatch(
        r#"
version 1.1

struct Inner {
  Int v
}

struct Outer {
  Inner inner
}

workflow w {
  Outer o = Outer { inner: Inner { v: "not an int" } }
}
"#,
        "bad member type one level down",
    );
}

// ---- Struct literal name compatibility -------------------------------------

/// Isolates the literal's name check (Java :785-791). `{ a: 1 }` is otherwise a
/// perfectly valid `Words` literal under this validator's omission rule, so
/// only the `Other` vs `Words` name comparison can reject it. Bites when that
/// comparison is removed.
#[test]
fn struct_literal_naming_an_incompatible_struct_is_rejected() {
    assert_rejects_with_type_mismatch(
        r#"
version 1.1

struct Words {
  Int a
  Int b
  Int c
}

struct Other {
  Int a
}

workflow w {
  Words lit = Other { a: 1 }
}
"#,
        "struct literal naming a structurally incompatible struct",
    );
}

/// The converse: a differently-named but structurally identical struct is
/// accepted, because the name check defers to `is_type_assignable`, which
/// implements WDL's structural struct-to-struct coercion rather than demanding
/// name equality.
#[test]
fn struct_literal_naming_a_structurally_identical_struct_is_accepted() {
    assert_accepts(
        r#"
version 1.1

struct Words {
  Int a
  Int b
  Int c
}

struct Twin {
  Int a
  Int b
  Int c
}

workflow w {
  Words lit = Twin { a: 1, b: 2, c: 3 }
}
"#,
        "struct literal naming a structurally identical struct",
    );
}

// ---- Omitted members: the deliberate divergence from Java ------------------

/// Reduced from the spec example `test_struct.wdl`, whose own comment reads
/// "it's okay to leave out username since it's optional". Java rejects this.
///
/// Bites on removing the dispatch branch, and on adopting Java's key-set
/// equality rule.
#[test]
fn map_literal_omitting_an_optional_member_is_accepted() {
    assert_accepts(
        r#"
version 1.1

struct Person {
  String name
  String? nickname
}

workflow w {
  Person p = { "name": "Sam" }
}
"#,
        "map literal omitting an optional struct member",
    );
}

/// The divergence pin. Does *not* bite on removing the dispatch branch (struct
/// literals were accepted on name alone before B4.4, so omission was already
/// permitted), but does bite on adopting Java's key-set equality rule, which
/// rejects it with the nonsensical "Cannot assign Person to type 'Person'".
#[test]
fn struct_literal_omitting_an_optional_member_is_accepted() {
    assert_accepts(
        r#"
version 1.1

struct Person {
  String name
  String? nickname
}

workflow w {
  Person p = Person { name: "Sam" }
}
"#,
        "struct literal omitting an optional struct member",
    );
}
