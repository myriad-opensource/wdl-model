//! `WdlAppendingProcessor` — renders the WDL object model back into source text.
//!
//! This is a concrete implementation of `WdlProcessor` that walks the document model
//! and appends a WDL representation to an internal `String` buffer.  It is directly
//! analogous to Java's `WdlAppendingProcessor`.
//!
//! ## Usage
//!
//! ```no_run
//! use wdl_model::processors::appending::WdlAppendingProcessor;
//! use wdl_model::processors::base::WdlProcessor;
//!
//! // Assume `doc` is a parsed `WdlDocument`.
//! # let doc = wdl_model::document::WdlDocument::new();
//! let mut ap = WdlAppendingProcessor::new();
//! ap.process_document(&doc);
//! let wdl_text = ap.into_string();
//! ```

use crate::definitions::{WdlEnum, WdlStruct, WdlStructMember, WdlTask, WdlWorkflow};
use crate::document::WdlDocument;
use crate::processors::base::WdlProcessor;
use crate::processors::render::{
    bound_declaration_to_wdl, expression_to_wdl, input_declaration_to_wdl,
    string_literal_to_wdl, type_to_wdl,
};
use crate::sections::{
    WdlCommand, WdlInput, WdlMetadata, WdlOutput, WdlParameterMetadata, WdlRequirements,
    WdlRuntime, WdlTaskHints, WdlWorkflowHints,
};
use crate::statements::{
    WdlBoundDeclaration, WdlCall, WdlConditional, WdlConditionalElseIf, WdlImportMembers,
    WdlImportStandard, WdlImportStar, WdlScatter, WdlStatement,
};
use crate::version::WdlVersion;

/// Renders the WDL object model back into source text.
///
/// Mirrors Java's `WdlAppendingProcessor`.
pub struct WdlAppendingProcessor {
    /// The accumulated rendered WDL text.
    pub out: String,
}

impl WdlAppendingProcessor {
    /// Create a new processor with an empty output buffer.
    pub fn new() -> Self {
        Self { out: String::new() }
    }

    /// Consume the processor and return the rendered WDL text.
    pub fn into_string(self) -> String {
        self.out
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn append(&mut self, s: &str) {
        self.out.push_str(s);
    }

    /// Append `indent_level + 1` two-space indents (mirrors Java's `indent(level)`).
    fn indent(&mut self, indent_level: usize) {
        for _ in 0..=indent_level {
            self.out.push_str("  ");
        }
    }

    // -----------------------------------------------------------------------
    // Section renderers — shared between task and workflow
    // -----------------------------------------------------------------------

    fn render_input(&mut self, node: &WdlInput) {
        self.append("  input {\n");
        for decl in &node.elements {
            self.append("    ");
            self.append(&input_declaration_to_wdl(decl));
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn render_output(&mut self, node: &WdlOutput) {
        self.append("  output {\n");
        for decl in &node.elements {
            self.append("    ");
            self.append(&bound_declaration_to_wdl(decl));
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn render_metadata(&mut self, node: &WdlMetadata) {
        self.append("  meta {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            self.append(":");
            if let Some(val) = &entry.value {
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn render_parameter_metadata(&mut self, node: &WdlParameterMetadata) {
        self.append("  parameter_meta {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            self.append(":");
            if let Some(val) = &entry.value {
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn render_task_hints(&mut self, node: &WdlTaskHints) {
        self.append("  hints {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            if let Some(val) = &entry.value {
                self.append(": ");
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn render_workflow_hints(&mut self, node: &WdlWorkflowHints) {
        self.append("  hints {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            if let Some(val) = &entry.value {
                self.append(": ");
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    // -----------------------------------------------------------------------
    // Workflow statement renderer (recursive for scatter / conditional)
    // -----------------------------------------------------------------------

    fn render_statement(&mut self, stmt: &WdlStatement, indent_level: usize) {
        match stmt {
            WdlStatement::BoundDeclaration(d) => self.render_stmt_declaration(d, indent_level),
            WdlStatement::Declaration(d) => {
                // Unbound declarations inside workflow bodies are a model artifact.
                debug_assert!(
                    false,
                    "Unexpected unbound declaration in workflow statement — possible loader bug"
                );
                // In release builds, render it as best we can.
                self.indent(indent_level);
                self.append(&type_to_wdl(&d.wdl_type));
                self.append(" ");
                self.append(&d.name);
                self.append("\n");
            }
            WdlStatement::Call(c) => self.render_stmt_call(c, indent_level),
            WdlStatement::Scatter(s) => self.render_stmt_scatter(s, indent_level),
            WdlStatement::Conditional(c) => self.render_stmt_conditional(c, indent_level),
        }
    }

    fn render_stmt_declaration(&mut self, node: &WdlBoundDeclaration, indent_level: usize) {
        self.indent(indent_level);
        self.append(&bound_declaration_to_wdl(node));
        self.append("\n");
    }

    fn render_stmt_call(&mut self, node: &WdlCall, indent_level: usize) {
        self.indent(indent_level);
        self.append("call ");
        self.append(&node.target_path_as_string());
        if let Some(alias) = &node.alias {
            self.append(" as ");
            self.append(alias);
        }
        for dep in &node.after_dependencies {
            self.append(" after ");
            self.append(dep);
        }
        if !node.inputs.is_empty() {
            self.append("  {");
            if node.legacy_input_colon_used {
                self.append(" input: ");
            }
            let parts: Vec<String> = node
                .inputs
                .iter()
                .map(|i| {
                    if let Some(val) = &i.value {
                        format!("{} = {}", i.key, expression_to_wdl(val))
                    } else {
                        i.key.clone()
                    }
                })
                .collect();
            self.append(&parts.join(", "));
            self.append("  }");
        }
        self.append("\n");
    }

    fn render_stmt_scatter(&mut self, node: &WdlScatter, indent_level: usize) {
        self.indent(indent_level);
        self.append("scatter (");
        self.append(&node.name);
        self.append(" in ");
        self.append(&expression_to_wdl(&node.collection));
        self.append(") {\n");
        let stmts: Vec<_> = node.statements.iter().collect();
        for s in stmts {
            self.render_statement(s, indent_level + 1);
        }
        self.indent(indent_level);
        self.append("}\n");
    }

    fn render_stmt_conditional(&mut self, node: &WdlConditional, indent_level: usize) {
        self.indent(indent_level);
        self.append("if (");
        self.append(&expression_to_wdl(&node.condition));
        self.append(") {\n");
        let then_stmts: Vec<_> = node.then_statements.iter().collect();
        for s in then_stmts {
            self.render_statement(s, indent_level + 1);
        }
        self.indent(indent_level);
        self.append("}");

        // else if chains
        let else_ifs: Vec<_> = node.else_ifs.iter().collect();
        for elif in else_ifs {
            self.render_else_if(elif, indent_level);
        }

        // else block
        if !node.else_statements.is_empty() {
            self.append(" else {\n");
            let else_stmts: Vec<_> = node.else_statements.iter().collect();
            for s in else_stmts {
                self.render_statement(s, indent_level + 1);
            }
            self.indent(indent_level);
            self.append("}");
        }

        self.append("\n");
    }

    fn render_else_if(&mut self, node: &WdlConditionalElseIf, indent_level: usize) {
        self.append(" else if (");
        self.append(&expression_to_wdl(&node.condition));
        self.append(") {\n");
        let stmts: Vec<_> = node.then_statements.iter().collect();
        for s in stmts {
            self.render_statement(s, indent_level + 1);
        }
        self.indent(indent_level);
        self.append("}");
    }
}

impl Default for WdlAppendingProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// WdlProcessor implementation
// ---------------------------------------------------------------------------

impl WdlProcessor for WdlAppendingProcessor {
    // -----------------------------------------------------------------------
    // Version + imports
    // -----------------------------------------------------------------------

    fn process_version(&mut self, _ctx: &WdlDocument, node: &WdlVersion) {
        self.append("version ");
        self.append(node.version_string());
        self.append("\n");
    }

    fn process_import_standard(&mut self, _ctx: &WdlDocument, node: &WdlImportStandard) {
        self.append("import ");
        self.append(&string_literal_to_wdl(&node.source, true));
        if let Some(alias) = &node.alias {
            self.append(" as ");
            self.append(alias);
        }
        if !node.members.is_empty() {
            self.append("\n");
            let parts: Vec<String> = node
                .members
                .iter()
                .map(|a| {
                    if let Some(alias) = &a.alias {
                        format!("  alias {} as {}", a.member, alias)
                    } else {
                        format!("  alias {}", a.member)
                    }
                })
                .collect();
            self.append(&parts.join("\n"));
        }
        self.append("\n");
    }

    fn process_import_members(&mut self, _ctx: &WdlDocument, node: &WdlImportMembers) {
        self.append("import { ");
        let parts: Vec<String> = node
            .members
            .iter()
            .map(|m| {
                if let Some(alias) = &m.alias {
                    format!("{} as {}", m.member, alias)
                } else {
                    m.member.clone()
                }
            })
            .collect();
        self.append(&parts.join(", "));
        self.append(" } from ");
        self.append(&string_literal_to_wdl(&node.source, true));
        self.append("\n");
    }

    fn process_import_star(&mut self, _ctx: &WdlDocument, node: &WdlImportStar) {
        self.append("import * from ");
        self.append(&string_literal_to_wdl(&node.source, true));
        self.append("\n");
    }

    // -----------------------------------------------------------------------
    // Enum
    // -----------------------------------------------------------------------

    fn process_enum(&mut self, _ctx: &WdlDocument, node: &WdlEnum) {
        self.append("enum ");
        self.append(&node.name);
        if let Some(vtype) = &node.value_type {
            self.append("[");
            self.append(&type_to_wdl(vtype));
            self.append("]");
        }
        self.append(" {\n");
        let parts: Vec<String> = node
            .elements
            .iter()
            .map(|c| {
                if let Some(val) = &c.value {
                    format!("  {} = {}", c.name, expression_to_wdl(val))
                } else {
                    format!("  {}", c.name)
                }
            })
            .collect();
        self.append(&parts.join(",\n"));
        self.append("\n}\n");
    }

    // -----------------------------------------------------------------------
    // Struct
    // -----------------------------------------------------------------------

    fn process_struct(&mut self, ctx: &WdlDocument, node: &WdlStruct) {
        self.append("struct ");
        self.append(&node.name);
        self.append("{\n");
        // Delegate to default traversal (which calls child callbacks).
        use crate::definitions::WdlStructElement;
        for elem in &node.elements {
            match elem {
                WdlStructElement::Member(m) => self.process_struct_member(node, m),
                WdlStructElement::Meta(m) => self.process_struct_metadata(node, m),
                WdlStructElement::ParameterMeta(m) => {
                    self.process_struct_parameter_metadata(node, m)
                }
            }
        }
        let _ = ctx; // unused — kept for trait signature
        self.append("}\n");
    }

    fn process_struct_member(&mut self, _ctx: &WdlStruct, node: &WdlStructMember) {
        self.append("  ");
        self.append(&type_to_wdl(&node.wdl_type));
        self.append(" ");
        self.append(&node.name);
        self.append("\n");
    }

    fn process_struct_parameter_metadata(
        &mut self,
        _ctx: &WdlStruct,
        node: &WdlParameterMetadata,
    ) {
        self.render_parameter_metadata(node);
    }

    fn process_struct_metadata(&mut self, _ctx: &WdlStruct, node: &WdlMetadata) {
        self.render_metadata(node);
    }

    // -----------------------------------------------------------------------
    // Task
    // -----------------------------------------------------------------------

    fn process_task(&mut self, ctx: &WdlDocument, node: &WdlTask) {
        self.append("task ");
        self.append(&node.name);
        self.append("{\n");
        // Delegate to default traversal.
        use crate::definitions::WdlTaskElement;
        for elem in &node.elements {
            match elem {
                WdlTaskElement::BoundDeclaration(d) => self.process_task_declaration(node, d),
                WdlTaskElement::Declaration(_) => {}
                WdlTaskElement::Input(s) => self.process_task_input(node, s),
                WdlTaskElement::Output(s) => self.process_task_output(node, s),
                WdlTaskElement::Command(s) => self.process_task_command(node, s),
                WdlTaskElement::Meta(s) => self.process_task_metadata(node, s),
                WdlTaskElement::ParameterMeta(s) => {
                    self.process_task_parameter_metadata(node, s)
                }
                WdlTaskElement::Requirements(s) => self.process_task_requirements(node, s),
                WdlTaskElement::Runtime(s) => self.process_task_runtime(node, s),
                WdlTaskElement::Hints(s) => self.process_task_hints(node, s),
            }
        }
        let _ = ctx;
        self.append("}\n");
    }

    fn process_task_declaration(&mut self, _ctx: &WdlTask, node: &WdlBoundDeclaration) {
        self.append("  ");
        self.append(&bound_declaration_to_wdl(node));
        self.append("\n");
    }

    fn process_task_input(&mut self, _ctx: &WdlTask, node: &WdlInput) {
        self.render_input(node);
    }

    fn process_task_output(&mut self, _ctx: &WdlTask, node: &WdlOutput) {
        self.render_output(node);
    }

    fn process_task_command(&mut self, _ctx: &WdlTask, node: &WdlCommand) {
        self.append("  command ");
        self.append(if node.multiline { "<<<" } else { "{" });
        self.append(&string_literal_to_wdl(&node.command_text, false));
        self.append(if node.multiline { ">>>" } else { "}" });
        self.append("\n");
    }

    fn process_task_parameter_metadata(
        &mut self,
        _ctx: &WdlTask,
        node: &WdlParameterMetadata,
    ) {
        self.render_parameter_metadata(node);
    }

    fn process_task_metadata(&mut self, _ctx: &WdlTask, node: &WdlMetadata) {
        self.render_metadata(node);
    }

    fn process_task_requirements(&mut self, _ctx: &WdlTask, node: &WdlRequirements) {
        self.append("  requirements {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            self.append(": ");
            if let Some(val) = &entry.value {
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn process_task_runtime(&mut self, _ctx: &WdlTask, node: &WdlRuntime) {
        self.append("  runtime {\n");
        for entry in &node.elements {
            self.append("    ");
            self.append(&entry.key);
            self.append(": ");
            if let Some(val) = &entry.value {
                self.append(&expression_to_wdl(val));
            }
            self.append("\n");
        }
        self.append("  }\n");
    }

    fn process_task_hints(&mut self, _ctx: &WdlTask, node: &WdlTaskHints) {
        self.render_task_hints(node);
    }

    // -----------------------------------------------------------------------
    // Workflow
    // -----------------------------------------------------------------------

    fn process_workflow(&mut self, ctx: &WdlDocument, node: &WdlWorkflow) {
        self.append("workflow ");
        self.append(&node.name);
        self.append("{\n");
        // Delegate to default traversal.
        use crate::definitions::WdlWorkflowElement;
        for elem in &node.elements {
            match elem {
                WdlWorkflowElement::BoundDeclaration(d) => {
                    self.process_workflow_declaration(node, d)
                }
                WdlWorkflowElement::Declaration(_) => {}
                WdlWorkflowElement::Input(s) => self.process_workflow_input(node, s),
                WdlWorkflowElement::Output(s) => self.process_workflow_output(node, s),
                WdlWorkflowElement::Meta(s) => self.process_workflow_metadata(node, s),
                WdlWorkflowElement::ParameterMeta(s) => {
                    self.process_workflow_parameter_metadata(node, s)
                }
                WdlWorkflowElement::Call(s) => self.process_workflow_call(node, s),
                WdlWorkflowElement::Conditional(s) => {
                    self.process_workflow_conditional(node, s)
                }
                WdlWorkflowElement::Scatter(s) => self.process_workflow_scatter(node, s),
                WdlWorkflowElement::Hints(s) => self.process_workflow_hints(node, s),
            }
        }
        let _ = ctx;
        self.append("}\n");
    }

    fn process_workflow_declaration(&mut self, _ctx: &WdlWorkflow, node: &WdlBoundDeclaration) {
        self.render_statement(&WdlStatement::BoundDeclaration(node.clone()), 0);
    }

    fn process_workflow_input(&mut self, _ctx: &WdlWorkflow, node: &WdlInput) {
        self.render_input(node);
    }

    fn process_workflow_output(&mut self, _ctx: &WdlWorkflow, node: &WdlOutput) {
        self.render_output(node);
    }

    fn process_workflow_metadata(&mut self, _ctx: &WdlWorkflow, node: &WdlMetadata) {
        self.render_metadata(node);
    }

    fn process_workflow_parameter_metadata(
        &mut self,
        _ctx: &WdlWorkflow,
        node: &WdlParameterMetadata,
    ) {
        self.render_parameter_metadata(node);
    }

    fn process_workflow_call(&mut self, _ctx: &WdlWorkflow, node: &WdlCall) {
        self.render_statement(&WdlStatement::Call(node.clone()), 0);
    }

    fn process_workflow_scatter(&mut self, _ctx: &WdlWorkflow, node: &WdlScatter) {
        self.render_statement(&WdlStatement::Scatter(node.clone()), 0);
    }

    fn process_workflow_conditional(&mut self, _ctx: &WdlWorkflow, node: &WdlConditional) {
        self.render_statement(&WdlStatement::Conditional(node.clone()), 0);
    }

    fn process_workflow_hints(&mut self, _ctx: &WdlWorkflow, node: &WdlWorkflowHints) {
        self.render_workflow_hints(node);
    }
}
