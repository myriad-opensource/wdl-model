//! Type-alias rewriting for imported structs (Task B4.2).
//!
//! When a document imports a struct under an alias, that struct's *member*
//! types still name types using the source document's names. Java rebuilds the
//! imported shape with every type reference renamed through the import's alias
//! map (`toImportedStructShape` / `rewriteTypeAliases`,
//! `WdlValidator.java:528-600`). Rust indexed imported structs with
//! `to_struct_shape`, which does no rewriting, so a member whose type was
//! itself an aliased struct kept a name that was never indexed locally.
//!
//! The gap was latent: nothing consulted imported struct *member* types until
//! `is_assignable_from` gained its struct-literal walk, at which point
//! `wdl_tests/non_runtime_completion/import_alias_nested` started failing with
//! `Cannot assign PersonAlias to type 'PersonAlias'` -- the outer names matched
//! but the nested member type `Address` resolved to nothing, because only the
//! alias `Addr` had been indexed.
//!
//! That fixture only covers a bare `TypeRef` member. These tests cover the
//! recursive arms -- `Array`, `Map`, `Pair`, and nesting of those -- which
//! `rewrite_type_aliases` also has to handle and which nothing else exercises.

use std::collections::HashMap;

use url::Url;
use wdl_model::errors::WdlSemanticError;
use wdl_model::loader::load_from_str_with_resolver;
use wdl_model::resolvers::{ImportResolver, WdlImportError};
use wdl_model::validators::WdlValidator;

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

fn mem_root() -> Url {
    Url::parse("mem://test/root.wdl").expect("valid URL")
}

fn validate_with_imports(root: &str, files: &[(&str, &str)]) -> Vec<WdlSemanticError> {
    let resolver = MemoryResolver::new(files);
    let doc = load_from_str_with_resolver(root, &mem_root(), &resolver)
        .unwrap_or_else(|e| panic!("load failed: {e}\n--- root ---\n{root}"));
    let mut validator = WdlValidator::new();
    let _ = validator.validate(&doc);
    validator.errors().to_vec()
}

fn assert_accepts(root: &str, files: &[(&str, &str)], what: &str) {
    let errors = validate_with_imports(root, files);
    assert!(
        errors.is_empty(),
        "{what}: expected zero diagnostics, got: {errors:?}"
    );
}

/// `Inner` is aliased to `Thing`; `Outer.items` is `Array[Inner]` in the source
/// and must become `Array[Thing]` locally.
#[test]
fn rewrites_alias_inside_array_member() {
    assert_accepts(
        r#"
version 1.3

import "lib.wdl"
  alias Inner as Thing
  alias Outer as Box

workflow root {
  Box b = Box { items: [Thing { v: 1 }] }
  output {
    Int first = b.items[0].v
  }
}
"#,
        &[(
            "lib.wdl",
            r#"
version 1.3

struct Inner {
  Int v
}

struct Outer {
  Array[Inner] items
}

workflow lib {}
"#,
        )],
        "Array[Inner] member rewritten to Array[Thing]",
    );
}

/// `Map[String, Inner]` must have only its *value* type rewritten.
#[test]
fn rewrites_alias_inside_map_value() {
    assert_accepts(
        r#"
version 1.3

import "lib.wdl"
  alias Inner as Thing
  alias Outer as Box

workflow root {
  Box b = Box { lookup: { "k": Thing { v: 1 } } }
  output {
    Int one = b.lookup["k"].v
  }
}
"#,
        &[(
            "lib.wdl",
            r#"
version 1.3

struct Inner {
  Int v
}

struct Outer {
  Map[String, Inner] lookup
}

workflow lib {}
"#,
        )],
        "Map[String, Inner] member rewritten to Map[String, Thing]",
    );
}

/// Both components of a `Pair` must be rewritten independently.
#[test]
fn rewrites_alias_inside_both_pair_components() {
    assert_accepts(
        r#"
version 1.3

import "lib.wdl"
  alias Left as L
  alias Right as R
  alias Outer as Box

workflow root {
  Box b = Box { both: (L { a: 1 }, R { b: 2 }) }
  output {
    Int x = b.both.left.a
  }
}
"#,
        &[(
            "lib.wdl",
            r#"
version 1.3

struct Left {
  Int a
}

struct Right {
  Int b
}

struct Outer {
  Pair[Left, Right] both
}

workflow lib {}
"#,
        )],
        "Pair[Left, Right] member rewritten to Pair[L, R]",
    );
}

/// Nested containers must be rewritten all the way down.
#[test]
fn rewrites_alias_inside_nested_containers() {
    assert_accepts(
        r#"
version 1.3

import "lib.wdl"
  alias Inner as Thing
  alias Outer as Box

workflow root {
  Box b = Box { grid: [[Thing { v: 1 }]] }
  output {
    Int one = b.grid[0][0].v
  }
}
"#,
        &[(
            "lib.wdl",
            r#"
version 1.3

struct Inner {
  Int v
}

struct Outer {
  Array[Array[Inner]] grid
}

workflow lib {}
"#,
        )],
        "Array[Array[Inner]] member rewritten to Array[Array[Thing]]",
    );
}

/// An un-aliased import must leave member type names untouched. Guards against
/// a rewrite map that renames when it should not.
#[test]
fn leaves_member_types_untouched_without_aliases() {
    assert_accepts(
        r#"
version 1.3

import "lib.wdl"

workflow root {
  Outer b = Outer { items: [Inner { v: 1 }] }
  output {
    Int first = b.items[0].v
  }
}
"#,
        &[(
            "lib.wdl",
            r#"
version 1.3

struct Inner {
  Int v
}

struct Outer {
  Array[Inner] items
}

workflow lib {}
"#,
        )],
        "unaliased import keeps source member type names",
    );
}
