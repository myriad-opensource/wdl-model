//! `WdlProcessor` trait — source-order traversal callbacks for the full WDL object model.
//!
//! This module mirrors the Java `WdlProcessor` interface and `WdlProcessorBase` abstract class:
//! - `WdlProcessor` is a Rust trait with default no-op leaf methods and default traversal
//!   logic in the compound callbacks (`process_document`, `process_struct`, `process_task`,
//!   `process_workflow`).
//! - `ResolvedImport<T>` captures the result of resolving an import symbol.
//! - Free functions (`resolve_imported_tasks`, etc.) implement the symbol-resolution logic
//!   that Java keeps in `WdlProcessorBase` instance methods.

use crate::definitions::{WdlEnum, WdlStruct, WdlStructMember, WdlTask, WdlWorkflow};
use crate::document::{WdlDocument, WdlDocumentElement};
use crate::sections::{
    WdlCommand, WdlInput, WdlMetadata, WdlOutput, WdlParameterMetadata, WdlRequirements,
    WdlRuntime, WdlTaskHints, WdlWorkflowHints,
};
use crate::statements::{
    WdlBoundDeclaration, WdlCall, WdlConditional, WdlImport, WdlImportMembers, WdlImportStandard,
    WdlImportStar, WdlScatter,
};
use crate::version::WdlVersion;

// Re-export render helpers so callers can reach them through this module.
pub use crate::processors::render::{
    bound_declaration_to_wdl, expression_to_wdl, input_declaration_to_wdl, string_literal_to_wdl,
    type_to_wdl, unbound_declaration_to_wdl,
};

// ---------------------------------------------------------------------------
// WdlProcessor trait
// ---------------------------------------------------------------------------

/// Visitor-style processor contract for the full WDL object model.
///
/// Mirrors Java's `WdlProcessor` interface + `WdlProcessorBase` default traversal in one trait.
///
/// Override only the callbacks you care about; the default implementations of the compound
/// callbacks (`process_document`, `process_struct`, `process_task`, `process_workflow`) perform
/// the standard source-order traversal to their children.
pub trait WdlProcessor {
    // -----------------------------------------------------------------------
    // Document level
    // -----------------------------------------------------------------------

    /// Walk the document root and dispatch to element-level callbacks.
    ///
    /// Default: calls `process_version`, then iterates `doc.elements` and dispatches.
    fn process_document(&mut self, doc: &WdlDocument) {
        if let Some(v) = &doc.wdl_version {
            self.process_version(doc, v);
        }
        for elem in &doc.elements {
            match elem {
                WdlDocumentElement::Import(WdlImport::Standard(i)) => {
                    self.process_import_standard(doc, i)
                }
                WdlDocumentElement::Import(WdlImport::Star(i)) => {
                    self.process_import_star(doc, i)
                }
                WdlDocumentElement::Import(WdlImport::Members(i)) => {
                    self.process_import_members(doc, i)
                }
                WdlDocumentElement::Enum(e) => self.process_enum(doc, e),
                WdlDocumentElement::Struct(s) => self.process_struct(doc, s),
                WdlDocumentElement::Task(t) => self.process_task(doc, t),
                WdlDocumentElement::Workflow(w) => self.process_workflow(doc, w),
            }
        }
    }

    /// Called for the `version` declaration.
    fn process_version(&mut self, _ctx: &WdlDocument, _node: &WdlVersion) {}

    /// Called for a standard `import "…" [as …]` statement.
    fn process_import_standard(&mut self, _ctx: &WdlDocument, _node: &WdlImportStandard) {}

    /// Called for a star `import * from "…"` statement.
    fn process_import_star(&mut self, _ctx: &WdlDocument, _node: &WdlImportStar) {}

    /// Called for a named-member `import { A, … } from "…"` statement.
    fn process_import_members(&mut self, _ctx: &WdlDocument, _node: &WdlImportMembers) {}

    /// Called for an `enum` definition.
    fn process_enum(&mut self, _ctx: &WdlDocument, _node: &WdlEnum) {}

    // -----------------------------------------------------------------------
    // Struct
    // -----------------------------------------------------------------------

    /// Walk a struct body and dispatch to struct-element callbacks.
    ///
    /// Default: iterates `node.elements` and dispatches.
    fn process_struct(&mut self, _ctx: &WdlDocument, node: &WdlStruct) {
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
    }

    /// Called for each typed member inside a struct.
    fn process_struct_member(&mut self, _ctx: &WdlStruct, _node: &WdlStructMember) {}

    /// Called for the `parameter_meta` section of a struct.
    fn process_struct_parameter_metadata(
        &mut self,
        _ctx: &WdlStruct,
        _node: &WdlParameterMetadata,
    ) {
    }

    /// Called for the `meta` section of a struct.
    fn process_struct_metadata(&mut self, _ctx: &WdlStruct, _node: &WdlMetadata) {}

    // -----------------------------------------------------------------------
    // Task
    // -----------------------------------------------------------------------

    /// Walk a task body and dispatch to task-element callbacks.
    ///
    /// Default: iterates `node.elements` and dispatches.
    fn process_task(&mut self, _ctx: &WdlDocument, node: &WdlTask) {
        use crate::definitions::WdlTaskElement;
        for elem in &node.elements {
            match elem {
                WdlTaskElement::BoundDeclaration(d) => self.process_task_declaration(node, d),
                WdlTaskElement::Declaration(_d) => {
                    // Unbound task declarations should not appear in practice.
                    // Panic in debug builds; silently skip in release.
                    debug_assert!(
                        false,
                        "Unexpected unbound declaration in task body — possible loader bug"
                    );
                }
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
    }

    /// Called for a bound declaration inside a task body.
    fn process_task_declaration(&mut self, _ctx: &WdlTask, _node: &WdlBoundDeclaration) {}

    /// Called for the `input { … }` section of a task.
    fn process_task_input(&mut self, _ctx: &WdlTask, _node: &WdlInput) {}

    /// Called for the `output { … }` section of a task.
    fn process_task_output(&mut self, _ctx: &WdlTask, _node: &WdlOutput) {}

    /// Called for the `command { … }` or `command <<< … >>>` section of a task.
    fn process_task_command(&mut self, _ctx: &WdlTask, _node: &WdlCommand) {}

    /// Called for the `parameter_meta { … }` section of a task.
    fn process_task_parameter_metadata(
        &mut self,
        _ctx: &WdlTask,
        _node: &WdlParameterMetadata,
    ) {
    }

    /// Called for the `meta { … }` section of a task.
    fn process_task_metadata(&mut self, _ctx: &WdlTask, _node: &WdlMetadata) {}

    /// Called for the `requirements { … }` section of a task.
    fn process_task_requirements(&mut self, _ctx: &WdlTask, _node: &WdlRequirements) {}

    /// Called for the legacy `runtime { … }` section of a task.
    fn process_task_runtime(&mut self, _ctx: &WdlTask, _node: &WdlRuntime) {}

    /// Called for the `hints { … }` section of a task.
    fn process_task_hints(&mut self, _ctx: &WdlTask, _node: &WdlTaskHints) {}

    // -----------------------------------------------------------------------
    // Workflow
    // -----------------------------------------------------------------------

    /// Walk a workflow body and dispatch to workflow-element callbacks.
    ///
    /// Default: iterates `node.elements` and dispatches.
    /// Note: `WdlWorkflowElement::Declaration` (unbound) should not appear in valid parsed
    /// WDL; this implementation panics in debug mode and skips it in release builds.
    fn process_workflow(&mut self, _ctx: &WdlDocument, node: &WdlWorkflow) {
        use crate::definitions::WdlWorkflowElement;
        for elem in &node.elements {
            match elem {
                WdlWorkflowElement::BoundDeclaration(d) => {
                    self.process_workflow_declaration(node, d)
                }
                WdlWorkflowElement::Declaration(_d) => {
                    // Unbound workflow declarations should not appear in valid parsed WDL.
                    debug_assert!(
                        false,
                        "Unexpected unbound declaration in workflow body — possible loader bug"
                    );
                }
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
    }

    /// Called for a bound declaration at workflow body scope.
    fn process_workflow_declaration(&mut self, _ctx: &WdlWorkflow, _node: &WdlBoundDeclaration) {
    }

    /// Called for the `input { … }` section of a workflow.
    fn process_workflow_input(&mut self, _ctx: &WdlWorkflow, _node: &WdlInput) {}

    /// Called for the `output { … }` section of a workflow.
    fn process_workflow_output(&mut self, _ctx: &WdlWorkflow, _node: &WdlOutput) {}

    /// Called for the `meta { … }` section of a workflow.
    fn process_workflow_metadata(&mut self, _ctx: &WdlWorkflow, _node: &WdlMetadata) {}

    /// Called for the `parameter_meta { … }` section of a workflow.
    fn process_workflow_parameter_metadata(
        &mut self,
        _ctx: &WdlWorkflow,
        _node: &WdlParameterMetadata,
    ) {
    }

    /// Called for a `call` statement in a workflow.
    ///
    /// Default is a no-op; `WdlProcessorBase` in Java is also a no-op here.
    fn process_workflow_call(&mut self, _ctx: &WdlWorkflow, _node: &WdlCall) {}

    /// Called for an `if` block in a workflow.
    ///
    /// Default is a no-op (mirrors Java `WdlProcessorBase.processWorkflowConditional`).
    /// Processors that need to recurse into conditionals must do so explicitly.
    fn process_workflow_conditional(&mut self, _ctx: &WdlWorkflow, _node: &WdlConditional) {}

    /// Called for a `scatter` block in a workflow.
    ///
    /// Default is a no-op. Processors that need to recurse must do so explicitly.
    fn process_workflow_scatter(&mut self, _ctx: &WdlWorkflow, _node: &WdlScatter) {}

    /// Called for the `hints { … }` section of a workflow.
    fn process_workflow_hints(&mut self, _ctx: &WdlWorkflow, _node: &WdlWorkflowHints) {}
}

// ---------------------------------------------------------------------------
// ResolvedImport<T>
// ---------------------------------------------------------------------------

/// The result of resolving an import symbol to its definition.
///
/// Mirrors `WdlProcessorBase.ResolvedImport<T>` in Java.
#[derive(Debug, Clone)]
pub struct ResolvedImport<T: Clone> {
    /// The name visible in the importing document (may include an alias).
    pub local_name: String,
    /// The original name of the symbol inside the imported document.
    pub imported_name: String,
    /// The namespace prefix used for standard imports (e.g. `"lib"` in `lib.task_name`).
    /// `None` for star and member imports.
    pub import_namespace: Option<String>,
    /// The resolved imported document.
    pub imported_document: WdlDocument,
    /// The actual resolved symbol.
    pub symbol: T,
}

// ---------------------------------------------------------------------------
// Import namespace helper
// ---------------------------------------------------------------------------

/// Compute the namespace prefix for a standard `import "…" [as alias]` statement.
///
/// Mirrors `WdlProcessorBase.importNamespace`.
pub fn import_namespace(imp: &WdlImportStandard) -> String {
    // Explicit alias overrides the file-name default.
    if let Some(alias) = &imp.alias {
        if !alias.is_empty() {
            return alias.clone();
        }
    }

    // Derive the default namespace from the import source path (strip directory + ".wdl").
    let src = &imp.source_text;
    let path_part = {
        // Strip any URI scheme prefix (e.g. "http://host/path" → "/path").
        let p = if let Some(pos) = src.find("://") {
            &src[pos + 3..]
        } else {
            src.as_str()
        };
        // Within the path component, use the part after the last '/'.
        let idx = p.rfind('/').map(|i| i + 1).unwrap_or(0);
        &p[idx..]
    };
    let basename = path_part.strip_suffix(".wdl").unwrap_or(path_part);
    basename.to_string()
}

// ---------------------------------------------------------------------------
// Import resolution free functions
// ---------------------------------------------------------------------------

/// Resolve all task definitions that are visible under `call_target` in `doc`.
///
/// `call_target` may be:
/// - a simple name (`"my_task"`) for star/member imports, or
/// - a qualified name (`"lib.my_task"`) for standard namespace imports.
///
/// Mirrors `WdlProcessorBase.resolveImportedTasks`.
pub fn resolve_imported_tasks(
    doc: &WdlDocument,
    call_target: &str,
) -> Vec<ResolvedImport<WdlTask>> {
    if call_target.is_empty() {
        return Vec::new();
    }

    let qualified = call_target.contains('.');
    let (namespace_part, member_part) = if qualified {
        let dot = call_target.find('.').unwrap();
        (&call_target[..dot], &call_target[dot + 1..])
    } else {
        ("", call_target)
    };

    let mut results = Vec::new();
    for imp in doc.import_statements() {
        let key = match imp.import_identifier() {
            Some(k) if !k.is_empty() => k,
            _ => continue,
        };
        let imported = match doc.imported_documents.get(key) {
            Some(d) => d,
            None => continue,
        };

        match imp {
            WdlImport::Standard(std_imp) => {
                let ns = import_namespace(std_imp);
                if !qualified || ns != namespace_part {
                    continue;
                }
                for task in imported.tasks() {
                    if task.name == member_part {
                        results.push(ResolvedImport {
                            local_name: format!("{}.{}", ns, member_part),
                            imported_name: member_part.to_string(),
                            import_namespace: Some(ns.clone()),
                            imported_document: imported.clone(),
                            symbol: task.clone(),
                        });
                    }
                }
            }
            WdlImport::Star(_) => {
                if qualified {
                    continue;
                }
                for task in imported.tasks() {
                    if task.name == member_part {
                        results.push(ResolvedImport {
                            local_name: member_part.to_string(),
                            imported_name: member_part.to_string(),
                            import_namespace: None,
                            imported_document: imported.clone(),
                            symbol: task.clone(),
                        });
                    }
                }
            }
            WdlImport::Members(mem_imp) => {
                if qualified {
                    continue;
                }
                for member in &mem_imp.members {
                    let local = member
                        .alias
                        .as_deref()
                        .filter(|a| !a.is_empty())
                        .unwrap_or(&member.member);
                    if local != member_part {
                        continue;
                    }
                    for task in imported.tasks() {
                        if task.name == member.member {
                            results.push(ResolvedImport {
                                local_name: local.to_string(),
                                imported_name: member.member.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: task.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    results
}

/// Resolve all workflow definitions that are visible under `call_target` in `doc`.
///
/// Mirrors `WdlProcessorBase.resolveImportedWorkflows`.
pub fn resolve_imported_workflows(
    doc: &WdlDocument,
    call_target: &str,
) -> Vec<ResolvedImport<WdlWorkflow>> {
    if call_target.is_empty() {
        return Vec::new();
    }

    let qualified = call_target.contains('.');
    let (namespace_part, member_part) = if qualified {
        let dot = call_target.find('.').unwrap();
        (&call_target[..dot], &call_target[dot + 1..])
    } else {
        ("", call_target)
    };

    let mut results = Vec::new();
    for imp in doc.import_statements() {
        let key = match imp.import_identifier() {
            Some(k) if !k.is_empty() => k,
            _ => continue,
        };
        let imported = match doc.imported_documents.get(key) {
            Some(d) => d,
            None => continue,
        };

        match imp {
            WdlImport::Standard(std_imp) => {
                let ns = import_namespace(std_imp);
                if !qualified || ns != namespace_part {
                    continue;
                }
                for wf in imported.workflows() {
                    if wf.name == member_part {
                        results.push(ResolvedImport {
                            local_name: format!("{}.{}", ns, member_part),
                            imported_name: member_part.to_string(),
                            import_namespace: Some(ns.clone()),
                            imported_document: imported.clone(),
                            symbol: wf.clone(),
                        });
                    }
                }
            }
            WdlImport::Star(_) => {
                if qualified {
                    continue;
                }
                for wf in imported.workflows() {
                    if wf.name == member_part {
                        results.push(ResolvedImport {
                            local_name: member_part.to_string(),
                            imported_name: member_part.to_string(),
                            import_namespace: None,
                            imported_document: imported.clone(),
                            symbol: wf.clone(),
                        });
                    }
                }
            }
            WdlImport::Members(mem_imp) => {
                if qualified {
                    continue;
                }
                for member in &mem_imp.members {
                    let local = member
                        .alias
                        .as_deref()
                        .filter(|a| !a.is_empty())
                        .unwrap_or(&member.member);
                    if local != member_part {
                        continue;
                    }
                    for wf in imported.workflows() {
                        if wf.name == member.member {
                            results.push(ResolvedImport {
                                local_name: local.to_string(),
                                imported_name: member.member.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: wf.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    results
}

/// Resolve all struct definitions visible as `visible_type_name` in `doc`.
///
/// Mirrors `WdlProcessorBase.resolveImportedStructs`.
pub fn resolve_imported_structs(
    doc: &WdlDocument,
    visible_type_name: &str,
) -> Vec<ResolvedImport<WdlStruct>> {
    resolve_imported_type_defs(doc, visible_type_name, true)
        .into_iter()
        .filter_map(|ri| {
            if let TypeDef::Struct(s) = ri.symbol {
                Some(ResolvedImport {
                    local_name: ri.local_name,
                    imported_name: ri.imported_name,
                    import_namespace: ri.import_namespace,
                    imported_document: ri.imported_document,
                    symbol: s,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Resolve all enum definitions visible as `visible_type_name` in `doc`.
///
/// Mirrors `WdlProcessorBase.resolveImportedEnums`.
pub fn resolve_imported_enums(
    doc: &WdlDocument,
    visible_type_name: &str,
) -> Vec<ResolvedImport<WdlEnum>> {
    resolve_imported_type_defs(doc, visible_type_name, false)
        .into_iter()
        .filter_map(|ri| {
            if let TypeDef::Enum(e) = ri.symbol {
                Some(ResolvedImport {
                    local_name: ri.local_name,
                    imported_name: ri.imported_name,
                    import_namespace: ri.import_namespace,
                    imported_document: ri.imported_document,
                    symbol: e,
                })
            } else {
                None
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Private helper — shared struct/enum resolution logic
// ---------------------------------------------------------------------------

/// Internal union used by `resolve_imported_type_defs`.
#[derive(Clone)]
enum TypeDef {
    Struct(WdlStruct),
    Enum(WdlEnum),
}

fn resolve_imported_type_defs(
    doc: &WdlDocument,
    visible_type_name: &str,
    want_structs: bool,
) -> Vec<ResolvedImport<TypeDef>> {
    use std::collections::HashMap;

    if visible_type_name.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();
    for imp in doc.import_statements() {
        let key = match imp.import_identifier() {
            Some(k) if !k.is_empty() => k,
            _ => continue,
        };
        let imported = match doc.imported_documents.get(key) {
            Some(d) => d,
            None => continue,
        };

        match imp {
            WdlImport::Standard(std_imp) => {
                // Build alias map: importedName → localName
                let aliases: HashMap<&str, &str> = std_imp
                    .members
                    .iter()
                    .filter(|m| !m.member.is_empty())
                    .map(|m| {
                        let local = m
                            .alias
                            .as_deref()
                            .filter(|a| !a.is_empty())
                            .unwrap_or(&m.member);
                        (m.member.as_str(), local)
                    })
                    .collect();

                if want_structs {
                    for s in imported.structs() {
                        let local = *aliases.get(s.name.as_str()).unwrap_or(&s.name.as_str());
                        if local == visible_type_name {
                            results.push(ResolvedImport {
                                local_name: local.to_string(),
                                imported_name: s.name.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: TypeDef::Struct(s.clone()),
                            });
                        }
                    }
                } else {
                    for e in imported.enums() {
                        let local = *aliases.get(e.name.as_str()).unwrap_or(&e.name.as_str());
                        if local == visible_type_name {
                            results.push(ResolvedImport {
                                local_name: local.to_string(),
                                imported_name: e.name.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: TypeDef::Enum(e.clone()),
                            });
                        }
                    }
                }
            }
            WdlImport::Star(_) => {
                if want_structs {
                    for s in imported.structs() {
                        if s.name == visible_type_name {
                            results.push(ResolvedImport {
                                local_name: visible_type_name.to_string(),
                                imported_name: s.name.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: TypeDef::Struct(s.clone()),
                            });
                        }
                    }
                } else {
                    for e in imported.enums() {
                        if e.name == visible_type_name {
                            results.push(ResolvedImport {
                                local_name: visible_type_name.to_string(),
                                imported_name: e.name.clone(),
                                import_namespace: None,
                                imported_document: imported.clone(),
                                symbol: TypeDef::Enum(e.clone()),
                            });
                        }
                    }
                }
            }
            WdlImport::Members(mem_imp) => {
                for member in &mem_imp.members {
                    let local = member
                        .alias
                        .as_deref()
                        .filter(|a| !a.is_empty())
                        .unwrap_or(&member.member);
                    if local != visible_type_name {
                        continue;
                    }
                    if want_structs {
                        for s in imported.structs() {
                            if s.name == member.member {
                                results.push(ResolvedImport {
                                    local_name: local.to_string(),
                                    imported_name: member.member.clone(),
                                    import_namespace: None,
                                    imported_document: imported.clone(),
                                    symbol: TypeDef::Struct(s.clone()),
                                });
                            }
                        }
                    } else {
                        for e in imported.enums() {
                            if e.name == member.member {
                                results.push(ResolvedImport {
                                    local_name: local.to_string(),
                                    imported_name: member.member.clone(),
                                    import_namespace: None,
                                    imported_document: imported.clone(),
                                    symbol: TypeDef::Enum(e.clone()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    results
}
