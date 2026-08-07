//! WDL top-level definition model (task, workflow, struct, enum).

use crate::expressions::WdlExpression;
use crate::sections::{
    WdlCommand, WdlInput, WdlMetadata, WdlOutput, WdlParameterMetadata, WdlRequirements,
    WdlRuntime, WdlTaskHints, WdlWorkflowHints,
};
use crate::statements::{WdlBoundDeclaration, WdlCall, WdlConditional, WdlDeclaration, WdlScatter};
use crate::types::WdlType;

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

/// Any element that can appear directly inside a task body.
/// Mirrors the `WdlTask.WdlTaskElement` marker interface.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlTaskElement {
    Input(WdlInput),
    Output(WdlOutput),
    Command(WdlCommand),
    Requirements(WdlRequirements),
    Runtime(WdlRuntime),
    Hints(WdlTaskHints),
    Meta(WdlMetadata),
    ParameterMeta(WdlParameterMetadata),
    Declaration(WdlDeclaration),
    BoundDeclaration(WdlBoundDeclaration),
}

/// Task definition node.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlTask {
    pub name: String,
    pub elements: Vec<WdlTaskElement>,
}

impl WdlTask {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Workflow
// ---------------------------------------------------------------------------

/// Any element that can appear directly inside a workflow body.
/// Mirrors the `WdlWorkflow.WdlWorkflowElement` marker interface.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlWorkflowElement {
    Input(WdlInput),
    Output(WdlOutput),
    Hints(WdlWorkflowHints),
    Meta(WdlMetadata),
    ParameterMeta(WdlParameterMetadata),
    Call(WdlCall),
    Scatter(WdlScatter),
    Conditional(WdlConditional),
    Declaration(WdlDeclaration),
    BoundDeclaration(WdlBoundDeclaration),
}

/// Workflow definition node.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlWorkflow {
    pub name: String,
    pub elements: Vec<WdlWorkflowElement>,
}

impl WdlWorkflow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Struct
// ---------------------------------------------------------------------------

/// Any element that can appear directly inside a struct body.
/// Mirrors the `WdlStruct.WdlStructElement` marker interface.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlStructElement {
    Member(WdlStructMember),
    Meta(WdlMetadata),
    ParameterMeta(WdlParameterMetadata),
}

/// A single typed member declaration inside a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlStructMember {
    pub wdl_type: WdlType,
    pub name: String,
}

impl WdlStructMember {
    pub fn new(wdl_type: WdlType, name: impl Into<String>) -> Self {
        Self {
            wdl_type,
            name: name.into(),
        }
    }
}

/// Struct definition node.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlStruct {
    pub name: String,
    pub elements: Vec<WdlStructElement>,
}

impl WdlStruct {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Enum
// ---------------------------------------------------------------------------

/// A single named choice inside an enum definition, with an optional value expression.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlEnumChoice {
    pub name: String,
    pub value: Option<WdlExpression>,
}

impl WdlEnumChoice {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn with_value(name: impl Into<String>, value: WdlExpression) -> Self {
        Self {
            name: name.into(),
            value: Some(value),
        }
    }
}

/// Enum definition node (WDL 1.3+).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlEnum {
    pub name: String,
    /// Optional explicit value type for the enum choices.
    pub value_type: Option<WdlType>,
    pub elements: Vec<WdlEnumChoice>,
}

impl WdlEnum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value_type: None,
            elements: Vec::new(),
        }
    }
}
