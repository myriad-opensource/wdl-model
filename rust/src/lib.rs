//! WDL (Workflow Description Language) object model, parser, traversal, and validation library.
//!
//! This is a Rust implementation mirroring the Java `wdl-model` library.
//! It supports WDL 1.0 through 1.3.

// Generated ANTLR4 grammar — suppress all lints on generated code.
pub mod grammar;

// Model layers
pub mod base;
pub mod definitions;
pub mod document;
pub mod errors;
pub mod expressions;
pub mod sections;
pub mod statements;
pub mod types;
pub mod version;

// Operational layers (stubs — filled in later phases)
pub mod loader;
pub mod processors;
pub mod resolvers;
pub mod validators;
