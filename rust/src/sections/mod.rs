//! WDL section model (input, output, command, runtime, requirements, hints, metadata).

use crate::expressions::{WdlExpression, WdlStringLiteral};
use crate::statements::{WdlBoundDeclaration, WdlDeclaration};

// ---------------------------------------------------------------------------
// input / output sections
// ---------------------------------------------------------------------------

/// An element inside an `input { … }` section.
///
/// WDL input declarations may be either unbound (`String name`) or bound with
/// a default value (`String name = "default"`).  This mirrors Java's
/// `WdlDeclaration` / `WdlBoundDeclaration` inheritance: both can appear in
/// `WdlInput.elements`.
#[derive(Debug, Clone, PartialEq)]
pub enum InputDeclaration {
    Unbound(WdlDeclaration),
    Bound(WdlBoundDeclaration),
}

impl InputDeclaration {
    /// The declared name, regardless of whether a default is present.
    pub fn name(&self) -> &str {
        match self {
            InputDeclaration::Unbound(d) => &d.name,
            InputDeclaration::Bound(d) => &d.name,
        }
    }
}

/// `input { … }` section — may appear in tasks and workflows.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlInput {
    pub elements: Vec<InputDeclaration>,
}

impl WdlInput {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlInput {
    fn default() -> Self {
        Self::new()
    }
}

/// `output { … }` section — declarations are always bound.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlOutput {
    pub elements: Vec<WdlBoundDeclaration>,
}

impl WdlOutput {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlOutput {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// command section
// ---------------------------------------------------------------------------

/// `command { … }` or `command <<< … >>>` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlCommand {
    pub command_text: WdlStringLiteral,
    /// `true` for the `<<< … >>>` multiline form.
    pub multiline: bool,
}

impl WdlCommand {
    pub fn new(command_text: WdlStringLiteral, multiline: bool) -> Self {
        Self {
            command_text,
            multiline,
        }
    }
}

// ---------------------------------------------------------------------------
// runtime section (legacy)
// ---------------------------------------------------------------------------

/// Single `key: value` entry in a `runtime { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlRuntimeEntry {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlRuntimeEntry {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// Legacy `runtime { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlRuntime {
    pub elements: Vec<WdlRuntimeEntry>,
}

impl WdlRuntime {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// requirements section
// ---------------------------------------------------------------------------

/// Single `key: value` entry in a `requirements { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlRequirementEntry {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlRequirementEntry {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// `requirements { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlRequirements {
    pub elements: Vec<WdlRequirementEntry>,
}

impl WdlRequirements {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlRequirements {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// hints sections
// ---------------------------------------------------------------------------

/// Single `key: value` entry in a task hints section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlTaskHint {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlTaskHint {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// `hints { … }` section inside a task.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlTaskHints {
    pub elements: Vec<WdlTaskHint>,
}

impl WdlTaskHints {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlTaskHints {
    fn default() -> Self {
        Self::new()
    }
}

/// Single `key: value` entry in a workflow hints section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlWorkflowHint {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlWorkflowHint {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// `hints { … }` section inside a workflow.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlWorkflowHints {
    pub elements: Vec<WdlWorkflowHint>,
}

impl WdlWorkflowHints {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlWorkflowHints {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// metadata sections
// ---------------------------------------------------------------------------

/// Single `key: value` entry in a `meta` or `parameter_meta` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMetadataEntry {
    pub key: String,
    pub value: Option<WdlExpression>,
}

impl WdlMetadataEntry {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn with_value(key: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// `meta { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMetadata {
    pub elements: Vec<WdlMetadataEntry>,
}

impl WdlMetadata {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// `parameter_meta { … }` section.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlParameterMetadata {
    pub elements: Vec<WdlMetadataEntry>,
}

impl WdlParameterMetadata {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for WdlParameterMetadata {
    fn default() -> Self {
        Self::new()
    }
}
