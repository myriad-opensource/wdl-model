"""Parser and model builder for WDL 1.x documents.

This is the main entry point for callers who want to turn WDL source into the Python object model.
Validation is optional: callers may load a document only, or load and validate in one step by
supplying a validator.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
from typing import Any, Iterable, Protocol, TypeVar

from antlr4 import CommonTokenStream, InputStream
from antlr4.error.ErrorListener import ErrorListener

from wdl_model.grammar.v1.WdlV1Lexer import WdlV1Lexer
from wdl_model.grammar.v1.WdlV1Parser import WdlV1Parser
from wdl_model.grammar.v1.WdlV1ParserVisitor import WdlV1ParserVisitor

from .base import WdlNode, WdlSourceRange
from .definitions import (
    WdlEnum,
    WdlEnumChoice,
    WdlStruct,
    WdlStructMember,
    WdlTask,
    WdlWorkflow,
)
from .errors import WdlException, WdlImportException, WdlSyntaxError
from .expressions import (
    Delimiter,
    PlaceHolderSymbol,
    WdlArrayLiteral,
    WdlBinaryOperation,
    WdlBinaryOperator,
    WdlBooleanLiteral,
    WdlExpression,
    WdlFloatLiteral,
    WdlFunctionCallOperation,
    WdlIndexAccessOperation,
    WdlIntLiteral,
    WdlMapEntry,
    WdlMapLiteral,
    WdlMemberAccessOperation,
    WdlNullLiteral,
    WdlNumberLiteral,
    WdlObjectEntry,
    WdlObjectLiteral,
    WdlPairLiteral,
    WdlStringComponent,
    WdlStringEscape,
    WdlStringLiteral,
    WdlStringPlaceholder,
    WdlStringPlaceholderOption,
    WdlStringPlaceholderOptionType,
    WdlStructEntry,
    WdlStructLiteral,
    WdlStringText,
    WdlTernaryOperation,
    WdlUnaryOperation,
    WdlUnaryOperator,
    WdlVariable,
)
from .sections import (
    WdlCommand,
    WdlInput,
    WdlMetadata,
    WdlMetadataEntry,
    WdlOutput,
    WdlParameterMetadata,
    WdlRequirementEntry,
    WdlRequirements,
    WdlRuntime,
    WdlRuntimeEntry,
    WdlTaskHint,
    WdlTaskHints,
    WdlWorkflowHint,
    WdlWorkflowHints,
)
from .statements import (
    WdlBoundDeclaration,
    WdlCall,
    WdlCallInput,
    WdlConditional,
    WdlConditionalElseIf,
    WdlDeclaration,
    WdlImportMember,
    WdlImportMembers,
    WdlImportStandard,
    WdlImportStar,
    WdlScatter,
    WdlStatement,
)
from .types import Type as WdlPrimitiveTypeEnum
from .types import (
    WdlArrayType,
    WdlMapType,
    WdlPairType,
    WdlPrimitiveType,
    WdlType,
    WdlTypeReferenceType,
)
from .wdl_document import WdlDocument
from .wdl_version import WdlVersion
from .resolvers import WdlImportResolverBase, WdlImportResolverHttpx


class _WdlErrorListener(ErrorListener):
    """Collect syntax diagnostics emitted by the lexer and parser."""

    def __init__(self) -> None:
        self.syntaxErrors: list[WdlSyntaxError] = []

    def syntaxError(
        self,
        recognizer: Any,
        offendingSymbol: Any,
        line: int,
        column: int,
        msg: str,
        e: Exception,
    ) -> None:
        self.syntaxErrors.append(
            WdlSyntaxError(message=msg, line=line, charPositionInLine=column, cause=e)
        )

    def throwIfErrored(self) -> None:
        if self.syntaxErrors:
            raise WdlException(self.syntaxErrors)


T = TypeVar("T", bound=WdlNode)


class WdlValidator(Protocol):
    """Protocol for validators that can be applied immediately after model construction."""

    def validateDocument(self, document: WdlDocument) -> None: ...


class WdlV1Loader(WdlV1ParserVisitor):
    """Visitor-based Python port of the Java WdlV1Loader model builder."""

    def __init__(self) -> None:
        self.stack: deque[WdlNode] = deque()

    @classmethod
    def load(
        cls,
        input_stream: InputStream,
        validator: WdlValidator | None = None,
        import_resolver: WdlImportResolverBase | None = None,
        current_document_location: str | None = None,
    ) -> WdlDocument:
        """Parse an ANTLR input stream into a ``WdlDocument`` and optionally validate it."""
        document = cls._parse_document(input_stream, current_document_location)
        if import_resolver is not None:
            cls._resolve_imports_recursive(document, import_resolver, {}, [], set())
        if validator is not None:
            validator.validateDocument(document)
        return document

    @classmethod
    def _parse_document(
        cls, input_stream: InputStream, current_document_location: str | None
    ) -> WdlDocument:
        """Parse one document and attach source location metadata."""
        lexer = WdlV1Lexer(input_stream)
        parser = WdlV1Parser(CommonTokenStream(lexer))

        error_listener = _WdlErrorListener()
        lexer.removeErrorListeners()
        parser.removeErrorListeners()
        lexer.addErrorListener(error_listener)
        parser.addErrorListener(error_listener)

        document_ctx = parser.document()
        error_listener.throwIfErrored()

        loader = cls()
        loader.visitDocument(document_ctx)
        document = loader.getDocument()
        document.setSourceLocation(current_document_location)
        return document

    @classmethod
    def load_from_string(
        cls,
        source_code: str,
        validator: WdlValidator | None = None,
        source_location: str | None = None,
        import_resolver: WdlImportResolverBase | None = None,
    ) -> WdlDocument:
        """Parse a source string into a ``WdlDocument`` and optionally validate it."""
        resolver = import_resolver
        if source_location is not None and resolver is None:
            resolver = WdlImportResolverHttpx()
        return cls.load(InputStream(source_code), validator, resolver, source_location)

    @classmethod
    def load_from_file(
        cls,
        file_path: str | Path,
        validator: WdlValidator | None = None,
        import_resolver: WdlImportResolverBase | None = None,
    ) -> WdlDocument:
        """Parse a UTF-8 source file into a ``WdlDocument`` and optionally validate it."""
        path = Path(file_path)
        resolver = (
            import_resolver if import_resolver is not None else WdlImportResolverHttpx()
        )
        return cls.load(
            InputStream(path.read_text(encoding="utf-8")),
            validator,
            resolver,
            path.resolve().as_uri(),
        )

    @classmethod
    def _resolve_imports_recursive(
        cls,
        document: WdlDocument,
        import_resolver: WdlImportResolverBase,
        loaded_by_id: dict[str, WdlDocument],
        active_import_stack: list[str],
        active_import_set: set[str],
    ) -> None:
        """Resolve imports depth-first with cycle detection keyed by resolved identity.

        ``loaded_by_id`` caches fully loaded documents, ``active_import_stack`` keeps
        the ordered import chain for diagnostics, and ``active_import_set`` provides
        fast membership checks for cycle detection.
        """
        current_source_location = document.getSourceLocation()
        if current_source_location is not None:
            # Keep both the ordered path and an O(1) membership set.
            active_import_stack.append(current_source_location)
            active_import_set.add(current_source_location)
            loaded_by_id.setdefault(current_source_location, document)

        try:
            document.importedDocuments().clear()
            current_location = document.getSourceLocation()
            for imp in document.importStatements():
                source_literal = imp.source
                if source_literal is None:
                    continue

                import_reference = cls._extract_string_literal_text(source_literal)
                resolved_import_location = import_resolver.resolve_import_location(
                    current_location, import_reference
                )
                import_identifier = resolved_import_location or import_reference
                imp.importIdentifier = import_identifier

                if import_identifier in active_import_set:
                    raise WdlImportException(
                        # Report the full cycle path so the loop is obvious.
                        f"Circular import detected: {' -> '.join([*active_import_stack, import_identifier])}",
                        import_identifier,
                    )

                import_source_text = import_resolver.resolve_import(
                    current_location, import_reference
                )
                imp.sourceText = import_source_text

                imported_document = loaded_by_id.get(import_identifier)
                if imported_document is None:
                    imported_document = cls._parse_document(
                        InputStream(import_source_text), resolved_import_location
                    )
                    loaded_by_id[import_identifier] = imported_document
                    cls._resolve_imports_recursive(
                        imported_document,
                        import_resolver,
                        loaded_by_id,
                        active_import_stack,
                        active_import_set,
                    )

                document.importedDocuments()[import_identifier] = imported_document
        finally:
            if current_source_location is not None:
                active_import_stack.pop()
                active_import_set.discard(current_source_location)

    @staticmethod
    def _extract_string_literal_text(source_literal: WdlStringLiteral) -> str:
        text_parts: list[str] = []
        for component in source_literal.components():
            if isinstance(component, WdlStringText):
                text_parts.append(component.text or "")
            elif isinstance(component, WdlStringEscape):
                text_parts.append(component.escapeText or "")
            else:
                raise AssertionError("Unsupported import URI element")
        return "".join(text_parts)

    def getDocument(self) -> WdlDocument:
        """Return the finished root document after a successful visitor traversal."""
        if not self.stack:
            raise AssertionError("Stack is empty")
        if len(self.stack) != 1 or not isinstance(self.stack[-1], WdlDocument):
            raise AssertionError("Stack does not contain exactly one WdlDocument")
        return self._pop_with_type(WdlDocument)

    def _pop_with_type(self, expected_type: type[T]) -> T:
        node = self.stack.pop()
        if not isinstance(node, expected_type):
            raise AssertionError(
                f"Expected {expected_type.__name__} on stack not {type(node).__name__}"
            )
        return node

    def _peek_with_type(self, expected_type: type[T]) -> T:
        node = self.stack[-1]
        if not isinstance(node, expected_type):
            raise AssertionError(
                f"Expected {expected_type.__name__} on stack not {type(node).__name__}"
            )
        return node

    def _find_with_type(self, expected_type: type[T]) -> T:
        for node in reversed(self.stack):
            if isinstance(node, expected_type):
                return node
        raise RuntimeError(f"Could not find {expected_type.__name__} in stack")

    @staticmethod
    def _maybe_token(ctx: Any, token_method_name: str) -> Any | None:
        method = getattr(ctx, token_method_name, None)
        if not callable(method):
            return None
        return method()

    @staticmethod
    def _strict_identifier_texts(ctx: Any) -> list[str]:
        method = getattr(ctx, "strictIdentifier", None)
        if not callable(method):
            return []
        result = method()
        if result is None:
            return []
        if isinstance(result, Iterable) and not isinstance(result, (str, bytes)):
            return [item.getText() for item in result]
        return [result.getText()]

    def _pop_expression_chain(self, expression_count: int) -> list[WdlExpression]:
        expressions = [
            self._pop_with_type(WdlExpression) for _ in range(expression_count)
        ]
        expressions.reverse()
        return expressions

    @staticmethod
    def _collect_binary_operator_symbols(ctx: Any) -> list[str]:
        children = list(ctx.getChildren())
        return [children[i].getText() for i in range(1, len(children), 2)]

    @staticmethod
    def _fold_binary_operations(
        expressions: list[WdlExpression], operators: list[WdlBinaryOperator]
    ) -> WdlExpression:
        folded: WdlExpression = expressions[0]
        for idx, operator in enumerate(operators):
            folded = WdlBinaryOperation(folded, operator, expressions[idx + 1])
        return folded

    @staticmethod
    def _range_of(ctx: Any) -> WdlSourceRange | None:
        """Build a WdlSourceRange from an ANTLR parser rule context's start/stop tokens."""
        start = getattr(ctx, "start", None)
        if start is None:
            return None
        stop = getattr(ctx, "stop", None) or start
        end_col = stop.column + len(stop.text or "")
        return WdlSourceRange(start.line, start.column, stop.line, end_col)

    # ------------------------------------------------------------------
    # Document & version
    # ------------------------------------------------------------------

    def visitDocument(self, ctx: Any) -> Any:
        self.stack.append(WdlDocument())
        return self.visitChildren(ctx)

    def visitVersionStatement(self, ctx: Any) -> Any:
        doc = self._peek_with_type(WdlDocument)
        version_token = self._maybe_token(ctx, "FLOAT")
        if version_token is not None:
            doc.setWdlVersion(WdlVersion.fromString(version_token.getText()))
        return None

    # ------------------------------------------------------------------
    # Imports
    # ------------------------------------------------------------------

    def visitImportStatementStandard(self, ctx: Any) -> Any:
        imp = WdlImportStandard()
        self.stack.append(imp)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlImportMember):
            imp.members().appendleft(self._pop_with_type(WdlImportMember))
        if self._maybe_token(ctx, "KEYWORD_AS") is not None:
            ids = self._strict_identifier_texts(ctx)
            if ids:
                imp.alias = ids[0]
        imp.source = self._pop_with_type(WdlStringLiteral)
        imp.source_range = self._range_of(ctx)
        self.stack.pop()
        self._peek_with_type(WdlDocument).elements().append(imp)
        return None

    def visitImportStatementStar(self, ctx: Any) -> Any:
        imp = WdlImportStar()
        self.stack.append(imp)
        self.visitChildren(ctx)
        imp.source = self._pop_with_type(WdlStringLiteral)
        imp.source_range = self._range_of(ctx)
        self.stack.pop()
        self._peek_with_type(WdlDocument).elements().append(imp)
        return None

    def visitImportStatementMembers(self, ctx: Any) -> Any:
        imp = WdlImportMembers()
        self.stack.append(imp)
        self.visitChildren(ctx)
        imp.source = self._pop_with_type(WdlStringLiteral)
        imp.source_range = self._range_of(ctx)
        self.stack.pop()
        self._peek_with_type(WdlDocument).elements().append(imp)
        return None

    def visitImportMembers(self, ctx: Any) -> Any:
        imp = self._peek_with_type(WdlImportMembers)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not imp:
            imp.members().appendleft(self._pop_with_type(WdlImportMember))
        return None

    def visitImportMember(self, ctx: Any) -> Any:
        member = WdlImportMember()
        self.stack.append(member)
        self.visitChildren(ctx)
        ids = self._strict_identifier_texts(ctx)
        if self._maybe_token(ctx, "KEYWORD_AS") is not None and len(ids) > 1:
            member.alias = ids[1]
        if ids:
            member.member = ids[0]
        return None

    def visitImportUriLiteral(self, ctx: Any) -> Any:
        # Current Python model has SINGLE_QUOTED and DOUBLE_ANGLE delimiters.
        literal = WdlStringLiteral(Delimiter.SINGLE_QUOTED)
        self.stack.append(literal)
        self.visitChildren(ctx)
        return None

    def visitImportUriElement(self, ctx: Any) -> Any:
        literal = self._peek_with_type(WdlStringLiteral)
        if self._maybe_token(ctx, "STRING_TEXT") is not None:
            literal.components().appendleft(
                WdlStringText(self._maybe_token(ctx, "STRING_TEXT").getText())
            )
        elif self._maybe_token(ctx, "STRING_ESCAPE") is not None:
            literal.components().appendleft(
                WdlStringEscape(self._maybe_token(ctx, "STRING_ESCAPE").getText())
            )
        else:
            raise AssertionError("Unsupported import URI element")
        self.visitChildren(ctx)
        return None

    def visitImportAlias(self, ctx: Any) -> Any:
        member = WdlImportMember()
        self.stack.append(member)
        self.visitChildren(ctx)
        ids = self._strict_identifier_texts(ctx)
        if self._maybe_token(ctx, "KEYWORD_AS") is not None and len(ids) > 1:
            member.alias = ids[1]
        if ids:
            member.member = ids[0]
        return None

    # ------------------------------------------------------------------
    # Struct / enum definitions
    # ------------------------------------------------------------------

    def visitStructDefinition(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        struct = WdlStruct(ids[0] if ids else None)
        self.stack.append(struct)
        self.visitChildren(ctx)
        if ids:
            struct.name = ids[0]
        struct.source_range = self._range_of(ctx)
        self.stack.pop()
        self._peek_with_type(WdlDocument).elements().append(struct)
        return None

    def visitStructItemMemberDeclaration(self, ctx: Any) -> Any:
        struct = self._find_with_type(WdlStruct)
        member = WdlStructMember()
        self.stack.append(member)
        self.visitChildren(ctx)
        decl_ctx = getattr(ctx, "structDeclaration", lambda: None)()
        if decl_ctx is not None:
            ids = self._strict_identifier_texts(decl_ctx)
            if ids:
                member.name = ids[0]
        member.type = self._pop_with_type(WdlType)
        self.stack.pop()
        struct.elements().append(member)
        return None

    def visitStructItemMetadata(self, ctx: Any) -> Any:
        struct = self._find_with_type(WdlStruct)
        self.visitChildren(ctx)
        struct.elements().append(self._pop_with_type(WdlMetadata))
        return None

    def visitStructItemParameterMetadata(self, ctx: Any) -> Any:
        struct = self._find_with_type(WdlStruct)
        self.visitChildren(ctx)
        struct.elements().append(self._pop_with_type(WdlParameterMetadata))
        return None

    def visitEnumDefinition(self, ctx: Any) -> Any:
        enum_def = WdlEnum()
        self.stack.append(enum_def)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlEnumChoice):
            enum_def.elements().appendleft(self._pop_with_type(WdlEnumChoice))
        if getattr(ctx, "enumTypeParameter", lambda: None)() is not None:
            enum_def.valueType = self._pop_with_type(WdlType)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            enum_def.name = ids[0]
        enum_def.source_range = self._range_of(ctx)
        self._pop_with_type(WdlEnum)
        self._peek_with_type(WdlDocument).elements().append(enum_def)
        return None

    def visitEnumChoice(self, ctx: Any) -> Any:
        choice = WdlEnumChoice()
        self.stack.append(choice)
        self.visitChildren(ctx)
        if self._maybe_token(ctx, "ASSIGNMENT") is not None:
            choice.setValue(self._pop_with_type(WdlExpression))
        ids = self._strict_identifier_texts(ctx)
        if ids:
            choice.setKey(ids[0])
        return None

    # ------------------------------------------------------------------
    # Declarations / sections
    # ------------------------------------------------------------------

    def visitUnboundDeclaration(self, ctx: Any) -> Any:
        decl = WdlDeclaration()
        self.stack.append(decl)
        self.visitChildren(ctx)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            decl.name = ids[0]
        decl.type = self._pop_with_type(WdlType)
        decl.environmentVariable = self._maybe_token(ctx, "KEYWORD_ENV") is not None
        decl.source_range = self._range_of(ctx)
        return None

    def visitBoundDeclaration(self, ctx: Any) -> Any:
        decl = WdlBoundDeclaration()
        self.stack.append(decl)
        self.visitChildren(ctx)
        decl.expression = self._pop_with_type(WdlExpression)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            decl.name = ids[0]
        decl.type = self._pop_with_type(WdlType)
        decl.environmentVariable = self._maybe_token(ctx, "KEYWORD_ENV") is not None
        decl.source_range = self._range_of(ctx)
        return None

    def visitInputSection(self, ctx: Any) -> Any:
        section = WdlInput()
        self.stack.append(section)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not section:
            section.elements().appendleft(self._pop_with_type(WdlDeclaration))
        return None

    def visitOutputSection(self, ctx: Any) -> Any:
        section = WdlOutput()
        self.stack.append(section)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not section:
            section.elements().appendleft(self._pop_with_type(WdlBoundDeclaration))
        return None

    def visitTaskDefinition(self, ctx: Any) -> Any:
        task = WdlTask()
        self.stack.append(task)
        self.visitChildren(ctx)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            task.name = ids[0]
        task.source_range = self._range_of(ctx)
        self._pop_with_type(WdlTask)
        self._peek_with_type(WdlDocument).elements().append(task)
        return None

    def visitTaskDeclaration(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlBoundDeclaration):
            task.elements().append(self._pop_with_type(WdlBoundDeclaration))
        return None

    def visitTaskInputSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlInput))
        return None

    def visitTaskOutputSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlOutput))
        return None

    def visitTaskCommandSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        command = WdlCommand()
        self.stack.append(command)
        self.visitChildren(ctx)
        self.stack.pop()
        task.elements().append(command)
        return None

    def visitCommandSection(self, ctx: Any) -> Any:
        command = self._peek_with_type(WdlCommand)
        self.visitChildren(ctx)
        command.setCommandText(self._pop_with_type(WdlStringLiteral))
        return None

    def visitMultilineStringCommand(self, ctx: Any) -> Any:
        self._find_with_type(WdlCommand).setMultiline(True)
        cmd_str = WdlStringLiteral(Delimiter.DOUBLE_ANGLE)
        self.stack.append(cmd_str)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not cmd_str:
            cmd_str.components().appendleft(self._pop_with_type(WdlStringComponent))
        return None

    def visitBracedCommand(self, ctx: Any) -> Any:
        self._find_with_type(WdlCommand).setMultiline(False)
        cmd_str = WdlStringLiteral(Delimiter.SINGLE_QUOTED)
        self.stack.append(cmd_str)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not cmd_str:
            cmd_str.components().appendleft(self._pop_with_type(WdlStringComponent))
        return None

    def visitMetadataSection(self, ctx: Any) -> Any:
        section = WdlMetadata()
        self.stack.append(section)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not section:
            section.elements().append(self._pop_with_type(WdlMetadataEntry))
        return None

    def visitParameterMetadataSection(self, ctx: Any) -> Any:
        section = WdlParameterMetadata()
        self.stack.append(section)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not section:
            section.elements().appendleft(self._pop_with_type(WdlMetadataEntry))
        return None

    def visitMetadataObject(self, ctx: Any) -> Any:
        val = WdlObjectLiteral()
        self.stack.append(val)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not val:
            val.entries().appendleft(self._pop_with_type(WdlObjectEntry))
        return None

    def visitMetadataObjectItem(self, ctx: Any) -> Any:
        if any(isinstance(node, WdlObjectLiteral) for node in self.stack):
            entry: WdlObjectEntry | WdlMetadataEntry = WdlObjectEntry()
        else:
            entry = WdlMetadataEntry()
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        dotted = getattr(ctx, "dottedIdentifier", lambda: None)()
        if dotted is not None:
            entry.setKey(dotted.getText())
        return None

    def visitMetadataArray(self, ctx: Any) -> Any:
        val = WdlArrayLiteral()
        self.stack.append(val)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not val:
            val.entries().appendleft(self._pop_with_type(WdlExpression))
        return None

    def visitTaskRuntimeSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlRuntime):
            task.elements().append(self._pop_with_type(WdlRuntime))
        return None

    def visitRuntimeSection(self, ctx: Any) -> Any:
        runtime = WdlRuntime()
        self.stack.append(runtime)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not runtime:
            runtime.elements().append(self._pop_with_type(WdlRuntimeEntry))
        return None

    def visitRuntimeItem(self, ctx: Any) -> Any:
        entry = WdlRuntimeEntry()
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        ids = self._strict_identifier_texts(ctx)
        if ids:
            entry.setKey(ids[0])
        return None

    def visitTaskRequirementsSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlRequirements))
        return None

    def visitRequirementsSection(self, ctx: Any) -> Any:
        reqs = WdlRequirements()
        self.stack.append(reqs)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not reqs:
            reqs.elements().append(self._pop_with_type(WdlRequirementEntry))
        return None

    def visitRequirementsItem(self, ctx: Any) -> Any:
        entry = WdlRequirementEntry()
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        ids = self._strict_identifier_texts(ctx)
        if ids:
            entry.setKey(ids[0])
        return None

    def visitTaskHintsSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlTaskHints))
        return None

    def visitHintsSectionTask(self, ctx: Any) -> Any:
        hints = WdlTaskHints()
        self.stack.append(hints)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not hints:
            hints.elements().append(self._pop_with_type(WdlTaskHint))
        return None

    def visitHintsItemTask(self, ctx: Any) -> Any:
        hint = WdlTaskHint()
        self.stack.append(hint)
        self.visitChildren(ctx)
        hint.setValue(self._pop_with_type(WdlExpression))
        ids = self._strict_identifier_texts(ctx)
        if ids:
            hint.setKey(ids[0])
        return None

    def visitHintsTypedObjectTask(self, ctx: Any) -> Any:
        obj = WdlObjectLiteral()
        self.stack.append(obj)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not obj:
            obj.entries().append(self._pop_with_type(WdlObjectEntry))
        return None

    def visitHintsObjectItemTask(self, ctx: Any) -> Any:
        entry = WdlObjectEntry()
        self.stack.append(entry)
        self.visit(getattr(ctx, "hintsValueTask")())
        entry.setValue(self._pop_with_type(WdlExpression))
        dotted = getattr(ctx, "dottedIdentifier", lambda: None)()
        if dotted is not None:
            entry.setKey(dotted.getText())
        return None

    def visitInputHintsObjectTask(self, ctx: Any) -> Any:
        obj = WdlObjectLiteral()
        self.stack.append(obj)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not obj:
            obj.entries().append(self._pop_with_type(WdlObjectEntry))
        return None

    def visitInputHintsItemTask(self, ctx: Any) -> Any:
        entry = WdlObjectEntry()
        self.stack.append(entry)
        self.visit(getattr(ctx, "hintsTypedObjectTask")())
        entry.setValue(self._pop_with_type(WdlExpression))
        dotted = getattr(ctx, "dottedIdentifier", lambda: None)()
        if dotted is not None:
            entry.setKey(dotted.getText())
        return None

    def visitOutputHintsObjectTask(self, ctx: Any) -> Any:
        obj = WdlObjectLiteral()
        self.stack.append(obj)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not obj:
            obj.entries().appendleft(self._pop_with_type(WdlObjectEntry))
        return None

    def visitOutputHintsItemTask(self, ctx: Any) -> Any:
        entry = WdlObjectEntry()
        self.stack.append(entry)
        self.visit(getattr(ctx, "hintsTypedObjectTask")())
        entry.setValue(self._pop_with_type(WdlExpression))
        dotted = getattr(ctx, "dottedIdentifier", lambda: None)()
        if dotted is not None:
            entry.setKey(dotted.getText())
        return None

    def visitTaskHintsArray(self, ctx: Any) -> Any:
        arr = WdlArrayLiteral()
        self.stack.append(arr)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not arr:
            arr.entries().appendleft(self._pop_with_type(WdlExpression))
        return None

    def visitTaskMetadataSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlMetadata))
        return None

    def visitTaskParameterMetadataSection(self, ctx: Any) -> Any:
        task = self._find_with_type(WdlTask)
        self.visitChildren(ctx)
        task.elements().append(self._pop_with_type(WdlParameterMetadata))
        return None

    # ------------------------------------------------------------------
    # Workflow definitions
    # ------------------------------------------------------------------

    def visitWorkflowDefinition(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        workflow = WdlWorkflow(ids[0] if ids else None)
        self.stack.append(workflow)
        self.visitChildren(ctx)
        workflow.source_range = self._range_of(ctx)
        self._pop_with_type(WdlWorkflow)
        self._peek_with_type(WdlDocument).elements().append(workflow)
        return None

    def visitWorkflowDeclaration(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlBoundDeclaration):
            workflow.elements().append(self._pop_with_type(WdlBoundDeclaration))
        return None

    def visitWorkflowInputSection(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlInput))
        return None

    def visitWorkflowOutputSection(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlOutput))
        return None

    def visitWorkflowCallStatement(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        self._find_with_type(WdlWorkflow).elements().append(
            self._pop_with_type(WdlCall)
        )
        return None

    def visitCallStatement(self, ctx: Any) -> Any:
        call_stmt = WdlCall()
        call_stmt.source_range = self._range_of(ctx)
        self.stack.append(call_stmt)
        return self.visitChildren(ctx)

    def visitCallTarget(self, ctx: Any) -> Any:
        call_stmt = self._peek_with_type(WdlCall)
        for identifier_text in self._strict_identifier_texts(ctx):
            call_stmt.targetPath().append(identifier_text)
        return self.visitChildren(ctx)

    def visitCallAlias(self, ctx: Any) -> Any:
        call_stmt = self._peek_with_type(WdlCall)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            call_stmt.alias = ids[0]
        return self.visitChildren(ctx)

    def visitCallAfterClause(self, ctx: Any) -> Any:
        call_stmt = self._peek_with_type(WdlCall)
        ids = self._strict_identifier_texts(ctx)
        if ids:
            call_stmt.afterDependencies().append(ids[0])
        return self.visitChildren(ctx)

    def visitCallInputBlock(self, ctx: Any) -> Any:
        call = self._peek_with_type(WdlCall)
        call.legacyInputColonUsed = self._maybe_token(ctx, "KEYWORD_INPUT") is not None
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlCallInput):
            call.inputs().appendleft(self._pop_with_type(WdlCallInput))
        return None

    def visitCallInputItem(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        call_input = WdlCallInput(ids[0] if ids else None)
        self.visitChildren(ctx)
        if getattr(ctx, "expression", lambda: None)() is not None:
            call_input.setValue(self._pop_with_type(WdlExpression))
        self.stack.append(call_input)
        return None

    def visitWorkflowConditionalStatement(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlConditional))
        return None

    def visitConditionalStatement(self, ctx: Any) -> Any:
        cond_stmt = WdlConditional()
        self.stack.append(cond_stmt)
        self.visitChildren(ctx)
        for _ in getattr(ctx, "workflowStatement", lambda: [])():
            cond_stmt.thenStatements().appendleft(self._pop_with_type(WdlStatement))
        cond_stmt.condition = self._pop_with_type(WdlExpression)
        cond_stmt.source_range = self._range_of(ctx)
        return None

    def visitConditionalElseIfClause(self, ctx: Any) -> Any:
        cond_stmt = self._find_with_type(WdlConditional)
        self.visitChildren(ctx)
        else_if = WdlConditionalElseIf()
        for _ in getattr(ctx, "workflowStatement", lambda: [])():
            else_if.thenStatements().appendleft(self._pop_with_type(WdlStatement))
        else_if.condition = self._pop_with_type(WdlExpression)
        cond_stmt.elseIfs().append(else_if)
        return None

    def visitConditionalElseClause(self, ctx: Any) -> Any:
        cond_stmt = self._find_with_type(WdlConditional)
        self.visitChildren(ctx)
        for _ in getattr(ctx, "workflowStatement", lambda: [])():
            cond_stmt.elseStatements().appendleft(self._pop_with_type(WdlStatement))
        return None

    def visitWorkflowScatterStatement(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlScatter))
        return None

    def visitScatterStatement(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        scatter = WdlScatter(ids[0] if ids else None)
        self.stack.append(scatter)
        self.visitChildren(ctx)
        scatter.collection = self._pop_with_type(WdlExpression)
        scatter.source_range = self._range_of(ctx)
        return None

    def visitScatterBody(self, ctx: Any) -> Any:
        scatter = self._find_with_type(WdlScatter)
        self.visitChildren(ctx)
        while self.stack and isinstance(self.stack[-1], WdlStatement):
            scatter.statements().appendleft(self._pop_with_type(WdlStatement))
        return None

    def visitWorkflowHintsSection(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().appendleft(self._pop_with_type(WdlWorkflowHints))
        return None

    def visitHintsSectionWorkflow(self, ctx: Any) -> Any:
        hints = WdlWorkflowHints()
        self.stack.append(hints)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not hints:
            hints.elements().appendleft(self._pop_with_type(WdlWorkflowHint))
        return None

    def visitHintsItemWorkflow(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        hint = WdlWorkflowHint(ids[0] if ids else None)
        self.stack.append(hint)
        self.visitChildren(ctx)
        hint.setValue(self._pop_with_type(WdlExpression))
        return None

    def visitWorkflowMetadataSection(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlMetadata))
        return None

    def visitWorkflowParameterMetadataSection(self, ctx: Any) -> Any:
        workflow = self._find_with_type(WdlWorkflow)
        self.visitChildren(ctx)
        workflow.elements().append(self._pop_with_type(WdlParameterMetadata))
        return None

    # ------------------------------------------------------------------
    # Types
    # ------------------------------------------------------------------

    def visitMapType(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        value_type = self._pop_with_type(WdlType)
        key_type = self._pop_with_type(WdlType)
        map_type = WdlMapType(
            key_type, value_type, self._maybe_token(ctx, "QUESTION_MARK") is not None
        )
        self.stack.append(map_type)
        return None

    def visitArrayType(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        member = self._pop_with_type(WdlType)
        arr = WdlArrayType(
            member,
            self._maybe_token(ctx, "PLUS") is not None,
            self._maybe_token(ctx, "QUESTION_MARK") is not None,
        )
        self.stack.append(arr)
        return None

    def visitPairType(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        right = self._pop_with_type(WdlType)
        left = self._pop_with_type(WdlType)
        pair = WdlPairType(
            left, right, self._maybe_token(ctx, "QUESTION_MARK") is not None
        )
        self.stack.append(pair)
        return None

    def visitObjectType(self, ctx: Any) -> Any:
        self.stack.append(
            WdlTypeReferenceType(
                "Object", self._maybe_token(ctx, "QUESTION_MARK") is not None
            )
        )
        return None

    def visitPrimitiveType(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        if self._maybe_token(ctx, "KEYWORD_BOOLEAN_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.BOOLEAN
        elif self._maybe_token(ctx, "KEYWORD_INT_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.INT
        elif self._maybe_token(ctx, "KEYWORD_FLOAT_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.FLOAT
        elif self._maybe_token(ctx, "KEYWORD_STRING_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.STRING
        elif self._maybe_token(ctx, "KEYWORD_FILE_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.FILE
        elif self._maybe_token(ctx, "KEYWORD_DIRECTORY_TYPE") is not None:
            t = WdlPrimitiveTypeEnum.DIRECTORY
        else:
            raise AssertionError(f"Unknown primitive type {ctx.getText()}")
        self.stack.append(
            WdlPrimitiveType(t, self._maybe_token(ctx, "QUESTION_MARK") is not None)
        )
        return None

    def visitTypeRefType(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        ids = self._strict_identifier_texts(ctx)
        name = ids[0] if ids else None
        self.stack.append(
            WdlTypeReferenceType(
                name, self._maybe_token(ctx, "QUESTION_MARK") is not None
            )
        )
        return None

    # ------------------------------------------------------------------
    # Expressions
    # ------------------------------------------------------------------

    def visitNullLiteral(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        self.stack.append(WdlNullLiteral())
        return None

    def visitNoneLiteral(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        self.stack.append(WdlNullLiteral())
        return None

    def visitBooleanLiteral(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        self.stack.append(
            WdlBooleanLiteral(self._maybe_token(ctx, "KEYWORD_TRUE") is not None)
        )
        return None

    def visitNumberLiteralInt(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        token = self._maybe_token(ctx, "INTEGER")
        self.stack.append(
            WdlIntLiteral(int(token.getText()) if token is not None else None)
        )
        return None

    def visitNumberLiteralFloat(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        token = self._maybe_token(ctx, "FLOAT")
        self.stack.append(
            WdlFloatLiteral(float(token.getText()) if token is not None else None)
        )
        return None

    def visitNumberLiteralSigned(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        if self._maybe_token(ctx, "MINUS") is not None:
            num = self._peek_with_type(WdlNumberLiteral)
            if hasattr(num, "negate"):
                num.negate()
        return None

    def visitQuotedString(self, ctx: Any) -> Any:
        expr = WdlStringLiteral(Delimiter.SINGLE_QUOTED)
        self.stack.append(expr)
        self.visitChildren(ctx)
        components: deque[WdlStringComponent] = deque()
        while self.stack and self.stack[-1] is not expr:
            components.append(self._pop_with_type(WdlStringComponent))
        for component in components:
            expr.components().appendleft(component)
        return None

    def visitMultilineString(self, ctx: Any) -> Any:
        expr = WdlStringLiteral(Delimiter.DOUBLE_ANGLE)
        self.stack.append(expr)
        self.visitChildren(ctx)
        components: deque[WdlStringComponent] = deque()
        while self.stack and self.stack[-1] is not expr:
            components.append(self._pop_with_type(WdlStringComponent))
        for component in components:
            expr.components().appendleft(component)
        return None

    def visitStringElementText(self, ctx: Any) -> Any:
        token = self._maybe_token(ctx, "STRING_TEXT")
        self.stack.append(WdlStringText(token.getText() if token is not None else None))
        return None

    def visitStringElementEscape(self, ctx: Any) -> Any:
        token = self._maybe_token(ctx, "STRING_ESCAPE")
        self.stack.append(
            WdlStringEscape(token.getText() if token is not None else None)
        )
        return None

    def visitStringPlaceholder(self, ctx: Any) -> Any:
        start = self._maybe_token(ctx, "STRING_PLACEHOLDER_START")
        start_text = start.getText() if start is not None else "~{"
        if start_text == "~{":
            symbol = PlaceHolderSymbol.TILDE
        elif start_text == "${":
            symbol = PlaceHolderSymbol.DOLLAR
        else:
            raise AssertionError(f"Unknown PlaceHolderSymbol {start_text}")
        placeholder = WdlStringPlaceholder(symbol=symbol)
        self.stack.append(placeholder)
        self.visitChildren(ctx)
        placeholder.expression = self._pop_with_type(WdlExpression)
        while self.stack and self.stack[-1] is not placeholder:
            if placeholder.option is not None:
                raise AssertionError("Placeholder option already set")
            placeholder.option = self._pop_with_type(WdlStringPlaceholderOption)
        return None

    def visitStringPlaceholderOptionSepDefault(self, ctx: Any) -> Any:
        placeholder = self._peek_with_type(WdlStringPlaceholder)
        if placeholder.option is not None:
            raise AssertionError("Placeholder option already set")
        self.visitChildren(ctx)
        identifier = self._maybe_token(ctx, "IDENTIFIER")
        identifier_text = identifier.getText() if identifier is not None else None
        if identifier_text not in {"sep", "default"}:
            raise ValueError("Unsupported placeholder option")
        # Python model currently has no dedicated SEP type, map both sep/default to DEFAULT.
        placeholder.option = WdlStringPlaceholderOption(
            WdlStringPlaceholderOptionType.DEFAULT,
            value=self._pop_with_type(WdlStringLiteral),
        )
        return None

    def visitStringPlaceholderOptionTrueFalse(self, ctx: Any) -> Any:
        placeholder = self._peek_with_type(WdlStringPlaceholder)
        if placeholder.option is not None:
            raise AssertionError("Placeholder option already set")
        self.visitChildren(ctx)
        false_str = self._pop_with_type(WdlStringLiteral)
        true_str = self._pop_with_type(WdlStringLiteral)
        placeholder.option = WdlStringPlaceholderOption(
            WdlStringPlaceholderOptionType.TRUE_FALSE,
            trueValue=true_str,
            falseValue=false_str,
        )
        return None

    def visitStringPlaceholderOptionFalseTrue(self, ctx: Any) -> Any:
        placeholder = self._peek_with_type(WdlStringPlaceholder)
        if placeholder.option is not None:
            raise AssertionError("Placeholder option already set")
        self.visitChildren(ctx)
        true_str = self._pop_with_type(WdlStringLiteral)
        false_str = self._pop_with_type(WdlStringLiteral)
        placeholder.option = WdlStringPlaceholderOption(
            WdlStringPlaceholderOptionType.TRUE_FALSE,
            trueValue=true_str,
            falseValue=false_str,
        )
        return None

    def visitMultilineStringElementText(self, ctx: Any) -> Any:
        token = self._maybe_token(ctx, "MULTILINE_STRING_TEXT")
        self.stack.append(WdlStringText(token.getText() if token is not None else None))
        return None

    def visitMultilineStringElementEscape(self, ctx: Any) -> Any:
        token = self._maybe_token(ctx, "MULTILINE_STRING_ESCAPE")
        self.stack.append(
            WdlStringEscape(token.getText() if token is not None else None)
        )
        return None

    def visitMultilineStringPlaceholder(self, ctx: Any) -> Any:
        if (
            self._maybe_token(ctx, "MULTILINE_STRING_TILDE_PLACEHOLDER_START")
            is not None
        ):
            symbol = PlaceHolderSymbol.TILDE
        elif (
            self._maybe_token(ctx, "MULTILINE_STRING_DOLLAR_PLACEHOLDER_START")
            is not None
        ):
            symbol = PlaceHolderSymbol.DOLLAR
        else:
            raise AssertionError("Unknown multiline placeholder symbol")
        placeholder = WdlStringPlaceholder(symbol=symbol)
        self.stack.append(placeholder)
        self.visitChildren(ctx)
        placeholder.expression = self._pop_with_type(WdlExpression)
        return None

    # Enum literal expression support
    def visitEnumQuotedString(self, ctx: Any) -> Any:
        return self.visitQuotedString(ctx)

    def visitEnumStringElement(self, ctx: Any) -> Any:
        if self._maybe_token(ctx, "STRING_TEXT") is not None:
            self.stack.append(
                WdlStringText(self._maybe_token(ctx, "STRING_TEXT").getText())
            )
        elif self._maybe_token(ctx, "STRING_ESCAPE") is not None:
            self.stack.append(
                WdlStringEscape(self._maybe_token(ctx, "STRING_ESCAPE").getText())
            )
        elif self._maybe_token(ctx, "STRING_DOLLAR_SIGN") is not None:
            self.stack.append(
                WdlStringText(self._maybe_token(ctx, "STRING_DOLLAR_SIGN").getText())
            )
        elif self._maybe_token(ctx, "STRING_TILDE") is not None:
            self.stack.append(
                WdlStringText(self._maybe_token(ctx, "STRING_TILDE").getText())
            )
        return None

    def visitEnumMultilineString(self, ctx: Any) -> Any:
        return self.visitMultilineString(ctx)

    def visitEnumMultilineStringElement(self, ctx: Any) -> Any:
        for token_name in (
            "MULTILINE_STRING_TEXT",
            "MULTILINE_STRING_ESCAPE",
            "MULTILINE_STRING_DOUBLE_CLOSE_ANGLE",
            "MULTILINE_STRING_SINGLE_CLOSE_ANGLE",
            "MULTILINE_STRING_DOLLAR_SIGN",
            "MULTILINE_STRING_TILDE",
        ):
            token = self._maybe_token(ctx, token_name)
            if token is None:
                continue
            if token_name == "MULTILINE_STRING_ESCAPE":
                self.stack.append(WdlStringEscape(token.getText()))
            else:
                self.stack.append(WdlStringText(token.getText()))
            break
        return None

    def visitEnumArrayLiteral(self, ctx: Any) -> Any:
        arr = WdlArrayLiteral()
        self.stack.append(arr)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not arr:
            arr.entries().appendleft(self._pop_with_type(WdlExpression))
        return None

    def visitEnumMapLiteral(self, ctx: Any) -> Any:
        map_lit = WdlMapLiteral()
        self.stack.append(map_lit)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not map_lit:
            val = self._pop_with_type(WdlExpression)
            key = self._pop_with_type(WdlExpression)
            map_lit.entries().appendleft(WdlMapEntry(key, val))
        return None

    def visitEnumObjectLiteral(self, ctx: Any) -> Any:
        obj = WdlObjectLiteral()
        self.stack.append(obj)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not obj:
            obj.entries().appendleft(self._pop_with_type(WdlObjectEntry))
        return None

    def visitEnumObjectLiteralItem(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        entry = WdlObjectEntry(ids[0] if ids else None)
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        return None

    def visitEnumStructLiteral(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        self.stack.append(WdlStructLiteral(ids[0] if ids else None))
        self.visitChildren(ctx)
        return None

    def visitEnumStructLiteralItem(self, ctx: Any) -> Any:
        struct_lit = self._find_with_type(WdlStructLiteral)
        ids = self._strict_identifier_texts(ctx)
        entry = WdlStructEntry(ids[0] if ids else None)
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        self.stack.pop()
        struct_lit.entries().append(entry)
        return None

    def visitEnumPairLiteral(self, ctx: Any) -> Any:
        pair = WdlPairLiteral()
        self.stack.append(pair)
        self.visitChildren(ctx)
        pair.right = self._pop_with_type(WdlExpression)
        pair.left = self._pop_with_type(WdlExpression)
        return None

    def visitArrayLiteral(self, ctx: Any) -> Any:
        arr = WdlArrayLiteral()
        self.stack.append(arr)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not arr:
            arr.entries().appendleft(self._pop_with_type(WdlExpression))
        return None

    def visitMapLiteral(self, ctx: Any) -> Any:
        map_lit = WdlMapLiteral()
        self.stack.append(map_lit)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not map_lit:
            val = self._pop_with_type(WdlExpression)
            key = self._pop_with_type(WdlExpression)
            map_lit.entries().appendleft(WdlMapEntry(key, val))
        return None

    def visitStructLiteral(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        struct_lit = WdlStructLiteral(ids[0] if ids else None)
        self.stack.append(struct_lit)
        self.visitChildren(ctx)
        if ids:
            struct_lit.name = ids[0]
        return None

    def visitStructLiteralItem(self, ctx: Any) -> Any:
        struct_lit = self._find_with_type(WdlStructLiteral)
        entry = WdlStructEntry()
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        ids = self._strict_identifier_texts(ctx)
        if ids:
            entry.setKey(ids[0])
        self.stack.pop()
        struct_lit.entries().append(entry)
        return None

    def visitObjectLiteral(self, ctx: Any) -> Any:
        obj = WdlObjectLiteral()
        self.stack.append(obj)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not obj:
            obj.entries().appendleft(self._pop_with_type(WdlObjectEntry))
        return None

    def visitObjectLiteralItem(self, ctx: Any) -> Any:
        entry = WdlObjectEntry()
        ids = self._strict_identifier_texts(ctx)
        if ids:
            entry.setKey(ids[0])
        self.stack.append(entry)
        self.visitChildren(ctx)
        entry.setValue(self._pop_with_type(WdlExpression))
        return None

    def visitPairLiteral(self, ctx: Any) -> Any:
        pair = WdlPairLiteral()
        self.stack.append(pair)
        self.visitChildren(ctx)
        pair.right = self._pop_with_type(WdlExpression)
        pair.left = self._pop_with_type(WdlExpression)
        return None

    # Binary operators
    def visitLogicalOrExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators = [WdlBinaryOperator.LOGICAL_OR] * len(operator_symbols)
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitLogicalAndExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators = [WdlBinaryOperator.LOGICAL_AND] * len(operator_symbols)
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitEqualityExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators: list[WdlBinaryOperator] = []
        for symbol in operator_symbols:
            if symbol == "==":
                operators.append(WdlBinaryOperator.EQUAL)
            elif symbol == "!=":
                operators.append(WdlBinaryOperator.NOT_EQUAL)
            else:
                raise AssertionError(f"Unknown equality operator: {symbol}")
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitComparisonExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators: list[WdlBinaryOperator] = []
        for symbol in operator_symbols:
            if symbol == "<":
                operators.append(WdlBinaryOperator.LESS)
            elif symbol == "<=":
                operators.append(WdlBinaryOperator.LESS_EQUAL)
            elif symbol == ">":
                operators.append(WdlBinaryOperator.GREATER)
            elif symbol == ">=":
                operators.append(WdlBinaryOperator.GREATER_EQUAL)
            else:
                raise AssertionError(f"Unknown comparison operator: {symbol}")
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitAdditiveExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators: list[WdlBinaryOperator] = []
        for symbol in operator_symbols:
            if symbol == "+":
                operators.append(WdlBinaryOperator.ADD)
            elif symbol == "-":
                operators.append(WdlBinaryOperator.SUBTRACT)
            else:
                raise AssertionError(f"Unknown additive operator: {symbol}")
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitMultiplicativeExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operator_symbols = self._collect_binary_operator_symbols(ctx)
        if not operator_symbols:
            return None
        expressions = self._pop_expression_chain(len(operator_symbols) + 1)
        operators: list[WdlBinaryOperator] = []
        for symbol in operator_symbols:
            if symbol == "*":
                operators.append(WdlBinaryOperator.MULTIPLY)
            elif symbol == "/":
                operators.append(WdlBinaryOperator.DIVIDE)
            elif symbol == "%":
                operators.append(WdlBinaryOperator.MODULUS)
            else:
                raise AssertionError(f"Unknown multiplicative operator: {symbol}")
        self.stack.append(self._fold_binary_operations(expressions, operators))
        return None

    def visitPowerExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        if self._maybe_token(ctx, "EXPONENTIATION") is None:
            raise AssertionError("Unknown power operator")
        right = self._pop_with_type(WdlExpression)
        left = self._pop_with_type(WdlExpression)
        self.stack.append(WdlBinaryOperation(left, WdlBinaryOperator.POWER, right))
        return None

    # Unary / postfix / primary
    def visitUnaryExprOperation(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        operand = self._pop_with_type(WdlExpression)
        if self._maybe_token(ctx, "MINUS") is not None:
            op = WdlUnaryOperator.MINUS
        elif self._maybe_token(ctx, "EXCLAMATION") is not None:
            op = WdlUnaryOperator.NOT
        else:
            raise AssertionError("Unknown unary operator")
        self.stack.append(WdlUnaryOperation(op, operand))
        return None

    def visitPostfixExprArrayIndex(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        index_expr = self._pop_with_type(WdlExpression)
        target_expr = self._pop_with_type(WdlExpression)
        self.stack.append(WdlIndexAccessOperation(target_expr, index_expr))
        return None

    def visitPostfixExprField(self, ctx: Any) -> Any:
        self.visitChildren(ctx)
        target_expr = self._pop_with_type(WdlExpression)
        ids = self._strict_identifier_texts(ctx)
        self.stack.append(
            WdlMemberAccessOperation(target_expr, ids[0] if ids else None)
        )
        return None

    def visitVariable(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        self.stack.append(WdlVariable(ids[0] if ids else None))
        return None

    def visitCallExpression(self, ctx: Any) -> Any:
        ids = self._strict_identifier_texts(ctx)
        expr = WdlFunctionCallOperation(ids[0] if ids else None)
        self.stack.append(expr)
        self.visitChildren(ctx)
        while self.stack and self.stack[-1] is not expr:
            expr.arguments().appendleft(self._pop_with_type(WdlExpression))
        return None

    def visitIfExpression(self, ctx: Any) -> Any:
        expr = WdlTernaryOperation()
        self.stack.append(expr)
        self.visitChildren(ctx)
        expr.falseValue = self._pop_with_type(WdlExpression)
        expr.trueValue = self._pop_with_type(WdlExpression)
        expr.condition = self._pop_with_type(WdlExpression)
        return None
