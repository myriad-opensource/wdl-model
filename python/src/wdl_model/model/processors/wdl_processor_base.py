"""Default source-order traversal for the Python WDL object model."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, TypeVar

from wdl_model.model.base import WdlNode, WdlStringKeyValue
from wdl_model.model.definitions import (
    WdlEnum,
    WdlStruct,
    WdlStructMember,
    WdlTask,
    WdlWorkflow,
)
from wdl_model.model.expressions import (
    Delimiter,
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlBooleanLiteral,
    WdlExpression,
    WdlFloatLiteral,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlIntLiteral,
    WdlLiteral,
    WdlMapLiteral,
    WdlMemberAccessOperation,
    WdlNullLiteral,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringEscape,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStringPlaceholderOptionType,
    WdlStringText,
    WdlStringToken,
    WdlStructLiteral,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlVariable,
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
    WdlBoundDeclaration,
    WdlCall,
    WdlConditional,
    WdlDeclaration,
    WdlImport,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
)
from wdl_model.model.types import (
    WdlArrayType,
    WdlMapType,
    WdlPairType,
    WdlPrimitiveType,
    WdlType,
    WdlTypeReferenceType,
)

from ..wdl_document import WdlDocument
from ..wdl_version import WdlVersion
from .wdl_processor import WdlProcessor

TNode = TypeVar("TNode", bound=WdlNode)


@dataclass(frozen=True)
class ResolvedImport(Generic[TNode]):
    """Imported symbol resolution result.

    The record includes both source and local visibility metadata so callers can understand how a
    symbol became visible in the importing document.
    """

    local_name: str
    imported_name: str
    import_namespace: str | None
    import_statement: WdlImport
    imported_document: WdlDocument
    symbol: TNode


class WdlProcessorBase(WdlProcessor):
    """Base class that walks documents and renders helpers for WDL model consumers."""

    def processDocument(self, node: WdlDocument) -> None:
        """Walk the document root and dispatch to the appropriate element-level callbacks."""
        if node.getWdlVersion() is not None:
            self.processVersion(node, node.getWdlVersion())
        for element in node.elements():
            if isinstance(element, WdlImportStandard):
                self.processImportStandard(node, element)
            elif isinstance(element, WdlImportMembers):
                self.processImportMembers(node, element)
            elif isinstance(element, WdlImportStar):
                self.processImportStar(node, element)
            elif isinstance(element, WdlEnum):
                self.processEnum(node, element)
            elif isinstance(element, WdlStruct):
                self.processStruct(node, element)
            elif isinstance(element, WdlTask):
                self.processTask(node, element)
            elif isinstance(element, WdlWorkflow):
                self.processWorkflow(node, element)
            else:
                self.processUnexpectedNode(node, element)

    def processVersion(self, ctx: WdlDocument, node: WdlVersion) -> None:
        pass

    def processImportStandard(self, ctx: WdlDocument, node: WdlImportStandard) -> None:
        pass

    def processImportMembers(self, ctx: WdlDocument, node: WdlImportMembers) -> None:
        pass

    def processImportStar(self, ctx: WdlDocument, node: WdlImportStar) -> None:
        pass

    def processEnum(self, ctx: WdlDocument, node: WdlEnum) -> None:
        pass

    def processStruct(self, ctx: WdlDocument, node: WdlStruct) -> None:
        for element in node.elements():
            if isinstance(element, WdlStructMember):
                self.processStructMember(node, element)
            elif isinstance(element, WdlParameterMetadata):
                self.processStructParameterMetadata(node, element)
            elif isinstance(element, WdlMetadata):
                self.processStructMetadata(node, element)
            else:
                self.processUnexpectedNode(node, element)

    def processStructMember(self, ctx: WdlStruct, node: WdlStructMember) -> None:
        pass

    def processStructParameterMetadata(
        self, ctx: WdlStruct, node: WdlParameterMetadata
    ) -> None:
        pass

    def processStructMetadata(self, ctx: WdlStruct, node: WdlMetadata) -> None:
        pass

    def processTask(self, ctx: WdlDocument, node: WdlTask) -> None:
        for element in node.elements():
            if isinstance(element, WdlBoundDeclaration):
                self.processTaskDeclaration(node, element)
            elif isinstance(element, WdlInput):
                self.processTaskInput(node, element)
            elif isinstance(element, WdlOutput):
                self.processTaskOutput(node, element)
            elif isinstance(element, WdlCommand):
                self.processTaskCommand(node, element)
            elif isinstance(element, WdlMetadata):
                self.processTaskMetadata(node, element)
            elif isinstance(element, WdlParameterMetadata):
                self.processTaskParameterMetadata(node, element)
            elif isinstance(element, WdlRequirements):
                self.processTaskRequirements(node, element)
            elif isinstance(element, WdlRuntime):
                self.processTaskRuntime(node, element)
            elif isinstance(element, WdlTaskHints):
                self.processTaskHints(node, element)
            else:
                self.processUnexpectedNode(node, element)

    def processTaskDeclaration(self, ctx: WdlTask, node: WdlBoundDeclaration) -> None:
        pass

    def processTaskInput(self, ctx: WdlTask, node: WdlInput) -> None:
        pass

    def processTaskOutput(self, ctx: WdlTask, node: WdlOutput) -> None:
        pass

    def processTaskCommand(self, ctx: WdlTask, node: WdlCommand) -> None:
        pass

    def processTaskParameterMetadata(
        self, ctx: WdlTask, node: WdlParameterMetadata
    ) -> None:
        pass

    def processTaskMetadata(self, ctx: WdlTask, node: WdlMetadata) -> None:
        pass

    def processTaskRequirements(self, ctx: WdlTask, node: WdlRequirements) -> None:
        pass

    def processTaskRuntime(self, ctx: WdlTask, node: WdlRuntime) -> None:
        pass

    def processTaskHints(self, ctx: WdlTask, node: WdlTaskHints) -> None:
        pass

    def processWorkflow(self, ctx: WdlDocument, node: WdlWorkflow) -> None:
        for element in node.elements():
            if isinstance(element, WdlBoundDeclaration):
                self.processWorkflowDeclaration(node, element)
            elif isinstance(element, WdlCall):
                self.processWorkflowCall(node, element)
            elif isinstance(element, WdlConditional):
                self.processWorkflowConditional(node, element)
            elif isinstance(element, WdlInput):
                self.processWorkflowInput(node, element)
            elif isinstance(element, WdlOutput):
                self.processWorkflowOutput(node, element)
            elif isinstance(element, WdlMetadata):
                self.processWorkflowMetadata(node, element)
            elif isinstance(element, WdlParameterMetadata):
                self.processWorkflowParameterMetadata(node, element)
            elif isinstance(element, WdlScatter):
                self.processWorkflowScatter(node, element)
            elif isinstance(element, WdlWorkflowHints):
                self.processWorkflowHints(node, element)
            else:
                self.processUnexpectedNode(node, element)

    def processWorkflowDeclaration(
        self, ctx: WdlWorkflow, node: WdlBoundDeclaration
    ) -> None:
        pass

    def processWorkflowInput(self, ctx: WdlWorkflow, node: WdlInput) -> None:
        pass

    def processWorkflowOutput(self, ctx: WdlWorkflow, node: WdlOutput) -> None:
        pass

    def processWorkflowMetadata(self, ctx: WdlWorkflow, node: WdlMetadata) -> None:
        pass

    def processWorkflowParameterMetadata(
        self, ctx: WdlWorkflow, node: WdlParameterMetadata
    ) -> None:
        pass

    def processWorkflowCall(self, ctx: WdlWorkflow, node: WdlCall) -> None:
        pass

    def processWorkflowConditional(
        self, ctx: WdlWorkflow, node: WdlConditional
    ) -> None:
        pass

    def processWorkflowScatter(self, ctx: WdlWorkflow, node: WdlScatter) -> None:
        pass

    def processWorkflowHints(self, ctx: WdlWorkflow, node: WdlWorkflowHints) -> None:
        pass

    def processUnexpectedNode(self, ctx: WdlNode, node: WdlNode) -> None:
        """Raise when traversal encounters a node that the current context does not expect."""
        raise TypeError(
            f"Unexpected node {node.__class__.__name__} under {ctx.__class__.__name__}"
        )

    def resolveImportedTasks(
        self, context: WdlDocument | None, call_target: str | None
    ) -> list[ResolvedImport[WdlTask]]:
        """Resolve imported tasks visible for a call target (for example ``ns.task`` or ``task``)."""
        return self._resolve_imported_callables(
            context, call_target, lambda doc: doc.tasks()
        )

    def resolveImportedWorkflows(
        self, context: WdlDocument | None, call_target: str | None
    ) -> list[ResolvedImport[WdlWorkflow]]:
        """Resolve imported workflows visible for a call target."""
        return self._resolve_imported_callables(
            context, call_target, lambda doc: doc.workflows()
        )

    def resolveImportedStructs(
        self, context: WdlDocument | None, visible_type_name: str | None
    ) -> list[ResolvedImport[WdlStruct]]:
        """Resolve imported struct definitions by local visible type name."""
        return self._resolve_imported_types(
            context,
            visible_type_name,
            True,  # structs
        )

    def resolveImportedEnums(
        self, context: WdlDocument | None, visible_type_name: str | None
    ) -> list[ResolvedImport[WdlEnum]]:
        """Resolve imported enum definitions by local visible type name."""
        return self._resolve_imported_types(
            context,
            visible_type_name,
            False,  # enums
        )

    def resolveImportedDocument(
        self, context: WdlDocument | None, imp: WdlImport | None
    ) -> WdlDocument | None:
        """Resolve the imported document model for a specific import statement."""
        if context is None or imp is None:
            return None
        key = imp.importIdentifier
        if key is None or not key.strip():
            return None
        return context.importedDocuments().get(key)

    def importNamespace(self, imp: WdlImportStandard) -> str:
        """Return the namespace used for a standard import."""
        if imp.alias is not None and imp.alias.strip():
            return imp.alias

        source = self.importSourceText(imp)
        if not source.strip():
            return ""

        path = source
        if "/" in path:
            path = path.rsplit("/", 1)[-1]
        if path.endswith(".wdl") and len(path) > 4:
            path = path[:-4]
        return path

    def importSourceText(self, imp: WdlImport | None) -> str:
        """Extract raw text for an import source literal."""
        if imp is None:
            return ""
        if imp.source is None:
            return imp.sourceText or ""
        chunks: list[str] = []
        for component in imp.source.components():
            if isinstance(component, WdlStringText):
                chunks.append(component.text or "")
            elif isinstance(component, WdlStringEscape):
                chunks.append(component.escapeText or "")
        return "".join(chunks)

    def _resolve_imported_callables(self, context, call_target, selector):
        if context is None or call_target is None or not call_target.strip():
            return []

        results = []
        qualified = "." in call_target
        namespace_part, member_part = (
            call_target.split(".", 1) if qualified else ("", call_target)
        )

        for imp in context.importStatements():
            imported = self.resolveImportedDocument(context, imp)
            if imported is None:
                continue

            if isinstance(imp, WdlImportStandard):
                namespace = self.importNamespace(imp)
                if not qualified or namespace != namespace_part:
                    continue
                for node in selector(imported):
                    name = getattr(node, "name", None)
                    if name == member_part:
                        results.append(
                            ResolvedImport(
                                local_name=f"{namespace}.{member_part}",
                                imported_name=member_part,
                                import_namespace=namespace,
                                import_statement=imp,
                                imported_document=imported,
                                symbol=node,
                            )
                        )
            elif isinstance(imp, WdlImportStar):
                if qualified:
                    continue
                for node in selector(imported):
                    name = getattr(node, "name", None)
                    if name == member_part:
                        results.append(
                            ResolvedImport(
                                local_name=member_part,
                                imported_name=member_part,
                                import_namespace=None,
                                import_statement=imp,
                                imported_document=imported,
                                symbol=node,
                            )
                        )
            elif isinstance(imp, WdlImportMembers):
                if qualified:
                    continue
                for member in imp.members():
                    local_name = member.alias if member.alias else member.member
                    if local_name != member_part:
                        continue
                    for node in selector(imported):
                        name = getattr(node, "name", None)
                        if name == member.member:
                            results.append(
                                ResolvedImport(
                                    local_name=local_name or "",
                                    imported_name=member.member or "",
                                    import_namespace=None,
                                    import_statement=imp,
                                    imported_document=imported,
                                    symbol=node,
                                )
                            )
        return results

    def _resolve_imported_types(self, context, visible_type_name, structs):
        if (
            context is None
            or visible_type_name is None
            or not visible_type_name.strip()
        ):
            return []

        results = []
        for imp in context.importStatements():
            imported = self.resolveImportedDocument(context, imp)
            if imported is None:
                continue

            selected_nodes = imported.structs() if structs else imported.enums()

            if isinstance(imp, WdlImportStandard):
                aliases = self._import_aliases(imp)
                for node in selected_nodes:
                    imported_name = getattr(node, "name", None)
                    if imported_name is None:
                        continue
                    local_name = aliases.get(imported_name, imported_name)
                    if local_name == visible_type_name:
                        results.append(
                            ResolvedImport(
                                local_name=local_name,
                                imported_name=imported_name,
                                import_namespace=None,
                                import_statement=imp,
                                imported_document=imported,
                                symbol=node,
                            )
                        )
            elif isinstance(imp, WdlImportStar):
                for node in selected_nodes:
                    imported_name = getattr(node, "name", None)
                    if imported_name == visible_type_name:
                        results.append(
                            ResolvedImport(
                                local_name=visible_type_name,
                                imported_name=visible_type_name,
                                import_namespace=None,
                                import_statement=imp,
                                imported_document=imported,
                                symbol=node,
                            )
                        )
            elif isinstance(imp, WdlImportMembers):
                for member in imp.members():
                    local_name = member.alias if member.alias else member.member
                    if local_name != visible_type_name:
                        continue
                    for node in selected_nodes:
                        imported_name = getattr(node, "name", None)
                        if imported_name == member.member:
                            results.append(
                                ResolvedImport(
                                    local_name=local_name or "",
                                    imported_name=member.member or "",
                                    import_namespace=None,
                                    import_statement=imp,
                                    imported_document=imported,
                                    symbol=node,
                                )
                            )

        return results

    def _import_aliases(self, imp: WdlImportStandard) -> dict[str, str]:
        aliases: dict[str, str] = {}
        for member in imp.members():
            if member.member is None or not member.member.strip():
                continue
            aliases[member.member] = member.alias if member.alias else member.member
        return aliases

    def keyValueToWdl(self, item: WdlStringKeyValue, delimiter: str = ": ") -> str:
        """Render a keyed expression entry back into WDL syntax."""
        if item.getValue() is None:
            return item.getKey() or ""
        return (
            f"{item.getKey() or ''}{delimiter}{self.expressionToWdl(item.getValue())}"
        )

    def declarationToWdl(self, declaration: WdlDeclaration) -> str:
        """Render a declaration node back into WDL syntax."""
        out = f"{self.typeToWdl(declaration.type)} {declaration.name or ''}".strip()
        if declaration.environmentVariable:
            out = f"env {out}"
        if (
            isinstance(declaration, WdlBoundDeclaration)
            and declaration.expression is not None
        ):
            out = f"{out} = {self.expressionToWdl(declaration.expression)}"
        return out

    def expressionToWdl(self, expr: WdlExpression | None) -> str:
        """Render an expression subtree back into WDL syntax."""
        if expr is None:
            return ""
        if isinstance(expr, WdlNullLiteral):
            return "None"
        if isinstance(expr, WdlBooleanLiteral):
            if expr.value is None:
                return "None"
            return "true" if expr.value else "false"
        if isinstance(expr, (WdlIntLiteral, WdlFloatLiteral, WdlLiteral)):
            return str(expr.getValue())
        if isinstance(expr, WdlBinaryOperation):
            return (
                f"{self.expressionToWdl(expr.left)} "
                f"{expr.operator.value if expr.operator else ''} "
                f"{self.expressionToWdl(expr.right)}"
            )
        if isinstance(expr, WdlUnaryOperation):
            return f"{expr.operator.value if expr.operator else ''}{self.expressionToWdl(expr.operand)}"
        if isinstance(expr, WdlTernaryOperation):
            return (
                f"if ({self.expressionToWdl(expr.condition)}) "
                f"{self.expressionToWdl(expr.trueValue)} "
                f"else {self.expressionToWdl(expr.falseValue)}"
            )
        if isinstance(expr, WdlArrayLiteral):
            return f"[{', '.join(self.expressionToWdl(x) for x in expr.entries())}]"
        if isinstance(expr, WdlMapLiteral):
            return (
                "{"
                + ", ".join(
                    f"{self.expressionToWdl(e.getKey())}: "
                    f"{self.expressionToWdl(e.getValue())}"
                    for e in expr.entries()
                )
                + "}"
            )
        if isinstance(expr, WdlObjectLiteral):
            return (
                "{"
                + ", ".join(
                    f"{e.getKey()}: {self.expressionToWdl(e.getValue())}"
                    for e in expr.entries()
                )
                + "}"
            )
        if isinstance(expr, WdlStructLiteral):
            return (
                f"{expr.name or ''} {{"
                + ", ".join(
                    f"{e.getKey()}: {self.expressionToWdl(e.getValue())}"
                    for e in expr.entries()
                )
                + "}"
            )
        if isinstance(expr, WdlPairLiteral):
            return f"({self.expressionToWdl(expr.left)}, {self.expressionToWdl(expr.right)})"
        if isinstance(expr, WdlFunctionCallOperation):
            return f"{expr.functionName or ''}({', '.join(self.expressionToWdl(a) for a in expr.arguments())})"
        if isinstance(expr, WdlMemberAccessOperation):
            return f"{self.expressionToWdl(expr.target)}.{expr.member or ''}"
        if isinstance(expr, WdlIndexAccessOperation):
            return f"{self.expressionToWdl(expr.target)}[{self.expressionToWdl(expr.index)}]"
        if isinstance(expr, WdlVariable):
            return expr.name or ""
        if isinstance(expr, WdlStringLiteral):
            return self.stringLiteralToWdl(expr, quote=True)
        return str(expr)

    def stringLiteralToWdl(
        self, strLit: WdlStringLiteral | None, quote: bool = True
    ) -> str:
        if strLit is None:
            return ""
        payload: list[str] = []
        for component in strLit.components():
            if isinstance(component, WdlStringText):
                payload.append(component.text or "")
            elif isinstance(component, WdlStringEscape):
                payload.append(component.escapeText or "")
            elif isinstance(component, WdlStringToken):
                payload.append(component.tokenText or "")
            elif isinstance(component, WdlStringPlaceholder):
                option_prefix = ""
                if component.option is not None:
                    if (
                        component.option.type
                        == WdlStringPlaceholderOptionType.TRUE_FALSE
                    ):
                        option_prefix = (
                            f"true={self.stringLiteralToWdl(component.option.trueValue, True)} "
                            f"false={self.stringLiteralToWdl(component.option.falseValue, True)} "
                        )
                    elif (
                        component.option.type == WdlStringPlaceholderOptionType.DEFAULT
                    ):
                        option_prefix = f"default={self.stringLiteralToWdl(component.option.value, True)} "
                payload.append(
                    f"{component.symbol.value}"
                    f"{{{option_prefix}{self.expressionToWdl(component.expression)}}}"
                )
        body = "".join(payload)
        if not quote:
            return body
        return (
            f'"{body}"'
            if strLit.delimiter == Delimiter.SINGLE_QUOTED
            else f"<<<{body}>>>"
        )

    def typeToWdl(self, typeNode: WdlType | None) -> str:
        if typeNode is None:
            return ""
        out: str
        if isinstance(typeNode, WdlPrimitiveType):
            out = (
                typeNode.primitiveType().toWdlString()
                if typeNode.primitiveType()
                else "Any"
            )
        elif isinstance(typeNode, WdlArrayType):
            out = f"Array[{self.typeToWdl(typeNode.memberType())}]"
            if typeNode.isNonEmpty():
                out = f"{out}+"
        elif isinstance(typeNode, WdlMapType):
            out = f"Map[{self.typeToWdl(typeNode.keyType())}, {self.typeToWdl(typeNode.valueType())}]"
        elif isinstance(typeNode, WdlPairType):
            out = (
                f"Pair[{self.typeToWdl(typeNode.leftType())},"
                f"{self.typeToWdl(typeNode.rightType())}]"
            )
        elif isinstance(typeNode, WdlTypeReferenceType):
            out = typeNode.referenceName() or ""
        else:
            out = str(typeNode)
        if typeNode.isOptional():
            out += "?"
        return out
