"""Processor that renders the Python WDL object model back into source text."""

from __future__ import annotations

from io import StringIO

from wdl_model.model.definitions import (
    WdlEnum,
    WdlStruct,
    WdlStructMember,
    WdlTask,
    WdlWorkflow,
)
from wdl_model.model.sections import (
    WdlCommand,
    WdlInput,
    WdlMetadata,
    WdlOutput,
    WdlParameterMetadata,
    WdlRequirements,
    WdlRuntime,
    WdlTaskHints,
    WdlWorkflowHints,
)
from wdl_model.model.statements import (
    ComponentType as StatementComponentType,
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
    WdlStatement,
)

from ..wdl_document import WdlDocument
from ..wdl_version import WdlVersion
from .wdl_processor_base import WdlProcessorBase


class WdlAppendingProcessor(WdlProcessorBase):
    """Concrete processor that appends a WDL rendering to a string buffer."""

    def __init__(self, out: StringIO | None = None):
        self.out = out or StringIO()

    def append(self, value: str) -> "WdlAppendingProcessor":
        """Append raw text to the output and return ``self`` for chaining."""
        self.out.write(value)
        return self

    def getValue(self) -> str:
        """Return the accumulated WDL text."""
        return self.out.getvalue()

    def processVersion(self, ctx: WdlDocument, node: WdlVersion) -> None:
        self.append(f"version {node.getVersionString()}\n")

    def processImportStandard(self, ctx: WdlDocument, node: WdlImportStandard) -> None:
        line = f"import {self.stringLiteralToWdl(node.source, True)}"
        if node.alias:
            line += f" as {node.alias}"
        self.append(line + "\n")
        for member in node.members():
            alias = f" as {member.alias}" if member.alias else ""
            self.append(f"  alias {member.member}{alias}\n")

    def processImportMembers(self, ctx: WdlDocument, node: WdlImportMembers) -> None:
        members = ", ".join(
            f"{m.member}{f' as {m.alias}' if m.alias else ''}" for m in node.members()
        )
        self.append(
            f"import {{ {members} }} from {self.stringLiteralToWdl(node.source, True)}\n"
        )

    def processImportStar(self, ctx: WdlDocument, node: WdlImportStar) -> None:
        self.append(f"import * from {self.stringLiteralToWdl(node.source, True)}\n")

    def processEnum(self, ctx: WdlDocument, node: WdlEnum) -> None:
        self.append(f"enum {node.name or ''}")
        if node.valueType is not None:
            self.append(f"[{self.typeToWdl(node.valueType)}]")
        self.append(" {\n")
        rendered = []
        for choice in node.elements():
            item = f"  {choice.getKey()}"
            if choice.getValue() is not None:
                item += f" = {self.expressionToWdl(choice.getValue())}"
            rendered.append(item)
        self.append(",\n".join(rendered))
        self.append("\n}\n")

    def processStruct(self, ctx: WdlDocument, node: WdlStruct) -> None:
        self.append(f"struct {node.name or ''}{{\n")
        super().processStruct(ctx, node)
        self.append("}\n")

    def processStructMember(self, ctx: WdlStruct, node: WdlStructMember) -> None:
        self.append(f"  {self.typeToWdl(node.type)} {node.name}\n")

    def processStructParameterMetadata(
        self, ctx: WdlStruct, node: WdlParameterMetadata
    ) -> None:
        self.processParameterMetadata(node)

    def processStructMetadata(self, ctx: WdlStruct, node: WdlMetadata) -> None:
        self.processMetadata(node)

    def processTask(self, ctx: WdlDocument, node: WdlTask) -> None:
        self.append(f"task {node.name or ''}{{\n")
        super().processTask(ctx, node)
        self.append("}\n")

    def processTaskDeclaration(self, ctx: WdlTask, node: WdlBoundDeclaration) -> None:
        self.append(f"  {self.declarationToWdl(node)}\n")

    def processTaskInput(self, ctx: WdlTask, node: WdlInput) -> None:
        self._process_input(node)

    def processTaskOutput(self, ctx: WdlTask, node: WdlOutput) -> None:
        self._process_output(node)

    def processTaskCommand(self, ctx: WdlTask, node: WdlCommand) -> None:
        open_delim = "<<<" if node.isMultiline() else "{"
        close_delim = ">>>" if node.isMultiline() else "}"
        self.append("  command ")
        self.append(open_delim)
        self.append(self.stringLiteralToWdl(node.getCommandText(), False))
        self.append(close_delim)
        self.append("\n")

    def processTaskParameterMetadata(
        self, ctx: WdlTask, node: WdlParameterMetadata
    ) -> None:
        self.processParameterMetadata(node)

    def processTaskMetadata(self, ctx: WdlTask, node: WdlMetadata) -> None:
        self.processMetadata(node)

    def processTaskRequirements(self, ctx: WdlTask, node: WdlRequirements) -> None:
        self.append("  requirements {\n")
        self.append(
            "\n".join(
                f"    {element.getKey()}: {self.expressionToWdl(element.getValue())}"
                for element in node.elements()
            )
        )
        self.append("\n  }\n")

    def processTaskRuntime(self, ctx: WdlTask, node: WdlRuntime) -> None:
        self.append("  runtime {\n")
        self.append(
            "\n".join(
                f"    {element.getKey()}: {self.expressionToWdl(element.getValue())}"
                for element in node.elements()
            )
        )
        self.append("\n  }\n")

    def processTaskHints(self, ctx: WdlTask, node: WdlTaskHints) -> None:
        self.processHints(node)

    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None:
        self.append(f"workflow {node.name or ''}{{\n")
        super().processWorkflow(ctx, node)
        self.append("}\n")

    def processWorkflowDeclaration(
        self, ctx: WdlWorkflow, node: WdlBoundDeclaration
    ) -> None:
        self.processStatement(node, 0)

    def processWorkflowInput(self, ctx: WdlWorkflow, node: WdlInput) -> None:
        self._process_input(node)

    def processWorkflowOutput(self, ctx: WdlWorkflow, node: WdlOutput) -> None:
        self._process_output(node)

    def processWorkflowMetadata(self, ctx: WdlWorkflow, node: WdlMetadata) -> None:
        self.processMetadata(node)

    def processWorkflowParameterMetadata(
        self, ctx: WdlWorkflow, node: WdlParameterMetadata
    ) -> None:
        self.processParameterMetadata(node)

    def processWorkflowCall(self, ctx: WdlWorkflow, node: WdlCall) -> None:
        self.processStatement(node, 0)

    def processWorkflowConditional(
        self, ctx: WdlWorkflow, node: WdlConditional
    ) -> None:
        self.processStatement(node, 0)

    def processWorkflowScatter(self, ctx: WdlWorkflow, node: WdlScatter) -> None:
        self.processStatement(node, 0)

    def processWorkflowHints(self, ctx: WdlWorkflow, node: WdlWorkflowHints) -> None:
        self.processHints(node)

    def processStatement(self, statement: WdlStatement, indent_level: int) -> None:
        component_type = statement.componentType()
        if component_type == StatementComponentType.DECLARATION:
            self.processStatementDeclaration(statement, indent_level)
        elif component_type == StatementComponentType.CALL:
            self.processStatementCall(statement, indent_level)
        elif component_type == StatementComponentType.SCATTER:
            self.processStatementScatter(statement, indent_level)
        elif component_type == StatementComponentType.CONDITIONAL:
            self.processStatementConditional(statement, indent_level)
        else:
            raise TypeError(f"Unhandled statement type: {component_type}")

    def indent(self, indent_level: int) -> None:
        self.append("  " * (indent_level + 1))

    def processStatementDeclaration(
        self, node: WdlBoundDeclaration, indent_level: int
    ) -> None:
        self.indent(indent_level)
        self.append(self.declarationToWdl(node))
        self.append("\n")

    def processStatementCall(self, node: WdlCall, indent_level: int) -> None:
        self.indent(indent_level)
        self.append("call ")
        self.append(node.targetPathAsString())
        if node.alias is not None:
            self.append(f" as {node.alias}")
        for dependency in node.afterDependencies():
            self.append(" after ")
            self.append(dependency)
        if node.inputs():
            self.append("  {")
            if node.legacyInputColonUsed:
                self.append(" input: ")
            self.append(
                ", ".join(
                    self.keyValueToWdl(call_input, " = ")
                    for call_input in node.inputs()
                )
            )
            self.append("  }")
        self.append("\n")

    def processStatementScatter(self, node: WdlScatter, indent_level: int) -> None:
        self.indent(indent_level)
        self.append("scatter (")
        self.append(node.name or "")
        self.append(" in ")
        self.append(self.expressionToWdl(node.collection))
        self.append(") {\n")
        for statement in node.statements():
            self.processStatement(statement, indent_level + 1)
        self.indent(indent_level)
        self.append("}\n")

    def processStatementConditional(
        self, node: WdlConditional, indent_level: int
    ) -> None:
        self.indent(indent_level)
        self.append("if (")
        self.append(self.expressionToWdl(node.condition))
        self.append(") {\n")
        for statement in node.thenStatements():
            self.processStatement(statement, indent_level + 1)
        self.indent(indent_level)
        self.append("}")

        if node.elseIfs():
            for else_if in node.elseIfs():
                self.append(" else if (")
                self.append(self.expressionToWdl(else_if.condition))
                self.append(") {\n")
                for statement in else_if.thenStatements():
                    self.processStatement(statement, indent_level + 1)
                self.indent(indent_level)
                self.append("}")

        if node.elseStatements():
            self.append(" else {\n")
            for statement in node.elseStatements():
                self.processStatement(statement, indent_level + 1)
            self.indent(indent_level)
            self.append("}")

        self.append("\n")

    def _process_input(self, node: WdlInput) -> None:
        self.append("  input {\n")
        self.append(
            "\n".join(
                f"    {self.declarationToWdl(declaration)}"
                for declaration in node.elements()
            )
        )
        self.append("\n  }\n")

    def _process_output(self, node: WdlOutput) -> None:
        self.append("  output {\n")
        self.append(
            "\n".join(
                f"    {self.declarationToWdl(declaration)}"
                for declaration in node.elements()
            )
        )
        self.append("\n  }\n")

    def processParameterMetadata(self, node: WdlParameterMetadata) -> None:
        self.append("  parameter_meta {\n")
        self.append(
            "\n".join(
                f"    {item.getKey()}:{self.expressionToWdl(item.getValue())}"
                for item in node.elements()
            )
        )
        self.append("\n  }\n")

    def processMetadata(self, node: WdlMetadata) -> None:
        self.append("  meta {\n")
        self.append(
            "\n".join(
                f"    {item.getKey()}:{self.expressionToWdl(item.getValue())}"
                for item in node.elements()
            )
        )
        self.append("\n  }\n")

    def processHints(self, hints: WdlTaskHints | WdlWorkflowHints) -> None:
        self.append("  hints {\n")
        self.append(
            "\n".join(
                f"    {self.keyValueToWdl(element)}" for element in hints.elements()
            )
        )
        self.append("\n  }\n")
