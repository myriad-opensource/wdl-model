//! Root document node for a parsed WDL source file.

use crate::definitions::{WdlEnum, WdlStruct, WdlTask, WdlWorkflow};
use crate::statements::WdlImport;
use crate::version::WdlVersion;
use indexmap::IndexMap;

/// Any top-level element that can appear directly in a WDL document.
/// Mirrors the `WdlDocument.WdlDocumentElement` marker interface.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlDocumentElement {
    Import(WdlImport),
    Struct(WdlStruct),
    Enum(WdlEnum),
    Task(WdlTask),
    Workflow(WdlWorkflow),
}

/// Root node for a parsed WDL document.
///
/// Mirrors `WdlDocument` in the Java implementation. Preserves the source-order
/// list of top-level elements and the map of resolved imported documents.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlDocument {
    pub wdl_version: Option<WdlVersion>,
    pub source_location: Option<String>,
    pub elements: Vec<WdlDocumentElement>,
    /// Imported documents keyed by their resolved import identifier URI.
    /// Uses `IndexMap` to preserve insertion order, mirroring Java's `LinkedHashMap`.
    pub imported_documents: IndexMap<String, WdlDocument>,
}

impl WdlDocument {
    pub fn new() -> Self {
        Self {
            wdl_version: None,
            source_location: None,
            elements: Vec::new(),
            imported_documents: IndexMap::new(),
        }
    }

    pub fn with_version(version: WdlVersion) -> Self {
        Self {
            wdl_version: Some(version),
            source_location: None,
            elements: Vec::new(),
            imported_documents: IndexMap::new(),
        }
    }

    /// Returns only top-level import statements.
    pub fn import_statements(&self) -> impl Iterator<Item = &WdlImport> {
        self.elements.iter().filter_map(|e| {
            if let WdlDocumentElement::Import(i) = e {
                Some(i)
            } else {
                None
            }
        })
    }

    /// Returns only top-level enum definitions.
    pub fn enums(&self) -> impl Iterator<Item = &WdlEnum> {
        self.elements.iter().filter_map(|e| {
            if let WdlDocumentElement::Enum(en) = e {
                Some(en)
            } else {
                None
            }
        })
    }

    /// Returns only top-level struct definitions.
    pub fn structs(&self) -> impl Iterator<Item = &WdlStruct> {
        self.elements.iter().filter_map(|e| {
            if let WdlDocumentElement::Struct(s) = e {
                Some(s)
            } else {
                None
            }
        })
    }

    /// Returns only top-level task definitions.
    pub fn tasks(&self) -> impl Iterator<Item = &WdlTask> {
        self.elements.iter().filter_map(|e| {
            if let WdlDocumentElement::Task(t) = e {
                Some(t)
            } else {
                None
            }
        })
    }

    /// Returns only top-level workflow definitions.
    pub fn workflows(&self) -> impl Iterator<Item = &WdlWorkflow> {
        self.elements.iter().filter_map(|e| {
            if let WdlDocumentElement::Workflow(w) = e {
                Some(w)
            } else {
                None
            }
        })
    }
}

impl Default for WdlDocument {
    fn default() -> Self {
        Self::new()
    }
}
