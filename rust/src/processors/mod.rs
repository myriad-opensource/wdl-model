//! WDL processor (visitor/traversal) framework.
//!
//! ## Module overview
//!
//! | Module | Purpose |
//! |---|---|
//! | [`render`] | Free-function helpers: model node → WDL source text |
//! | [`base`] | `WdlProcessor` trait + `ResolvedImport` + import resolution free functions |
//! | [`expression`] | `WdlExpressionProcessor` trait + depth-first expression walker |
//! | [`function`] | `WdlFunctionProcessor` trait + per-function dispatch |
//! | [`appending`] | `WdlAppendingProcessor` — concrete pretty-printer |
//!
//! ## Quick start
//!
//! ```no_run
//! use wdl_model::processors::appending::WdlAppendingProcessor;
//! use wdl_model::processors::base::WdlProcessor;
//!
//! # let doc = wdl_model::document::WdlDocument::new();
//! let mut ap = WdlAppendingProcessor::new();
//! ap.process_document(&doc);
//! println!("{}", ap.into_string());
//! ```

pub mod appending;
pub mod base;
pub mod expression;
pub mod function;
pub mod render;

// Re-export the most-used items at the module root for convenience.
pub use appending::WdlAppendingProcessor;
pub use base::{
    WdlProcessor,
    ResolvedImport,
    import_namespace,
    resolve_imported_enums,
    resolve_imported_structs,
    resolve_imported_tasks,
    resolve_imported_workflows,
};
pub use expression::WdlExpressionProcessor;
pub use function::WdlFunctionProcessor;
pub use render::{
    bound_declaration_to_wdl,
    expression_to_wdl,
    input_declaration_to_wdl,
    string_literal_to_wdl,
    type_to_wdl,
    unbound_declaration_to_wdl,
};
