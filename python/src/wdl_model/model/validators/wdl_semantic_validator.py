"""Baseline semantic validator for Python WDL documents.

This module hosts the validator that should fail ordinary document loading. It covers baseline
semantic rules such as declaration assignability, private and required call inputs, invalid member
and index access, and version-gated function availability.

Representative failures are covered by fixtures such as
`spec_examples/v1_3/private_declaration_fail.wdl`,
`spec_examples/v1_3/select_first_empty_fail.wdl`, and
`spec_examples/v1_3/write_json_fail.wdl`.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from wdl_model.model.definitions import WdlEnum, WdlStruct, WdlTask, WdlWorkflow
from wdl_model.model.errors import (
    WdlException,
    WdlSemanticError,
    WdlSemanticErrorCode,
    WdlSemanticSeverity,
)
from wdl_model.model.expressions import (
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlBooleanLiteral,
    WdlBinaryOperator,
    WdlUnaryOperator,
    WdlUnaryOperation,
    WdlTernaryOperation,
    WdlFloatLiteral,
    WdlFunction,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlIntLiteral,
    WdlMapLiteral,
    WdlMemberAccessOperation,
    WdlNullLiteral,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStructLiteral,
    WdlVariable,
)
from wdl_model.model.processors import (
    WdlFunctionProcessorBase,
    WdlProcessorBase,
)
from wdl_model.model.sections import WdlInput, WdlOutput
from wdl_model.model.statements import (
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlImport,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
)
from wdl_model.model.types import WdlArrayType, WdlTypeReferenceType
from wdl_model.model.types import (
    Type as WdlPrimitiveTypeEnum,
    WdlMapType,
    WdlPairType,
    WdlPrimitiveType,
    WdlType,
)

from ..wdl_document import WdlDocument
from ..wdl_version import WdlVersion


_UNKNOWN = object()


@dataclass
class _TaskContract:
    required_inputs: set[str]
    input_types: dict[str, WdlType | None]
    outputs: set[str]
    output_types: dict[str, WdlType | None]
    private_declarations: set[str]


class WdlSemanticValidator(WdlProcessorBase, WdlFunctionProcessorBase):
    """Baseline semantic validator shared by the Python loader and tests."""

    _FUNCTION_ADDED_IN: dict[WdlFunction, WdlVersion] = {
        WdlFunction.MIN: WdlVersion.V1_1,
        WdlFunction.MAX: WdlVersion.V1_1,
        WdlFunction.SUFFIX: WdlVersion.V1_1,
        WdlFunction.QUOTE: WdlVersion.V1_1,
        WdlFunction.SQUOTE: WdlVersion.V1_1,
        WdlFunction.SEP: WdlVersion.V1_1,
        WdlFunction.UNZIP: WdlVersion.V1_1,
        WdlFunction.SELECT_FIRST: WdlVersion.V1_1,
        WdlFunction.SELECT_ALL: WdlVersion.V1_1,
        WdlFunction.KEYS: WdlVersion.V1_1,
        WdlFunction.AS_PAIRS: WdlVersion.V1_1,
        WdlFunction.AS_MAP: WdlVersion.V1_1,
        WdlFunction.COLLECT_BY_KEY: WdlVersion.V1_1,
        WdlFunction.CHUNK: WdlVersion.V1_2,
        WdlFunction.CONTAINS: WdlVersion.V1_2,
        WdlFunction.CONTAINS_KEY: WdlVersion.V1_2,
        WdlFunction.VALUES: WdlVersion.V1_2,
        WdlFunction.MATCHES: WdlVersion.V1_2,
        WdlFunction.FIND: WdlVersion.V1_2,
        WdlFunction.JOIN_PATHS: WdlVersion.V1_2,
        WdlFunction.VALUE: WdlVersion.V1_3,
    }

    def __init__(self, throw_on_warnings: bool = True) -> None:
        """Create a validator.

        Args:
            throw_on_warnings: Whether warning-only diagnostics should raise `WdlException`.
                This is mainly useful when the linting layer is used for reporting but should not
                fail the current validation pass.
        """
        self._errors: list[WdlSemanticError] = []
        self._task_contracts: dict[str, _TaskContract] = {}
        self._struct_members: dict[str, set[str]] = {}
        self._struct_member_types: dict[str, dict[str, WdlType | None]] = {}
        self._scope_types: dict[str, Any] = {}
        self._scope_values: dict[str, Any] = {}
        self._call_outputs: dict[str, set[str]] = {}
        self._call_output_types: dict[str, dict[str, WdlType | None]] = {}
        self._imported_task_contracts: dict[str, _TaskContract] = {}
        self._imported_workflow_outputs: dict[str, set[str]] = {}
        self._imported_workflow_output_types: dict[str, dict[str, WdlType | None]] = {}
        self._visible_type_names: set[str] = set()
        self._document_version: WdlVersion | None = None
        self._throw_on_warnings = throw_on_warnings

    def validateDocument(self, document: WdlDocument) -> None:
        """Validate a WDL document and raise when the configured throw policy requires it."""
        self._errors = []
        self._document_version = (
            document.getWdlVersion() if document is not None else None
        )
        self._index_top_level_contracts(document)
        self._index_import_visibility(document)
        self.processDocument(document)
        if self._should_throw_for_collected_diagnostics():
            raise WdlException(self._errors)

    def setThrowOnWarnings(self, throw_on_warnings: bool) -> "WdlSemanticValidator":
        """Set whether warning-only diagnostics should raise and return `self` for chaining."""
        self._throw_on_warnings = throw_on_warnings
        return self

    def isThrowOnWarnings(self) -> bool:
        """Return the current warning throw policy."""
        return self._throw_on_warnings

    def _should_throw_for_collected_diagnostics(self) -> bool:
        if not self._errors:
            return False
        has_error = any(
            err.severity == WdlSemanticSeverity.ERROR for err in self._errors
        )
        if has_error:
            return True
        has_warning = any(
            err.severity == WdlSemanticSeverity.WARNING for err in self._errors
        )
        return self._throw_on_warnings and has_warning

    def _index_top_level_contracts(self, document: WdlDocument) -> None:
        self._task_contracts = {}
        self._struct_members = {}
        self._struct_member_types = {}
        self._visible_type_names = set()

        for element in document.elements():
            if isinstance(element, WdlTask):
                required_inputs: set[str] = set()
                outputs: set[str] = set()
                private_declarations: set[str] = set()
                input_types: dict[str, WdlType | None] = {}
                output_types: dict[str, WdlType | None] = {}

                for task_element in element.elements():
                    if isinstance(task_element, WdlInput):
                        for decl in task_element.elements():
                            if decl.name is not None:
                                input_types[decl.name] = decl.type
                                if (
                                    not isinstance(decl, WdlBoundDeclaration)
                                    and decl.type is not None
                                    and not decl.type.isOptional()
                                ):
                                    required_inputs.add(decl.name)
                    elif isinstance(task_element, WdlOutput):
                        for decl in task_element.elements():
                            if decl.name is not None:
                                outputs.add(decl.name)
                                output_types[decl.name] = decl.type
                    elif isinstance(task_element, WdlBoundDeclaration):
                        if task_element.name is not None:
                            private_declarations.add(task_element.name)

                if element.name is not None:
                    self._task_contracts[element.name] = _TaskContract(
                        required_inputs=required_inputs,
                        input_types=input_types,
                        outputs=outputs,
                        output_types=output_types,
                        private_declarations=private_declarations,
                    )

            elif isinstance(element, WdlStruct):
                if element.name is None:
                    continue
                self._visible_type_names.add(element.name)
                members: set[str] = set()
                member_types: dict[str, WdlType | None] = {}
                for struct_element in element.elements():
                    member_name = getattr(struct_element, "name", None)
                    if member_name is not None:
                        members.add(member_name)
                        member_types[member_name] = getattr(
                            struct_element, "type", None
                        )
                self._struct_members[element.name] = members
                self._struct_member_types[element.name] = member_types
            elif isinstance(element, WdlEnum):
                if element.name is not None:
                    self._visible_type_names.add(element.name)

    def _index_import_visibility(self, document: WdlDocument) -> None:
        self._imported_task_contracts = {}
        self._imported_workflow_outputs = {}
        self._imported_workflow_output_types = {}

        local_symbol_names: set[str] = set()
        for element in document.elements():
            name = getattr(element, "name", None)
            if name:
                local_symbol_names.add(name)

        seen_namespaces: set[str] = set()
        visible_type_origins: dict[str, str] = {}

        for imp in document.importStatements():
            imported_document = self._resolve_imported_document(document, imp)
            if imported_document is None:
                continue

            self._validate_import_version(document, imported_document)

            if isinstance(imp, WdlImportStandard):
                namespace = self._import_namespace(imp)
                if namespace in seen_namespaces:
                    self._add_error(
                        f"Duplicate import namespace '{namespace}'",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )
                seen_namespaces.add(namespace)
                if namespace in local_symbol_names:
                    self._add_error(
                        f"Import namespace '{namespace}' conflicts with local declaration",
                        code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                    )

                aliases = self._import_aliases(imp)
                for imported_name in aliases:
                    if not self._imported_type_exists(imported_document, imported_name):
                        self._add_error(
                            f"Import alias target '{imported_name}' does not exist",
                            code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                        )

                for task in imported_document.tasks():
                    if task.name:
                        self._imported_task_contracts[f"{namespace}.{task.name}"] = (
                            self._task_contract_for(task)
                        )
                for workflow in imported_document.workflows():
                    if workflow.name:
                        out_names, out_types = self._workflow_output_contract(workflow)
                        key = f"{namespace}.{workflow.name}"
                        self._imported_workflow_outputs[key] = out_names
                        self._imported_workflow_output_types[key] = out_types

                self._register_visible_imported_types(
                    imported_document,
                    lambda type_name: aliases.get(type_name, type_name),
                    visible_type_origins,
                )

            elif isinstance(imp, WdlImportStar):
                for task in imported_document.tasks():
                    if task.name:
                        self._imported_task_contracts[task.name] = (
                            self._task_contract_for(task)
                        )
                for workflow in imported_document.workflows():
                    if workflow.name:
                        out_names, out_types = self._workflow_output_contract(workflow)
                        self._imported_workflow_outputs[workflow.name] = out_names
                        self._imported_workflow_output_types[workflow.name] = out_types

                self._register_visible_imported_types(
                    imported_document,
                    lambda type_name: type_name,
                    visible_type_origins,
                )

            elif isinstance(imp, WdlImportMembers):
                self._validate_member_aliases(imp)
                for member in imp.members():
                    local_name = member.alias if member.alias else member.member
                    if local_name is None or member.member is None:
                        continue
                    if local_name in local_symbol_names:
                        self._add_error(
                            f"Imported symbol '{local_name}' conflicts with local declaration",
                            code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                        )
                    symbol_kind = self._imported_symbol_kind(
                        imported_document, member.member
                    )
                    if symbol_kind is None:
                        self._add_error(
                            f"Import member '{member.member}' does not exist",
                            code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                        )
                        continue
                    if symbol_kind == "task":
                        task = next(
                            (
                                t
                                for t in imported_document.tasks()
                                if t.name == member.member
                            ),
                            None,
                        )
                        if task is not None:
                            self._imported_task_contracts[local_name] = (
                                self._task_contract_for(task)
                            )
                    elif symbol_kind == "workflow":
                        wf = next(
                            (
                                w
                                for w in imported_document.workflows()
                                if w.name == member.member
                            ),
                            None,
                        )
                        if wf is not None:
                            out_names, out_types = self._workflow_output_contract(wf)
                            self._imported_workflow_outputs[local_name] = out_names
                            self._imported_workflow_output_types[local_name] = out_types
                    elif symbol_kind in {"struct", "enum"}:
                        self._register_visible_type_name(
                            local_name,
                            f"{imp.importIdentifier or 'import'}::{member.member}",
                            visible_type_origins,
                        )

    def _resolve_imported_document(
        self, document: WdlDocument, imp: WdlImport
    ) -> WdlDocument | None:
        if imp.importIdentifier is None or not imp.importIdentifier.strip():
            return None
        return document.importedDocuments().get(imp.importIdentifier)

    def _import_namespace(self, imp: WdlImportStandard) -> str:
        if imp.alias is not None and imp.alias.strip():
            return imp.alias
        source = self._import_source_text(imp)
        if "/" in source:
            source = source.rsplit("/", 1)[-1]
        if source.endswith(".wdl") and len(source) > 4:
            source = source[:-4]
        return source

    def _import_source_text(self, imp: WdlImport) -> str:
        if imp.source is None:
            return imp.sourceText or ""
        out: list[str] = []
        for component in imp.source.components():
            if hasattr(component, "text") and component.text is not None:
                out.append(component.text)
            elif hasattr(component, "escapeText") and component.escapeText is not None:
                out.append(component.escapeText)
        return "".join(out)

    def _import_aliases(self, imp: WdlImportStandard) -> dict[str, str]:
        aliases: dict[str, str] = {}
        for member in imp.members():
            if member.member is None:
                continue
            aliases[member.member] = member.alias if member.alias else member.member
        return aliases

    def _imported_type_exists(
        self, imported_document: WdlDocument, type_name: str
    ) -> bool:
        return any(s.name == type_name for s in imported_document.structs()) or any(
            e.name == type_name for e in imported_document.enums()
        )

    def _register_visible_imported_types(
        self,
        imported_document: WdlDocument,
        local_name_mapper,
        visible_type_origins: dict[str, str],
    ) -> None:
        for struct in imported_document.structs():
            if struct.name is None:
                continue
            local_name = local_name_mapper(struct.name)
            self._register_visible_type_name(
                local_name,
                f"{imported_document.getSourceLocation() or '<import>'}::struct::{struct.name}",
                visible_type_origins,
            )
            members: set[str] = set()
            member_types: dict[str, WdlType | None] = {}
            for struct_element in struct.elements():
                member_name = getattr(struct_element, "name", None)
                if member_name is not None:
                    members.add(member_name)
                    member_types[member_name] = getattr(struct_element, "type", None)
            self._struct_members[local_name] = members
            self._struct_member_types[local_name] = member_types

        for enum in imported_document.enums():
            if enum.name is None:
                continue
            local_name = local_name_mapper(enum.name)
            self._register_visible_type_name(
                local_name,
                f"{imported_document.getSourceLocation() or '<import>'}::enum::{enum.name}",
                visible_type_origins,
            )

    def _register_visible_type_name(
        self,
        type_name: str,
        origin: str,
        visible_type_origins: dict[str, str],
    ) -> None:
        previous_origin = visible_type_origins.get(type_name)
        if previous_origin is not None and previous_origin != origin:
            self._add_error(
                f"Imported type '{type_name}' is defined by multiple imports",
                code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
            )
            return
        visible_type_origins[type_name] = origin
        self._visible_type_names.add(type_name)

    def _task_contract_for(self, task: WdlTask) -> _TaskContract:
        required_inputs: set[str] = set()
        outputs: set[str] = set()
        private_declarations: set[str] = set()
        input_types: dict[str, WdlType | None] = {}
        output_types: dict[str, WdlType | None] = {}

        for task_element in task.elements():
            if isinstance(task_element, WdlInput):
                for decl in task_element.elements():
                    if decl.name is not None:
                        input_types[decl.name] = decl.type
                        if (
                            not isinstance(decl, WdlBoundDeclaration)
                            and decl.type is not None
                            and not decl.type.isOptional()
                        ):
                            required_inputs.add(decl.name)
            elif isinstance(task_element, WdlOutput):
                for decl in task_element.elements():
                    if decl.name is not None:
                        outputs.add(decl.name)
                        output_types[decl.name] = decl.type
            elif isinstance(task_element, WdlBoundDeclaration):
                if task_element.name is not None:
                    private_declarations.add(task_element.name)

        return _TaskContract(
            required_inputs=required_inputs,
            input_types=input_types,
            outputs=outputs,
            output_types=output_types,
            private_declarations=private_declarations,
        )

    def _workflow_output_contract(
        self, workflow: WdlWorkflow
    ) -> tuple[set[str], dict[str, WdlType | None]]:
        outputs: set[str] = set()
        output_types: dict[str, WdlType | None] = {}
        for workflow_element in workflow.elements():
            if isinstance(workflow_element, WdlOutput):
                for decl in workflow_element.elements():
                    if decl.name is not None:
                        outputs.add(decl.name)
                        output_types[decl.name] = decl.type
        return outputs, output_types

    def _imported_symbol_kind(
        self, imported_document: WdlDocument, symbol_name: str
    ) -> str | None:
        if any(task.name == symbol_name for task in imported_document.tasks()):
            return "task"
        if any(
            workflow.name == symbol_name for workflow in imported_document.workflows()
        ):
            return "workflow"
        if any(struct.name == symbol_name for struct in imported_document.structs()):
            return "struct"
        if any(enum.name == symbol_name for enum in imported_document.enums()):
            return "enum"
        return None

    def _validate_member_aliases(self, imp: WdlImportMembers) -> None:
        seen_local_names: set[str] = set()
        for member in imp.members():
            local_name = member.alias if member.alias else member.member
            if local_name is None:
                continue
            if local_name in seen_local_names:
                self._add_error(
                    f"Duplicate imported member alias '{local_name}'",
                    code=WdlSemanticErrorCode.DUPLICATE_DEFINITION,
                )
            seen_local_names.add(local_name)

    def _validate_import_version(
        self, current_document: WdlDocument, imported_document: WdlDocument
    ) -> None:
        current = current_document.getWdlVersion()
        imported = imported_document.getWdlVersion()
        if current is None or imported is None:
            return
        if (imported.major, imported.minor) > (current.major, current.minor):
            self._add_error(
                f"Imported document version {imported.getVersionString()} is newer than importing document version {current.getVersionString()}",
                code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
            )

    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None:
        prev_types = self._scope_types
        prev_values = self._scope_values
        prev_call_outputs = self._call_outputs
        prev_call_output_types = self._call_output_types

        self._scope_types = {}
        self._scope_values = {}
        self._call_outputs = {}
        self._call_output_types = {}

        try:
            super().processWorkflow(ctx, node)
        finally:
            self._scope_types = prev_types
            self._scope_values = prev_values
            self._call_outputs = prev_call_outputs
            self._call_output_types = prev_call_output_types

    def processWorkflowInput(self, ctx: WdlWorkflow, node: WdlInput) -> None:
        for decl in node.elements():
            if decl.name is not None:
                self._scope_types[decl.name] = decl.type
            if isinstance(decl, WdlBoundDeclaration) and decl.name is not None:
                self._validateExpression(decl.expression)
                self._scope_values[decl.name] = self._evaluate(decl.expression)

    def processWorkflowDeclaration(
        self, ctx: WdlWorkflow, node: WdlBoundDeclaration
    ) -> None:
        self._validateBoundDeclaration(node)

    def processWorkflowOutput(self, ctx: WdlWorkflow, node: WdlOutput) -> None:
        for decl in node.elements():
            self._validateBoundDeclaration(decl)

    def processWorkflowCall(self, ctx: WdlWorkflow, node: WdlCall) -> None:
        target = node.targetPathAsString() if node.targetPath() else None
        unqualified_target = node.targetPath()[-1] if node.targetPath() else None
        contract = None
        if target is not None:
            contract = self._task_contracts.get(unqualified_target)
            if contract is None:
                contract = self._imported_task_contracts.get(target)

        provided_inputs: set[str] = set()

        for call_input in node.inputs():
            key = call_input.getKey() or ""
            root_name = key.split(".", 1)[0]
            if root_name:
                provided_inputs.add(root_name)
            if contract is not None and root_name in contract.private_declarations:
                self._add_error(
                    f"Call input '{root_name}' is private in task '{target}'",
                    code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                )
            if (
                contract is not None
                and root_name
                and root_name not in contract.input_types
            ):
                self._add_error(
                    f"Call input '{root_name}' does not exist in task '{target}'",
                    code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                )
            self._validateExpression(call_input.getValue())
            if contract is not None and root_name in contract.input_types:
                expected = contract.input_types[root_name]
                if not self._is_assignable_from(expected, call_input.getValue()):
                    self._add_error(
                        f"Call input '{root_name}' type is incompatible with task '{target}' input type",
                        code=WdlSemanticErrorCode.TYPE_MISMATCH,
                    )

        if contract is not None:
            for required in contract.required_inputs:
                if required not in provided_inputs:
                    self._add_error(
                        f"Call to task '{target}' is missing required input '{required}'",
                        code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                    )

        call_name = node.alias or target
        if call_name is not None:
            if contract is not None:
                self._call_outputs[call_name] = contract.outputs
                self._call_output_types[call_name] = contract.output_types
            elif target in self._imported_workflow_outputs:
                self._call_outputs[call_name] = self._imported_workflow_outputs[target]
                self._call_output_types[call_name] = (
                    self._imported_workflow_output_types[target]
                )
            elif unqualified_target in self._imported_workflow_outputs:
                self._call_outputs[call_name] = self._imported_workflow_outputs[
                    unqualified_target
                ]
                self._call_output_types[call_name] = (
                    self._imported_workflow_output_types[unqualified_target]
                )
            else:
                self._call_outputs[call_name] = set()
                self._call_output_types[call_name] = {}

    def processWorkflowScatter(self, ctx: WdlWorkflow, node: WdlScatter) -> None:
        self._validateExpression(node.collection)
        for statement in node.statements():
            self._processWorkflowStatement(ctx, statement)

    def processWorkflowConditional(
        self, ctx: WdlWorkflow, node: WdlConditional
    ) -> None:
        self._validateExpression(node.condition)
        for statement in node.thenStatements():
            self._processWorkflowStatement(ctx, statement)
        for else_if in node.elseIfs():
            self._validateExpression(else_if.condition)
            for statement in else_if.thenStatements():
                self._processWorkflowStatement(ctx, statement)
        for statement in node.elseStatements():
            self._processWorkflowStatement(ctx, statement)

    def _processWorkflowStatement(self, workflow: WdlWorkflow, statement: Any) -> None:
        if isinstance(statement, WdlBoundDeclaration):
            self.processWorkflowDeclaration(workflow, statement)
        elif isinstance(statement, WdlCall):
            self.processWorkflowCall(workflow, statement)
        elif isinstance(statement, WdlScatter):
            self.processWorkflowScatter(workflow, statement)
        elif isinstance(statement, WdlConditional):
            self.processWorkflowConditional(workflow, statement)

    def _validateBoundDeclaration(self, node: WdlBoundDeclaration) -> None:
        if node.name is not None:
            self._scope_types[node.name] = node.type

        self._validateExpression(node.expression)
        if not self._is_assignable_from(node.type, node.expression):
            self._add_error(
                f"Declaration '{node.name or '<unnamed>'}' type is incompatible with expression",
                code=WdlSemanticErrorCode.TYPE_MISMATCH,
            )

        if isinstance(node.type, WdlArrayType) and node.type.isNonEmpty():
            if (
                isinstance(node.expression, WdlArrayLiteral)
                and len(node.expression.entries()) == 0
            ):
                self._add_error(
                    f"Declaration '{node.name or '<unnamed>'}' requires a non-empty array",
                    code=WdlSemanticErrorCode.TYPE_MISMATCH,
                )

        if node.name is not None:
            self._scope_values[node.name] = self._evaluate(node.expression)

    def _validateExpression(self, expr: Any) -> None:
        if expr is None:
            return

        if isinstance(expr, WdlFunctionCallOperation):
            self._validate_function_version_availability(expr)
            self.processFunctionCall(expr)
            for arg in expr.arguments():
                self._validateExpression(arg)
            return

        if isinstance(expr, WdlIndexAccessOperation):
            self._validateExpression(expr.target)
            self._validateExpression(expr.index)

            target_value = self._evaluate(expr.target)
            index_value = self._evaluate(expr.index)

            if isinstance(target_value, list) and isinstance(index_value, int):
                if index_value < 0 or index_value >= len(target_value):
                    self._add_error(
                        "Array index out of bounds",
                        code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                    )
            elif isinstance(target_value, dict) and index_value is not _UNKNOWN:
                if index_value not in target_value:
                    self._add_error(
                        f"Map key does not exist: {index_value}",
                        code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                    )
            return

        if isinstance(expr, WdlMemberAccessOperation):
            self._validateExpression(expr.target)

            if isinstance(expr.target, WdlVariable) and expr.target.name is not None:
                target_name = expr.target.name
                member_name = expr.member or ""

                if target_name in self._call_outputs:
                    if member_name not in self._call_outputs[target_name]:
                        self._add_error(
                            f"'{member_name}' is not an output field of call '{target_name}'",
                            code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                        )
                elif target_name in self._scope_types:
                    declared_type = self._scope_types[target_name]
                    if isinstance(declared_type, WdlTypeReferenceType):
                        struct_name = declared_type.referenceName()
                        members = self._struct_members.get(struct_name or "")
                        if members is not None and member_name not in members:
                            self._add_error(
                                f"Field '{member_name}' does not exist in struct '{struct_name}'",
                                code=WdlSemanticErrorCode.UNKNOWN_REFERENCE,
                            )
            return

        if isinstance(expr, WdlArrayLiteral):
            for entry in expr.entries():
                self._validateExpression(entry)
            return

        if isinstance(expr, WdlMapLiteral):
            for entry in expr.entries():
                self._validateExpression(entry.getKey())
                self._validateExpression(entry.getValue())
            return

        if isinstance(expr, WdlPairLiteral):
            self._validateExpression(expr.left)
            self._validateExpression(expr.right)
            return

        if isinstance(expr, WdlObjectLiteral):
            for entry in expr.entries():
                self._validateExpression(entry.getValue())
            return

        if isinstance(expr, WdlStructLiteral):
            for entry in expr.entries():
                self._validateExpression(entry.getValue())
            return

        if isinstance(expr, WdlStringLiteral):
            for component in expr.components():
                if isinstance(component, WdlStringPlaceholder):
                    self._validateExpression(component.expression)
                    option = component.option
                    if option is not None:
                        self._validateExpression(option.value)
                        self._validateExpression(option.trueValue)
                        self._validateExpression(option.falseValue)
            return

        for attr_name in (
            "left",
            "right",
            "operand",
            "condition",
            "trueValue",
            "falseValue",
        ):
            if hasattr(expr, attr_name):
                self._validateExpression(getattr(expr, attr_name))

    def _evaluate(self, expr: Any) -> Any:
        if expr is None:
            return _UNKNOWN

        if isinstance(expr, WdlNullLiteral):
            return None

        if hasattr(expr, "getValue"):
            return expr.getValue()

        if isinstance(expr, WdlStringLiteral):
            text_parts: list[str] = []
            for component in expr.components():
                if hasattr(component, "text") and component.text is not None:
                    text_parts.append(component.text)
                elif (
                    hasattr(component, "escapeText")
                    and component.escapeText is not None
                ):
                    text_parts.append(component.escapeText)
                elif (
                    hasattr(component, "tokenText") and component.tokenText is not None
                ):
                    text_parts.append(component.tokenText)
                else:
                    return _UNKNOWN
            return "".join(text_parts)

        if isinstance(expr, WdlVariable):
            if expr.name is None:
                return _UNKNOWN
            if expr.name == "None":
                return None
            return self._scope_values.get(expr.name, _UNKNOWN)

        if isinstance(expr, WdlArrayLiteral):
            return [self._evaluate(entry) for entry in expr.entries()]

        if isinstance(expr, WdlPairLiteral):
            left = self._evaluate(expr.left)
            right = self._evaluate(expr.right)
            if left is _UNKNOWN or right is _UNKNOWN:
                return _UNKNOWN
            return (left, right)

        if isinstance(expr, WdlMapLiteral):
            out: dict[Any, Any] = {}
            for entry in expr.entries():
                key = self._evaluate(entry.getKey())
                value = self._evaluate(entry.getValue())
                if key is _UNKNOWN:
                    return _UNKNOWN
                try:
                    out[key] = value
                except TypeError:
                    return _UNKNOWN
            return out

        return _UNKNOWN

    def _infer_type(self, expr: Any) -> WdlType | None:
        if expr is None:
            return None

        if isinstance(expr, WdlIntLiteral):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.INT)
        if isinstance(expr, WdlFloatLiteral):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
        if isinstance(expr, WdlBooleanLiteral):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
        if isinstance(expr, WdlStringLiteral):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
        if isinstance(expr, WdlNullLiteral):
            return None

        if isinstance(expr, WdlVariable):
            if expr.name == "None":
                return None
            return self._scope_types.get(expr.name)

        if isinstance(expr, WdlArrayLiteral):
            member_type: WdlType | None = None
            for item in expr.entries():
                member_type = self._merge_types(member_type, self._infer_type(item))
            return WdlArrayType(member_type)

        if isinstance(expr, WdlPairLiteral):
            left_type = self._infer_type(expr.left)
            right_type = self._infer_type(expr.right)
            if left_type is None or right_type is None:
                return None
            return WdlPairType(left_type, right_type)

        if isinstance(expr, WdlMapLiteral):
            key_type: WdlType | None = None
            value_type: WdlType | None = None
            for entry in expr.entries():
                key_type = self._merge_types(key_type, self._infer_type(entry.getKey()))
                value_type = self._merge_types(
                    value_type, self._infer_type(entry.getValue())
                )
            return WdlMapType(key_type, value_type)

        if isinstance(expr, WdlIndexAccessOperation):
            target_type = self._infer_type(expr.target)
            if isinstance(target_type, WdlArrayType):
                return target_type.memberType()
            if isinstance(target_type, WdlMapType):
                return target_type.valueType()
            return None

        if isinstance(expr, WdlMemberAccessOperation):
            if isinstance(expr.target, WdlVariable) and expr.target.name is not None:
                target_name = expr.target.name
                member_name = expr.member or ""
                if target_name in self._call_output_types:
                    return self._call_output_types[target_name].get(member_name)
                target_type = self._scope_types.get(target_name)
                if isinstance(target_type, WdlTypeReferenceType):
                    members = self._struct_member_types.get(
                        target_type.referenceName() or ""
                    )
                    if members is not None:
                        return members.get(member_name)
            return None

        if isinstance(expr, WdlFunctionCallOperation):
            return self._infer_function_type(expr)

        if isinstance(expr, WdlUnaryOperation):
            if expr.operator == WdlUnaryOperator.NOT:
                return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
            if expr.operator == WdlUnaryOperator.MINUS:
                return self._infer_type(expr.operand)

        if isinstance(expr, WdlBinaryOperation):
            if expr.operator in {
                WdlBinaryOperator.LOGICAL_OR,
                WdlBinaryOperator.LOGICAL_AND,
                WdlBinaryOperator.EQUAL,
                WdlBinaryOperator.NOT_EQUAL,
                WdlBinaryOperator.LESS,
                WdlBinaryOperator.LESS_EQUAL,
                WdlBinaryOperator.GREATER,
                WdlBinaryOperator.GREATER_EQUAL,
            }:
                return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
            if expr.operator in {
                WdlBinaryOperator.ADD,
                WdlBinaryOperator.SUBTRACT,
                WdlBinaryOperator.MULTIPLY,
                WdlBinaryOperator.DIVIDE,
                WdlBinaryOperator.MODULUS,
                WdlBinaryOperator.POWER,
            }:
                left = self._infer_type(expr.left)
                right = self._infer_type(expr.right)
                if self._is_primitive(
                    left, WdlPrimitiveTypeEnum.FLOAT
                ) or self._is_primitive(right, WdlPrimitiveTypeEnum.FLOAT):
                    return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
                if self._is_primitive(
                    left, WdlPrimitiveTypeEnum.INT
                ) and self._is_primitive(right, WdlPrimitiveTypeEnum.INT):
                    return WdlPrimitiveType(WdlPrimitiveTypeEnum.INT)
                if expr.operator == WdlBinaryOperator.ADD and (
                    self._is_primitive(left, WdlPrimitiveTypeEnum.STRING)
                    or self._is_primitive(right, WdlPrimitiveTypeEnum.STRING)
                ):
                    return WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
                return None

        if isinstance(expr, WdlTernaryOperation):
            return self._merge_types(
                self._infer_type(expr.trueValue), self._infer_type(expr.falseValue)
            )

        return None

    def _validate_function_version_availability(
        self, function_call: WdlFunctionCallOperation
    ) -> None:
        fn = function_call.getFunction()
        if fn == WdlFunction.NONSTANDARD or self._document_version is None:
            return
        added_in = self._FUNCTION_ADDED_IN.get(fn)
        if added_in is None:
            return
        if (self._document_version.major, self._document_version.minor) < (
            added_in.major,
            added_in.minor,
        ):
            self._add_error(
                f"Function '{fn.toWdlString()}' is not available in WDL {self._document_version.getVersionString()} (added in {added_in.getVersionString()})",
                code=WdlSemanticErrorCode.FUNCTION_NOT_AVAILABLE_IN_VERSION,
            )

    def _infer_function_type(
        self, function_call: WdlFunctionCallOperation
    ) -> WdlType | None:
        fn = function_call.function
        if fn in {
            WdlFunction.DEFINED,
            WdlFunction.CONTAINS,
            WdlFunction.CONTAINS_KEY,
            WdlFunction.MATCHES,
        }:
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
        if fn in {WdlFunction.LENGTH, WdlFunction.READ_INT}:
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.INT)
        if fn == WdlFunction.READ_FLOAT:
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
        if fn in {
            WdlFunction.READ_STRING,
            WdlFunction.STDOUT,
            WdlFunction.STDERR,
            WdlFunction.WRITE_LINES,
            WdlFunction.WRITE_TSV,
            WdlFunction.WRITE_MAP,
            WdlFunction.WRITE_OBJECT,
            WdlFunction.WRITE_OBJECTS,
            WdlFunction.WRITE_JSON,
            WdlFunction.BASENAME,
            WdlFunction.PREFIX,
            WdlFunction.SUFFIX,
            WdlFunction.QUOTE,
            WdlFunction.SQUOTE,
            WdlFunction.SEP,
        }:
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING)
        if fn == WdlFunction.READ_BOOLEAN:
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.BOOLEAN)
        if fn in {WdlFunction.READ_LINES, WdlFunction.GLOB}:
            return WdlArrayType(WdlPrimitiveType(WdlPrimitiveTypeEnum.STRING))
        if fn == WdlFunction.RANGE:
            return WdlArrayType(WdlPrimitiveType(WdlPrimitiveTypeEnum.INT))
        if fn == WdlFunction.SELECT_FIRST and function_call.arguments():
            arg_type = self._infer_type(function_call.arguments()[0])
            if isinstance(arg_type, WdlArrayType):
                return arg_type.memberType()
        if fn == WdlFunction.ZIP and len(function_call.arguments()) >= 2:
            left = self._infer_type(function_call.arguments()[0])
            right = self._infer_type(function_call.arguments()[1])
            if isinstance(left, WdlArrayType) and isinstance(right, WdlArrayType):
                lm = left.memberType()
                rm = right.memberType()
                if lm is not None and rm is not None:
                    return WdlArrayType(WdlPairType(lm, rm))
        if fn == WdlFunction.AS_MAP and function_call.arguments():
            arg_type = self._infer_type(function_call.arguments()[0])
            if isinstance(arg_type, WdlArrayType) and isinstance(
                arg_type.memberType(), WdlPairType
            ):
                pair = arg_type.memberType()
                return WdlMapType(pair.leftType(), pair.rightType())
        if fn == WdlFunction.KEYS and function_call.arguments():
            arg_type = self._infer_type(function_call.arguments()[0])
            if isinstance(arg_type, WdlMapType):
                return WdlArrayType(arg_type.keyType())
        if fn == WdlFunction.VALUES and function_call.arguments():
            arg_type = self._infer_type(function_call.arguments()[0])
            if isinstance(arg_type, WdlMapType):
                return WdlArrayType(arg_type.valueType())
        return None

    def _is_assignable_from(self, expected: WdlType | None, expr: Any) -> bool:
        if expected is None or expr is None:
            return True
        if isinstance(expr, WdlNullLiteral):
            return expected.isOptional()
        if isinstance(expr, WdlVariable) and expr.name == "None":
            return expected.isOptional()
        actual = self._infer_type(expr)
        if actual is None:
            return True
        if not self._is_type_assignable(expected, actual):
            return False

        if isinstance(expected, WdlArrayType) and isinstance(expr, WdlArrayLiteral):
            member_type = expected.memberType()
            if member_type is None:
                return True
            return all(
                self._is_assignable_from(member_type, item) for item in expr.entries()
            )

        if isinstance(expected, WdlMapType) and isinstance(expr, WdlMapLiteral):
            key_type = expected.keyType()
            value_type = expected.valueType()
            return all(
                self._is_assignable_from(key_type, entry.getKey())
                and self._is_assignable_from(value_type, entry.getValue())
                for entry in expr.entries()
            )

        if isinstance(expected, WdlPairType) and isinstance(expr, WdlPairLiteral):
            return self._is_assignable_from(
                expected.leftType(), expr.left
            ) and self._is_assignable_from(expected.rightType(), expr.right)

        return True

    def _is_type_assignable(
        self, expected: WdlType | None, actual: WdlType | None
    ) -> bool:
        if expected is None or actual is None:
            return True
        if not expected.isOptional() and actual.isOptional():
            return False

        if expected.componentType() != actual.componentType():
            if self._is_primitive(
                expected, WdlPrimitiveTypeEnum.FLOAT
            ) and self._is_primitive(actual, WdlPrimitiveTypeEnum.INT):
                return True
            return False

        if isinstance(expected, WdlPrimitiveType) and isinstance(
            actual, WdlPrimitiveType
        ):
            return expected.primitiveType() == actual.primitiveType()

        if isinstance(expected, WdlArrayType) and isinstance(actual, WdlArrayType):
            return self._is_type_assignable(expected.memberType(), actual.memberType())

        if isinstance(expected, WdlMapType) and isinstance(actual, WdlMapType):
            return self._is_type_assignable(
                expected.keyType(), actual.keyType()
            ) and self._is_type_assignable(expected.valueType(), actual.valueType())

        if isinstance(expected, WdlPairType) and isinstance(actual, WdlPairType):
            return self._is_type_assignable(
                expected.leftType(), actual.leftType()
            ) and self._is_type_assignable(expected.rightType(), actual.rightType())

        if isinstance(expected, WdlTypeReferenceType) and isinstance(
            actual, WdlTypeReferenceType
        ):
            return expected.referenceName() == actual.referenceName()

        return True

    def _merge_types(
        self, current: WdlType | None, nxt: WdlType | None
    ) -> WdlType | None:
        if nxt is None:
            return current
        if current is None:
            return nxt
        if self._is_type_assignable(current, nxt):
            return current
        if self._is_type_assignable(nxt, current):
            return nxt
        if self._is_primitive(current, WdlPrimitiveTypeEnum.INT) and self._is_primitive(
            nxt, WdlPrimitiveTypeEnum.FLOAT
        ):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
        if self._is_primitive(
            current, WdlPrimitiveTypeEnum.FLOAT
        ) and self._is_primitive(nxt, WdlPrimitiveTypeEnum.INT):
            return WdlPrimitiveType(WdlPrimitiveTypeEnum.FLOAT)
        return None

    def _is_primitive(self, t: WdlType | None, p: WdlPrimitiveTypeEnum) -> bool:
        return isinstance(t, WdlPrimitiveType) and t.primitiveType() == p

    def processSelectFirst(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "select_first expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        first_arg = functionCall.arguments()[0]

        arg_type = self._infer_type(first_arg)
        if arg_type is not None and not isinstance(arg_type, WdlArrayType):
            self._add_error(
                "select_first expects an array argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

        if isinstance(first_arg, WdlArrayLiteral) and len(first_arg.entries()) == 0:
            self._add_error(
                "select_first array is empty",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return

        value = self._evaluate(first_arg)
        if isinstance(value, list):
            if len(value) == 0:
                self._add_error(
                    "select_first array is empty",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
            elif all(item is None for item in value):
                self._add_error(
                    "select_first array contains only None values",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

    def processAsMap(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "as_map expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        first_arg = functionCall.arguments()[0]

        first_arg_type = self._infer_type(first_arg)
        if first_arg_type is not None:
            if not isinstance(first_arg_type, WdlArrayType) or not isinstance(
                first_arg_type.memberType(), WdlPairType
            ):
                self._add_error(
                    "as_map expects Array[Pair[K,V]]",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

        if not isinstance(first_arg, WdlArrayLiteral):
            return

        seen: set[Any] = set()
        for entry in first_arg.entries():
            if not isinstance(entry, WdlPairLiteral):
                continue
            key = self._evaluate(entry.left)
            if key is _UNKNOWN:
                continue
            if key in seen:
                self._add_error(
                    f"as_map has duplicate key: {key}",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )
                return
            seen.add(key)

    def processLength(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "length expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        arg_type = self._infer_type(functionCall.arguments()[0])
        if arg_type is not None and not (
            isinstance(arg_type, WdlArrayType)
            or isinstance(arg_type, WdlMapType)
            or self._is_primitive(arg_type, WdlPrimitiveTypeEnum.STRING)
        ):
            self._add_error(
                "length expects an Array, Map, or String argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processContainsKey(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 2:
            self._add_error(
                "contains_key expects exactly 2 arguments",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        map_expr = functionCall.arguments()[0]
        key_expr = functionCall.arguments()[1]

        if isinstance(map_expr, WdlMapLiteral):
            map_type = self._infer_type(map_expr)
            if isinstance(map_type, WdlMapType) and not self._is_assignable_from(
                map_type.keyType(), key_expr
            ):
                self._add_error(
                    "contains_key key argument type is incompatible with map key type",
                    code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
                )

    def processZip(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 2:
            self._add_error(
                "zip expects exactly 2 arguments",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return

        left_type = self._infer_type(functionCall.arguments()[0])
        right_type = self._infer_type(functionCall.arguments()[1])
        if (left_type is not None and not isinstance(left_type, WdlArrayType)) or (
            right_type is not None and not isinstance(right_type, WdlArrayType)
        ):
            self._add_error(
                "zip expects two array arguments",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

        left = self._evaluate(functionCall.arguments()[0])
        right = self._evaluate(functionCall.arguments()[1])

        if (
            isinstance(left, list)
            and isinstance(right, list)
            and len(left) != len(right)
        ):
            self._add_error(
                "zip arguments must have the same length",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def processWriteJson(self, functionCall: WdlFunctionCallOperation) -> None:
        if len(functionCall.arguments()) != 1:
            self._add_error(
                "write_json expects exactly 1 argument",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )
            return
        if self._contains_non_string_map_key(functionCall.arguments()[0]):
            self._add_error(
                "write_json argument contains a map with non-string keys",
                code=WdlSemanticErrorCode.INVALID_FUNCTION_ARGUMENTS,
            )

    def _contains_non_string_map_key(self, expr: Any) -> bool:
        if expr is None:
            return False

        if isinstance(expr, WdlMapLiteral):
            for entry in expr.entries():
                key_value = self._evaluate(entry.getKey())
                if not isinstance(key_value, str):
                    return True
                if self._contains_non_string_map_key(entry.getValue()):
                    return True
            return False

        if isinstance(expr, WdlArrayLiteral):
            return any(
                self._contains_non_string_map_key(item) for item in expr.entries()
            )

        if isinstance(expr, WdlPairLiteral):
            return self._contains_non_string_map_key(
                expr.left
            ) or self._contains_non_string_map_key(expr.right)

        if isinstance(expr, WdlObjectLiteral):
            return any(
                self._contains_non_string_map_key(entry.getValue())
                for entry in expr.entries()
            )

        if isinstance(expr, WdlStructLiteral):
            return any(
                self._contains_non_string_map_key(entry.getValue())
                for entry in expr.entries()
            )

        if isinstance(expr, WdlVariable):
            value = self._scope_values.get(expr.name or "", _UNKNOWN)
            if value is not _UNKNOWN and self._contains_non_string_map_key_in_value(
                value
            ):
                return True
            return False

        return False

    def _contains_non_string_map_key_in_value(self, value: Any) -> bool:
        if isinstance(value, dict):
            if any(not isinstance(k, str) for k in value):
                return True
            return any(
                self._contains_non_string_map_key_in_value(v) for v in value.values()
            )

        if isinstance(value, list):
            return any(
                self._contains_non_string_map_key_in_value(item) for item in value
            )

        if isinstance(value, tuple):
            return any(
                self._contains_non_string_map_key_in_value(item) for item in value
            )

        return False

    def _add_error(
        self,
        message: str,
        code: WdlSemanticErrorCode = WdlSemanticErrorCode.GENERIC_SEMANTIC_ERROR,
    ) -> None:
        self._errors.append(
            WdlSemanticError(
                message=message,
                line=0,
                charPositionInLine=0,
                code=code,
            )
        )
