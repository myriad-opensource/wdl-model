//! Generated ANTLR4 lexer and parser for WDL v1.
#![allow(unused_parens)]
//!
//! The contents of this module are generated at build time by `build.rs` from:
//!   wdl-grammar/antlr4/v1/WdlV1Lexer.g4
//!   wdl-grammar/antlr4/v1/WdlV1Parser.g4
//! (consumed via the `rust/antlr4` symlink -> `../wdl-grammar/antlr4`)
//!
//! Nothing here is checked into the repository. This mirrors the `java/`
//! module, where `antlr4-maven-plugin` regenerates into
//! `target/generated-sources/antlr4/` on every build, and means the parser
//! cannot drift out of sync with the grammar submodule. Building therefore
//! requires a JRE; the ANTLR tool itself is vendored as
//! `rust/antlr4-rust-tool.jar`.
//!
//! Post-generation patch: WdlV1ParserParserContext renamed to WdlV1ParserContext.
//!
//! `grammar_modules.rs` is written by `build.rs` and declares one `pub mod` per
//! generated file, pointing at `$OUT_DIR` via `#[path]`.

include!(concat!(env!("OUT_DIR"), "/grammar_modules.rs"));
