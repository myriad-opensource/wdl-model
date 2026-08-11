"""Linting validator for Python WDL documents.

This layer sits on top of static analysis and emits warning-severity diagnostics for usage issues,
including unused declarations, unused scatter variables, and unreferenced call outputs.
Callers can keep these diagnostics non-raising by setting `setThrowOnWarnings(False)`.
"""

from __future__ import annotations

from dataclasses import dataclass, field

from wdl_model.model.definitions import WdlTask, WdlWorkflow
from wdl_model.model.expressions import (
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlExpression,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlMapLiteral,
    WdlMemberAccessOperation,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStructLiteral,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlVariable,
)
from wdl_model.model.errors import WdlSemanticErrorCode
from wdl_model.model.sections import (
    WdlCommand,
    WdlInput,
    WdlOutput,
    WdlRequirements,
    WdlRuntime,
    WdlTaskHints,
    WdlWorkflowHints,
)
from wdl_model.model.statements import (
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlScatter,
)
from wdl_model.model.types import (
    Type as WdlPrimitiveTypeEnum,
    WdlArrayType,
    WdlMapType,
    WdlPairType,
    WdlPrimitiveType,
    WdlType,
    WdlTypeReferenceType,
)
from wdl_model.model.wdl_document import WdlDocument

from .wdl_static_analysis_semantic_validator import WdlStaticAnalysisSemanticValidator


@dataclass
class _Usage:
    used_variables: set[str] = field(default_factory=set)
    used_call_output_targets: set[str] = field(default_factory=set)

    def merge(self, other: "_Usage") -> None:
        self.used_variables.update(other.used_variables)
        self.used_call_output_targets.update(other.used_call_output_targets)


class WdlLintingSemanticValidator(WdlStaticAnalysisSemanticValidator):
    """Linting validator that extends static analysis with usage-oriented warning diagnostics."""

    def processDocument(self, node: WdlDocument) -> None:
        super().processDocument(node)
        self._lint_deprecated_document_features(node)

    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None:
        super().processWorkflow(ctx, node)
        self._lint_deprecated_workflow_types(node)
        self._lint_workflow(node)

    def processTask(self, ctx: WdlDocument, node: WdlTask) -> None:
        super().processTask(ctx, node)
        self._lint_deprecated_task_features(node)
        self._lint_task(node)

    def _lint_deprecated_document_features(self, document: WdlDocument) -> None:
        for imp in document.importStatements():
            source = self._import_source_text(imp)
            if source.startswith("file://"):
                self._add_error(
                    f"Import source uses deprecated file:// URI: '{source}'",
                    code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
                )

    def _lint_deprecated_workflow_types(self, workflow: WdlWorkflow) -> None:
        for element in workflow.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    self._lint_deprecated_type_usage(
                        declaration.type, "workflow input", declaration.name
                    )
            elif isinstance(element, WdlBoundDeclaration):
                self._lint_deprecated_type_usage(
                    element.type, "workflow declaration", element.name
                )
            elif isinstance(element, WdlOutput):
                for declaration in element.elements():
                    self._lint_deprecated_type_usage(
                        declaration.type, "workflow output", declaration.name
                    )

    def _lint_deprecated_task_features(self, task: WdlTask) -> None:
        for element in task.elements():
            if isinstance(element, WdlRuntime):
                self._add_error(
                    f"Task '{task.name or '<task>'}' uses deprecated runtime section; use requirements/hints instead",
                    code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
                )
            elif isinstance(element, WdlRequirements):
                for entry in element.elements():
                    if entry.getKey() == "docker":
                        self._add_error(
                            f"Task '{task.name or '<task>'}' uses deprecated requirements key 'docker'; use 'container'",
                            code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
                        )
            elif isinstance(element, WdlInput):
                for declaration in element.elements():
                    self._lint_deprecated_type_usage(
                        declaration.type, "task input", declaration.name
                    )
            elif isinstance(element, WdlBoundDeclaration):
                self._lint_deprecated_type_usage(
                    element.type, "task declaration", element.name
                )
            elif isinstance(element, WdlOutput):
                for declaration in element.elements():
                    self._lint_deprecated_type_usage(
                        declaration.type, "task output", declaration.name
                    )

    def _lint_deprecated_type_usage(
        self, type_node: WdlType | None, scope: str, name: str | None
    ) -> None:
        if type_node is None:
            return

        if isinstance(type_node, WdlPrimitiveType):
            if type_node.primitiveType() == WdlPrimitiveTypeEnum.OBJECT:
                self._add_error(
                    f"Deprecated Object type used in {scope} '{name or '<unnamed>'}'",
                    code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
                )
            return

        if isinstance(type_node, WdlArrayType):
            self._lint_deprecated_type_usage(type_node.memberType(), scope, name)
            return
        if isinstance(type_node, WdlMapType):
            self._lint_deprecated_type_usage(type_node.keyType(), scope, name)
            self._lint_deprecated_type_usage(type_node.valueType(), scope, name)
            return
        if isinstance(type_node, WdlPairType):
            self._lint_deprecated_type_usage(type_node.leftType(), scope, name)
            self._lint_deprecated_type_usage(type_node.rightType(), scope, name)
            return
        if isinstance(type_node, WdlTypeReferenceType):
            if type_node.referenceName() == "Object":
                self._add_error(
                    f"Deprecated Object type used in {scope} '{name or '<unnamed>'}'",
                    code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
                )

    def _lint_workflow(self, workflow: WdlWorkflow) -> None:
        declared_names: set[str] = set()
        call_names: set[str] = set()
        usage = _Usage()

        for element in workflow.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    if declaration.name is not None:
                        declared_names.add(declaration.name)
                    if isinstance(declaration, WdlBoundDeclaration):
                        self._collect_expression_usage(declaration.expression, usage)
            elif isinstance(element, WdlBoundDeclaration):
                if element.name is not None:
                    declared_names.add(element.name)
                self._collect_expression_usage(element.expression, usage)
            elif isinstance(element, WdlCall):
                self._collect_call_usage(element, usage, call_names)
            elif isinstance(element, WdlScatter):
                self._collect_scatter_usage(element, usage, declared_names, call_names)
            elif isinstance(element, WdlConditional):
                self._collect_conditional_usage(
                    element, usage, declared_names, call_names
                )
            elif isinstance(element, WdlOutput):
                for declaration in element.elements():
                    self._collect_expression_usage(declaration.expression, usage)
            elif isinstance(element, WdlWorkflowHints):
                for hint in element.elements():
                    self._collect_expression_usage(hint.getValue(), usage)

        for name in declared_names:
            if name not in usage.used_variables:
                self._add_error(
                    f"Lint: workflow declaration '{name}' is never used",
                    code=WdlSemanticErrorCode.LINT_UNUSED_WORKFLOW_DECLARATION,
                )
        for call_name in call_names:
            if call_name not in usage.used_call_output_targets:
                self._add_error(
                    f"Lint: call '{call_name}' outputs are never referenced",
                    code=WdlSemanticErrorCode.LINT_UNUSED_CALL_OUTPUT,
                )

    def _lint_task(self, task: WdlTask) -> None:
        declared_names: set[str] = set()
        usage = _Usage()

        for element in task.elements():
            if isinstance(element, WdlInput):
                for declaration in element.elements():
                    if declaration.name is not None:
                        declared_names.add(declaration.name)
                    if isinstance(declaration, WdlBoundDeclaration):
                        self._collect_expression_usage(declaration.expression, usage)
            elif isinstance(element, WdlBoundDeclaration):
                if element.name is not None:
                    declared_names.add(element.name)
                self._collect_expression_usage(element.expression, usage)
            elif isinstance(element, WdlOutput):
                for declaration in element.elements():
                    self._collect_expression_usage(declaration.expression, usage)
            elif isinstance(element, WdlCommand):
                self._collect_string_literal_usage(element.getCommandText(), usage)
            elif isinstance(element, WdlRuntime):
                for entry in element.elements():
                    self._collect_expression_usage(entry.getValue(), usage)
            elif isinstance(element, WdlRequirements):
                for entry in element.elements():
                    self._collect_expression_usage(entry.getValue(), usage)
            elif isinstance(element, WdlTaskHints):
                for hint in element.elements():
                    self._collect_expression_usage(hint.getValue(), usage)

        for name in declared_names:
            if name not in usage.used_variables:
                self._add_error(
                    f"Lint: task declaration '{name}' is never used",
                    code=WdlSemanticErrorCode.LINT_UNUSED_TASK_DECLARATION,
                )

    def _collect_statements_usage(
        self,
        statements,
        usage: _Usage,
        declared_names: set[str],
        call_names: set[str],
    ) -> None:
        for statement in statements:
            if isinstance(statement, WdlBoundDeclaration):
                if statement.name is not None:
                    declared_names.add(statement.name)
                self._collect_expression_usage(statement.expression, usage)
            elif isinstance(statement, WdlCall):
                self._collect_call_usage(statement, usage, call_names)
            elif isinstance(statement, WdlScatter):
                self._collect_scatter_usage(
                    statement, usage, declared_names, call_names
                )
            elif isinstance(statement, WdlConditional):
                self._collect_conditional_usage(
                    statement, usage, declared_names, call_names
                )

    def _collect_conditional_usage(
        self,
        conditional: WdlConditional,
        usage: _Usage,
        declared_names: set[str],
        call_names: set[str],
    ) -> None:
        self._collect_expression_usage(conditional.condition, usage)
        self._collect_statements_usage(
            conditional.thenStatements(), usage, declared_names, call_names
        )
        for else_if in conditional.elseIfs():
            self._collect_expression_usage(else_if.condition, usage)
            self._collect_statements_usage(
                else_if.thenStatements(), usage, declared_names, call_names
            )
        self._collect_statements_usage(
            conditional.elseStatements(), usage, declared_names, call_names
        )

    def _collect_scatter_usage(
        self,
        scatter: WdlScatter,
        usage: _Usage,
        declared_names: set[str],
        call_names: set[str],
    ) -> None:
        self._collect_expression_usage(scatter.collection, usage)
        scatter_var = scatter.name
        if scatter_var is not None:
            declared_names.add(scatter_var)

        body_usage = _Usage()
        self._collect_statements_usage(
            scatter.statements(), body_usage, declared_names, call_names
        )
        usage.merge(body_usage)
        if scatter_var is not None and scatter_var not in body_usage.used_variables:
            self._add_error(
                f"Lint: scatter variable '{scatter_var}' is never used",
                code=WdlSemanticErrorCode.LINT_UNUSED_SCATTER_VARIABLE,
            )

    def _collect_call_usage(
        self,
        call: WdlCall,
        usage: _Usage,
        call_names: set[str],
    ) -> None:
        target = call.targetPath()[-1] if call.targetPath() else None
        call_name = call.alias or target
        if call_name is not None:
            call_names.add(call_name)
        for call_input in call.inputs():
            self._collect_expression_usage(call_input.getValue(), usage)

    def _collect_expression_usage(
        self, expr: WdlExpression | None, usage: _Usage
    ) -> None:
        if expr is None:
            return

        if isinstance(expr, WdlVariable):
            if expr.name is not None:
                usage.used_variables.add(expr.name)
            return

        if isinstance(expr, WdlBinaryOperation):
            self._collect_expression_usage(expr.left, usage)
            self._collect_expression_usage(expr.right, usage)
            return

        if isinstance(expr, WdlUnaryOperation):
            self._collect_expression_usage(expr.operand, usage)
            return

        if isinstance(expr, WdlTernaryOperation):
            self._collect_expression_usage(expr.condition, usage)
            self._collect_expression_usage(expr.trueValue, usage)
            self._collect_expression_usage(expr.falseValue, usage)
            return

        if isinstance(expr, WdlFunctionCallOperation):
            for arg in expr.arguments():
                self._collect_expression_usage(arg, usage)
            return

        if isinstance(expr, WdlIndexAccessOperation):
            self._collect_expression_usage(expr.target, usage)
            self._collect_expression_usage(expr.index, usage)
            return

        if isinstance(expr, WdlMemberAccessOperation):
            if isinstance(expr.target, WdlVariable) and expr.target.name is not None:
                usage.used_call_output_targets.add(expr.target.name)
            self._collect_expression_usage(expr.target, usage)
            return

        if isinstance(expr, WdlArrayLiteral):
            for item in expr.entries():
                self._collect_expression_usage(item, usage)
            return

        if isinstance(expr, WdlMapLiteral):
            for entry in expr.entries():
                self._collect_expression_usage(entry.getKey(), usage)
                self._collect_expression_usage(entry.getValue(), usage)
            return

        if isinstance(expr, WdlPairLiteral):
            self._collect_expression_usage(expr.left, usage)
            self._collect_expression_usage(expr.right, usage)
            return

        if isinstance(expr, WdlObjectLiteral):
            for entry in expr.entries():
                self._collect_expression_usage(entry.getValue(), usage)
            return

        if isinstance(expr, WdlStructLiteral):
            for entry in expr.entries():
                self._collect_expression_usage(entry.getValue(), usage)
            return

        if isinstance(expr, WdlStringLiteral):
            self._collect_string_literal_usage(expr, usage)
            self._lint_deprecated_placeholder_options(expr)

    def _lint_deprecated_placeholder_options(
        self, string_literal: WdlStringLiteral
    ) -> None:
        for component in string_literal.components():
            if not isinstance(component, WdlStringPlaceholder):
                continue
            option = component.option
            if option is None:
                continue
            self._add_error(
                "Deprecated placeholder option syntax is used",
                code=WdlSemanticErrorCode.LINT_DEPRECATED_FEATURE,
            )

    def _collect_string_literal_usage(
        self, string_literal: WdlStringLiteral | None, usage: _Usage
    ) -> None:
        if string_literal is None:
            return
        self._lint_deprecated_placeholder_options(string_literal)
        for component in string_literal.components():
            if not isinstance(component, WdlStringPlaceholder):
                continue
            self._collect_expression_usage(component.expression, usage)
            option = component.option
            if option is None:
                continue
            self._collect_string_literal_usage(option.value, usage)
            self._collect_string_literal_usage(option.trueValue, usage)
            self._collect_string_literal_usage(option.falseValue, usage)
